use pulldown_cmark::{html, Parser};
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;
extern crate clap;
use clap::{Arg, ArgMatches, App};

struct Article {
    markdown: String,
    slug: String,
    title: String,
}

impl Article {
    fn new(markdown: String, slug: String) -> Article {
        match Article::get_title(&markdown) {
            Some(title) => Article {
                markdown: markdown.clone(),
                slug,
                title: title.to_owned(),
            },
            None => Article {
                markdown,
                slug,
                title: String::from(""),
            },
        }
    }

    fn get_title(markdown: &str) -> Option<&str> {
        let pattern = "# ";
        let lines: Vec<&str> = markdown.split("\n").collect();
        for line in lines {
            if line.starts_with(&pattern) {
                let (_, title) = &line.split_at(pattern.len());
                return Some(title.trim());
            }
        }
        None
    }

    fn as_html(&self) -> String {
        let parser = Parser::new(&self.markdown);
        let mut html_buf = String::new();
        html::push_html(&mut html_buf, parser);
        html_buf
    }

    fn render(&self, footer: &str) -> String {
        let article = &self.as_html();
        format!(
            r#"
            <!doctype html>
            <html>
                <head>
                    <title>ragle.io</title>
                    <link rel="stylesheet" href="/styles.css">
                    <meta charset="utf-8">
                </head>
                <body>
                    <main>
                        <div class="article">
                            {}
                            {}
                        </div>
                    </main>
                </body>
            </html>
            "#,
            article, footer
        )
    }
}

const TMP_ROOT: &str = "tmp";
const DEST_ROOT: &str = "site";
const SOURCE_ROOT: &str = "source";

fn copy_dir(src: &Path, dest: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(src).expect(&format!("Cannot read dir for copy {:?}", src)) {
        if let Ok(f) = entry {
            let filetype = f.file_type()?;
            if filetype.is_dir() {
                let dir_name = dest.join(f.file_name());
                fs::create_dir(&dir_name)?;
                copy_dir(&f.path(), Path::new(&dir_name))?;
            } else {
                fs::copy(&f.path().as_os_str(), &dest.join(f.file_name()))
                    .expect(&format!("Cannot copy {:?} to {:?}", &f, &dest));
            }
        }
    }
    Ok(())
}

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

fn get_args<'a>() -> ArgMatches<'a> {
    App::new("blarf")
    .version("1.0")
    .author("Tom Ragle")
    .about("Generates a static blog")
    .arg(Arg::with_name("email")
         .short("e")
         .long("email")
         .help("Sets contact email address")
         .required(true)
         .takes_value(true))
    .arg(Arg::with_name("source")
         .short("s")
         .long("src")
         .help("Sets source directory")
         .takes_value(true))
    .arg(Arg::with_name("destination")
         .short("d")
         .long("dest")
         .help("Sets destination directory")
         .takes_value(true))
    .get_matches()
}

fn main() -> std::io::Result<()> {
    let args = get_args();
    let source = args.value_of("source").unwrap_or(SOURCE_ROOT);
    let destination = args.value_of("destination").unwrap_or(DEST_ROOT);
    let email = args.value_of("email").unwrap();

    let mut articles = get_articles(source)?;
    articles.reverse();

    create_tmp()?;
    write_articles(&articles, email)?;
    copy_dir(&Path::new(source).join("public"), Path::new(TMP_ROOT))?;
    write_site(destination)?;

    Ok(())
}
