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
            if (window.umami)
                umami('shortcut: /');
            break
        case 'w': // Focus search bar
            changeSearchType(null, "0");
            if (window.umami && Util.isIndexPage())
                umami('shortcut: w');
            break;
        case 'k': // Change to Word Tab
            changeSearchType(null, "1");
            if (window.umami && !Util.isIndexPage())
                umami('shortcut: k');
            break;
        case 's': // Change to Sentence Tab
            changeSearchType(null, "2");
            if (window.umami && !Util.isIndexPage())
                umami('shortcut: s');
            break;
        case 'n': // Change to Names Tab
            changeSearchType(null, "3");
            if (window.umami && !Util.isIndexPage())
                umami('shortcut: n');
            break;
        case 'p': // Play first Audio on page
            $(".audioBtn").first().trigger("click");
            if (window.umami && !Util.isIndexPage())
                umami('shortcut: p');
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
    // Check if element should not be copied
    if (!shouldCopyFurigana(event))
        return;

    // Copy and show message
    preventDefaultHighlight(event, 100, true, false);
    JotoTools.copyTextAndEcho($(event.target).html().trim(), "QOL_FURI_COPIED");
});

// Copies full Furigana to clipboard on dblclick
$('.furigana-preview').on("dblclick", (event) => {
    // Check if element should not be copied
    if (!shouldCopyFurigana(event))
        return;

    // Find all furigana
    let parent = $(event.target.parentElement.parentElement);
    let furi = "";
    parent.find('.furigana-preview, .inline-kana-preview').each((i, element) => {
        furi += element.innerHTML.trim();
    });

    // Copy and show the correct message
    preventDefaultHighlight(event, 100, false);
    Util.copyToClipboard(furi);
    $('.msg-message.msg-success.msg-visible').last().remove();
    $('.msg-message.msg-success.msg-visible').last().html(getText("QOL_FURI_COPIED_ALL"));
});

// Copies translations to clipboard on double click
$('.kanji-preview').on("dblclick", (event) => {
    // Check if element should not be copied
    if (!shouldCopyKanji())
        return;

    // Copy
    preventDefaultHighlight(event, 500, false);
    copyTranslationAndShowMessage(event.target.parentElement.parentElement);
});

// Prevent double click highlight
document.querySelectorAll(".furigana-kanji-container").forEach(container => {
    container.addEventListener('mousedown', function (event) {
        if (event.detail > 1) {
            event.preventDefault();
        }
    }, false);
});

// Copies translations to clipboard on double click
$('.inline-kana-preview').on("dblclick", (event) => {
    // Check if element should not be copied
    if (!shouldCopyKanji())
        return;

    // Copy
    preventDefaultHighlight(event, 500, false);
    copyTranslationAndShowMessage(event.target.parentElement);
});

// <rub>-tag Fix for standard double click 
document.querySelectorAll(".furigana-kanji-container").forEach(container => {
    container.addEventListener("dblclick", () => {
        // Dont do anything if auto-copy is turned on
        if (shouldCopyKanji()) {
            return;
        }

        // Get and clear the selection
        let selection = window.getSelection();
        selection.removeAllRanges();

        // Select all non-furigana children #1 Firefox exclusive: Multiple selection ranges
        if (navigator.userAgent.search("Firefox") > -1) {
            container.childNodes.forEach((child) => {
                var range = document.createRange();
                range.setStartBefore(child);

                if (child.tagName === "RUBY") {
                    range.setEndAfter(child.children[0]);
                } else {
                    range.setEndAfter(child);
                }

	            selection.addRange(range);
            });
        
        // Select all non-furigana children #2
        } else {
            var range = document.createRange();
            range.setStartBefore(container);
            let lastChild = container.lastChild;
            
            if (lastChild.tagName === "RUBY") {
                range.setEndAfter(lastChild.children[0]);
            } else {
                range.setEndAfter(lastChild);
            }
            
            selection.addRange(range);
        }
    });

});

// Check conditions for copying Furigana 
function shouldCopyFurigana(event) {
    // Prevent copying if the text was just a placeholder
    if (event.target.innerHTML == "&nbsp;")
        return false;

    // Prevent if furigana is part of the sentence reader
    if ($(event.target).parents().toArray().includes($("#sr")[0])) {
        return false;
    }

    // Prevent if user has removed the feature
    return Settings.other.enableDoubleClickCopy.val;
}

// Check conditions for copying Kanji 
function shouldCopyKanji() {
    // Prevent if user has removed the feature
    return Settings.other.enableDoubleClickCopy.val;
}

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
    let onlyKana = true;

    // Find all childs that are of interest
    $(textParent).find('.kanji-preview, .inline-kana-preview').each((i, element) => {
        let txt = element.innerHTML.trim();
        fullContent += txt

        for (char of txt) {
            let isKanji = char.match(kanjiRegEx);
            if (isKanji) {
                onlyKana = false;
            } else {
                onlyKanji = false;
            }
        }
    });

    // Copy and visual feedback
    JotoTools.copyTextAndEcho(fullContent,  onlyKanji ? getText("QOL_KANJI_COPIED") : (onlyKana ? getText("QOL_KANA_COPIED") : getText("QOL_SENTENCE_COPIED")))
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
    let focus_searchbar = Util.toBoolean(Cookies.get("focus_searchbar"));
    let is_index = Util.isIndexPage();

    if (focus_searchbar && !is_index) {
        preventNextApiCall = true;
    }

    if (focus_searchbar || is_index) {
        let s = $('#search');
        s.focus();
        Util.setCaretPosition("search", -1);
        if (Util.toBoolean(Cookies.get("select_searchbar_content"))) {
            s[0].setSelectionRange(0, s[0].value.length);
        }
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
        JotoTools.copyTextAndEcho(url, "QOL_AUDIO_COPIED");
    });

    // Disables the dropdown's animation until the first onclick event
    $(".input-field.first-wrap").one("click", (event) => {
        $('.choices__list.choices__list--dropdown.index').addClass('animate');
    })
    
    // Install the serviceWorker for PWA
    if ('serviceWorker' in navigator) {
        navigator.serviceWorker.register('/service-worker.js', {
            scope: "."
        })
        .catch(function(error) {
          console.log('Service worker registration failed, error:', error);
        });
    }

    // Change URL to contain the language code
    if (Util.isInPath("search")) {
        let currentParams = new URLSearchParams(document.location.search);

        let txt = document.getElementById("search").value; 
        let index = currentParams.get("i") || undefined;
        let type = currentParams.get("t") || $('#search-type').val();
        let lang = currentParams.get("l") || Cookies.get("default_lang");
        let page = currentParams.get("p") || $(".pagination-circle.active").html();

        history.replaceState({}, 'Jotoba', JotoTools.createUrl(txt, type, page || 1, lang, index));
    }
});
