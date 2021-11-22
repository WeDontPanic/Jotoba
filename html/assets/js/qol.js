/**
 * This JS-File contains some Quality of Life improvements for the website
 */

var shiftPressed = false;

// Prevent random dragging of <a> elements
$('a').mousedown((event) => {
    event.preventDefault();
});

$(document).on('keyup keydown keypress', function(e){ shiftPressed = e.shiftKey} );

// Key Events for easy usability
$(document).on("keypress", (event) => {
    if ($('input:text').is(":focus")) return;
    
    switch (event.key) {
        case '/': // Focus search bar
            event.preventDefault();
            $('#search').focus();
            $('#search').select();
            break
        case 'w': // Focus search bar
            changeSearchType(null, "0");
            break;
        case 'k': // Change to Word Tab
            changeSearchType(null, "1");
            break;
        case 's': // Change to Sentence Tab
            changeSearchType(null, "2");
            break;
        case 'n': // Change to Names Tab
            changeSearchType(null, "3");
            break;
        case 'p': // Play first Audio on page
            $(".audioBtn").first().trigger("click");
            break;
        case "Enter": // Do a search while rad-picker is opened
            if (!$(".overlay.radical").hasClass("hidden")) {
               $(".btn-search").click();
            }
            break;
        default:
            if (event.key > 0 && event.key < 10) {
                let kanji = $('.kanji-preview.large.black')[event.key-1]
                if (kanji !== undefined) {
                    kanji.click();
                }
            }
    }
});

// Copies Furigana to clipboard on click
$('.furigana-preview').on("click", (event) => {
    preventDefaultHighlight(event, 100, true, false);
    Util.showMessage("success", "furigana copied to clipboard.");
    Util.copyToClipboard($(event.target).html().trim());
});

// Copies full Furigana to clipboard on dblclick
$('.furigana-preview').on("dblclick", (event) => {
    // Disable Events for a short time
    preventDefaultHighlight(event, 100, false);

    // Show the correct message
    $('.msg-message.msg-success.msg-visible').last().remove();
    $('.msg-message.msg-success.msg-visible').last().html("<b>full</b> furigana copied to clipboard");

    // Find all furigana
    let parent = $(event.target.parentElement.parentElement);
        let furi = "";
        parent.find('.furigana-preview, .inline-kana-preview').each((i, element) => {
            furi += element.innerHTML.trim();
        });
        Util.copyToClipboard(furi);
    });

// Copies translations to clipboard on double click
$('.kanji-preview').on("dblclick", (event) => {
    preventDefaultHighlight(event, 500, false);
    copyTranslationAndShowMessage(event.target.parentElement.parentElement);
});

// Copies translations to clipboard on double click
$('.inline-kana-preview').on("dblclick", (event) => {
    preventDefaultHighlight(event, 500, false);
    copyTranslationAndShowMessage(event.target.parentElement);
});

// Prevents the default User highlighting
function preventDefaultHighlight(event, timeoutDurationMs, disableClick, disableDoubleClick) {
    startEventTimeout(event.target, timeoutDurationMs, disableClick, disableDoubleClick);
	event.preventDefault();
    Util.deleteSelection();
}

// Disbaled onclick events for a short period of time
function startEventTimeout(targetElement, durationMs, disableClick = true, disableDoubleClick = true) {
    // Disbale events for single clicks
    if (disableClick) {
        let eventFunc = $._data(targetElement, "events").click[0].handler;
        $._data(targetElement, "events").click[0].handler = () => {};    
        setTimeout(() => {
            $._data(targetElement, "events").click[0].handler = eventFunc;
        }, durationMs);
    }

    // Disable events for double clicks
    if (disableDoubleClick) {
        let eventFuncDbl = $._data(targetElement, "events").dblclick[0].handler;
        $._data(targetElement, "events").dblclick[0].handler = () => {};
    
        setTimeout(() => {
            $._data(targetElement, "events").dblclick[0].handler = eventFuncDbl;
        }, durationMs);
    }    
}

// Used by kanji/kana copy to combine all parts, starts from the flex (parent)
function copyTranslationAndShowMessage(textParent) {
    let fullContent = "";
    let onlyKanji = true;

    // Find all childs that are of interest
    $(textParent).find('.kanji-preview, .inline-kana-preview').each((i, element) => {
        let txt = element.innerHTML.trim();
        fullContent += txt
        if (txt.charCodeAt(0) < 19968) {
            onlyKanji = false;
        }
    });

    // Copy and visual feedback
    Util.copyToClipboard(fullContent);
    Util.showMessage("success", onlyKanji ? "kanji copied to clipboard." : "meaning copied to clipboard.");
}

// Changes the search type in the upper row depending on the users input
function changeSearchType(html, newType) {
    var search_value = $('#search').val();
    if (search_value.length > 0) {
        Util.loadUrl(JotoTools.createUrl(search_value, newType));
    }
}

// Focus Search Bar on load if the user wants it to (or on index page)
Util.awaitDocumentReady(() => {
    if (Util.toBoolean(Cookies.get("focus_searchbar")) || document.location.href.slice(0, -1) == document.location.origin) {
        preventNextApiCall = true;
        $('#search').focus();
        Util.setCaretPosition("search", -1);
    }
});

// Wait for the Document to load completely
Util.awaitDocumentReady(() => {

    // Iterate all audio Btns on the page (if any) and enable their audio feature
    $('.audioBtn').each((e, i) => {
        let audioParent = $(i);

        audioParent.click((e) => {
            let audio = $(e.target).children()[0];
            audio.play();
        });
    });

    // Allow right-click on "Play audio" buttons to copy the proper asset-url
    $(".audioBtn").contextmenu((event) => {
        event.preventDefault();
        var url = window.location.origin + $(event.target).attr('data');
        Util.copyToClipboard(url);
        Util.showMessage("success", "Audio URL copied to clipboard");
    });

    // Disables the dropdown's animation until the first onclick event
    $(".input-field.first-wrap").one("click", (event) => {
        $('.choices__list.choices__list--dropdown.index').addClass('animate');
    })
    
    // Install the serviceWorker for PWA
    if ('serviceWorker' in navigator) {
        navigator.serviceWorker.register('/service-worker.js')
        .catch(function(error) {
          console.log('Service worker registration failed, error:', error);
        });
    }

});
