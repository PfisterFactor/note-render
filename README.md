# note-render
A markdown/latex note rendering application in Rust

Call note-render from the command line with a markdown file and a window will open showing realtime live changes to the file as it is modified. 
I used this for taking technical notes in my classes in markdown.

It can also render latex (using KaTeX) if you put it within '$$' tags (inline) or ```latex (multiline).

# Building

Build using cargo. I can't remember what version I built it originally on, but I can verify it works on stable 1.58.0.


```
$ cargo build
```

# Screenshots
![App Example](https://github.com/PfisterFactor/note-render/raw/master/appscreen.png)
