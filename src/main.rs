use std::io::stdin;
use std::io::stdout;

use git_remote_bluetooth::run;
use git_remote_bluetooth::Result;

fn main() -> Result<()> {
    run(&mut stdin().lock(), &mut stdout())?;
    Ok(())
}
