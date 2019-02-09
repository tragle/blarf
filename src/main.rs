use pulldown_cmark::{html, Parser};
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::BufWriter;

struct Article {
    markdown: String,
    slug: String,
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
                <link rel="stylesheet" href="styles.css">
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
    for entry in fs::read_dir("articles")? {
        let entry = &entry?;
        let path = entry.path();
        let slug = path.file_stem().unwrap().to_str().unwrap().to_owned();
        let mut file = File::open(&path).unwrap();
        let mut markdown = String::new();
        file.read_to_string(&mut markdown)?;
        articles.push(Article { markdown, slug });
    }

    articles.reverse();
    const ROOT: &str = "blog";
    let first = 0;
    let last = articles.len() - 1;

    for i in first..=last {
        let article = &articles[i];
        let file = File::create(format!("{}/{}.html", ROOT, article.slug))?;
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
        writer.write_all(html.as_bytes())?;
        if i == last {
            let index_file = File::create(format!("{}/index.html", ROOT))?;
            let mut writer = BufWriter::new(&index_file);
            writer.write_all(html.as_bytes())?;
        }
    }

    Ok(())
}
