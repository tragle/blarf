# blarf

barfs blogs

## Quick start

### Installation

```
make
make install
```

### Use

Create a source directory with an articles directory containing markdown files, and a public directory with a styles.css and anything else needed by the site.

```
source_dir/
|_ articles/
| |_ 1-my_first_post.md
| |_ 2-neat_stuff.md
|_ public/
| |_ styles.css
| |_ img/
| | |_ face.png
| | |_ vacation.png
| |_ other_stuff/
|   |_ cv.pdf
```

Run blarf to create the site.

```
blarf --email me@example.com --src source_dir --dest my_site
```

You will end up with a directory like this:

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


