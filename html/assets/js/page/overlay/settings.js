/*
* This JS-File everything related to the settings overlay
*/

function Settings() { }

// Analytics. Use your own or leave empty
var analyticsUrl = '';
var analyticsAttributes = null;

// Default "language" settings
Settings.language = {
    searchLang: { isCookie: true, id: "default_lang", dataType: "string", val: JotoTools.toJotobaLanguage(Cookies.get("default_lang") || navigator.language || navigator.userLanguage || "en-US") },
    pageLang: { isCookie: true, id: "page_lang", dataType: "string", val: Cookies.get("page_lang") || "en-US" },
}

// Default "search" settings
Settings.search = {
    alwaysShowEnglish: { isCookie: true, id: "show_english", dataType: "boolean", val: true },
    showEnglishOnTop: { isCookie: true, id: "show_english_on_top", dataType: "boolean", val: false },
    showExampleSentences: { isCookie: true, id: "show_sentences", dataType: "boolean", val: true },
    showFurigana: { isCookie: true, id: "sentence_furigana", dataType: "boolean", val: true },
    focusSearchbar: { isCookie: false, id: "focus_searchbar", dataType: "boolean", val: false },
    selectSearchbarContent: { isCookie: false, id: "select_searchbar_content", dataType: "boolean", val: false },
    itemsPerPage: { isCookie: true, id: "items_per_page", dataType: "int", val: 10 },
    kanjiPerPage: { isCookie: true, id: "kanji_page_size", dataType: "int", val: 4 },
}

// Default "display" settings
Settings.display = {
    theme: { isCookie: false, id: "theme", dataType: "string", val: "light" },
    kanjiAnimationSpeed: { isCookie: false, id: "kanji_speed", dataType: "float", val: 1 },
    showKanjiOnLoad: { isCookie: false, id: "show_kanji_on_load", dataType: "boolean", val: true },
    showKanjiNumbers: { isCookie: false, id: "show_kanji_numbers", dataType: "boolean", val: false },
}

// Default "other" settings
Settings.other = {
    enableDoubleClickCopy: { isCookie: false, id: "dbl_click_copy", dataType: "boolean", val: true },
    cookiesAllowed: { isCookie: false, id: "allow_cookies", dataType: "int", val: 0 },
    firstVisit: { isCookie: false, id: "first_time", dataType: "boolean", val: true },
    privacyChosen: { isCookie: false, id: "privacy_chosen", dataType: "boolean", val: false },
}

// Saves a settings-object into localStorage / Cookies
Settings.saveSettings = function (object) {
    for (let [key, entry] of Object.entries(object)) {
        if (entry.isCookie) {
            Cookies.set(entry.id, entry.val, { path: '/', expires: 365 });
        } else {
            localStorage.setItem(entry.id, entry.val);
        }
    }
}

// Loads a settings-object from localStorage / Cookies
Settings.loadSettings = function (object) {
    for (let [key, entry] of Object.entries(object)) {
        let data = "";

        // Try to get the data
        if (entry.isCookie) {
            data = Cookies.get(entry.id, entry.val);
        } else {
            data = localStorage.getItem(entry.id);
        }

        // Not found => ignore
        if (!data) {
            continue;
        }

        // Found => parse and overwrite
        switch (entry.dataType) {
            case "boolean":
                object[key].val = Util.toBoolean(data);
                break;
            case "int":
                object[key].val = parseInt(data);
                break;
            case "float":
                object[key].val = parseFloat(data);
                break;
            default:
                object[key].val = data;
        }
    }
}

// Alters a "language" setting and reloads if needed
Settings.alterLanguage = function (key, value, reloadPage) {
    Settings.language[key].val = value;
    Settings.saveSettings(Settings.language);

    if (reloadPage) {
        location.reload();
    }
}

// Used for the Choices-Hook on function calls
alterLanguage_search = function (html, value) {
    let reloadPage = window.location.href.includes("/search");
    Settings.alterLanguage("searchLang", value, reloadPage);
}

// Used for the Choices-Hook on function calls
alterLanguage_page = function (html, value) {
    Settings.alterLanguage("pageLang", value, true);
}

// Alters a "search" setting and reloads if needed
Settings.alterSearch = function (key, value, updateSub) {
    Settings.search[key].val = value;
    Settings.saveSettings(Settings.search);

    if (updateSub) {
        OverlaySettings.updateSubEntries();
    }
}

// Alters a "display" setting and reloads if needed
Settings.alterDisplay = function (key, value) {
    Settings.display[key].val = value;
    Settings.saveSettings(Settings.display);
}

// Alters a "other" setting and reloads if needed
Settings.alterOther = function (key, value) {
    Settings.other[key].val = value;
    Settings.saveSettings(Settings.other);
}

// Opens the Settings Overlay and accepts cookie usage
Settings.cookiesAccepted = function (manuallyCalled) {
    Settings.alterOther("cookiesAllowed", "1");

    if (manuallyCalled)
        Util.showMessage("success", getText("SETTINGS_COOKIE_ACCEPT"));

    $('#cookie-footer').addClass("hidden");

    Util.loadScript(analyticsUrl, true, analyticsAttributes);
    Util.setMdlCheckboxState("cookie_settings", true);
    Settings.alterOther("privacyChosen", true);
}

// Revokes the right to store user Cookies
Settings.revokeCookieAgreement = function (manuallyCalled) {
    $('#cookie-footer').addClass("hidden");

    if (manuallyCalled)
        Util.showMessage("success", getText("SETTINGS_COOKIE_REJECT"));

    Cookies.set("allow_cookies", "0", { path: '/', expires: 365 });
    Util.setMdlCheckboxState("cookie_settings", false);
    Settings.alterOther("privacyChosen", true);
}

// Special handling for allow_cookies
Settings.onCookiesAcceptChange = function (allowed) {
    if (allowed) {
        Settings.cookiesAccepted(true);
    } else {
        Settings.revokeCookieAgreement(true);
    }
}

// Prepare the settings overlay's data initially
async function prepareSettingsOverlay() {

    // Prepare the Settings Overlay
    OverlaySettings.updateDropdowns();
    OverlaySettings.updateCheckboxes();
    OverlaySettings.updateSubEntries();
    OverlaySettings.updateSliders();
    OverlaySettings.updateInputs();

    // Show Cookie footer if needed
    if (!Settings.other.privacyChosen.val) {
        $("#cookie-footer").removeClass("hidden");
    }
};

// Load Settings on initial load
Util.awaitDocumentInteractive(() => {
    Settings.loadSettings(Settings.search);
    Settings.loadSettings(Settings.display);
    Settings.loadSettings(Settings.other);
});

Util.awaitDocumentReady(() => {
    Settings.loadSettings(Settings.language);
    prepareSettingsOverlay();

    // Add the info-icon on initial page load if needed
    if (Settings.other.firstVisit.val) {
        $(".infoBtn").addClass("new");
    }
});