/**
 * This JS-File contains index-only scripts.
 */

// When opening an overlay, scroll it into view
function scrollSearchIntoView() {
    var top = $('#search').offset().top;
    Util.scrollTo(top, 500);
}