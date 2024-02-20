# LTL ACTUS

The [algorithmic contract types unified standard](https://actusfrf.org) executed in [linear temporal logic](https://en.wikipedia.org/wiki/Linear_temporal_logic)

## Example (WIP): Execution trace of a loan

No dependencies between tests, but the log file will be written correctly if the runner is single threaded.

```sh
nix develop ./../#rs
$ RUST_LOG=debug cargo test -- --test-threads=1
$ cat execution_trace.log
```

This is intended to be a **demo** in a **toy version of ACTUS**
