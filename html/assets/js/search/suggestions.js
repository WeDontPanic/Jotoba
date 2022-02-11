function Suggestions() {};

/**
 * Updates the suggestions help and respects selected radicals if given some
 * 
 * @param radicalArray {[]} containing radicals that need to be contained in searched kanji
 */
Suggestions.updateSuggestions = function(radicalArray) {

    // Tooltips for # - searches
    let lastWord = Util.getLastWordOfString(input.value);
    if (lastWord.includes("#")) {
        API.getHashtagData(lastWord, loadSuggestionApiData);
        
    // Tooltips for everything else
    } else if (input.value.length > 0) {
        API.getSuggestionApiData(radicalArray, loadSuggestionApiData, removeSuggestions);

    // Remove suggestions if the input is empty
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

// Called only by [getSuggestionApiData]. Loads data called from the API into the frontend
function loadSuggestionApiData(result) {

    // Remove current suggestions
    removeSuggestions();

    // Return if no suggestions were found
    if (result.suggestions.length == 0) {

        // Prevent future requests if no result was found and input was > 8 chars
        if (input.value >= 8) { 
            API.suggestionStop = input.value.length;
        }

        // Return
        return;
    } else {
        // Show Suggestions Containers
        if ($(".overlay.radical").hasClass("hidden")) {
            sContainer.parentElement.classList.remove("hidden");
        } else {
            rContainer.classList.remove("hidden");
        }
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
                searchValue = encodeURIComponent(s.slice(0, s.length-1).join(" ")) + " " + encodeURIComponent(primaryResult);
                break;
            default:
                searchValue = encodeURIComponent(primaryResult);
        }

        // Add to Page
        sContainer.innerHTML += 
            ' <a href="/search/'+searchValue+'?t='+currentPage+'" class="search-suggestion"> ' +
            '   <span class="primary-suggestion">'+primaryResult+'</span> ' +
            '   <span class="secondary-suggestion">'+secondaryResult+'</span> ' +
            ' </a> ';    

        rContainer.innerHTML += 
            ' <a href="/search/'+searchValue+'?t='+currentPage+'" class="search-suggestion"> ' +
            '   <span class="primary-suggestion">'+primaryResult+'</span> ' +
            ' </a> ';      
    }
}

// Removes all current suggestions including shadowText
function removeSuggestions() {
    sContainer.innerHTML = "";
    rContainer.innerHTML = "";
    shadowText.innerHTML = "";
    currentSuggestion = "";
    currentSuggestionIndex = 0;
    availableSuggestions = 0;
    sContainer.parentElement.classList.add("hidden");
    rContainer.classList.add("hidden");
}