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

// Copies Kanji (right side) to clipboard on click
$('.kanji-preview.large').on("click", (event) => {
    showMessage("success", "kanji copied to clipboard.");
    copyToClipboard(event.target.innerHTML.trim());
});

// Copies Furigana to clipboard on click
$('.furigana-kanji-container > .furigana-preview').on("click", (event) => {
    showMessage("success", "furigana copied to clipboard.");
    copyToClipboard($(event.target).html().trim());
});

// Copies translations to clipboard on double click
$('.furigana-kanji-container > .kanji-preview').on("dblclick", (event) => {
	event.preventDefault();
    showMessage("success", "translation copied to clipboard.");
    copyTranslation(event.target.parentElement.parentElement);
});

// Copies translations to clipboard on double click
$('.inline-kana-preview').on("dblclick", (event) => {
	event.preventDefault();
    showMessage("success", "translation copied to clipboard.");
    copyTranslation(event.target.parentElement);
});

// Used by kanji/kana copy to combine all parts, starts from the flex (parent)
function copyTranslation(textParent) {
    // Find all childs that are of interest
    let fullTranslation = "";
    textParent.childNodes.forEach((entry) => {
        if (entry.classList != undefined) {
            // Kanji
            if (entry.classList.contains("furigana-kanji-container")) {
                entry.childNodes.forEach((subEntry) => {
                    if (subEntry.classList != undefined && subEntry.classList.contains("kanji-preview")) {
                        fullTranslation += subEntry.innerHTML.trim();
                    }
                });
            }
            // Kana
            if (entry.classList.contains("inline-kana-preview")) {
                fullTranslation += entry.innerHTML.trim();
            }

        } 
    });
    copyToClipboard(fullTranslation);
}