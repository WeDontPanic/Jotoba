/*
*   Handles functions related to the suggestion Overlay. Load before search.js!
*/

Suggestions.overlay = function () {}; 

// Shows the suggestions overlay
Suggestions.overlay.show = function() {
    if (availableSuggestions > 0 && input.value.length > 0) {
        sContainer.parentElement.classList.remove("hidden");
        if (typeof scrollSearchIntoView === "function") {
            scrollSearchIntoView();
        }
    } else {
        sContainer.parentElement.classList.add("hidden");
    } 
}

// Searches for the currently selected suggestion
Suggestions.overlay.activateSelection = function() {
    $("#suggestion-container > .search-suggestion")[currentSuggestionIndex-1].click();
}

// Selects the suggestion at the index above (-1) or beneath (1)
Suggestions.overlay.changeSuggestionIndex = function(direction) {

    // Remove highlight from last suggestion
    if (currentSuggestionIndex != 0) { 
        $("#suggestion-container > .search-suggestion")[currentSuggestionIndex-1].classList.remove("selected");
    }
    
    // Calculate new suggestion index
    currentSuggestionIndex = Math.positiveMod(currentSuggestionIndex + direction, availableSuggestions + 1);
    
    // Set new highlight
    if (currentSuggestionIndex != 0) { 
        
        // Get current suggestion
        let suggestion = $("#suggestion-container > .search-suggestion")[currentSuggestionIndex-1];
        let s_children = suggestion.children;
        
        // Add Furigana. If Kanji are used, select the secondary suggestion. If user types kanji, show him kanji instead
        if (s_children[1].innerHTML.length > 0 && input.value.match(kanjiRegEx) === null) {
            currentSuggestion = s_children[1].innerHTML.substring(1, s_children[1].innerHTML.length - 1);
        } else {
            currentSuggestion = s_children[0].innerHTML;
        }

        // Mark the suggestion's row
        suggestion.classList.add("selected");
    }
    else {
        currentSuggestion = "";
    }

    // Update shadow text
    setShadowText();
}

