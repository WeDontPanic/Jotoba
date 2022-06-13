/**
 * This JS-File implements the Kanji Animation and compound dropdown features
 */

// Kanji settings
var kanjiSettings = [];
const Animation = { none: 0, forward: 1, backwards: 2 };

// Default kanji speed (only used on init)
let speed = localStorage.getItem("kanji_speed") || 1;

// Initially set the speed tags according to the settings
Util.awaitDocumentReady(() => {
    $(".speed-tag").each((i, e) => {
        e.children[1].innerHTML = (Math.round(Settings.display.kanjiAnimationSpeed.val * 100) + "%");
        e.nextElementSibling.value = Settings.display.kanjiAnimationSpeed.val;
    });
});

// Initially prepare svg-settings
$(".anim-container").each((i, e) => {
    // The Kanji
    let kanjiLiteral = e.id.split("_")[0];

    // Figure out how many paths there are
    let paths = getPaths(kanjiLiteral);

    // Specific settings
    kanjiSettings[kanjiLiteral] = {
        strokeCount: paths.length,
        speed: speed,
        timestamp: 0,
        index: 0,
        showNumbers: false,
        animationDirection: Animation.none,
        isAutomated: false,
    }

    // Needs the settings to be loaded first
    Util.awaitDocumentReady(() => {
        kanjiSettings[kanjiLiteral].index = Settings.display.showKanjiOnLoad.val ? paths.length : 0;
        kanjiSettings[kanjiLiteral].showNumbers = Settings.display.showKanjiNumbers.val;

        // If the user wants to hide Kanji on load
        if (!Settings.display.showKanjiOnLoad.val) {
            $("#" + kanjiLiteral + "_svg > svg path:not(.bg)").each((i, e) => {
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
    let kanjiLiteral = this.dataset.kanji;

    kanjiSettings[kanjiLiteral].speed = this.value;

    let ident = kanjiLiteral + "_speed";
    let speed = Math.round((parseFloat(this.value) * 100));

    $("#" + ident).html(speed + "%");
    sessionStorage.setItem(ident, speed);

    let playBtnState = document.getElementById(kanjiLiteral + "_play").dataset.state;

    if (kanjiSettings[kanjiLiteral].animationDirection !== Animation.none && playBtnState === "pause") {
        refreshAnimations(kanjiLiteral);
    }
});

// Returns the paths related to the kanji
function getPaths(kanjiLiteral) {
    let svg = document.getElementById(kanjiLiteral + "_svg").firstElementChild;
    return svg.querySelectorAll("path:not(.bg)");
}

// Refresh the currently running animation. Used for changing the current animation speed
async function refreshAnimations(kanjiLiteral) {
    let paths = getPaths(kanjiLiteral);
    let startTime = prepareAutoplay(kanjiLiteral);

    // Iterate all strokes that are potentially animating
    for (let i = 0; i < paths.length; i++) {
        let len = paths[i].getTotalLength();
        let currentLen = $(paths[i]).css("stroke-dashoffset");

        // Stroke is currently animating
        if (len !== currentLen && currentLen !== "0px") {
            // Reset current animation
            $(paths[i]).css("stroke-dashoffset", $(paths[i]).css("stroke-dashoffset"));

            // Animate and wait if the animations was automated
            let animationPromise = doAnimationStep(kanjiLiteral, paths[i], kanjiSettings[kanjiLiteral].animationDirection === Animation.forward, false);
            if (kanjiSettings[kanjiLiteral].isAutomated) {
                kanjiSettings[kanjiLiteral].index = i + 1;
                await animationPromise;

                if (startTime < kanjiSettings[kanjiLiteral].timestamp) {
                    return;
                }
            }

            toggleNumbers(kanjiLiteral);
        }
    }

    // Conclude potential autoplay
    if (kanjiSettings[kanjiLiteral].isAutomated) {
        concludeAutoplay(kanjiLiteral);
    }
}

// Prepares the required steps to start auto-playing an animation
function prepareAutoplay(kanjiLiteral) {
    let startTime = Date.now();

    kanjiSettings[kanjiLiteral].timestamp = startTime;
    kanjiSettings[kanjiLiteral].isAutomated = true;

    let playBtn = document.getElementById(kanjiLiteral + "_play");

    playBtn.dataset.state = "pause";
    playBtn.children[0].classList.add("hidden");
    playBtn.children[1].classList.remove("hidden");

    return startTime;
}

// Prepares the last steps to end auto-playing an animation
function concludeAutoplay(kanjiLiteral) {
    let playBtn = document.getElementById(kanjiLiteral + "_play");

    kanjiSettings[kanjiLiteral].isAutomated = false;

    playBtn.dataset.state = "play";
    playBtn.children[0].classList.remove("hidden");
    playBtn.children[1].classList.add("hidden");
}

// Based on the current state, show or pause the animation
async function doOrPauseAnimation(kanjiLiteral) {
    let playBtn = document.getElementById(kanjiLiteral + "_play");

    if (playBtn.dataset.state === "play") {
        if (kanjiSettings[kanjiLiteral].index == kanjiSettings[kanjiLiteral].strokeCount) {
            await undoAnimation(kanjiLiteral, true);
        }

        doAnimation(kanjiLiteral);
        return;
    }

    pauseAnimation(kanjiLiteral);
}

// Automatically draws the whole image
async function doAnimation(kanjiLiteral) {
    let startTime = prepareAutoplay(kanjiLiteral);

    let paths = getPaths(kanjiLiteral);

    for (let index = kanjiSettings[kanjiLiteral].index; index < paths.length; index++) {
        if (startTime < kanjiSettings[kanjiLiteral].timestamp) {
            return;
        }

        kanjiSettings[kanjiLiteral].index++;
        kanjiSettings[kanjiLiteral].animationDirection = Animation.forward;

        await doAnimationStep(kanjiLiteral, paths[index], true);

        if (startTime < kanjiSettings[kanjiLiteral].timestamp) {
            return;
        }

        toggleNumbers(kanjiLiteral);
        kanjiSettings[kanjiLiteral].animationDirection = Animation.none;
    }

    concludeAutoplay(kanjiLiteral);
}

// Automatically removes the whole image
async function undoAnimation(kanjiLiteral, awaitLast) {
    let startTime = prepareAutoplay(kanjiLiteral);

    let paths = getPaths(kanjiLiteral);

    for (kanjiSettings[kanjiLiteral].index > -1; kanjiSettings[kanjiLiteral].index--;) {
        if (startTime < kanjiSettings[kanjiLiteral].timestamp) {
            return;
        }

        kanjiSettings[kanjiLiteral].animationDirection = Animation.backwards;

        let awaitAnimationStep = awaitLast && kanjiSettings[kanjiLiteral].index === 0;
        await doAnimationStep(kanjiLiteral, paths[kanjiSettings[kanjiLiteral].index], false, !awaitAnimationStep);

        if (startTime < kanjiSettings[kanjiLiteral].timestamp) {
            return;
        }

        toggleNumbers(kanjiLiteral);
        kanjiSettings[kanjiLiteral].animationDirection = Animation.none;
    }

    kanjiSettings[kanjiLiteral].index = 0;
    concludeAutoplay(kanjiLiteral);
}

// Pauses the animation midway
async function pauseAnimation(kanjiLiteral) {
    kanjiSettings[kanjiLiteral].timestamp = Date.now();

    let playBtn = document.getElementById(kanjiLiteral + "_play");

    playBtn.dataset.state = "play";
    playBtn.children[0].classList.remove("hidden");
    playBtn.children[1].classList.add("hidden");
}

// Draws or removes the given path
async function doAnimationStep(kanjiLiteral, path, forward, fastReset) {
    path.classList.remove("hidden");

    let len = path.getTotalLength();
    let drawTime = len * 10 * (!fastReset ? (1 / kanjiSettings[kanjiLiteral].speed) : 0.5);

    let transition = "transition: stroke-dashoffset " + drawTime + "ms ease 0s, stroke " + (forward ? 0 : drawTime) + "ms ease 0s;";
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

    if (kanjiSettings[kanjiLiteral].index + direction == -1 || kanjiSettings[kanjiLiteral].index + direction > kanjiSettings[kanjiLiteral].strokeCount) {
        return;
    }

    let path = getPaths(kanjiLiteral)[direction > 0 ? kanjiSettings[kanjiLiteral].index : kanjiSettings[kanjiLiteral].index - 1];

    kanjiSettings[kanjiLiteral].index += direction;
    kanjiSettings[kanjiLiteral].animationDirection = direction > 0 ? Animation.forward : Animation.backwards;

    await doAnimationStep(kanjiLiteral, path, direction > 0);

    if (startTime <= kanjiSettings[kanjiLiteral].timestamp) {
        kanjiSettings[kanjiLiteral].animationDirection = Animation.none;
    }

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

/* -- Kanji decomposition tree -- */

// Generates the tree diagram
var pendingRequests = 0;
function generateTreeDiagram() {
    var width = 1000,
        height = 1000;

    var i = 0;

    var tree = d3.layout.tree()
        .size([height, width]);

    // set visible
    document.getElementById("tree-target").innerHTML = "";
    document.getElementById("backdrop").classList.remove("hidden");

    // Add the SVG to the body
    var svg = d3.select("#tree-target").append("svg")
        .classed("svg-content-responsive", true)
        .classed("svg-container", true)
        .attr("preserveAspectRatio", "xMinYMin meet")
        .attr("viewBox", "0 0 " + width + " " + height)
        .append("g");

    // Build the tree
    root = treeData[0];

    // Compute the new tree layout
    var nodes = tree.nodes(root).reverse(),
        links = tree.links(nodes);

    // Normalize for fixed-depth
    nodes.forEach(function (d) { d.y = d.depth * 100; });

    // Declare the nodes
    var node = svg.selectAll("g.node")
        .data(nodes, function (d) { return d.id || (d.id = ++i); });

    // Declare the links
    var link = svg.selectAll("path.link")
        .data(links, function (d) { return d.target.id; });

    // Enter the nodes
    var nodeEnter = node.enter().append("g")
        .attr("class", "node")
        .attr("transform", function (d) {
            return "translate(" + d.x + "," + d.y + ")";
        });

    // Circle style, color, fill
    nodeEnter.append("circle")
        .attr("r", 25)
        .style("fill", "rgba(222,227,231,255)");

    // Text
    nodeEnter.append("text")
        .attr("y", function (d) { // Text offset
            return d.children || d._children ? 5 : 5;
        })
        .attr("text-anchor", "middle")
        .text(function (d) { return d.name; })
        .style("fill-opacity", 1);

    // Straight lines
    link.enter().insert("line")
        .attr("class", "link")
        .attr("x1", function (d) { return d.source.x; })
        .attr("y1", function (d) { return d.source.y; })
        .attr("x2", function (d) { return d.target.x; })
        .attr("y2", function (d) { return d.target.y; });

    // Move lines in front of circle to hide the lines (only needed for straight lines)
    document.querySelectorAll(".link").forEach(e => {
        var node = e;
        var parent = e.parentNode;
        parent.removeChild(node);
        parent.prepend(e);
    });

    const srcUrl = "/assets/svg/glyphes/";

    // Figure out how many requests are required
    document.querySelectorAll("text").forEach((e) => {
        getSvgContent(e, srcUrl + e.innerHTML + ".svg");
        pendingRequests++;
    });

    svg = document.querySelector('svg');

    const { xMin, xMax, yMin, yMax } = [...svg.children].reduce((acc, el) => {
        const { x, y, width, height } = el.getBBox();
        if (!acc.xMin || x < acc.xMin) acc.xMin = x;
        if (!acc.xMax || x + width > acc.xMax) acc.xMax = x + width;
        if (!acc.yMin || y < acc.yMin) acc.yMin = y;
        if (!acc.yMax || y + height > acc.yMax) acc.yMax = y + height;
        return acc;
    }, {});

    const viewbox = `${xMin} ${yMin} ${xMax - xMin} ${yMax - yMin}`;

    svg.setAttribute('viewBox', viewbox);
}

function getSvgContent(target, url) {
    var text = "";

    console.log(url);
    fetch(url).then(r => {
        pendingRequests--;

        r.text().then(t => {
            if (text.length < 5) {
                console.log("couldn't find " + target.innerHTML);
                return;
            }

            console.log(t);
            storedText = text;
            replaceTextWithPath(target, text);
        });

    }).catch(e => console.log(e));
}

function replaceTextWithPath(target, svg) {
    let g = svg.split("\n");

    //target.replaceWith(g[3]);

    let getNodes = str => {
        console.log(target, str);
        let x = new DOMParser().parseFromString(str, 'image/svg+xml');
        return x.children[0];
    }

    let node = getNodes(g[3]);
    target.replaceWith(node);

    if (pendingRequests == 0)
        document.querySelectorAll("svg").forEach(s => s.innerHTML += "");
}



























const treeData = [
    {
        "name": "観",
        "children": [
            {
                "name": "𮥶",
                "children": [
                    {
                        "name": "𠂉",
                        "children": [
                            {
                                "name": "一"
                            }
                        ]
                    },
                    {
                        "name": "一"
                    },
                    {
                        "name": "隹"
                    }
                ]
            },
            {
                "name": "見",
                "children": [
                    {
                        "name": "目",
                        "children": [
                            {
                                "name": "囗"
                            }
                        ]
                    },
                    {
                        "name": "儿",
                        "children": [
                            {
                                "name": "丿"
                            },
                            {
                                "name": "乚"
                            }
                        ]
                    }
                ]
            }
        ]
    }

];