# blarf

barfs blogs

## Quick start

### Installation

Requires rust and make.

```
make
make install
```

### Use

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

