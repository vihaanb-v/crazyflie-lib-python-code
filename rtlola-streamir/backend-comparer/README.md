# RTLola Backends-Comparison

Runs two different RtLola monitoring implementations on different specifications and traces and compares their output.

## Tests

The tests are given to the program as a json-file in the following form:

```json
[
	{
		"spec": "specs/simple_sync.lola",
		"traces": [
		"traces/simple.csv",
		"traces/two-ints.csv"
		]
	},
	...
]
```

## Runner
The `Runner` trait defines something that can produce logging information about the run of a trace on a RtLola monitor.

### Interpreter
Runs the trace and specification with the `rtlola-interpreter` in offline-mode.

The LEFT_BINARY (or RIGHT_BINARY) argument has to point to the `rtlola-cli` binary.

### CCompiler
Runs the trace on a monitor compiled for the given specification. 
The monitor was build by compiling the specification to C-Code by the `rtlola-compiler`, and then compiling the C-Code into a binary with `gcc`.

The LEFT_BINARY (or RIGHT_BINARY) argument has to point to the `c-language` binary.

### Reference Output
Simply returns the (correct) log information in a reference file.

The LEFT_BINARY (or RIGHT_BINARY) argument has to point to a directory with the following structure:

    <LEFT_BINARY>
	 ├─spec-name
	 │  ├─trace1.ref
	 │  ├─trace2.ref
	 │  └─...
	 ├─other-spec-name
	 │  └─...
	 └─...

Where the *.ref files contain the log output that the interpreter (or compiler) would return.