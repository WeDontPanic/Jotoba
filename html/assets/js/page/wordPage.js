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