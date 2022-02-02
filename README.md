# note-render
A markdown/latex note rendering application in Rust

Call note-render from the command line with a markdown file and a window will open showing realtime live changes to the file as it is modified. 
I use this for taking notes in my classes in markdown.

It can also render latex (using KaTeX) if you put it within '$$' tags (inline) or ```latex (multiline).

# Building

Build using cargo. I can't remember what version I built it originally on, but I can verify it works on nightly 1.53.0.

Requires nightly, it could probably run on stable with a little change but I haven't gotten around to it.

```
$ cargo +nightly build
```

# Screenshots
![App Example](https://github.com/PfisterFactor/note-render/raw/master/appscreen.png)
