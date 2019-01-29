# looke

A rust port of a subset of node.js module [looks-same](https://github.com/gemini-testing/looks-same).

To determine if two images look the same to human color perception.

# Install

```
cargo install looke
```

# Usage

```
USAGE:
    looke [FLAGS] [OPTIONS] <ref-image> <image>

FLAGS:
    -h, --help         Prints help information
    -t, --tolerance    tolerance for image diff
    -V, --version      Prints version information

OPTIONS:
    -d, --diff-image <diff-image>    path for saving diff output image

ARGS:
    <ref-image>    path to reference image
    <image>        path to image
```

If they look not the same, exit with 1.
