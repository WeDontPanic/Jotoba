/**
 * This JS-File contains index-only scripts.
 */

// When opening an overlay, scroll it into view
function scrollOverlayIntoView(stopElementId) {
    var top = $(stopElementId).offset().top;
    Util.scrollTo(top, 500);
}