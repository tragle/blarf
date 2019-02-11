extern crate clap;
use clap::{App, Arg, ArgMatches};
use std::fs;
use std::path::Path;

/// Recursively copies contents of `src` directory to `dest` directory.
///
/// # Example
///
/// ```
/// use std::path::Path;
///
/// let from_dir = Path::new("source");
/// let to_dir = Path::new("my_site");
/// copy_dir(from_dir, to_dir)?;
/// ```
pub fn copy_dir(src: &Path, dest: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(src).unwrap_or_else(|_| panic!("Cannot read dir for copy {:?}", src))
    {
        if let Ok(f) = entry {
            let filetype = f.file_type()?;
            if filetype.is_dir() {
                let dir_name = dest.join(f.file_name());
                fs::create_dir(&dir_name)?;
                copy_dir(&f.path(), Path::new(&dir_name))?;
            } else {
                fs::copy(&f.path().as_os_str(), &dest.join(f.file_name()))
                    .unwrap_or_else(|_| panic!("Cannot copy {:?} to {:?}", &f, &dest));
            }
        }
    }
    Ok(())
}

/// Prompts for command-line arguments.
///
/// # Examples
///
/// ```
/// let args = get_args();
/// let articles_dir = args.value_of("articles").unwrap();
/// ```
pub fn get_args<'a>() -> ArgMatches<'a> {
    App::new("blarf")
        .version("1.0")
        .author("Tom Ragle")
        .about("Generates a static blog")
        .arg(
            Arg::with_name("articles")
                .short("a")
                .long("articles")
                .help("Sets article source directory")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("css")
                .short("c")
                .long("css")
                .help("Sets css file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("email")
                .short("e")
                .long("email")
                .help("Sets contact email address")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("static")
                .short("s")
                .long("static")
                .help("Sets static files source directory")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("destination")
                .short("d")
                .long("dest")
                .help("Sets destination directory")
                .takes_value(true),
        )
        .get_matches()
}
