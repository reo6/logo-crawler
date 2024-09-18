## How to run

``` shell
nix-shell
cargo build --release
RUST_LOG="off" ./target/release/data-eng-interview < websites.csv
```

You can use ``samplecsv.py`` to generate shorter samples:

``` shell
mkdir sample-csv
python samplecsv.py 10
# Now use sample-csv/10.csv
```

> [!NOTE]
> I have all my notes at [``notes.md``](./notes.md).
