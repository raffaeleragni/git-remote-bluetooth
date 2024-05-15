use std::io::{BufRead, Write};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn run(mut reader: impl BufRead, mut writer: impl Write) -> Result<()> {
    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        let trl = line.trim();
        if trl.eq("capabilities") {
            writer.write_all("push\nfetch\n\n".as_bytes())?;
        }
        line.clear();
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
        assert_eq!(4, lines.len());
        assert_eq!("push", lines[0]);
        assert_eq!("fetch", lines[1]);
        assert_eq!("", lines[2]);
        assert_eq!("", lines[3]);
    }

    #[test]
    fn test_multicommand() {
        let mut input = "capabilities\ncapabilities\n".as_bytes();
        let mut output = Vec::new();
        run(&mut input, &mut output).unwrap();
        let output = std::str::from_utf8(&output).unwrap();
        let lines: Vec<&str> = output.split('\n').collect();
        assert_eq!(7, lines.len());
        assert_eq!("push", lines[0]);
        assert_eq!("fetch", lines[1]);
        assert_eq!("", lines[2]);
        assert_eq!("push", lines[3]);
        assert_eq!("fetch", lines[4]);
        assert_eq!("", lines[5]);
        assert_eq!("", lines[6]);
    }
}
