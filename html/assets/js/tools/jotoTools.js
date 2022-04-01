/*
*   Collection-File like utils.js but that are made specifically for Jotoba
*/

// The JotoTools "parent"
function JotoTools () {};

// Creates a Jotoba-Search URL using the given parameters
JotoTools.createUrl = function(searchText, searchType, targetPage, languageCode, sentenceIndex) {
    let url = window.location.origin;

    if (searchText !== undefined) {
        url += "/search/" + encodeURIComponent(searchText);
    } 

    if (searchType !== undefined) {
        url += "?t=" + searchType;
    }

    if (targetPage !== undefined) {
        url += "&p=" + targetPage;
    }

    if (languageCode !== undefined) {
        url += "&l=" + languageCode;
    }

    if (sentenceIndex !== undefined) {
        url += "&i=" + sentenceIndex;
    }

    return url;
}

// Takes a link path starting with / and appends it to the Joto-URL (https://jotoba.de {/path})
JotoTools.pathToUrl = function(path) {
    return window.location.origin + path;
}

// Creates a Jotoba URL for the given page
JotoTools.getPageUrl = function(pageName) {
    let url = window.location.origin;
    url += "/" + pageName;

    return url;
}

// Returns the currently searched string
JotoTools.getCurrentSearch = function() {
    return document.location.pathname.split("/")[2];
}

// Returns the value of the current Search [Words, Sentence...]
JotoTools.getCurrentSearchType = function() {
    return $('#search-type').val();
}

// Parses a language code into the Joto needs
JotoTools.toJotobaLanguage = function(code) {
    code = code.toLowerCase().substr(0, 2);
    switch (code) {
        case "en":
            code = "en-US";
            break;
        case "sv":
            code = "sv-SE";
            break;
        case "ru":
            code = "ru";
            break;
        case "hu":
            code = "hu";
            break;
        default:
            code += "-"+code.toUpperCase();
            if (!JotoTools.isSupportedSearchLang(code))
                code = "en-US";
    }
    return code;
}

// Checks if a given language code is supported as a search lang
JotoTools.isSupportedSearchLang = function(code) {
    switch (code) {
        case "en-US":
        case "de-DE":
        case "es-ES":
        case "fr-FR":
        case "nl-NL":
        case "sv-SE":
        case "ru":
        case "hu":
        case "sl-SI":
            return true;
        default:
            return false;
    }
}

// Copies the given text and echoes the given Message
JotoTools.copyTextAndEcho = function(text, messageID) {
    Util.copyToClipboard(text);
    Util.showMessage("success", getText(messageID));
}