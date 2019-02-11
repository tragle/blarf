extern crate blarf;

use blarf::{Config, exec};

const ARTICLES_ROOT: &str = "articles";
const DEST_ROOT: &str = "site";

fn main() -> std::io::Result<()> {
    let args = blarf::util::get_args();
    let articles_dir = args.value_of("articles").unwrap_or(ARTICLES_ROOT);
    let static_dir: Option<&str> = Some(args.value_of("static")).unwrap_or(None);
    let destination_dir = args.value_of("destination").unwrap_or(DEST_ROOT);
    let email: Option<&str> = Some(args.value_of("email")).unwrap_or(None);
    let css_path = Some(args.value_of("css")).unwrap_or(None);

    let config = Config{
        articles_dir,
        static_dir,
        destination_dir,
        email,
        css_path,
    };

    exec(config)?;

    println!("blarfed {}", destination_dir);

    Ok(())
}
