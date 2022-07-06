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
    showFullGraph: { isCookie: false, id: "show_full_graph", dataType: "boolean", val: true },
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
    trackingAllowed: { isCookie: false, id: "tracking_allowed", dataType: "boolean", val: true },
    firstVisit: { isCookie: false, id: "first_time", dataType: "boolean", val: true }
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
Settings.trackingAccepted = function (manuallyCalled) {
    if (manuallyCalled)
        Util.showMessage("success", getText("SETTINGS_COOKIE_ACCEPT"));

    Settings.alterOther("trackingAllowed", true);
    loadAnalytics();
    Util.setMdlCheckboxState("tracking_settings", true);
}

// Revokes the right to store user Cookies
Settings.trackingDeclined = function (manuallyCalled) {
    if (manuallyCalled)
        Util.showMessage("success", getText("SETTINGS_COOKIE_REJECT"));

    Settings.alterOther("trackingAllowed", false);
    Util.setMdlCheckboxState("tracking_settings", false);
}

// Special handling for tracking_allowed
Settings.onTrackingAcceptChange = function (allowed) {
    if (allowed) {
        Settings.trackingAccepted(true);
    } else {
        Settings.trackingDeclined(true);
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

    // Load analytics if allowed -> At this points any external source with high prio has already been loaded in and should have overwritten the analytics vars
    if (Settings.other.trackingAllowed.val && analyticsUrl.length > 0) {
        loadAnalytics();
    }
});

function loadAnalytics() {
    Util.awaitDocumentReady(() => {
        Util.loadScript(analyticsUrl, true, analyticsAttributes, () => {
            // Prepare any css-based events after the script is ready
            let buttons = document.querySelectorAll(".p");

            for (var i = 0; i < buttons.length; i++) {
                buttons[i].addEventListener('click', handleEvent);
            }

            function handleEvent(event) {
                if (window.plausible) {
                    let attribute =  event.target.getAttribute('data-p');
                    if (!attribute) return;

                    let eventData = attribute.split(/,(.+)/);
                    let events = [JSON.parse(eventData[0]), JSON.parse(eventData[1] || '{}')];
                    plausible(...events);
                }
            }
        });
    });
}