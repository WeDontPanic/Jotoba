/*
*   Handles functions related to the suggestion Overlay. Load before search.js!
*/

// Searches for the currently selected suggestion
function activateSelection() {
    $(".search-suggestion")[currentSuggestionIndex-1].click();
}

// Selects the suggestion at the index above (-1) or beneath (1)
function changeSuggestionIndex(direction) {

    // Remove highlight from last suggestion
    if (currentSuggestionIndex != 0) { 
        $(".search-suggestion")[currentSuggestionIndex-1].classList.remove("selected");
    }
    
    // Calculate new suggestion index
    currentSuggestionIndex = Math.positiveMod(currentSuggestionIndex + direction, availableSuggestions + 1);
    
    // Set new highlight
    if (currentSuggestionIndex != 0) { 
        
        // Get current suggestion
        let suggestion = $(".search-suggestion")[currentSuggestionIndex-1];
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

// Calls the API to get input suggestions
function getSuggestionApiData() {

    // Check if API call should be prevented
    if (preventNextApiCall) {
        preventNextApiCall = false;
        return;
    }
    else if (preventApiCallUntilDelete) {
        if (textToPrevent.length <= input.value.length && input.value.substring(0, textToPrevent.length) === textToPrevent) {
            return;
        } else {
            preventApiCallUntilDelete = false;
            textToPrevent = "";
        }
    }

    // Create the JSON
    let lang = Cookies.get("default_lang");
    let type = JotoTools.getCurrentSearchType();
    let txt = input.value;
    
    if (txt.length == 0) {
        return;
    }

    let inputJSON = {
        "input": txt,
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

    // Remove current suggestions
    removeSuggestions();

    // Return if no suggestions were found
    if (result.suggestions.length == 0) {

        // Prevent future requests if no result was found and input was > 8 chars
        if (input.value >= 8) { 
            preventApiCallUntilDelete = true;
            textToPrevent = input.value;
        }

        // Return
        return;
    }

    // Set suggestion type
    currentSuggestionType = result.suggestion_type;

    // Set the amount of possible suggestions
    availableSuggestions = result.suggestions.length;
    if (availableSuggestions > 10) {
        availableSuggestions = 10;
    }

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

        // Get target page
        var currentPage = JotoTools.getCurrentSearchType();

        // Generate the /search/
        let searchValue = "";

        switch (currentSuggestionType) {
            case "kanji_reading":
                searchValue = encodeURIComponent(primaryResult) + " " + encodeURIComponent(result.suggestions[i].primary);
                break;
            case "hashtag":
                let s = input.value.split(" ");
                searchValue = s.slice(0, s.length-1).join(" ") + " " + encodeURIComponent(primaryResult);
                break;
            default:
                searchValue = encodeURIComponent(primaryResult);
        }

        // Add to Page
        if (rad_overlay.classList.contains("hidden")) {
            container.innerHTML += 
            ' <a href="/search/'+searchValue+'?t='+currentPage+'" class="search-suggestion"> ' +
            '   <span class="primary-suggestion">'+primaryResult+'</span> ' +
            '   <span class="secondary-suggestion">'+secondaryResult+'</span> ' +
            ' </a> ';      
        } else {
            container_rad.innerHTML += 
            ' <a href="/search/'+searchValue+'?t='+currentPage+'" class="search-suggestion"> ' +
            '   <span class="primary-suggestion">'+primaryResult+'</span> ' +
            ' </a> ';      
        }
    }

    // Load Container if there is text present
    showContainer();
}