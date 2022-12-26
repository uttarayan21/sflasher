use clap::CommandFactory;
use clap_complete::{generate_to, shells::*};
use std::env;
use std::io::Error;

include!("src/cli.rs");
fn main() -> Result<(), Error> {
    println!("cargo:rerun-if-changed=src/cli.rs");

    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut cmd = Args::command();
    let path = generate_to(Bash, &mut cmd, "sflasher", &outdir)?;
    println!("cargo:warning=completion file is generated: {:?}", path);
    let path = generate_to(Fish, &mut cmd, "sflasher", &outdir)?;
    println!("cargo:warning=completion file is generated: {:?}", path);
    let path = generate_to(Zsh, &mut cmd, "sflasher", &outdir)?;
    println!("cargo:warning=completion file is generated: {:?}", path);

    Ok(())
}
