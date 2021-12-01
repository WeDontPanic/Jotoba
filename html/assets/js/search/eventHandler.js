/*
*   Made to Handle search related events. Loads after search.js!
*/

// Key Events focussing on the search
$(document).on("keydown", (event) => {
    if (!$('#search').is(":focus")) return;

    // Switch the key code for potential changes
    switch (event.key) {
        case "ArrowUp": // Use suggestion above current
            event.preventDefault();
            changeSuggestionIndex(-1);
            break;
        case "ArrowDown": // Use suggestion beneath current
        case "Tab":
            event.preventDefault();
            var direction = 1;
            if (event.key == "Tab" && shiftPressed) {
              direction = -1;
            }
            changeSuggestionIndex(direction);
            break;
        case "Enter": // Start the search
            if (currentSuggestionIndex > 0) {
                event.preventDefault();
                activateSelection();
            } else {
                $('#searchBtn').click();
            }
            break;
    }
});

// Event whenever the user types into the search bar
document.querySelector("#search").addEventListener("input", e => {
    if (input.value != oldInputValue) {
        callApiAndSetShadowText();
    }
    oldInputValue = input.value;

    toggleSearchIcon(200);
});

// Check if input was focussed / not focussed to show / hide overlay 長い
document.querySelector("#search").addEventListener("focus", e => {
    if (!keepSuggestions) {
        callApiAndSetShadowText();
    }
    showContainer();
    keepSuggestions = false;
});

// Event whenever the user types into the search bar
document.querySelector("#kanji-search").addEventListener("input", e => {
    getRadicalSearchResults();
});

// Outside-Click event (used to hide overlays...)
document.addEventListener("click", e => {
    // When clicking anything but the search bar or dropdown
    if (!Util.isChildOf(searchRow, e.target)) {
        container.classList.add("hidden");
        keepSuggestions = true;
    }
});

// Check on resize if shadow text would overflow the search bar and show / hide it
window.addEventListener("resize", e => {
    setShadowText();
});

// Scroll sentence-reader to display selected index
Util.awaitDocumentReady(() => {
    let sentencePart = $('.sentence-part.selected');

    if (sentencePart.length > 0) {
        $('#sr')[0].scrollTop = (sentencePart.offset().top);
    }
});

// Initialize Pagination Buttons
Util.awaitDocumentReady(() => {
    $('.pagination-item:not(.disabled) > button').on("click", (e) => {
        var searchValue = $('#search').val();
        var searchType = $('#search-type').val();
        var targetPage = $(e.target.parentNode).attr("target-page");
        Util.loadUrl(JotoTools.createUrl(searchValue, searchType, targetPage));
    });
});
