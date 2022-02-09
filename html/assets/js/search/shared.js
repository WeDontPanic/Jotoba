/**
 * This JS-File contains variables shared between files to improve performance
 */

const kanjiRegEx = '([一-龯|々|𥝱|𩺊])';
const hashtags = [
    "#adverb", "#auxilary", "#conjungation", "#noun", "#prefix", "#suffix", "#particle", "#sfx",
    "#verb", "#adjective", "#counter", "#expression", "#interjection", "#pronoun", "#numeric", "#transitive", "#intransitive",
    "#unclassified", "#word", "#sentence", "#name", "#kanji", "#abbreviation", "#N5", "#N4", "#N3", "#N2", "#N1", "#hidden", "#Irregular-Ichidan"
];

var currentSuggestion = ""; 
var currentSuggestionIndex = 0; // 0 => nothing
var availableSuggestions = 0;
var preventNextApiCall = false;

var input, searchRow, shadowText, sContainer, rContainer;

Util.awaitDocumentInteractive(() => {
    input = document.getElementById("search");
    searchRow = document.getElementById("search-row");
    shadowText = document.getElementById("shadow-text");
    sContainer = document.getElementById("suggestion-container");
    rContainer = document.getElementById("suggestion-container-rad");
});