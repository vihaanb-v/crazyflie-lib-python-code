pub(crate) mod constructs;
pub(crate) mod expressions;
mod guards;
pub(crate) mod io;
pub mod main_function;
mod memory;
mod names;
mod statements;
mod types;

use std::{collections::HashMap, path::PathBuf, sync::Mutex};

use constructs::StructDefinition;
use itertools::Itertools;
use main_function::MainFunction;
use memory::MemoryStruct;
use rtlola_streamir::{
    formatter::{
        files::{ConstructStore, ConstructWriteError, FilesFormatter},
        StreamIrFormatter,
    },
    ir::{expressions::Expr, memory::Memory, StreamIr, StreamReference},
};
use statements::CycleFunction;
use types::CType;

pub struct CFormatter {
    construct_store: ConstructStore<Self>,
    sr2memory: HashMap<StreamReference, Memory>,
    expr_counter: Mutex<HashMap<(Expr, Option<StreamReference>), usize>>,
    num_exprs: Mutex<usize>,
    overwrite: bool,
    main: MainFunction,
    static_strings: Mutex<HashMap<&'static str, usize>>,
    next_static_string: Mutex<usize>,
    verdict_streams: Vec<StreamReference>,
    output_dir: PathBuf,
}

impl CFormatter {
    pub fn new(
        ir: &StreamIr,
        overwrite: bool,
        main: MainFunction,
        verdict_streams: Vec<StreamReference>,
        output_dir: PathBuf,
    ) -> Self {
        Self {
            construct_store: ConstructStore::default(),
            sr2memory: ir.sr2memory.clone(),
            expr_counter: Mutex::default(),
            num_exprs: Mutex::new(0),
            overwrite,
            main,
            static_strings: Mutex::default(),
            next_static_string: Mutex::new(0),
            verdict_streams,
            output_dir,
        }
    }
}

impl StreamIrFormatter for CFormatter {
    type Return = Result<(), ConstructWriteError>;

    fn id(&self) -> String {
        "c-formatter".into()
    }

    fn format(self, ir: StreamIr) -> Self::Return {
        let StreamIr { stmt, .. } = ir;
        self.import_own(self.monitor_file(), "monitor");
        let _ = self.call_function(CycleFunction(stmt), &[MemoryStruct.argument_name(&self)]);
        // let _ = self.call_function(AcceptEventFunction, &[]);
        self.require_struct(MemoryStruct);
        self.main.insert_requirement(&self);
        self.generate_files()
    }
}

impl CFormatter {
    fn stream_ty(&self, sr: StreamReference) -> CType {
        CType::Lola(self.sr2memory[&sr].ty.clone())
    }

    fn streams(&self) -> impl Iterator<Item = StreamReference> + '_ {
        self.sr2memory.keys().sorted().copied()
    }

    fn inputs(&self) -> impl Iterator<Item = StreamReference> + '_ {
        self.sr2memory
            .keys()
            .filter(|o| matches!(o, StreamReference::In(_)))
            .sorted()
            .copied()
    }

    #[allow(dead_code)]
    fn outputs(&self) -> impl Iterator<Item = StreamReference> + '_ {
        self.sr2memory
            .keys()
            .filter(|o| matches!(o, StreamReference::Out(_)))
            .sorted()
            .copied()
    }
}
