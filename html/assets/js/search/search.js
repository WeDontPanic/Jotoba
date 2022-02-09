/**
 * This JS-File contains functions handling the website search (e.g. Search suggestions)
 */

// #-QuickSearches, hardcoded to reduce server callbacks
const hashtags = [
    "#adverb", "#auxilary", "#conjungation", "#noun", "#prefix", "#suffix", "#particle", "#sfx",
    "#verb", "#adjective", "#counter", "#expression", "#interjection", "#pronoun", "#numeric", "#transitive", "#intransitive",
    "#unclassified", "#word", "#sentence", "#name", "#kanji", "#abbreviation", "#N5", "#N4", "#N3", "#N2", "#N1", "#hidden", "#Irregular-Ichidan"
];

// Prepare Search / Voice Icon when loading the page
Util.awaitDocumentReady(() => {
    toggleSearchIcon(0);
});

// Shows the suggestion container if availableSuggestions > 0 and something was typed
function showContainer() {
    if (availableSuggestions > 0 && input.value.length > 0) {
        sContainer.parentElement.classList.remove("hidden");
        if (typeof scrollSearchIntoView === "function") {
            scrollSearchIntoView();
        }
    } else {
        sContainer.parentElement.classList.add("hidden");
    } 
}

// Shows the Voice / Search Icon when possible
function toggleSearchIcon(duration) {
    if (document.getElementById("search").value.length == 0) {
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
// @args radicals -> Array containing radicals that need to be contained in searched kanji
function callApiAndSetShadowText(radicals) {

    // Tooltips for # - searches
    let lastWord = Util.getLastWordOfString(input.value);
    if (lastWord.includes("#")) {
        getHashtagData(lastWord);
    }
    // Load new API data
    else if (input.value.length > 0) {
        getSuggestionApiData(radicals);
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
    sContainer.innerHTML = "";
    rContainer.innerHTML = "";
    shadowText.innerHTML = "";
    currentSuggestion = "";
    currentSuggestionIndex = 0;
    availableSuggestions = 0;
    showContainer();
}

// Loads API data by creating the json from known values instead of calling backend
function getHashtagData(currentText) {
    let suggestions = [];
    for (let i = 0; i < hashtags.length; i++) {
        if (hashtags[i].toLowerCase().includes(currentText.toLowerCase())) {
            suggestions.push({"primary": hashtags[i]});

            if (suggestions.length == 10) {
                break;
            }
        }
    }

    let resultJSON =  {
        "suggestions": suggestions,
        "suggestion_type": "hashtag"
    }

    loadSuggestionApiData(resultJSON);
}

// Interrupts the form's submit and makes the user visit the correct page
function onSearchStart() {
    var search_value = $('#search').val();
    var search_type = $('#search-type').val();

    if (search_value.length == 0) {
        Util.loadUrl(JotoTools.createUrl());
    } else {
        Util.loadUrl(JotoTools.createUrl(search_value, search_type));
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

// Opens the Help Page
function openHelpPage() {
    document.getElementsByClassName("infoBtn")[0].classList.remove("new");
    if (localStorage != null)
        localStorage.setItem("first_time", "false");
    Util.loadUrl("/help");
}
