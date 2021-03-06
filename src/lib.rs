//! # blarf
//!
//! `blarf` is a static site generator. This crate provides both the binary and library used by the
//! binary.
//!
//! Run the binary to transform your markdown files and other assets into a static site.
//!
//! # Example
//!
//! Assuming a set of markdown files live in `articles/`, and a set of static assets (such as image
//! files) live in `public/`, you can generate a new set of files like so:
//!
//! ```
//! blarf -e me@example.com -a articles -s public
//! ```
//!
//! This will produce an output directory `site/`, which you may copy to your favorite web host and
//! serve.
//!
//! For a full set of options, run `blarf --help`.

pub mod article;
pub mod util;

use article::Article;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;

const TMP_ROOT: &str = "tmp";
const CSS_FILE: &str = "styles.css";
static CSS_STYLES: &'static str = include_str!("../static/styles.css");

/// Configuration values for the `exec` function
pub struct Config<'a> {
    /// directory holding markdown documents
    pub articles_dir: &'a Path,
    /// directory holding static assets to be copied
    pub static_dir: &'a Option<&'a Path>,
    /// output directory
    pub destination_dir: &'a Path,
    /// email address for contact link
    pub email: Option<&'a str>,
    /// stylesheet -- if none is supplied a default will be pulled in
    pub css_path: Option<&'a Path>,
}

/// Executes site creation.
///
/// Reads files from the locations provided in `Config`, transforms markdown to HTML, and writes
/// the results to disk at `Config.destination_dir` (overwriting if the directory exists).
pub fn exec(config: Config) -> std::io::Result<()> {
    create_tmp()?;

    let default_css_path = Path::new(CSS_FILE);
    let mut default_css_file = File::create(default_css_path)?;
    default_css_file.write_all(CSS_STYLES.as_bytes())?;

    let css_path = config.css_path.unwrap_or(default_css_path);
    let mut articles = get_articles(config.articles_dir)?;
    articles.sort_by(|a, b| a.slug.cmp(&b.slug));
    // articles.reverse();

    write_articles(&articles, config.email, css_path.to_str().unwrap())?;
    copy_files(css_path, config.static_dir)?;
    write_site(config.destination_dir)
        .unwrap_or_else(|_| panic!("Could not write {:?}", config.destination_dir));
    fs::remove_file(default_css_path)?;

    Ok(())
}

fn write_site(dest_dir: &Path) -> std::io::Result<()> {
    let _ = fs::remove_dir_all(dest_dir);
    fs::rename(TMP_ROOT, dest_dir)?;
    Ok(())
}

fn create_tmp() -> std::io::Result<()> {
    let _ = fs::remove_dir_all(TMP_ROOT);
    fs::create_dir(TMP_ROOT).unwrap_or_else(|_| panic!("Cannot create tmp dir at {}", TMP_ROOT));
    let articles_folder = format!("{}/articles", TMP_ROOT);
    fs::create_dir(&articles_folder)
        .unwrap_or_else(|_| panic!("Cannot create dir {}", &articles_folder));
    Ok(())
}

fn get_articles(articles_dir: &Path) -> std::result::Result<Vec<Article>, std::io::Error> {
    let mut articles: Vec<Article> = vec![];
    for entry in
        fs::read_dir(&articles_dir).unwrap_or_else(|_| panic!("Cannot read dir {:?}", articles_dir))
    {
        let entry = &entry?;
        let path = entry.path();
        let slug = path.file_stem().unwrap().to_str().unwrap().to_owned();
        let mut file = File::open(&path).unwrap();
        let mut markdown = String::new();
        file.read_to_string(&mut markdown)
            .expect("Cannot read article file");
        articles.push(Article::new(markdown, slug));
    }
    Ok(articles)
}

fn write_article(file_name: &str, html: &str) -> std::io::Result<()> {
    let file = File::create(file_name)
        .unwrap_or_else(|_| panic!("Cannot create article file {}", file_name));
    let mut writer = BufWriter::new(&file);
    writer
        .write_all(html.as_bytes())
        .unwrap_or_else(|_| panic!("Cannot write article file at {}", file_name));
    Ok(())
}

fn write_articles(articles: &[Article], email: Option<&str>, css: &str) -> std::io::Result<()> {
    let first = 0;
    let last = articles.len() - 1;

    for i in first..=last {
        let article = &articles[i];
        let footer = Article::render_footer(i, &articles, email);
        let html = &article.render(&css, &footer);
        write_article(
            &format!("{}/articles/{}.html", TMP_ROOT, article.slug),
            &html,
        )?;
        if i == last {
            write_article(&format!("{}/index.html", TMP_ROOT), &html)?;
        }
    }
    Ok(())
}

fn copy_files(css_path: &Path, static_dir: &Option<&Path>) -> std::io::Result<()> {
    if let Some(static_dir) = static_dir {
        util::copy_dir(static_dir, Path::new(TMP_ROOT)).expect("Could not copy static directory");
    }

    if let Some(css_name) = css_path.file_name() {
        fs::copy(css_path, Path::new(TMP_ROOT).join(css_name)).expect("Could not copy css");
    }
    Ok(())
}
