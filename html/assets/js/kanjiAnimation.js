/**
 * This JS-File implements the Kanji Animation
 */

// Restarts the animation of the kanji strokes
function restartAnimation(target, delayMultiplier) {
    if (delayMultiplier == undefined)
        delayMultiplier = 1;

    // Element Delay and Speed
    let paths = [...$(target).children('.draw2')];
    for (var i = 0; i < paths.length; i++) {
        var elm = paths[i];

        // Set Animation
        elm.style.animationDelay = (i * 0.8 * 1 / delayMultiplier) + "s";
        elm.style.animationDuration = ((1 / ((i + 1) * delayMultiplier))*10)+ "s";

        // Replace old svg with new one; restarting the animation
        var newone = elm.cloneNode(true);
        elm.parentNode.replaceChild(newone, elm);
    }
}

// Kanji and SVG list
var kanjis = $('.kanjisvg');
var sliders = $('.speedSlider');

// Restart Animation by clicking on Kanji
kanjis.click(function(e) {
    e.preventDefault();
    restartAnimation(e.target);
});

// Tell every slider their kanji and text field
sliders.each(function() {
    this.textField = $(this).parent().parent().find('span')[0];
    this.kanjisvg = $(this).parent().parent().parent().children('.kanjisvgParent').children()[0];
});

// Adjust svg's draw speed using the slider
sliders.on('input', function() {
    this.textField.innerHTML = "Animation speed: "+this.value;
    restartAnimation(this.kanjisvg, this.value);
});
