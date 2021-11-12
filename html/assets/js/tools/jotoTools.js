/*
*   Collection-File like utils.js but that are made specifically for Jotoba
*/

// The JotoTools "parent"
function JotoTools () {};

// Creates a Jotoba-Search URL using the given parameters
JotoTools.createUrl = function(searchText, searchType, targetPage) {
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

    return url;
}

// Returns the value of the current Search [Words, Sentence...]
JotoTools.getCurrentSearchType = function() {
    return $(".choices__item.choices__item--choice.choices__item--selectable.selected").attr("data-value");
}