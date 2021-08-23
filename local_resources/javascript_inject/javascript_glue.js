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
        let inline_math = document.body.getElementsByClassName("hidden-inline-math");
        let display_math = document.body.getElementsByClassName("hidden-display-math");
        for (let i = 0; i < inline_math.length; i++) {
            let math_element = inline_math[i].nextElementSibling;
            if (math_element === null || math_element.className !== "inline-math") {
                math_element = document.createElement("span");
                math_element.className = "inline-math";
                inline_math[i].parentNode.insertBefore(math_element,inline_math[i].nextSibling);
            }
            if (inline_math[i].doRender === true) {
                katex.render(inline_math[i].textContent, math_element, {displayMode: false});
                inline_math[i].doRender = false;
            }
        }
        for (let i = 0; i < display_math.length; i++) {
            let math_element = display_math[i].nextElementSibling;
            if (math_element === null || math_element.className !== "display-math") {
                math_element = document.createElement("div");
                math_element.className = "display-math";
                display_math[i].parentNode.insertBefore(math_element,display_math[i].nextSibling);
            }
            if (display_math[i].doRender === true) {
                katex.render(display_math[i].textContent, math_element, {displayMode: true});
                display_math[i].doRender = false;
            }
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

function textChanged(content) {
    IncrementalDOM.currentElement().doRender = true;
    return content;
}

function display_math(content) {
    IncrementalDOM.elementOpen("div",null,null,"class","hidden-display-math","style","display: none;");
    IncrementalDOM.text(content,textChanged);
    IncrementalDOM.elementClose('div');
    if (IncrementalDOM.currentPointer().className === "display-math") {
        IncrementalDOM.skipNode();
    }
}
function inline_math(content) {
    IncrementalDOM.elementOpen("span",null,null,"class","hidden-inline-math","style","display: none;");
    IncrementalDOM.text(content,textChanged);
    IncrementalDOM.elementClose('span');
    if (IncrementalDOM.currentPointer() !== null && IncrementalDOM.currentPointer().className === "inline-math") {
        IncrementalDOM.skipNode();
    }
}

function on_ready() {
    external.invoke('ready');

    document.body.addEventListener("click", (event) => {
        external.invoke("open_file");
    })
}
disable_hyperlinks();
