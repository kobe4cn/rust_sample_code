use anyhow::Result;
use boot_camp::Config;
use boot_camp::run;
use std::env;

fn main() -> Result<()> {
    //
    let args: Vec<String> = env::args().collect();
    let config = Config::new(args.into_iter())?;
    eprintln!("query: {}", config.query);
    println!("file_path: {}", config.file_path);
    println!("ignore_case: {}", config.ignore_case);

    run(config)?;
    Ok(())
}
