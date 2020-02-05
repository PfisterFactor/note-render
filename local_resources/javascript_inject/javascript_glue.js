// Disable all hyperlinks, we want a display not a browser
function disable_hyperlinks() {
    window.addEventListener('click', function (e) {
      console.log(e.target);
      if (typeof e.target.tagName !== 'undefined' || e.target.tagName !== null && e.target.tagName === "A") {
          e.preventDefault();
      }
    })
}


function doDiffDom(newhtml) {
    let parser = new DOMParser();
    let parsed_html = parser.parseFromString("<body id=\"content\" class=\"markdown-body\" onload=\"on_ready()\">"+newhtml+"</body>","text/html");
    let multi_line= parsed_html.getElementsByClassName("language-latex")
    for (var i = 0; i < multi_line.length; i++) {
        katex.render(multi_line[i].textContent,multi_line[i],{
            displayMode: true
        })
    }
    let inline = parsed_html.getElementsByClassName("inline-math")
    for (var i = 0; i < inline.length; i++) {
        katex.render(inline[i].textContent,inline[i],{
            displayMode: false
        });
    }
    let diff = diffD.diff(document.body,parsed_html.body);
    diffD.apply(document.body,diff);
}
// Called every time load_html() is called in the rust code.
function on_body_change() {
}

function on_ready() {
    window.diffD = new diffDOM.DiffDOM();
    external.invoke('ready');
}
disable_hyperlinks();
