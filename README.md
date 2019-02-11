# blarf

static site generator

## Installation

Requires Rust.

Option 1 -- install to ~/.cargo/.bin/

```
cargo install blarf
```

Option 2 -- install to /usr/local/bin

```
make
make install
```

## Instructions

blarf looks for a directory containing markdown articles, and optionally a stylesheet, email address, and a directory containing static assets. If no stylesheet is provided, blarf uses a default stylesheet.

```
blog_src/
|_ styles.css
|_ articles/
| |_ 1-my_first_post.md
| |_ 2-neat_stuff.md
|_ public/
| |_ img/
| | |_ face.png
| | |_ vacation.png
| |_ other_stuff/
|   |_ cv.pdf
```

Run blarf to create the site.

```
blarf \
--email me@example.com \
--css blog_src/styles.css  \
--static blog_src/public \
--dest my_site
```

You will end up with an output directory like this:

```
my_site/
|_ index.html
|_ styles.css
|_ img/
| |_ face.png
| |_ vacation.png
|_ other_stuff/
| |_ cv.pdf
|_ articles/
| |_ 1-my_first_post.html
| |_ 2-neat_stuff.html
```

Serve and enjoy.

### Article structure

Articles should conform to a few of conventions for best results:

* Articles are listed in the order they appear in the articles directory, so you may want to prefix filenames with numbers, e.g. _1-my_first_post.md, 2-neat_stuff.md_ etc.
* Each markdown document should have a top-level heading somewhere (with a single hash mark, e.g. `# My first post!`). This becomes the article's title in the page's HTML and in the list of historical links in the footer.
* If you are using the default styling, you can use the following:
  - 4-level heading displays a subtitle, like a date, e.g. `#### November 24, 2018`
  - 5-level heading displays a specially-formatted signature: e.g. `##### TR`
  - blockquotes with an author get special formatting, e.g.
    ```
    <blockquote>
      <p>All things must pass.</p>
      <p class="author">George Harrison</p>
    </blockquote>
    ```
  - images and captions within figures get special formatting, .e.g.
    ```    
    <figure>
      <img src="/img/Sint_Servaasbrug.jpg" alt="Sint Servaasbrug" />
      <figcaption>Sint Servaasbrug</figcaption>
    </figure>
    ```
