/*
*   Made to handle API calls. Load before search.js!
*/


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
        ' <div class="search-suggestion" onclick="onSuggestionClick(this);"> ' +
        '   <span class="primary-suggestion">'+primaryResult+'</span> ' +
        '   <span class="secondary-suggestion">'+secondaryResult+'</span> ' +
        ' </div> ';      
    }

    // Activate first suggestion
    if (!suggestionChosen) {
        //changeSuggestionIndex(1, true);
    }

    // Load Container if there is text present
    showContainer();
}