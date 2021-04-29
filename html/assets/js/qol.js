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
        case '/':
            event.preventDefault();
            $('#search').focus();
            $('#search').select();
            break
        case 'w':
            changeSearchType("0");
            break;
        case 'k':
            changeSearchType("1");
            break;
        case 's':
            changeSearchType("2");
            break;
        case 'n':
            changeSearchType("3");
            break;
        case 'p':
            $(".audioBtn").first().trigger("click");
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
    }
});

// Copies Furigana to clipboard on click
$('.furigana-kanji-container > .furigana-preview').on("click", (event) => {
    showMessage("success", "furigana copied to clipboard.");
    copyToClipboard($(event.target).html().trim());
});

// Copies translations to clipboard on double click
$('.furigana-kanji-container > .kanji-preview').on("dblclick", (event) => {
	event.preventDefault();
    copyTranslationAndShowMessage(event.target.parentElement.parentElement);
});

// Copies translations to clipboard on double click
$('.inline-kana-preview').on("dblclick", (event) => {
	event.preventDefault();
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
    copyToClipboard(fullTranslation);
	
	if (onlyKanji) {
		showMessage("success", "kanji copied to clipboard.");
	} else {
		showMessage("success", "meaning copied to clipboard.");
	}
}

// Changes the search type in the upper row depending on the users input
function changeSearchType(newType) {
  var search_value = $('#search').val();
  if (search_value.length > 0) {
    var params = new URLSearchParams(location.search);
    params.set('type', newType);
    params.set('search', search_value);
    window.location.search = params.toString();
  }
}

// Resets the value of the search input
function emptySearchInput() {
    $('#search').val("");
    $('#search').focus();
}

// Jumps to the top or kanji part (mobile only)
function jumpToTop() {
  document.body.scrollTop = 0; // For Safari
  document.documentElement.scrollTop = 0; // For Chrome, Firefox, IE and Opera
}

// The Jmp Buttons
var topBtn = $("#jmp-btn-top");

// Window Scroll checks
window.onscroll = function() {
    console.log(getBrowserWidth());
    if (getBrowserWidth() < 600 && (document.body.scrollTop > 20 || document.documentElement.scrollTop > 20)) {
        topBtn.css("display", "block");
    } else {
        topBtn.css("display", "none");
    }
  }

// Iterate all audio Btns on the page (if any) and enable their audio feature
$('.audioBtn').each((e, i) => {

    let audioParent = $(i);

    audioParent.click((e) => {
        let audio = $(e.target).children()[0];
        audio.play();
    });

});
