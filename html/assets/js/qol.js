/**
 * This JS-File contains some Quality of Life improvements for the website
 */

// Press / to focus search bar
$(document).on("keypress", (event) => {
    if (event.key === '/') {
        event.preventDefault();
        $('#search').focus();
    }  
});

/*
// Copies Kanji (right side) to clipboard on click
$('.kanji-preview.large').on("click", (event) => {
    showMessage("success", "kanji copied to clipboard.");
    copyToClipboard(event.target.innerHTML.trim());
});
*/

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