use pulldown_cmark::{html, Parser};

pub struct Article {
    pub markdown: String,
    pub slug: String,
    pub title: String,
}

impl Article {
    pub fn new(markdown: String, slug: String) -> Article {
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

    pub fn render(&self, footer: &str) -> String {
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


