/**
 * This JS-File contains functions handling the website search (e.g. Search suggestions)
 */

// #-QuickSearches, hardcoded to reduce server callbacks
const hashtags = [
    "#adverb", "#auxilary", "#conjungation", "#noun", "#prefix", "#suffix", "#particle", "#sfx",
    "#verb", "#adjective", "#counter", "#expression", "#interjection", "#pronoun", "#numeric", "#transitive", "#intransitive",
    "#unclassified", "#word", "#sentence", "#name", "#kanji",
];

// Elements used
const searchRow = document.querySelector("#search-row");
const input = document.querySelector("#search");
const shadowText = document.getElementById("shadow-text");
const container = document.getElementById("suggestion-container");
const kanjiRegEx = '([一-龯|々|𥝱|𩺊])';

// Global variables used
var currentSuggestion = "";
var currentSuggestionType = "default";
var currentSuggestionIndex = -1;
var availableSuggestions = 0;
var keepSuggestions = false;
var oldInputValue = "";
var lastRequest = undefined;

// Key Events focussing on the search
$(document).on("keydown", (event) => {
    if (!$('#search').is(":focus")) return;

    // Switch the key code for potential changes
    switch (event.key) {
        case "ArrowUp": // Use suggestion above current
            event.preventDefault();
            changeSuggestionIndex(-1);
            break;
        case "ArrowDown": // Use suggestion beneath current
            event.preventDefault();
            changeSuggestionIndex(1);
            break;
        case "Tab": // Append current suggestion
            if (currentSuggestionIndex > -1) {
                activateSelection();
                showContainer();
            } else {
                changeSuggestionIndex(1);
            }
            event.preventDefault();
            break;
        case "Enter": // Start the search
            if (currentSuggestionIndex > -1) {
                event.preventDefault();
                activateSelection();
                document.getElementsByClassName("btn-search")[0].click();
            } else {
                document.getElementsByClassName("btn-search")[0].click();
            }
            break;
    }
});

// Shows the suggestion container if availableSuggestions > 0 and something was typed
function showContainer() {
    if (availableSuggestions > 0 && input.value.length > 0) {
        container.classList.remove("hidden");
        if (typeof scrollSearchIntoView === "function") {
            scrollSearchIntoView();
        }
    } else {
        container.classList.add("hidden");
    } 
}

// Event whenever the user types into the search bar
input.addEventListener("input", e => {
    if (input.value != oldInputValue) {
        callApiAndSetShadowText();
    }
    oldInputValue = input.value;
});

// Check if input was focussed / not focussed to show / hide overlay 長い
input.addEventListener("focus", e => {
    if (!keepSuggestions) {
        callApiAndSetShadowText();
    }
    showContainer();
    keepSuggestions = false;
});
document.addEventListener("click", e => {
    // When clicking anything but the search bar or dropdown
    if (!Util.isChildOf(searchRow, e.target)) {
        container.classList.add("hidden");
        keepSuggestions = true;
    }
});

// Check on resize if shadow text would overflow the search bar and show / hide it
window.addEventListener("resize", e => {
    setShadowText();
});

// Function to be called by input events. Updates the API data and shadow txt
function callApiAndSetShadowText() {

    // Tooltips for # - searches
    let lastWord = Util.getLastWordOfString(input.value);
    if (lastWord.includes("#")) {
        getHashtagData(lastWord);
    }
    // Load new API data
    else if (input.value.length > 0) {
        getApiData();
    } else {
        removeSuggestions();
    }

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
// If setDirectly = true, the index will be used directly
function changeSuggestionIndex(direction, setDirectly) {
    let oldIndex = currentSuggestionIndex;

    // Set directly
    if (setDirectly) {
        currentSuggestionIndex = direction;
    }
    // Scroll up or down
    else if (currentSuggestionIndex + direction < -1) {
        currentSuggestionIndex = availableSuggestions - 1;
    } 
    else if (currentSuggestionIndex + direction == availableSuggestions) {
        currentSuggestionIndex = -1;
    } else {
        currentSuggestionIndex += direction;
    }

    // Get newly selected suggestion
    if (currentSuggestionIndex != -1) { 
        let suggestion = getSuggestion(currentSuggestionIndex);
    
        // Add Furigana. If Kanji are used, select the secondary suggestion. If user types kanji, show him kanji instead
        if (suggestion[1].innerHTML.length > 0 && input.value.match(kanjiRegEx) === null) {
            currentSuggestion = suggestion[1].innerHTML.substring(1, suggestion[1].innerHTML.length - 1);
        } else {
            currentSuggestion = suggestion[0].innerHTML;
        }

        // Mark the suggestion's row
        suggestion[2].classList.add("selected");
    }
   
    // Remove mark on old row
    let oldSuggestion = (oldIndex < 0 ? undefined : getSuggestion(oldIndex));
    if (oldSuggestion != undefined) {
        oldSuggestion[2].classList.remove("selected");
    }

    // Update shadow text
    setShadowText();
}

// Adds the currently selected suggestion to the search input
function activateSelection(element) {

    // The primary suggestion to use
    let suggestion = "";

    // If element is given as parameter directly, use its the suggestion instead
    if (element !== undefined) {
        switch (currentSuggestionType) {
            case "kanji_reading":
                let se = element.querySelector(".secondary-suggestion");
                suggestion =  element.querySelector(".primary-suggestion").innerHTML + " " + se.innerHTML.substring(1, se.innerHTML.length - 1);
                break;
            default:
                suggestion = element.querySelector(".primary-suggestion").innerHTML;
        }
    } 
    // Else, find the suggestion by searching for the current index
    else {
        switch (currentSuggestionType) {
            case "kanji_reading":
                let s = getSuggestion(currentSuggestionIndex);
                suggestion = s[0].innerHTML + " " + s[1].innerHTML.substring(1, s[1].innerHTML.length - 1);
                break;
            default:
                suggestion = getSuggestion(currentSuggestionIndex)[0].innerHTML;
        }
    }

    // Fix some weird characters
    suggestion = suggestion.replace("&amp;", "&");

    // Remove last text from string and append new word
    input.value = input.value.substring(0, input.value.lastIndexOf(" "));
    if (suggestion.startsWith("#")) {
        input.value += " " + suggestion;   
    }
    else {
        input.value = suggestion;
    }
    

    // Reset dropdown
    removeSuggestions();
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
    currentSuggestionIndex = -2;
    availableSuggestions = 0;
    showContainer();
}

// Loads API data by creating the json from known values instead of calling backend
function getHashtagData(currentText) {
    let suggestions = [];
    for (let i = 0; i < hashtags.length; i++) {
        if (hashtags[i].includes(currentText)) {
            suggestions.push({"primary": hashtags[i]});

            if (suggestions.length == 10) {
                break;
            }
        }
    }

    let resultJSON =  {
        "suggestions": suggestions
    }

    loadApiData(resultJSON);
}

// Calls the API to get input suggestions
function getApiData() {

    // Create the JSON
    let lang = Cookies.get("default_lang");
    let type = $('#search-type').val();

    let inputJSON = {
        "input": input.value,
        "search_type": type,
        "lang": lang === undefined ? "en-US" : lang
    }

    // Abort any requests sent earlier
    if (lastRequest !== undefined) {
        lastRequest.abort();
    }

    // Send Request to backend
    lastRequest = $.ajax({ 
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
            // Error = reset everything if not aborted
            if (result.statusText !== "abort") {
                removeSuggestions();
            }
        } 
    }); 
}

// Loads data called from the API into the frontend
function loadApiData(result) {

    // Keep old suggestion if it exists in the list again
    let oldSuggestion = currentSuggestion;
    let suggestionChosen = false;

    // Remove current suggestions
    removeSuggestions();

    // Return if no suggestions were found
    if (result.suggestions.length == 0) {
        console.log("return");
        return;
    }

    // Set suggestion type
    currentSuggestionType = result.suggestion_type;

    // Set the amount of possible suggestions
    availableSuggestions = result.suggestions.length;

    // Add suggestions
    for (let i = 0; i < availableSuggestions; i++) {

        // Result variables
        let primaryResult = "";
        let secondaryResult = "";

        // Only one result
        if (result.suggestions[i].secondary === undefined) {
            primaryResult = result.suggestions[i].primary;
        }
        // Two results, kanji needs to be in the first position here
        else {
            primaryResult = result.suggestions[i].secondary;
            secondaryResult = "(" + result.suggestions[i].primary + ")";
        }

        // Add to Page
        container.innerHTML += 
        ' <div class="search-suggestion" onclick="onSuggestionClick(this);"> ' +
        '   <span class="primary-suggestion">'+primaryResult+'</span> ' +
        '   <span class="secondary-suggestion">'+secondaryResult+'</span> ' +
        ' </div> ';      

        // Activate suggestion, if available again
        if (oldSuggestion == primaryResult) {
            changeSuggestionIndex(i, true);
            suggestionChosen = true;
        }
    }

    // Activate first suggestion
    if (!suggestionChosen) {
        changeSuggestionIndex(1);
    }

    // Load Container if there is text present
    showContainer();
}

// Handles clicks on the suggestion dropdown
function onSuggestionClick(element) {
    activateSelection(element);
    document.getElementsByClassName("btn-search")[0].click()
}

// Interrupts the form's submit and makes the user visit the correct page
function onSearchStart() {
    var search_value = $('#search').val();
    var search_type = $('#search-type').val();

    if (search_value.length == 0) {
        window.location = window.location.origin;
    } else {
        window.location = window.location.origin + "/search/" + encodeURIComponent(search_value) + "?t=" + search_type;
    }

    return false;
}