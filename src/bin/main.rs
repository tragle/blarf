extern crate blarf;

use std::path::Path;
use blarf::{Config, exec};

const ARTICLES_ROOT: &str = "articles";
const DEST_ROOT: &str = "site";

fn main() -> std::io::Result<()> {
    let args = blarf::util::get_args();

    let articles_dir: &Path = &args.value_of("articles").map_or(Path::new(ARTICLES_ROOT), |d| Path::new(d));
    let static_dir: &Option<&Path> = &args.value_of("static").map(|d| Path::new(d));
    let destination_dir: &Path = &args.value_of("destination").map_or(Path::new(DEST_ROOT), |d| Path::new(d));;
    let email = args.value_of("email");
    let css_path = args.value_of("css").map(|p| Path::new(p));

    let config = Config{
        articles_dir,
        static_dir,
        destination_dir,
        email,
        css_path,
    };

    exec(config)?;

    println!("blarfed {:?}", destination_dir);

    Ok(())
}
