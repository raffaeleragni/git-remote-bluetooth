use std::io::{BufRead, Write};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn run(mut reader: impl BufRead, mut writer: impl Write) -> Result<()> {
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let line = line.trim();
    if line == "capabilities" {
        writer.write_all("push\nfetch\n".as_bytes())?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        let mut input = "\n".as_bytes();
        let mut output = Vec::new();
        run(&mut input, &mut output).unwrap();
        let output = std::str::from_utf8(&output).unwrap();
        let lines: Vec<&str> = output.split('\n').collect();
        assert_eq!(1, lines.len());
        assert_eq!("", lines[0]);
    }

    #[test]
    fn test_capabilities() {
        let mut input = "capabilities\n".as_bytes();
        let mut output = Vec::new();
        run(&mut input, &mut output).unwrap();
        let output = std::str::from_utf8(&output).unwrap();
        let lines: Vec<&str> = output.split('\n').collect();
        assert_eq!(3, lines.len());
        assert_eq!("push", lines[0]);
        assert_eq!("fetch", lines[1]);
        assert_eq!("", lines[2]);
    }
}
