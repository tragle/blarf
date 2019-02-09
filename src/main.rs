use pulldown_cmark::{html, Parser};
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::BufWriter;

struct Article {
    markdown: String,
    slug: String,
}

const DEST_ROOT: &str = "tmp";
const SOURCE_ROOT: &str = "source";

fn create_tmp() -> std::io::Result<()> {
    let _ = fs::remove_dir_all(DEST_ROOT);
    fs::create_dir(DEST_ROOT).expect(&format!("Cannot create tmp dir at {}", DEST_ROOT));
    let articles_folder = format!("{}/articles", DEST_ROOT);
    fs::create_dir(&articles_folder).expect(&format!("Cannot create dir {}", &articles_folder));
    Ok(())
}

fn copy_public() -> std::io::Result<()> {
    let public_folder = format!("{}/public", DEST_ROOT);
    let img_folder = format!("{}/img", public_folder);
    fs::create_dir(&public_folder).expect(&format!("Cannot create dir {}", &public_folder));
    fs::create_dir(&img_folder).expect(&format!("Cannot create dir {}", &img_folder));
    fs::copy(
        format!("{}/public/styles.css", SOURCE_ROOT),
        format!("{}/public/styles.css", DEST_ROOT),
    )
    .expect("Cannot copy styles.css");
    let source_img_folder = format!("{}/public/img", SOURCE_ROOT);
    for source_file in
        fs::read_dir(&source_img_folder).expect(&format!("Cannot read dir {}", &source_img_folder))
    {
        if let Ok(f) = source_file {
            let filetype = f.file_type().expect("Cannot read filetype");
            if filetype.is_file() {
                fs::copy(
                    f.path().as_os_str(),
                    format!(
                        "{}/public/img/{}",
                        DEST_ROOT,
                        f.file_name().to_str().unwrap()
                    ),
                )?;
            }
        }
    }
    Ok(())
}

fn parse_article_markdown(article: &Article) -> String {
    let parser = Parser::new(&article.markdown);
    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);
    html_buf
}

fn render_article(article: &str, footer: &str) -> String {
    format!(
        r#"
        <!doctype html>
        <html>
            <head>
                <title>ragle.io</title>
                <link rel="stylesheet" href="/public/styles.css">
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

fn render_footer(prev: Option<&String>, next: Option<&String>) -> String {
    let prev_str = match prev {
        Some(val) => format!("<a href=\"/{}.html\">previous</a>", val),
        None => String::new(),
    };
    let next_str = match next {
        Some(val) => format!("<a href=\"/{}.html\">next</a>", val),
        None => String::new(),
    };
    format!(
        r#"
    <footer>
        {}
        <a id="contact" href="mailto:tragle@gmail.com">contact</a>
        <a id="home" href="/">home</a>
        {}
    </footer>
    "#,
        prev_str, next_str
    )
}

fn main() -> std::io::Result<()> {
    let mut articles: Vec<Article> = vec![];
    create_tmp()?;
    copy_public()?;
    let articles_dir = format!("{}/articles", SOURCE_ROOT);
    for entry in fs::read_dir(&articles_dir).expect(&format!("Cannot read dir {}", articles_dir)) {
        let entry = &entry?;
        let path = entry.path();
        let slug = path.file_stem().unwrap().to_str().unwrap().to_owned();
        let mut file = File::open(&path).unwrap();
        let mut markdown = String::new();
        file.read_to_string(&mut markdown)
            .expect("Cannot read article file");
        articles.push(Article { markdown, slug });
    }

    articles.reverse();
    let first = 0;
    let last = articles.len() - 1;

    for i in first..=last {
        let article = &articles[i];
        let file_name = format!("{}/articles/{}.html", DEST_ROOT, article.slug);
        let file =
            File::create(&file_name).expect(&format!("Cannot create article file {}", &file_name));
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
        let mut writer = BufWriter::new(&file);
        let html_buf = parse_article_markdown(&article);
        let footer = render_footer(prev_slug, next_slug);
        let html = render_article(&html_buf, &footer);
        writer
            .write_all(html.as_bytes())
            .expect(&format!("Cannot write article file at {}", &file_name));
        if i == last {
            let index_file_name = format!("{}/index.html", DEST_ROOT);
            let index_file = File::create(&index_file_name)
                .expect(&format!("Cannot write index file at {}", &index_file_name));
            let mut writer = BufWriter::new(&index_file);
            writer.write_all(html.as_bytes())?;
        }
    }

    Ok(())
}
