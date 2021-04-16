// Restarts the animation of the kanji strokes
function restartAnimation(delayMultiplier) {
    if (delayMultiplier == undefined)
        delayMultiplier = 1;

    // Element Delay and Speed
    let paths = document.getElementsByClassName("draw2");
    for (var i = 0; i < paths.length; i++) {
        var elm = paths[i];

        // Set Animation
        elm.style.animationDelay = (i * 0.8 * 1 / delayMultiplier) + "s";
        elm.style.animationDuration = (1 / ((i + 1) * delayMultiplier)) + "s";

        var newone = elm.cloneNode(true);
        elm.parentNode.replaceChild(newone, elm);
    }
}

// Clicking on the kanji svg will restart it's animation
var element = document.getElementById("kanjiSvg");
element.addEventListener("click", function(e) {
    e.preventDefault;
    restartAnimation();
}, false);

// Adjust svg's draw speed using the slider
var slider = document.getElementById("speedSlider");
slider.oninput = function() {;
    $('#currentAnimationSpeed').html("Animation speed: "+this.value);
    restartAnimation(this.value);
}