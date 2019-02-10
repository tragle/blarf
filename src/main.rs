mod util;
mod article;

use std::fs::{self, File};
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;

use article::Article;

const TMP_ROOT: &str = "tmp";
const DEST_ROOT: &str = "site";
const SOURCE_ROOT: &str = "source";

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

fn render_footer(prev: Option<&String>, next: Option<&String>, links: &String, email: &str) -> String {
    let prev_str = match prev {
        Some(val) => format!("<a href=\"/articles/{}.html\">&larr;</a>", val),
        None => String::from("<span class=\"disabled\">&larr;</span>"),
    };
    let next_str = match next {
        Some(val) => format!("<a href=\"/articles/{}.html\">&rarr;</a>", val),
        None => String::from("<span class=\"disabled\">&rarr;</span>"),
    };
    format!(
        r#"
    <footer>
        <div class="nav">
            <a href="/">&uarr;</a>
        </div>
        <div class="nav">
            {}
            <span class="article-list">
                {}
            </span>
            {}
        </div>
        <div class="contact">
            <a id="contact" href="mailto:{}">&#9993;</a>
        </div>
    </footer>
    "#,
        prev_str, links, next_str, email
    )
}

fn render_article_links(articles: &Vec<Article>) -> String {
    articles
        .iter()
        .rev()
        .map(|article| {
            let title = &article.title;
            let slug = &article.slug;
            format!(r#"<a href="/articles/{}.html">{}</a>"#, slug, title).to_owned()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn get_articles(source_dir: &str) -> std::result::Result<Vec<Article>, std::io::Error> {
    let mut articles: Vec<Article> = vec![];
    let articles_dir = format!("{}/articles", source_dir);
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

fn write_articles(articles: &Vec<Article>, email: &str) -> std::io::Result<()> {
    let links = render_article_links(&articles);

    let first = 0;
    let last = articles.len() - 1;

    for i in first..=last {
        let article = &articles[i];
        let prev_slug = if i > first {
            Some(&articles[i - 1].slug)
        } else {
            None
        };
        let next_slug = if i < last {
            Some(&articles[i + 1].slug)
        } else {
            None
        };
        let footer = render_footer(prev_slug, next_slug, &links, email);
        let html = &article.render(&footer);
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

fn main() -> std::io::Result<()> {
    let args = util::get_args();
    let source = args.value_of("source").unwrap_or(SOURCE_ROOT);
    let destination = args.value_of("destination").unwrap_or(DEST_ROOT);
    let email = args.value_of("email").unwrap();

    let mut articles = get_articles(source)?;
    articles.reverse();

    create_tmp()?;
    write_articles(&articles, email)?;
    util::copy_dir(&Path::new(source).join("public"), Path::new(TMP_ROOT))?;
    write_site(destination)?;

    Ok(())
}
