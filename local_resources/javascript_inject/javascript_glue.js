// Disable all hyperlinks, we want a display not a browser
function disable_hyperlinks() {
    window.addEventListener('click', function (e) {
      console.log(e.target);
      if (typeof e.target.tagName !== 'undefined' || e.target.tagName !== null && e.target.tagName === "A") {
          e.preventDefault();
      }
    })
}

function doIncrementalDom(dom_string) {
    IncrementalDOM.patch(document.body,function() {
        // Don't shoot me
        window.eval(dom_string);
    })

}
// Called every time load_page() is called in the rust code.
// Renders only equations that have changed, a new node will be created if any "html()" content changes, meaning we can detect when an equation has changed
// by looking at if our "math_was_rendered" variable is defined
function on_body_change() {
        let math = document.body.getElementsByClassName("inline-math");
        let display_math = document.body.getElementsByClassName("display-math");
        for (let i = 0; i < math.length; i++) {
            if (math[i].math_was_rendered === undefined || math[i].math_was_rendered === false) {
                math[i].style.visibility = "visible";
                katex.render(math[i].textContent, math[i], {displayMode: false});
            }
            math[i].math_was_rendered = true
        }
        for (let i = 0; i < display_math.length; i++) {
            if (display_math[i].math_was_rendered === undefined || display_math[i].math_was_rendered === false) {
                display_math[i].style.height = "auto";
                display_math[i].style.visibility = "visible";
                katex.render(display_math[i].textContent, display_math[i], {displayMode: true});
            }
            display_math[i].math_was_rendered = true;
        }


}
// Helper function for rendering html blobs with incremental dom
function html(content) {
    const el = IncrementalDOM.elementOpen('html-blob');
    if (el.__cachedInnerHtml !== content) {
        el.__cachedInnerHtml = content;
        el.innerHTML = content;
    }
    IncrementalDOM.skip();
    IncrementalDOM.elementClose('html-blob');
}


function on_ready() {
    external.invoke('ready');
}
disable_hyperlinks();
