/**
 * This JS-File contains some Quality of Life improvements for the website
 */

// Prevent random dragging of <a> elements
$('a').mousedown((event) => {
    event.preventDefault();
});

// Key Events, or how I like to call 'em: The Jojii-Only events
$(document).on("keypress", (event) => {
    if ($('#search').is(":focus")) return;
    
    switch (event.key) {
        case '/': // Focus search bar
            event.preventDefault();
            $('#search').focus();
            $('#search').select();
            break
        case 'w': // Focus search bar
            changeSearchType("0");
            break;
        case 'k': // Change to Word Tab
            changeSearchType("1");
            break;
        case 's': // Change to Sentence Tab
            changeSearchType("2");
            break;
        case 'n': // Change to Names Tab
            changeSearchType("3");
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

// You might just unlock a secret
var lastKeys = [];
var lastKeyDown = -1;
$(document).on("keydown", (event) => {
    if (event.keyCode != lastKeyDown) {
        lastKeyDown = event.keyCode;
        lastKeys.push(event.keyCode);
        if (lastKeys.length == 9) {
            lastKeys.shift();
        }
    }
    if (lastKeys.toString() === "38,40,37,39,37,39,66,65") {
        parseSchemeCode("1A1A1C252527C3083F9407416Fi32ZZZOR6Fi32");
        if (Cookies.get("user_agreement") !== "true") {
            Util.deleteCookies();
        }
    }
});

// Copies Furigana to clipboard on click
$('.furigana-kanji-container > .furigana-preview').on("click", (event) => {
    Util.showMessage("success", "furigana copied to clipboard.");
    Util.copyToClipboard($(event.target).html().trim());
});

// Copies full Furigana to clipboard on dblclick
$('.furigana-kanji-container > .furigana-preview').on("dblclick", (event) => {
    $('.ajs-message.ajs-success.ajs-visible').last().remove();
    $('.ajs-message.ajs-success.ajs-visible').last().html("<b>full</b> furigana copied to clipboard");

    // Find all furigana
    let parent = $(event.target.parentElement.parentElement);
    let furi = "";
    parent.find('.furigana-preview, .inline-kana-preview').each((i, element) => {
        furi += element.innerHTML.trim();
    });
    Util.copyToClipboard(furi);
});

// Copies translations to clipboard on double click
$('.furigana-kanji-container > .kanji-preview').on("dblclick copy", (event) => {
	event.preventDefault();
    Util.deleteSelection();
    copyTranslationAndShowMessage(event.target.parentElement.parentElement);
});

// Copies translations to clipboard on double click
$('.inline-kana-preview').on("dblclick copy", (event) => {
	event.preventDefault();
    Util.deleteSelection();
    copyTranslationAndShowMessage(event.target.parentElement);
});

// Used by kanji/kana copy to combine all parts, starts from the flex (parent)
function copyTranslationAndShowMessage(textParent) {
    // Find all childs that are of interest
    let fullTranslation = "";
    let onlyKanji = true;
    textParent.childNodes.forEach((entry) => {
        if (entry.classList != undefined) {
            // Kanji
            if (entry.classList.contains("furigana-kanji-container")) {
                entry.childNodes.forEach((subEntry) => {
                    if (subEntry.classList != undefined && subEntry.classList.contains("kanji-preview")) {
                        let text = subEntry.innerHTML.trim();
                        fullTranslation += text;
                        if (text.charCodeAt(0) < 19968)
                            onlyKanji = false;
                    }
                });
            }
            // Kana
            if (entry.classList.contains("inline-kana-preview")) {
                let text = entry.innerHTML.trim();
                fullTranslation += text;
                if (text.charCodeAt(0) < 19968)
                    onlyKanji = false;
            }
        } 
    });
    Util.copyToClipboard(fullTranslation);
	
	if (onlyKanji) {
		Util.showMessage("success", "kanji copied to clipboard.");
	} else {
		Util.showMessage("success", "meaning copied to clipboard.");
	}
}

// Changes the search type in the upper row depending on the users input
function changeSearchType(newType) {
    var search_value = $('#search').val();
    if (search_value.length > 0) {
        var params = new URLSearchParams();
        params.set('type', newType);
        params.set('search', search_value);
        window.location = window.location.origin + "/search?" + params.toString();
    }
}

// Resets the value of the search input
function emptySearchInput() {
    $('#search').val("");
    $('#search').focus();
}

// Focus Search Bar on index page
$(document).ready(() => {
    if (window.location.href.substring(0,window.location.href.length - 1) == window.location.origin) {
        $('#search').focus();
        $('#search').select();
    }
});

// Iterate all audio Btns on the page (if any) and enable their audio feature
$('.audioBtn').each((e, i) => {
    let audioParent = $(i);

    audioParent.click((e) => {
        let audio = $(e.target).children()[0];
        audio.play();
    });

});
