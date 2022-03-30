use xshell::{Result, Shell, cmd};

fn main() -> Result<()> {
    let sh = Shell::new()?;
    let res = cmd!(sh, "ls /tmp2").quiet().output()?;
    dbg!(res);
    // println!("> {:?}", res);
    println!("done");
    Ok(())
}
