/**
 * This JS-File contains functions handling the website search (e.g. Search suggestions)
 */

// #-QuickSearches, hardcoded to reduce server callbacks
const hashtags = [
    "#adverb", "#auxilary", "#conjungation", "#noun", "#prefix", "#suffix", "#particle", "#sfx",
    "#verb", "#adjective", "#counter", "#expression", "#interjection", "#pronoun", "#numeric", "#transitive", "#intransitive",
    "#unclassified", "#word", "#sentence", "#name", "#kanji", "#abbreviation", "#N5", "#N4", "#N3", "#N2", "#N1"
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
var currentSuggestionIndex = 0;
var availableSuggestions = 0;
var keepSuggestions = false;
var oldInputValue = "";
var lastRequest = undefined;
var preventApiCallUntilDelete = false;
var textToPrevent = "";

// Prepare Search / Voice Icon when loading the page
toggleSearchIcon(0);

// Mark the currently selected search type (only used for mobile so far)
markCurrentSearchType();

// Marks the current search's type, so it can be displayed in another color
function markCurrentSearchType() {
    let searchType = $('#search-type').val();

    for (let i = 0; i < 4; i ++) {
        if (i == searchType) {
            $('.choices__item[data-value="'+i+'"]').addClass('selected');
        } else {
            $('.choices__item[data-value="'+i+'"]').removeClass('selected');
        }
    }
}

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

// Shows the Voice / Search Icon when possible
function toggleSearchIcon(duration) {
    if (input.value.length == 0) {
        $('#searchBtn.search-embedded-btn').hide(duration);
        $('#voiceBtn.search-embedded-btn').show(duration);
    } else {
        $('#searchBtn.search-embedded-btn').show(duration);
        $('#voiceBtn.search-embedded-btn').hide(duration);
    }
}

// Resets the value of the search input
function emptySearchInput() {
    $('#search').val("");
    $('#search').focus();
    toggleSearchIcon(200);
}

// Function to be called by input events. Updates the API data and shadow txt
function callApiAndSetShadowText() {

    // Tooltips for # - searches
    let lastWord = Util.getLastWordOfString(input.value);
    if (lastWord.includes("#")) {
        getHashtagData(lastWord);
    }
    // Load new API data
    else if (input.value.length > 0) {
        getSuggestionApiData();
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

    if (setDirectly) {
      currentSuggestionIndex = direction;
    } else {
      currentSuggestionIndex = mod(currentSuggestionIndex + direction, availableSuggestions + 1);
    }

    // Get newly selected suggestion
    if (currentSuggestionIndex != 0) { 
        let suggestion = getSuggestion(currentSuggestionIndex-1);
    
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
    if (oldIndex != 0) {
        getSuggestion(oldIndex-1)[2].classList.remove("selected");
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
                let s = getSuggestion(currentSuggestionIndex - 1);
                suggestion = s[0].innerHTML + " " + s[1].innerHTML.substring(1, s[1].innerHTML.length - 1);
                break;
            default:
                suggestion = getSuggestion(currentSuggestionIndex - 1)[0].innerHTML;
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
        currentSubstr = target.substring(0, i).toLowerCase();
        let index = input.value.toLowerCase().lastIndexOf(currentSubstr)

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
    currentSuggestionIndex = 0;
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

    loadSuggestionApiData(resultJSON);
}

// Handles clicks on the suggestion dropdown
function onSuggestionClick(element) {
    activateSelection(element);
    document.getElementById("searchBtn").click();
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

// When opening an overlay, scroll it into view
function scrollSearchIntoView() {
    if (document.location.origin+"/" === document.location.href) {
        var top = $('#search').offset().top;
        Util.scrollTo(top, 500);
    }
}

// Closes all overlays connected to the search bar
function closeAllSubSearchbarOverlays(overlayToIgnore) {
    if (overlayToIgnore !== "speech")
        $('.overlay.speech').addClass('hidden');
    if (overlayToIgnore !== "radical") 
        $('.overlay.radical').addClass('hidden');
    if (overlayToIgnore !== "image")
        $('.overlay.image').addClass('hidden');
}

// Initialize Pagination Buttons
$('.pagination-item:not(.disabled) > button').on("click", (e) => {
    var search_value = $('#search').val();
    var search_type = $('#search-type').val();
    var targetPage = $(e.target.parentNode).attr("target-page");
    window.location = window.location.origin + "/search/" + encodeURIComponent(search_value) + "?t=" + search_type + "&p=" + targetPage;
});
