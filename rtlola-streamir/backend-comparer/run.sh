comparison_dir=$(dirname "$0")
cd $comparison_dir
rust_compiler_dir="../rtlola2rust"
interpreter_dir="../interpreter"

echo "Building rtlola2rust"
cargo build --manifest-path "$rust_compiler_dir/Cargo.toml" --release
cargo build --manifest-path "$interpreter_dir/Cargo.toml" --release

cargo run -- \
  -r rust-compiler -b "../target/release/rtlola2rust" \
  -r optimized-rust-compiler -b "../target/release/rtlola2rust" \
  -r new-interpreter -b "../target/release/rtlola-jit-interpreter" \
  $@
