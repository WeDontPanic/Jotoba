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
var screenWidth = $(window).width();

$(window).resize(() => {
    // Mobile scrolling sends resize events because of the (dis-)appearing url input. Simple fix: ignore height changes.
    if ($(window).width() == screenWidth) {
        return;
    }

    screenWidth = $(window).width();
    hideUnusedExpanders();
    centerSentenceReaderIfNeeded();
});

// If the reader is overflown, remove the center to avoid weird style errors
function centerSentenceReaderIfNeeded() {
    if (sr === undefined || sr === null)
        return;
        
    if (Util.checkOverflow(sr)) {
        sr.parentElement.classList.add("no-center");
    } else {
        sr.parentElement.classList.remove("no-center");
    }
}

// Scrolls the sentence reader onto the selected element
Util.awaitDocumentReady(scrollSentenceReaderIntoView);
function scrollSentenceReaderIntoView() {
    let selected = $(".sentence-part.selected")[0];
    if (selected !== undefined) {
        $(".search-annotation").scrollLeft(selected.offsetLeft - $(".search-annotation")[0].offsetLeft);
        $(".search-annotation").scrollTop(selected.offsetTop - $(".search-annotation")[0].offsetTop);
    }
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