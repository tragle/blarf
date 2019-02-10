# blarf

barfs blogs

## Quick start

### Installation

```
make
make install
```

### Use

Blarf looks for a stylesheet, a directory containing markdown articles, and a directory containing static assets.

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

