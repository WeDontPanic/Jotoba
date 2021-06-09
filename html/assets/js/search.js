/**
 * This JS-File contains functions handling the website search (e.g. Search suggestions)
 */

// Elements used
const input = document.querySelector("#search");
const shadowText = document.getElementById("shadow-text");
const container = document.getElementById("suggestion-container");

// Global variables used
var currentSuggestion = "";
var currentSuggestionIndex = -1;
var availableSuggestions = 0;

// Key Events focussing on the search
$(document).on("keydown", (event) => {
    if (!$('#search').is(":focus")) return;

    // Switch the key code for potential changes
    switch (event.key) {
        case 'ArrowUp': // Use suggestion above current
            event.preventDefault();
            changeSuggestionIndex(-1);
            break;
        case 'ArrowDown': // Use suggestion beneath current
            event.preventDefault();
            changeSuggestionIndex(1);
            break;
        case "Enter": // Do a search while rad-picker is opened or set current suggestion
            if (currentSuggestion != -1) {
                activateSelection();
                event.preventDefault();
            } 
            break;
    }
});

// Event whenever the user types into the search bar
input.addEventListener("input", e => {
    callApiAndSetShadowText();
});

// Always check if shadow text can still be displayed or needs to be hidden
setInterval(() => {
    setShadowText();
}, 400);

window.addEventListener("resize", e => {
    setShadowText();
});

// Function to be called by input events. Updates the API data and shadow txt
function callApiAndSetShadowText() {
    // Load new API data
    getApiData();

    // Set shadow text
    setShadowText();
}

// Sets the shadow's text whenever possible
function setShadowText() {
    // If input is overflown, dont show text
    if (Util.checkOverflow(shadowText) && shadowText.innerHTML != "") {
        shadowText.innerHTML = "";
        return
    }

    // Make invisible temporarily
    shadowText.style.opacity = 0;

    // Check how much of suggestion is typed already
    let currentSubstr = getCurrentSubstring();

    // Add missing suggestion to shadow text
    if (currentSubstr.length > 0) {
        shadowText.innerHTML = input.value + currentSuggestion.substring(currentSubstr.length);
    } else {
        shadowText.innerHTML = "";
    }   

    // If it would overflow with new text, don't show
    if (Util.checkOverflow(shadowText)) {
        shadowText.innerHTML = "";
    }

    // Make visible again
    shadowText.style.opacity = 0.4;
}

// Returns the primary [0], secondary [1] suggestion and the parent [2]
function getSuggestion(index) {
    // Get newly selected suggestion
    let suggestion = document.querySelectorAll(".search-suggestion")[index];

    // Find primary and secondary suggestion <span>
    let primarySuggestion = suggestion.querySelector(".primary-suggestion");
    let secondarySuggestion = suggestion.querySelector(".secondary-suggestion");

    return [primarySuggestion, secondarySuggestion, suggestion];
}

// Selects the suggestion at the index above (-1) or beneath (1)
function changeSuggestionIndex(direction) {
    let oldIndex = currentSuggestionIndex;

    // Scroll up or down
    if (currentSuggestionIndex + direction < 0) {
        currentSuggestionIndex = availableSuggestions - 1;
    } 
    else if (currentSuggestionIndex + direction == availableSuggestions) {
        currentSuggestionIndex = 0;
    } else {
        currentSuggestionIndex += direction;
    }

    // Get newly selected suggestion
    let oldSuggestion = (oldIndex == -1 ? undefined : getSuggestion(oldIndex));
    let suggestion = getSuggestion(currentSuggestionIndex);

    // Add Furigana. If Kanji are used, select the secondary suggestion
    if (suggestion[1].innerHTML.length > 0) {
        currentSuggestion = suggestion[1].innerHTML.substring(1, suggestion[1].innerHTML.length - 1);
    } else {
        currentSuggestion = suggestion[0].innerHTML;
    }

    // Mark the suggestion's row
    suggestion[2].classList.add("selected");
    if (oldSuggestion != undefined) {
        oldSuggestion[2].classList.remove("selected");
    }

    // Update shadow text
    setShadowText();
}

// Adds the currently selected suggestion to the search input
function activateSelection(element) {

    // Get newly selected suggestion
    let suggestion = getSuggestion(currentSuggestionIndex);

    // If element is given as parameter directly
    if (element !== undefined) {
        suggestion = element;
        suggestion[0] = element.querySelector(".primary-suggestion");
        suggestion[1] = element.querySelector(".secondary-suggestion");

        if (suggestion[1].innerHTML.length > 0) {
            currentSuggestion = suggestion[1].innerHTML.substring(1, suggestion[1].innerHTML.length - 1);
        }  else {
            currentSuggestion = suggestion[0].innerHTML;
        }
    }

    // Check how many chars of the suggestion the user already typed
    let currentSubstr = getCurrentSubstring();

    // No Kanji used -> Insert rest of text
    if (suggestion[1].innerHTML.length == 0) {
        if (currentSubstr.length > 0) {
            input.value += currentSuggestion.substring(currentSubstr.length);
        } else {
            input.value += currentSuggestion;
        }
    }
    
    // If it uses Kanji, the furigana user-input has to be deleted before inserting the rest
    else {
        let typedFuri = getCurrentSubstring();
        let typedKanji = getCurrentSubstring(suggestion[0].innerHTML);

        // User typed Furi, remove typed characters and insert Kanji
        if (typedFuri.length > 0) {
            input.value = input.value.substring(0, input.value.length - "にほ".length) + suggestion[0].innerHTML;
        }

        // User typed Kanji, insert rest of Kanji
        else {
            input.value += suggestion[0].innerHTML.substring(typedKanji.length);
        }
    }
}

// Returns the substring of what the user already typed for the current suggestion
// If target is not empty, the substring of target will be searched instead
function getCurrentSubstring(target) {
    let currentSubstr = "";
    let foundSubstr = false;

    if (target === undefined) {
        target = currentSuggestion;
    }

    for (let i = target.length; i > 0; i--) {

        currentSubstr = target.substring(0, i);
        let index = input.value.lastIndexOf(currentSubstr)

        if (index == -1) {
            continue;
        }

        if (index + currentSubstr.length === input.value.length) {
            foundSubstr = true;
            break;
        }
    }

    return foundSubstr ? currentSubstr : "";
}

// Removes all current suggestions including shadowText
function removeSuggestions() {
    shadowText.innerHTML = "";
    container.innerHTML = "";
    currentSuggestion = "";
    currentSuggestionIndex = -1;
    availableSuggestions = 0;
}

// Calls the API to get input suggestions
function getApiData() {

    // Create the JSON
    let inputJSON = {
        "input": input.value
    }

    // Send Request to backend
    $.ajax({ 
        type : "POST", 
        url : "/api/suggestion", 
        data: JSON.stringify(inputJSON),
        headers: {
            'Content-Type': 'application/json'
        },
        success : function(result) { 
            // Load the results into frontend
            loadApiData(result);
        }, 
        error : function(result) { 
            // Ignore
        } 
    }); 
}

// Loads data called from the API into the frontend
function loadApiData(result) {

    // result = {"suggestions":[{"kana":"にほんご","kanji":"日本語"},{"kana":"にほんごかんきょう"},{"kana":"にほんごきょうほん","kanji":"日本語教本"},{"kana":"にほんごがくしゃ","kanji":"日本語学者"},{"kana":"にほんごがっこう","kanji":"日本語学校"},{"kana":"にほんごきょういく","kanji":"日本語教育"},{"kana":"にほんごがく","kanji":"日本語学"},{"kana":"にほんごじまく","kanji":"日本語字幕"},{"kana":"にほんごぞく","kanji":"日本語族"},{"kana":"にほんごか","kanji":"日本語化"}]};

    // Remove current suggestions
    removeSuggestions();

    // Return if no suggestions were found
    if (result.suggestions.length == 0) {
        return;
    }

    // Set the amount of possible suggestions
    availableSuggestions = result.suggestions.length;

    // Add suggestions
    for (let i = 0; i < availableSuggestions; i++) {

        // Get Kana and Kanji
        let kana = result.suggestions[i].kana;
        let kanji = result.suggestions[i].kanji;

        // Add Brackets or remove if undefined
        if (kanji === undefined) {
            kanji = kana;
            kana = "";
        } else {
            kana = "(" + kana + ")";
        }

        // Add to Page
        container.innerHTML += 
        ' <div class="search-suggestion" onclick="activateSelection(this);"> ' +
        '   <span class="primary-suggestion">'+kanji+'</span> ' +
        '   <span class="secondary-suggestion">'+kana+'</span> ' +
        ' </div> ';        
    }

    // Activate first suggestion
    changeSuggestionIndex(1);
}
