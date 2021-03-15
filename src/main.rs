use anyhow::{ensure, Context, Result};
use regex::Regex;
use std::io::{self, BufRead, Write};

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    ensure!(
        args.len() == 2,
        "need exactly two arguments for stdout and stderr regex"
    );
    let stdout_regex = Regex::new(args.next().unwrap().as_str()).context("invalid stdout regex")?;
    let stderr_regex = Regex::new(args.next().unwrap().as_str()).context("invalid stderr regex")?;
    main_(
        io::stdin().lock(),
        io::stdout().lock(),
        io::stderr().lock(),
        &stdout_regex,
        &stderr_regex,
    )
}

enum Out {
    Stdout,
    Stderr,
}

fn main_(
    mut stdin: impl BufRead,
    mut stdout: impl Write,
    mut stderr: impl Write,
    stdout_regex: &Regex,
    stderr_regex: &Regex,
) -> Result<()> {
    let mut out = Out::Stdout;
    let mut line = String::new();
    while stdin
        .read_line(&mut line)
        .context("failed to read from stdin")?
        != 0
    {
        if stdout_regex.is_match(line.as_str()) {
            out = Out::Stdout;
        } else if stderr_regex.is_match(line.as_str()) {
            out = Out::Stderr;
        }
        match out {
            Out::Stdout => stdout
                .write_all(line.as_bytes())
                .context("failed to write to stdout")?,
            Out::Stderr => stderr
                .write_all(line.as_bytes())
                .context("failed to write to stderr")?,
        }
        line.clear();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test() {
        let stdout_regex =
            Regex::new(r"^[0-9]+-[0-9]+-[0-9]+T[0-9]+:[0-9]+:[0-9]+\.[0-9]+Z +(INFO|TRACE)")
                .unwrap();
        let stderr_regex =
            Regex::new(r"^[0-9]+-[0-9]+-[0-9]+T[0-9]+:[0-9]+:[0-9]+\.[0-9]+Z +ERROR").unwrap();
        let stdin = "\
            2021-03-12T10:36:48.485Z INFO 0\n\
            2021-03-12T10:36:48.485Z TRACE 1\n\
            2\n\
            2021-03-12T10:36:48.485Z ERROR 3\n\
            4\n\
            2021-03-12T10:36:48.485Z INFO 5\n\
        ";
        let mut stdout = Vec::<u8>::new();
        let mut stderr = Vec::<u8>::new();
        main_(
            Cursor::new(stdin),
            Cursor::new(&mut stdout),
            Cursor::new(&mut stderr),
            &stdout_regex,
            &stderr_regex,
        )
        .unwrap();
        let stdout = String::from_utf8(stdout).unwrap();
        let stderr = String::from_utf8(stderr).unwrap();
        let expected_stdout = "\
            2021-03-12T10:36:48.485Z INFO 0\n\
            2021-03-12T10:36:48.485Z TRACE 1\n\
            2\n\
            2021-03-12T10:36:48.485Z INFO 5\n\
        ";
        let expected_stderr = "\
            2021-03-12T10:36:48.485Z ERROR 3\n\
            4\n\
        ";
        assert_eq!(stdout, expected_stdout);
        assert_eq!(stderr, expected_stderr);
    }
}
