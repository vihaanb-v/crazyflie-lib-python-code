# cflib: Crazyflie python library [![CI](https://github.com/bitcraze/crazyflie-lib-python/workflows/CI/badge.svg)](https://github.com/bitcraze/crazyflie-lib-python/actions)

cflib is an API written in Python that is used to communicate with the Crazyflie
and Crazyflie 2.0 quadcopters. It is intended to be used by client software to
communicate with and control a Crazyflie quadcopter. For instance the [Crazyflie PC client](https://www.github.com/bitcraze/crazyflie-clients-python)  uses the cflib.

See [below](#platform-notes) for platform specific instruction.
For more info see our [documentation](https://www.bitcraze.io/documentation/repository/crazyflie-lib-python/master/).

## Installation
See the [installation instructions](docs/installation/install.md) in the github docs folder.

## Official Documentation

Check out the [Bitcraze crazyflie-lib-python documentation](https://www.bitcraze.io/documentation/repository/crazyflie-lib-python/master/) on our website.

## Contribute
Go to the [contribute page](https://www.bitcraze.io/contribute/) on our website to learn more.

### Test code for contribution
Run the automated build locally to test your code

	python3 tools/build/build

#RTLola Addition

For all commands we assume that the current working directory is set to the directory containing this readme:
```
$ ls
c_compiler  compiler_lib  rtlola-frontend  Cargo.toml  README.md
```

## Compiling `rtlola2c`

To build the compiler, we require a Rust installation. Consider https://www.rust-lang.org/tools/install for instructions.

Compile the RTLola compiler (and ignore any warnings for now) using:
```
$ cargo build --release
```

The compiled binary can be then found in `./target/release/rtlola2c`.

## Running `rtlola2c`

Consider an RTLola specification `spec.lola` in our working directory. Compile it to C using

```
target/release/rtlola2c spec.lola --out-dir monitor
```

which creates a directory `monitor` containing two files `monitor.c` and `monitor.h`.
Note that the compilation fails, if one of the output files already exists.
If [clang-format](https://clang.llvm.org/docs/ClangFormat.html) is installed, the generated files will be automatically formatted.
Otherwise, a warning will be displayed, but it can be safely ignored.

### Optimizations

If the resulting C code is to large for the embedded device, you can try to optimize the intermediate representation using the `--optimize` flag:

```
target/release/rtlola2c spec.lola --out-dir monitor --optimize
```

### Main

By default the resulting C code does not contain a `main`-function, but instead only the two API functions explained below.
To generate a simple main, invoke the compiler using `--main csv-offline`:

```
target/release/rtlola2c spec.lola --out-dir monitor --main csv-offline
```

### Verbosity

Using the `--verbosity` flag, you can define which streams are included in the verdict.
Use `--verbosity trigger` to only include the results from trigger evaluations, or `--verbosity outputs` to include all new values from output streams as well.

## API

The resulting binary contains two functions to interact with the monitor:

- `memory_init`: Initialize the memory before starting the monitoring, and
- `accept_event`: To send new inputs to the monitor and return the resulting verdict.

Consider the following RTLola specification:
```
input a : UInt64
trigger a > a.offset(by:-1).defaults(to:5)
```
The compiled monitor contains the struct
```c
typedef struct {
  bool has_a;
  uint64_t a;
} Event;
```
which represents a new input to the monitor.
Given that each input is optional, the `has_a` field tells the monitor whether this input contains a new value for the input `a`.

After giving an event to the monitor using `accept_event`, the monitor returns a verdict
```c
typedef struct {
  bool has_trigger_0;
  char *trigger_0;
  double time;
} Verdict;
```
which contains the information whether the trigger was evaluated in this cycle (`has_trigger_0`) as well as the corresponding trigger message.

Consider this very minimal example demonstrating the usage of the API:
```c
void main() {
	Memory memory;
	memory_init(&memory, 0);

	Event e; Verdict v;
	
	e = (Event){
		.has_a = true,
		.a = 1
	};
	v = accept_event(&memory, e, 0);
	assert(v.has_trigger_0 == false);

	e = (Event){
		.has_a = true,
		.a = 2
	};
	v = accept_event(&memory, e, 1);
	assert(v.has_trigger_0 == true);
}
```
The second argument of `memory_init` and the third argument of `accept_event` represents the time of the start/the time of the event in seconds.
