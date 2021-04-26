/**
 * This JS-File contains some Quality of Life improvements for the website
 */

// Prevent random dragging of <a> elements
$('a').mousedown((event) => {
    event.preventDefault();
});

// Press / to focus search bar
$(document).on("keypress", (event) => {
    if (event.key === '/') {
        event.preventDefault();
        $('#search').focus();
        $('#search').select();
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
        parseSchemeCode("1A1A1C252527C3083F9407416Fi32Z6Fi32ZOR6Fi32");
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

// Iterate all audio Btns on the page (if any) and enable their audio feature
$('.audioBtn').each((e, i) => {

    let audioParent = $(i);

    audioParent.click((e) => {
        let audio = $(e.target).children()[0];
        audio.play();
    });

});