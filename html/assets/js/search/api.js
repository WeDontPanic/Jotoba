function API() {};

// Used to store old Requests so they can be cancelled when no longer needed
API.lastRequest = undefined;
// Numbers > -1 mean that no API call will be made when input.length is above the value
API.suggestionStop = -1;

/** 
 *  Calls the API to get input suggestions
 *  @param radicalArray {[]} containing radicals that need to be contained in searched kanji
*/
API.getSuggestionApiData = function(radicalArray, successFn, errorFn) {
    // Check if API call should be prevented
    if (preventNextApiCall) {
        preventNextApiCall = false;
        return;
    }
    // Prevent if a request failed and the input is >= the text it failed against
    if (API.suggestionStop > -1 && input.value.length > API.suggestionStop) {
        return;
    }
    else {
        API.suggestionStop = -1;
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
        "lang": lang === undefined ? "en-US" : lang,
        "radicals": radicalArray || []
    }

    // Abort any requests sent earlier
    if (API.lastRequest !== undefined) {
        API.lastRequest.abort();
    }

    // Send Request to backend
    API.lastRequest = $.ajax({ 
        type : "POST", 
        url : "/api/suggestion", 
        data: JSON.stringify(inputJSON),
        headers: {
            'Content-Type': 'application/json'
        },
        success : function(result) { 
            successFn(result);
        }, 
        error : function(result) { 
            if (result.statusText !== "abort") {
                errorFn(result);
            }
        } 
    }); 
}

/**
 * Emulates the API behaviour for suggestions; returning Hashtag values instead
 * @param currentText {string} a single word without spaces, representing the #-value
 * @param callback {function} function to call after collecting suggestions
 */
API.getHashtagData = function(currentText, callback) {
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

    callback(resultJSON);
}