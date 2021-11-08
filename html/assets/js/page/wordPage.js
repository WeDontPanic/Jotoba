// Enable sentence-example expander
$(".expander").on("click", (event) => {
    event.target.classList.toggle("on");
    event.target.parentElement.children[0].classList.toggle("collapsed");
});