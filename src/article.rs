use pulldown_cmark::{html, Parser};

/// Data container for a single article
pub struct Article {
    /// Raw markdown of article
    pub markdown: String,
    /// Short name of the article, appropriate for generating hyperlinks
    ///
    /// ### Example:
    /// `1-my-first-post`
    pub slug: String,
    /// Title of the article, appropriate for display
    ///
    /// ### Example:
    /// `My First Post`
    pub title: String,
}

impl Article {
    /// Constructor for `Article`
    ///
    /// Note that this will attempt to write the `title` field of the `Article` using
    /// `Article::get_title`, defaulting to an empty string if none is found.
    ///
    /// # Example
    /// ```
    /// let markdown = String::from("# My first post\nHello world!");
    /// let slug = "1-my-first-post"
    /// let article = Article::new(markdown, slug);
    /// ```
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

    /// Render the main template to html
    ///
    /// # Example
    /// ```
    /// let html = article.render("styles.css", "<p>copyright 2019</p>");
    /// ```
    pub fn render(&self, css: &str, footer: &str) -> String {
        let article = &self.as_html();
        let title = &self.title;
        format!(
            r#"
            <!doctype html>
            <html>
                <head>
                    <title>{}</title>
                    <link rel="stylesheet" href="/{}">
                    <meta charset="utf-8">
                    <meta name="viewport" content="user-scalable=no, width=device-width, initial-scale=1">
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
            title, css, article, footer
        )
    }

    /// Render a footer, with navigation controls based on position in list of articles
    pub fn render_footer(index: usize, articles: &[Article], email: Option<&str>) -> String {
        let (prev, next) = Article::get_slugs(index, articles);
        let links = Article::render_article_links(articles);
        let prev_str = match prev {
            Some(val) => format!("<a href=\"/articles/{}.html\">&larr;</a>", val),
            None => String::from("<span class=\"disabled\">&larr;</span>"),
        };
        let next_str = match next {
            Some(val) => format!("<a href=\"/articles/{}.html\">&rarr;</a>", val),
            None => String::from("<span class=\"disabled\">&rarr;</span>"),
        };
        let email_str = match email {
            Some(e) => format!(
                r#"
                        <div class="contact">
                            <a id="contact" href="mailto:{}">&#9993;</a>
                        </div>
            "#,
                e
            ),
            None => String::from(""),
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
            {}
        </footer>
        "#,
            prev_str, links, next_str, email_str
        )
    }

    /// Parse title from markdown
    ///
    /// Returns text of first top-level heading (like `# My first post`), or `None`
    pub fn get_title(markdown: &str) -> Option<&str> {
        let pattern = "# ";
        let lines: Vec<&str> = markdown.split('\n').collect();
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

    fn get_slugs(i: usize, articles: &[Article]) -> (Option<&str>, Option<&str>) {
        let first = 0;
        let last = articles.len() - 1;
        let prev_slug: Option<&str> = if i > first {
            Some(&articles[i - 1].slug)
        } else {
            None
        };
        let next_slug: Option<&str> = if i < last {
            Some(&articles[i + 1].slug)
        } else {
            None
        };
        (prev_slug, next_slug)
    }

    fn render_article_links(articles: &[Article]) -> String {
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
}
