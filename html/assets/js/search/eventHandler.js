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
            Suggestions.overlay.changeSuggestionIndex(-1);
            break;
        case "ArrowDown": // Use suggestion beneath current
        case "Tab":
            event.preventDefault();
            var direction = 1;
            if (event.key == "Tab" && shiftPressed) {
              direction = -1;
            }
            Suggestions.overlay.changeSuggestionIndex(direction);
            break;
        case "Enter": // Start the search
            if (currentSuggestionIndex > 0) {
                event.preventDefault();
                Suggestions.overlay.activateSelection();
            } else {
                $('#searchBtn').click();
            }
            break;
    }
});

// Adding listeners
Util.awaitDocumentReady(() => {

    // Also show shadow text if user clicked before focus event could be caught
    if ($(input).is(":focus")) {
        Suggestions.updateSuggestions();
    }

    // Event whenever the user types into the search bar
    document.getElementById("search").addEventListener("input", e => {
        Suggestions.updateSuggestions();
        toggleSearchIcon(200);
    });

    // Check if input was focussed / not focussed to show / hide overlay
    document.getElementById("search").addEventListener("focus", e => {
        Suggestions.updateSuggestions();
    });

    // Event whenever the user types into the search bar
    document.querySelector("#kanji-search").addEventListener("input", e => {
        getRadicalSearchResults();
    });

    // When clicking anything but the search bar or dropdown (used to hide overlays)
    document.addEventListener("click", e => {
        if (!Util.isChildOf(searchRow, e.target)) {
            sContainer.parentElement.classList.add("hidden");
        }
    });

    // Check on resize if shadow text would overflow the search bar and show / hide it
    window.addEventListener("resize", e => {
        setShadowText();
    });
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
        var searchValue = JotoTools.getCurrentSearch();
        var searchType = JotoTools.getCurrentSearchType();
        var targetPage = $(e.target.parentNode).attr("target-page");
        Util.loadUrl(JotoTools.createUrl(searchValue, searchType, targetPage));
    });
});
