/**
 * This JS-File implements the Kanji Animation and compound dropdown feature
 */

const svgColorScheme = [
    ["rgb(0, 0, 0)", "rgb(153, 153, 153)", "rgb(221, 221, 221)", "rgb(52, 168, 60)", "0.7"], // light
    ["rgb(211, 207, 201)", "rgb(105, 105, 105)","rgb(92, 92, 92)", "rgb(95, 241, 96)", "0.75"]  // dark
]

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
var sliders = $('.slidecontainer > .speedSlider:not(.settings)');

// Restart Animation by clicking on Kanji
kanjis.click(function(e) {
    e.preventDefault();
    restartAnimation(e.target, this.slider.value);
});

// Tell every kanji their slider and initially start them
kanjis.each(function() {
    this.slider = $(this).parent().parent().find('.slider')[0];
    restartAnimation(this, localStorage.getItem("kanji_speed"));
});

// Tell every slider their kanji, text field and intial speed
sliders.each(function() {
    this.textField = $(this).parent().parent().find('span')[0];
    this.kanjisvg = $(this).parent().parent().parent().children('.kanjisvgParent').children()[0];
    let speed = localStorage.getItem("kanji_speed") || 1;
    this.value = speed;
    this.textField.innerHTML = "Animation speed: "+ speed;
});

// Adjust svg's draw speed using the slider
sliders.on('input', function() {
    this.textField.innerHTML = "Animation speed: "+this.value;
    restartAnimation(this.kanjisvg, this.value);
});

// Toggles compounds visible / hidden
function toggleCompounds(event) {
    let compoundParent = event.target.parentElement.parentElement; 
    compoundParent.children[compoundParent.children.length-1].classList.toggle("hidden");
    event.target.parentElement.children[0].classList.toggle("closed");
}

// Changes the Stroke-SVG colors according to the given color scheme. args = light / dark
function changeStrokeSvgColorScheme(scheme) {
    let index = scheme == "light" ? 0 : 1;
    let indexInv = scheme == "light" ? 1 : 0;
    let obj = $('object');
    if (obj.length == 0)
        return;
    let contentDoc = $(obj[0].contentDocument);

    contentDoc.find("path").filter(function() {
        return $(this).css('stroke') == svgColorScheme[indexInv][0];
    }).css("stroke", svgColorScheme[index][0]);
    contentDoc.find("path").filter(function() {
        return $(this).css('stroke') == svgColorScheme[indexInv][1];
    }).css("stroke", svgColorScheme[index][1]);
    contentDoc.find("line").css("stroke", svgColorScheme[index][2]);
    contentDoc.find("circle").css("fill", svgColorScheme[index][3]);
    contentDoc.find("circle").css("opacity", svgColorScheme[index][4]);
}

// When document is ready, initially set the color scheme
Util.awaitDocumentReady(() => {
    changeStrokeSvgColorScheme(localStorage.getItem('theme'));
});

// Catch theme changes and apply to svg
document.addEventListener('theme-changed', function (e) {
    changeStrokeSvgColorScheme(localStorage.getItem('theme'));
},false);

// Toggle all compounds on keypress
$(document).on("keypress", (event) => {
    if ($('input:text').is(":focus")) return;
    
    if (event.key == "c") {
        $(".compounds-dropdown").toggleClass("closed");
        $(".compounds-parent").toggleClass("hidden");
    }
});