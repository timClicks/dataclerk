build:
    cargo build --release

fmt:
    rustfmt src/*.rs

clean:
    rm -rf target/release