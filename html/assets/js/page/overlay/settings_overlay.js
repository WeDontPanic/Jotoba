/** This JS file is used for the connection between the settings "backend" and "frontend" */

function OverlaySettings() {}


// Toggles a single element visible / hidden
var toggleSubEntry = function(id, show) {
    if (show) {
        $(id).removeClass("hidden");
    } else {
        $(id).addClass("hidden");
    }
}

// Sets a slider to the given value
var setSliderEntry = function (sliderId, textId, value) {
    $(sliderId).val(Settings.display.kanjiAnimationSpeed.val);
    $(textId).html(Math.round(Settings.display.kanjiAnimationSpeed.val * 100) + "%");
}

// Sets a specific input's value
var setInput = function (id, value) {
    let kanjiInput = $(id);
    kanjiInput.val(value);
    
    if (value) {
        kanjiInput.parent().addClass("is-dirty");
    }
}

// Updates all dropdowns
OverlaySettings.updateDropdowns = function() {
    // "Language" page
    document.querySelectorAll("#search-lang-select > .choices__item--choice").forEach((e) => {
        if (e.dataset.value == Settings.language.searchLang.val) {
            let choicesInner = e.parentElement.parentElement.parentElement.children[0].children;

            choicesInner[0].children[0].innerHTML = e.innerHTML;
            choicesInner[1].children[0].innerHTML = e.innerHTML;
        }
    });
    document.querySelectorAll("#page-lang-select > .choices__item--choice").forEach((e) => {
        if (e.dataset.value == Settings.language.pageLang.val) {
            let choicesInner = e.parentElement.parentElement.parentElement.children[0].children;

            choicesInner[0].children[0].innerHTML = e.innerHTML;
            choicesInner[1].children[0].innerHTML = e.innerHTML;
        }
    });
}

// Updates all checkboxes
OverlaySettings.updateCheckboxes = function() {
    // "Search" page
    Util.setMdlCheckboxState("show_eng_settings", Settings.search.alwaysShowEnglish.val);
    Util.setMdlCheckboxState("show_eng_on_top_settings", Settings.search.showEnglishOnTop.val);
    Util.setMdlCheckboxState("show_example_sentences_settings", Settings.search.showExampleSentences.val);
    Util.setMdlCheckboxState("show_sentence_furigana_settings", Settings.search.showFurigana.val);
    Util.setMdlCheckboxState("focus_search_bar_settings", Settings.search.focusSearchbar.val);
    Util.setMdlCheckboxState("select_searchbar_content_settings", Settings.search.selectSearchbarContent.val);

    // "Display" page
    Util.setMdlCheckboxState("use_dark_mode_settings", Settings.display.theme.val === "dark");
    Util.setMdlCheckboxState("show_kanji_on_load_settings", Settings.display.showKanjiOnLoad.val);
    Util.setMdlCheckboxState("show_kanji_numbers_settings", Settings.display.showKanjiNumbers.val);
    
    // "Other" page
    Util.setMdlCheckboxState("dbl_click_copy_settings", Settings.other.enableDoubleClickCopy.val);
    Util.setMdlCheckboxState("cookie_settings", Settings.other.cookiesAllowed.val);
}

// Updates all Sub entries
OverlaySettings.updateSubEntries = function() {
    // "Search" page
    toggleSubEntry("#eng_on_top_parent", Settings.search.alwaysShowEnglish.val);
    toggleSubEntry("#select_searchbar_content_parent", Settings.search.focusSearchbar.val);
}

// Updates all sliders
OverlaySettings.updateSliders = function() {
    // "Display" page
    setSliderEntry("#show_anim_speed_settings", "#show_anim_speed_settings_slider", Settings.display.kanjiAnimationSpeed.val);
}

// Updates all inputs
OverlaySettings.updateInputs = function() {
    setInput("#items_per_page_input", Settings.search.itemsPerPage.val);
    setInput("#kanji_per_page_input", Settings.search.kanjiPerPage.val);
}