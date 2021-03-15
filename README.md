# Regex Stream Split

regex-stream-split is an executable that forwards stdin to either stdout or stderr based on regular
expressions.

It is intended to be used by piping another executable through it. This can be useful with an
executable that outputs log messages to stdout while having an alerting system that reacts to stderr
output.

To run it pass two regexes as command line arguments. Every line from stdin is matched against the
regexes. If the first matches then the line is written to stdout and if the second then to stderr.
If the line matches neither then it goes to the most recently used stream which makes this
work for multi line log messages.

# Example

You have an executable `ex` that outputs the following log messages to stdout:

```
0s INFO message one
1s TRACE message two
second line
2s ERROR message three
second line
3s INFO message four
```

You run it piped through regex-stream-split:

```sh
ex | regex-stream-split "^[0-9]+s (INFO|TRACE)" "^[0-9]+s (WARN|ERROR)"
```

stdout:

```
0s INFO message one
1s TRACE message two
second line
3s INFO message four
```

stderr:

```
2s ERROR message three
second line
```

# Building

Written in [Rust](https://www.rust-lang.org/) so build it with `cargo build --release`.
