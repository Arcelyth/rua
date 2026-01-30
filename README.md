# RUA

A simple CLI tool to convert images into ascii art and print them on the console.

## Install

You can build **Rua** from source code:
```bash
cargo build --release
cd target/debug
```

## Usage 

```bash
rua ./image.png 
```

![Snapshot](./snapshot1.png "Snapshot")

You can add `--width` to specify the output width and `--color` to output with color.

![SnapshotWithColor](./snapshot2.png "Snapshot with color")


