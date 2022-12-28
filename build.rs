use clap::CommandFactory;
use clap_complete::{generate_to, shells::*};
use std::env;
use std::io::Error;

fn main() -> Result<(), Error> {
    println!("cargo:rerun-if-changed=src/cli.rs");

    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    mod cli {
        #![allow(unused)]
        include!("src/cli.rs");
    }
    let mut cmd = cli::Args::command();
    let path = generate_to(Bash, &mut cmd, "sflasher", &outdir)?;
    println!("cargo:warning=completion file is generated: {:?}", path);
    let path = generate_to(Fish, &mut cmd, "sflasher", &outdir)?;
    println!("cargo:warning=completion file is generated: {:?}", path);
    let path = generate_to(Zsh, &mut cmd, "sflasher", &outdir)?;
    println!("cargo:warning=completion file is generated: {:?}", path);

    #[cfg(feature = "install")]
    {
        let path = generate_to(Bash, &mut cmd, "sflasher", "/etc/bash_completion.d")?;
        println!("cargo:warning=completion file is installed: {:?}", path);
        let path = generate_to(Fish, &mut cmd, "sflasher", "/etc/fish/completions")?;
        println!("cargo:warning=completion file is installed: {:?}", path);
        let path = generate_to(Zsh, &mut cmd, "sflasher", "/usr/share/zsh/site-functions")?;
        println!("cargo:warning=completion file is installed: {:?}", path);
    }

    Ok(())
}
