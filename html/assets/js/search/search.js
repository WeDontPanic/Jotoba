/**
 * This JS-File contains functions handling the website search (e.g. Search suggestions)
 */

// Prepare Search / Voice Icon when loading the page
Util.awaitDocumentReady(() => {
    toggleSearchIcon(0);
});

// Shows the Voice / Search Icon when possible
function toggleSearchIcon(duration) {
    if (document.getElementById("search").value.length == 0) {
        $('#searchBtn.search-embedded-btn').hide(duration);
        $('#voiceBtn.search-embedded-btn').show(duration);
    } else {
        $('#searchBtn.search-embedded-btn').show(duration);
        $('#voiceBtn.search-embedded-btn').hide(duration);
    }
}

// Resets the value of the search input
function emptySearchInput() {
    $('#search').val("");
    $('#search').focus();
    toggleSearchIcon(200);
}

// Returns the substring of what the user already typed for the current suggestion
// If target is not empty, the substring of target will be searched instead
function getCurrentSubstring(target) {
    let currentSubstr = "";
    let foundSubstr = false;

    if (target === undefined) {
        target = currentSuggestion;
    }

  for (let i = target.length; i > 0; i--) {
        currentSubstr = target.substring(0, i).toLowerCase();
        let index = input.value.toLowerCase().lastIndexOf(currentSubstr)

        if (index == -1) {
            continue;
        }

        if (index + currentSubstr.length === input.value.length) {
            foundSubstr = true;
            break;
        }
    }

    return foundSubstr ? currentSubstr : "";
}

// Interrupts the form's submit and makes the user visit the correct page
function onSearchStart() {
    var search_value = $('#search').val();
    var search_type = $('#search-type').val();

    if (search_value.length == 0) {
        Util.loadUrl(JotoTools.createUrl());
    } else {
        Util.loadUrl(JotoTools.createUrl(search_value, search_type));
    }

    return false;
}

// When opening an overlay, scroll it into view
function scrollSearchIntoView() {
    if (document.location.origin+"/" === document.location.href) {
        var top = $('#search').offset().top;
        Util.scrollTo(top, 500);
    }
}

// Closes all overlays connected to the search bar
function closeAllSubSearchbarOverlays(overlayToIgnore) {
    if (overlayToIgnore !== "speech")
        $('.overlay.speech').addClass('hidden');
    if (overlayToIgnore !== "radical") 
        $('.overlay.radical').addClass('hidden');
    if (overlayToIgnore !== "image")
        $('.overlay.image').addClass('hidden');
}

// Opens the Help Page
function openHelpPage() {
    document.getElementsByClassName("infoBtn")[0].classList.remove("new");
    Settings.alterOther("firstVisit", false, );

    Util.loadUrl("/help");
}
