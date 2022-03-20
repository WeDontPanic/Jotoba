/**
 * This JS-File implements the Kanji Animation and compound dropdown feature
 */

// Kanji settings
var kanjiSettings = [];

// Default kanji speed (only used on init)
let speed = localStorage.getItem("kanji_speed") || 1;

// Initially set the speed tags according to the settings
$(".speed-tag").each((i, e) => {
    e.children[1].innerHTML = (Math.round(Settings.display.kanjiAnimationSpeed.val * 100) + "%");
});

// Initially prepare svg-settings
$(".anim-container").each((i, e) => {
    // The Kanji
    let kanji = e.id.split("_")[0];

    // Specific settings
    kanjiSettings[kanji] = {
        strokeCount: parseInt(e.dataset.strokes),
        speed: speed,
        timestamp: 0,
        index: 0,
        showNumbers: false,
    }

    // Needs the settings to be loaded first
    Util.awaitDocumentReady(() => {
        kanjiSettings[kanji].index = Settings.display.showKanjiOnLoad.val ? parseInt(e.dataset.strokes) : 0;
        kanjiSettings[kanji].showNumbers = Settings.display.showKanjiNumbers.val;

        // If the user wants to hide Kanji on load
        if (!Settings.display.showKanjiOnLoad.val) {
            $("#"+kanji+"_svg > svg path:not(.bg)").each((i, e) => {
                e.classList.add("hidden");
                e.style.strokeDashoffset = e.getTotalLength();
             });
        }

        // If user wants to hide numbers: hide them
        if (!Settings.display.showKanjiNumbers.val) {
            $(e).find("text").addClass("hidden");
        }
    });
});

// Adjust svg's draw speed using the slider
$('.speedSlider:not(.settings)').on('input', function () {
    kanjiSettings[this.dataset.kanji].speed = this.value;

    let ident = this.dataset.kanji + "_speed";
    let speed = Math.round((parseFloat(this.value) * 100));

    $("#" + ident).html(speed + "%");
    sessionStorage.setItem(ident, speed);
});

// Prepares the required steps to start auto-playing an animation
function prepareAutoplay(kanjiLiteral) {
    let startTime = Date.now();

    kanjiSettings[kanjiLiteral].timestamp = startTime;

    let playBtn = document.getElementById(kanjiLiteral + "_play");

    playBtn.dataset.state = "pause";
    playBtn.children[0].classList.add("hidden");
    playBtn.children[1].classList.remove("hidden");

    return startTime;
}

// Prepares the last steps to end auto-playing an animation
function concludeAutoplay(kanjiLiteral) {
    let playBtn = document.getElementById(kanjiLiteral+ "_play");

    playBtn.dataset.state = "play";
    playBtn.children[0].classList.remove("hidden");
    playBtn.children[1].classList.add("hidden");
}

// Based on the current state, show or pause the animation
async function doOrPauseAnimation(kanjiLiteral) {
    let playBtn = document.getElementById(kanjiLiteral+ "_play");

    if (playBtn.dataset.state === "play") {
        if (kanjiSettings[kanjiLiteral].index == kanjiSettings[kanjiLiteral].strokeCount) {
            await undoAnimation(kanjiLiteral);
        }
        
        console.log(kanjiSettings[kanjiLiteral].index);
        doAnimation(kanjiLiteral);
        return;
    }

    pauseAnimation(kanjiLiteral);
} 

// Automatically draws the whole image
async function doAnimation(kanjiLiteral) {
    let startTime = prepareAutoplay(kanjiLiteral);

    let svg = document.getElementById(kanjiLiteral + "_svg").firstElementChild;
    let paths = svg.querySelectorAll("path:not(.bg)");

    for (let index = kanjiSettings[kanjiLiteral].index; index < paths.length; index++) {
        if (startTime < kanjiSettings[kanjiLiteral].timestamp) {
            return;
        }

        kanjiSettings[kanjiLiteral].index++;
        await doAnimationStep(kanjiLiteral, paths[index], true);
        toggleNumbers(kanjiLiteral);

        if (startTime < kanjiSettings[kanjiLiteral].timestamp) {
            return;
        }
    }

    concludeAutoplay(kanjiLiteral);
}

// Automatically removes the whole image
async function undoAnimation(kanjiLiteral) {
    let startTime = prepareAutoplay(kanjiLiteral);

    let svg = document.getElementById(kanjiLiteral + "_svg").firstElementChild;
    let paths = svg.querySelectorAll("path:not(.bg)");

    for (kanjiSettings[kanjiLiteral].index > -1; kanjiSettings[kanjiLiteral].index--;) {
        if (startTime < kanjiSettings[kanjiLiteral].timestamp) {
            return;
        }

        await doAnimationStep(kanjiLiteral, paths[kanjiSettings[kanjiLiteral].index], false, kanjiSettings[kanjiLiteral].index > 0);
        toggleNumbers(kanjiLiteral);


        if (startTime < kanjiSettings[kanjiLiteral].timestamp) {
            return;
        }
    }

    kanjiSettings[kanjiLiteral].index = 0;
    concludeAutoplay(kanjiLiteral);
}

// Pauses the animation midway
async function pauseAnimation(kanjiLiteral) {
    kanjiSettings[kanjiLiteral].timestamp = Date.now();

    let playBtn = document.getElementById(kanjiLiteral+ "_play");

    playBtn.dataset.state = "play";
    playBtn.children[0].classList.remove("hidden");
    playBtn.children[1].classList.add("hidden");
}

// Draws or removes the given path
async function doAnimationStep(kanjiLiteral, path, forward, fastReset) {
    path.classList.remove("hidden");

    let len = path.getTotalLength();
    let drawTime = len * 10 * (!fastReset ? (1 / kanjiSettings[kanjiLiteral].speed) : 0.5);

    let transition = "transition: " + drawTime + "ms ease 0s, stroke " + (forward ? 0 : drawTime) + "ms ease 0s;";
    let dashArray = "stroke-dasharray: " + len + "," + len + ";";
    let strokeDashoffset = "stroke-dashoffset: " + (forward ? "0;" : (len + ";"));

    path.style = transition + dashArray + strokeDashoffset + (forward ? "" : "stroke: var(--danger);");

    return new Promise(resolve => setTimeout(resolve, !fastReset ? drawTime : 0));
}

// Draws or removes the given path based on the button clicked
async function doAnimationStep_onClick(kanjiLiteral, direction) {
    let startTime = Date.now();
    kanjiSettings[kanjiLiteral].timestamp = startTime;
    concludeAutoplay(kanjiLiteral);

    let svg = document.getElementById(kanjiLiteral + "_svg").firstElementChild;

    if (kanjiSettings[kanjiLiteral].index + direction == -1 || kanjiSettings[kanjiLiteral].index + direction > kanjiSettings[kanjiLiteral].strokeCount) {
        return;
    }

    let p = svg.querySelectorAll("path:not(.bg)")[direction > 0 ? kanjiSettings[kanjiLiteral].index : kanjiSettings[kanjiLiteral].index - 1];
    kanjiSettings[kanjiLiteral].index += direction;

    await doAnimationStep(kanjiLiteral, p, direction > 0);
    toggleNumbers(kanjiLiteral);
}

// Sets the SVG numbers visible / invisible or updates them if the param was not provided
function toggleNumbers(kanjiLiteral, visible) {
    let svg = document.getElementById(kanjiLiteral + "_svg").firstElementChild;
    let texts = svg.querySelectorAll("text");

    if (visible !== undefined && !Settings.display.showKanjiNumbers.val) {
        kanjiSettings[kanjiLiteral].showNumbers = visible;
    }

    if (kanjiSettings[kanjiLiteral].showNumbers) {
        for (let i = 0; i < texts.length; i++) {
            if (i < kanjiSettings[kanjiLiteral].index) {
                texts[i].classList.remove("hidden");
            } else {
                texts[i].classList.add("hidden");
            }
        }
    } else {
        for (let i = 0; i < texts.length; i++) {
            texts[i].classList.add("hidden");
        }
    }
}

// Toggles compounds visible / hidden
function toggleCompounds(event) {
    let compoundParent = event.target.parentElement.parentElement;
    compoundParent.children[compoundParent.children.length - 1].classList.toggle("hidden");
    event.target.parentElement.children[0].classList.toggle("closed");
}

// Toggle all compounds on keypress
$(document).on("keypress", (event) => {
    if ($('input:text').is(":focus")) return;

    if (event.key == "c") {
        $(".compounds-dropdown").toggleClass("closed");
        $(".compounds-parent").toggleClass("hidden");
    }
});