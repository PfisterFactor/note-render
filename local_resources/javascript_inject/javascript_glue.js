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
function on_body_change() {
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
