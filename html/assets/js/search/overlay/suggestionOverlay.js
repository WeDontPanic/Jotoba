/*
*   Handles functions related to the suggestion Overlay. Load before search.js!
*/

// Returns the suggestion Parent. Useful if element is a children-element
function getSuggestionParent(element) {
    if (element.classList.contains("search-suggestion"))
        return element;
    else 
        return element.parentElement;
}

// Returns a suggestion either by receiving its element or by using the current index.
function getSuggestion(element) {
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
                let s = getSuggestionByIndex(currentSuggestionIndex - 1);
                suggestion = s[0].innerHTML + " " + s[1].innerHTML.substring(1, s[1].innerHTML.length - 1);
                break;
            default:
                suggestion = getSuggestionByIndex(currentSuggestionIndex - 1)[0].innerHTML;
        }
    }

    // Fix some weird characters
    return suggestion.replace("&amp;", "&");
}

// Returns the primary [0], secondary [1] suggestion and the parent [2]
function getSuggestionByIndex(index) {
    // Get newly selected suggestion
    let suggestion = document.querySelectorAll(".search-suggestion")[index];

    // Find primary and secondary suggestion <span>
    let primarySuggestion = suggestion.querySelector(".primary-suggestion");
    let secondarySuggestion = suggestion.querySelector(".secondary-suggestion");

    return [primarySuggestion, secondarySuggestion, suggestion];
}

// Adds the currently selected suggestion to the search input
function activateSelection(element) {

    // Get suggestion
    let suggestion = getSuggestion(element);

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
        let suggestion = getSuggestionByIndex(currentSuggestionIndex-1);
    
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
        getSuggestionByIndex(oldIndex-1)[2].classList.remove("selected");
    }

    // Update shadow text
  setShadowText();
}

// Calls the API to get input suggestions
function getSuggestionApiData() {

    // Check if API call should be prevented
    if (preventApiCallUntilDelete) {
        if (textToPrevent.length <= input.value.length && input.value.substring(0, textToPrevent.length) === textToPrevent) {
            return;
        } else {
            preventApiCallUntilDelete = false;
            textToPrevent = "";
        }
    }

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
            loadSuggestionApiData(result);
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
function loadSuggestionApiData(result) {

    // Keep old suggestion if it exists in the list again
    let oldSuggestion = currentSuggestion;
    let suggestionChosen = false;

    // Remove current suggestions
    removeSuggestions();

    // Return if no suggestions were found and 
    if (result.suggestions.length == 0) {
        // Prevent future requests if no result was found 
        preventApiCallUntilDelete = true;
        textToPrevent = input.value;

        // Return
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
        ' <a href="/search/'+primaryResult+'" class="search-suggestion"> ' +
        '   <span class="primary-suggestion">'+primaryResult+'</span> ' +
        '   <span class="secondary-suggestion">'+secondaryResult+'</span> ' +
        ' </a> ';      
    }

    // Activate first suggestion
    if (!suggestionChosen) {
        //changeSuggestionIndex(1, true);
    }

    // Load Container if there is text present
    showContainer();
}