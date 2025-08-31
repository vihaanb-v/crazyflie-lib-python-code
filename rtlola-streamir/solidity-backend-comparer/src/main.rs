use std::{
    collections::HashMap,
    fs::File,
    io::BufWriter,
    path::PathBuf,
    process::{Command, Stdio},
};

use anyhow::Context;
use clap::Parser;
use itertools::Itertools;
use rtlola_streamir::{
    ir::{StreamReference, Type},
    ParserConfig,
};
use rtlola_streamir_interpreter::{csv::CsvEventSource, verdict::Change, Monitor};
use std::io::Write;

#[derive(Parser)]
struct Args {
    spec: PathBuf,
    trace: PathBuf,
    #[clap(long)]
    output_streams: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let Args {
        spec,
        trace,
        output_streams: verdict_output_streams,
    } = Args::parse();

    let interface_path = PathBuf::from("test_environment/interface.toml");
    let test_case_path = PathBuf::from("test_environment/test/Contract.js");

    // parse specification to get input names
    let config = ParserConfig::from_path(spec.clone()).context("loading specification")?;
    let ir = rtlola_streamir::parse(&config).context("parsing to streamir")?;
    let all_inputs = ir
        .inputs()
        .sorted()
        .map(|i| ir.sr2memory[&StreamReference::In(i)].name.clone())
        .collect::<Vec<_>>();

    let outputs = if verdict_output_streams.is_empty() {
        ir.outputs()
            .sorted()
            .filter(|out| ir.sr2memory[&out.sr()].num_parameters() == 0)
            .collect::<Vec<_>>()
    } else {
        verdict_output_streams
            .into_iter()
            .map(|stream_name| {
                ir.sr2memory
                    .iter()
                    .find(|(_sr, m)| m.name == stream_name)
                    .expect("output stream not found")
                    .0
                    .out_idx()
            })
            .collect()
    };

    let mut interface =
        BufWriter::new(File::create(interface_path).context("creating interface file")?);

    let mut test_case =
        BufWriter::new(File::create(test_case_path).context("creating test case file")?);

    let mut function_names = HashMap::new();
    for inputs in all_inputs.iter().powerset() {
        if inputs.is_empty() {
            continue;
        }
        let name = format!("func_{}", inputs.iter().join("_"));

        writeln!(interface, "[[function]]\nname=\"{name}\"")?;
        for input in &inputs {
            writeln!(interface, "[[function.argument]]\nname=\"{input}\"")?;
        }
        writeln!(interface)?;

        let activation = all_inputs
            .iter()
            .map(|name| inputs.contains(&name))
            .collect::<Vec<_>>();

        function_names.insert(activation, name);
    }

    writeln!(
        test_case,
        "const {{ expect }} = require(\"chai\");
describe(\"MyTest\", function () {{"
    )?;

    writeln!(test_case, "it(\"should run\", async function () {{",)?;

    let mut reader = CsvEventSource::new(File::open(&trace).context("opening trace")?, &ir);

    let mut monitor = Monitor::build(ir.clone(), true);

    let mut values = outputs
        .iter()
        .map(|v| {
            match ir.sr2memory[&v.sr()].ty {
                Type::Int(_) | Type::UInt(_) => "0",
                Type::Bool => "false",
                _ => unimplemented!(),
            }
            .to_string()
        })
        .collect::<Vec<_>>();

    writeln!(
        test_case,
        "const contract = await ethers.deployContract(\"Contract\");"
    )?;

    while let Some((event, ts)) = reader.next_event()? {
        let activation = event.0.iter().map(|i| i.is_some()).collect::<Vec<_>>();
        if event.0.iter().all(|i| i.is_none()) {
            continue;
        }
        let function = &function_names[&activation];

        let input_values = event.0.iter().flatten().map(|s| s.to_string()).join(", ");

        let verdict = monitor.accept_event(event, ts);
        assert!(verdict.timed.is_empty());
        let verdict = verdict.event;

        for (output, changes) in verdict.outputs {
            let Some(i) = outputs
                .iter()
                .enumerate()
                .find_map(|(i, sr)| (*sr == output).then_some(i))
            else {
                continue; // parameterized stream
            };
            for change in changes {
                match change {
                    Change::Value(None, v) => {
                        values[i] = v.to_string();
                    }
                    Change::Value(Some(_), _) => unreachable!(),
                    _ => {}
                }
            }
        }

        writeln!(
            test_case,
            "await expect(contract.{}({}))",
            function, input_values
        )?;
        writeln!(test_case, ".to.emit(contract, \"Verdict\")")?;
        writeln!(test_case, ".withArgs({});", values.join(", "))?;
    }

    writeln!(test_case, "}});")?;
    writeln!(test_case, "}});")?;

    test_case.flush()?;
    interface.flush()?;

    for opt_level in ["partial-eval", "memory", "rewriting", "all"] {
        let mut cmd = Command::new("../target/release/rtlola2solidity");
        cmd.arg("--optimize")
            .arg("partial-eval")
            .arg("--output-file")
            .arg("test_environment/contracts/Contract.sol")
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .arg("--overwrite")
            .arg(&spec)
            .arg("test_environment/interface.toml");

        for output in &outputs {
            cmd.args(["--output-streams", &ir.sr2memory[&output.sr()].name]);
        }

        assert!(cmd
            .spawn()
            .context("running solidity compiler")?
            .wait()?
            .success());

        let res = Command::new("npx")
            .args(["hardhat", "test"])
            .current_dir("test_environment")
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .status()
            .context("error running hardhat")?;
        match res.success() {
            true => println!("PASS {} {} {}", spec.display(), trace.display(), opt_level),
            false => println!("FAIL {} {} {}", spec.display(), trace.display(), opt_level),
        }
    }

    Ok(())
}
