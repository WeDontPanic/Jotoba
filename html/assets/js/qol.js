/**
 * This JS-File contains some Quality of Life improvements for the website
 */

var shiftPressed = false;

// Prevent random dragging of <a> elements
$('a').mousedown((event) => {
    event.preventDefault();
});

$(document).on('keyup keydown keypress', function(e){ shiftPressed = e.shiftKey} );

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

// Is it Easter already?
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
        parseSchemeCode("1A1A1C252527C3083F9407416Fi32636363ZC4C4C4OR6Fi32D3D3D3");
        if (Cookies.get("allow_cookies") !== "1") {
            deleteCookies(true);
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
function changeSearchType(html, newType) {
    var search_value = $('#search').val();
    if (search_value.length > 0) {
        window.location = window.location.origin + "/search/" + search_value + "?t=" + newType;
    }
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

// Does this % thingy but only within signed value range: mod(-6,4) == 2 instead of -2
function mod(n, m) {
  return ((n % m) + m) % m;
}
