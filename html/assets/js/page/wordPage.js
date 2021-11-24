// Enable sentence-example expander
$(".expander").on("click", (event) => {
    event.target.classList.toggle("on");
    event.target.parentElement.children[0].classList.toggle("collapsed");
});

// On first load and on every page resize: check where the expander-triangle is needed
hideUnusedExpanders();
$(window).resize(() => {
    hideUnusedExpanders();
});

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