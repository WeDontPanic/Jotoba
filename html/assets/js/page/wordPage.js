// Object reference for sentence reader
const sr = document.getElementById("sr");

// Enable sentence-example expander
$(".expander").on("click", (event) => {
    event.target.classList.toggle("on");
    event.target.parentElement.children[0].classList.toggle("collapsed");
});

// On first load and on every page resize: check where the expander-triangle is needed & whether sentence reader should be centered
hideUnusedExpanders();
centerSentenceReaderIfNeeded();
$(window).resize(() => {
    hideUnusedExpanders();
    centerSentenceReaderIfNeeded();
});

// If the reader is overflown, remove the center to avoid weird style errors
function centerSentenceReaderIfNeeded() {
    if (Util.checkOverflow(sr)) {
        sr.classList.remove("center");
    } else {
        sr.classList.add("center");
    }
}

// Scrolls the sentence reader onto the selected element
Util.awaitDocumentReady(scrollSentenceReaderIntoView);
function scrollSentenceReaderIntoView() {
    // Wait for document to be completly ready
    let docWait = window.setInterval(() => {
        if (document.readyState == "complete") {
            let selected = $(".sentence-part.selected")[0];
            if (selected !== undefined) {
                $(".search-annotation").scrollLeft(selected.offsetLeft - $(".search-annotation")[0].offsetLeft);
            }
            window.clearTimeout(docWait);
        }
    }, 10);
}

// Check if the expander-triangle should be hidden
function hideUnusedExpanders() {
    $(".expander").each((i,e) => {
        if (e.parentElement.children[0].scrollHeight < 40) {
            e.classList.add("hidden");
        } else {
            e.classList.remove("hidden");
        }
     });
}