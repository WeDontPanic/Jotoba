// Toggles the given translation visible / invisible
function toggleTranslation(element) {
    let parent = $(element.parentElement);

    parent.find(".sentence-translation").toggle("hidden");
    parent.find(".lang-separator").toggle("hidden");
    parent.find(".sentence-toggle").toggleClass("hidden");
}