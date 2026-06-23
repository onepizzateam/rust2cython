# bio_sequence example

Demonstrates rust2cython on a bioinformatics utility library — the primary
target audience for this tool.

## generate bindings

```bash
cd ../..
cargo run -- examples/bio_sequence/src/lib.rs -o examples/bio_sequence/ -n bio_sequence
```

## build

```bash
cd examples/bio_sequence
cargo build --release
cp ../../target/release/libbio_sequence.so ./bio_sequence.so  # Linux
# cp ../../target/release/libbio_sequence.dylib ./bio_sequence.so  # macOS
pip install cython numpy
python setup.py build_ext --inplace
```

## run

```bash
python demo.py
```
