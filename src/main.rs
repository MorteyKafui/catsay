use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use std::io::{self, Read};

#[derive(Parser)]
struct Options {
    #[clap(default_value = "Meow!")]
    message: String,
    #[clap(short = 'd', long = "dead")]
    dead: bool,
    #[clap(short = 'f', long = "file")]
    catfile: Option<std::path::PathBuf>,
    #[clap(short = 'i', long = "stdin")]
    stdin: bool,
}

fn main() -> Result<()> {
    let options = Options::parse();
    let mut message = String::new();
    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
    } else {
        message = options.message;
    }

    let eye = if options.dead { "x" } else { "o" };

    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.")
    }

    match &options.catfile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path)
                .with_context(|| format!("Could not read file {:?}", path))?;
            let eye = format!("{}", eye.red().bold());
            let cat_picture = cat_template.replace("{eye}", &eye);
            println!("{}", message.bright_yellow().underline().on_purple());
            println!("{}", &cat_picture);
        }
        None => {
            println!("{}", message.bright_yellow().underline().on_purple());
            println!(" \\");
            println!(" \\");
            println!(" /\\_/\\");
            println!(" ( {eye} {eye} )", eye = eye.red().bold());
            println!(" =( I )=");
        }
    }

    Ok(())
}
