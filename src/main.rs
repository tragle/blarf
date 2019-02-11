mod article;
mod util;

use std::fs::{self, File};
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;

use article::Article;

const TMP_ROOT: &str = "tmp";
const DEST_ROOT: &str = "site";
const ARTICLES_ROOT: &str = "articles";
const CSS_FILE: &str = "styles.css";

fn write_site(dest_dir: &str) -> std::io::Result<()> {
    let _ = fs::remove_dir_all(dest_dir);
    fs::rename(TMP_ROOT, dest_dir)?;
    Ok(())
}

fn create_tmp() -> std::io::Result<()> {
    let _ = fs::remove_dir_all(TMP_ROOT);
    fs::create_dir(TMP_ROOT).expect(&format!("Cannot create tmp dir at {}", TMP_ROOT));
    let articles_folder = format!("{}/articles", TMP_ROOT);
    fs::create_dir(&articles_folder).expect(&format!("Cannot create dir {}", &articles_folder));
    Ok(())
}

fn get_articles(articles_dir: &str) -> std::result::Result<Vec<Article>, std::io::Error> {
    let mut articles: Vec<Article> = vec![];
    for entry in fs::read_dir(&articles_dir).expect(&format!("Cannot read dir {}", articles_dir)) {
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
    let file = File::create(file_name).expect(&format!("Cannot create article file {}", file_name));
    let mut writer = BufWriter::new(&file);
    writer
        .write_all(html.as_bytes())
        .expect(&format!("Cannot write article file at {}", file_name));
    Ok(())
}

fn write_articles(articles: &Vec<Article>, email: Option<&str>, css: &str) -> std::io::Result<()> {
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

fn copy_files(css_path: &Path, static_dir: Option<&Path>) -> std::io::Result<()> {
    if let Some(static_dir) = static_dir {
        util::copy_dir(static_dir, Path::new(TMP_ROOT)).expect("Could not copy static directory");
    }

    if let Some(css_name) = css_path.file_name() {
        fs::copy(css_path, Path::new(TMP_ROOT).join(css_name)).expect("Could not copy css");
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args = util::get_args();
    let articles_dir = args.value_of("articles").unwrap_or(ARTICLES_ROOT);
    let static_dir: Option<&str> = Some(args.value_of("static")).unwrap_or(None);
    let destination = args.value_of("destination").unwrap_or(DEST_ROOT);
    let email: Option<&str> = Some(args.value_of("email")).unwrap_or(None);
    let css = args.value_of("css").unwrap_or(CSS_FILE);
    let css_path = Path::new(css);
    let static_path: Option<&Path> = match static_dir {
        Some(dir) => Some(Path::new(dir)),
        None => None,
    };

    let mut articles = get_articles(articles_dir)?;
    articles.reverse();

    create_tmp()?;
    write_articles(&articles, email, css_path.to_str().unwrap())?;
    copy_files(css_path, static_path)?;
    write_site(destination).expect(&format!("Could not write {}", destination));

    println!("blarfed {}", destination);

    Ok(())
}
