/**
 * This JS-File contains variables shared between files to improve performance
 */

const kanjiRegEx = '([一-龯|々|𥝱|𩺊])';
const hashtags = [
  "#adverb", "#auxilary", "#conjunction", "#noun", "#prefix", "#suffix", "#particle", "#sfx",
  "#verb", "#adjective", "#counter", "#expression", "#interjection", "#pronoun", "#numeric", "#transitive", "#intransitive",
  "#unclassified", "#word", "#sentence", "#name", "#kanji", "#abbreviation","#katakana", "#N5", "#N4", "#N3", "#N2", "#N1", "#JLPT5", "#JLPT4", "#JLPT3", "#JLPT2", "#JLPT1", "#hidden", "#Irregular-Ichidan",
  "#Abbreviation", "#Archaism", "#ChildrensLanguage", "#Colloquialism", "#Dated", "#Derogatory", "#Familiarlanguage",
  "#Femaleterm", "#Honorific", "#Humblelanguage", "#Idomatic", "#Legend", "#Formal", "#MangaSlang", "#Maleterm", "#InternetSlang",
  "#Obsolete", "#Obscure", "#Onomatopoeic", "#PersonName", "#Placename", "#Poeticalterm", "#PoliteLanguage", "#Proverb", "#Quotation", "#Rare", "#Religion", "#Sensitive",
  "#Slang", "#UsuallyKana", "#Vulgar", "#Artwork", "#Yojijukugo",
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
