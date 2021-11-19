/*
* This JS-File everything related to the settings overlay
*/

// Cookies that track the user
const trackingCookies = [
    "allow_cookies"
];

// Analytics. Use your own or leave empty
var analyticsUrl = '';
var analyticsAttributes = null;

// Opens the Settings Overlay and accepts cookie usage
function cookiesAccepted(manuallyCalled) {
    Cookies.set("allow_cookies", "1", {path: '/'});
    if (manuallyCalled)
        Util.showMessage("success", "Thanks for making Jotoba better!");

    $('#cookie-footer').addClass("hidden");

    Util.loadScript(analyticsUrl, true, analyticsAttributes);
}

// Revokes the right to store user Cookies
function revokeCookieAgreement(manuallyCalled) {
    $('#cookie-footer').addClass("hidden");

    if (manuallyCalled)
        Util.showMessage("success", "Successfully deleted your cookie data.");

    Util.deleteSelectedCookies(trackingCookies);
    Cookies.set("allow_cookies", "0", {path: '/'});
}

/* ------------------------------------------------------------------- */

// On load, get all the cookie's data and prepare settings overlay
Util.awaitDocumentReady(() => {
    loadCookieData();
    Util.mdlScrollFix(); 
});

// Load the cookie's data into important stuff
function loadCookieData() {

    // Language Settings
    let search_lang = JotoTools.toJotobaLaguage(Cookies.get("default_lang") || navigator.language || navigator.userLanguage || "en-US");
    let page_lang = Cookies.get("page_lang") || "en-US";

    // Search Settings
    let english_always = Util.toBoolean(Cookies.get("show_english"));
    let english_on_top = Util.toBoolean(Cookies.get("show_english_on_top"));
    let example_sentences = Util.toBoolean(Cookies.get("show_sentences"));
    let focus_searchbar = Util.toBoolean(Cookies.get("focus_searchbar"));
    let items_per_page = Cookies.get("items_per_page");
    let kanji_per_page = Cookies.get("kanji_page_size");

    // Display Settings
    let theme = localStorage.getItem("theme");
    let kanji_speed = localStorage.getItem("kanji_speed");

    // Other Settings
    let cookies_allowed = Util.toBoolean(Cookies.get("allow_cookies"));

    // Set essentials
    if (Cookies.get("default_lang") === undefined) {
        Cookies.set("default_lang", search_lang);
    }

    // Execute 
    setLanguageSettings(search_lang, page_lang);
    setSearchSettings(english_always, english_on_top, example_sentences, focus_searchbar, items_per_page, kanji_per_page);
    setDisplaySettings(theme, kanji_speed);
    setOtherSettings(cookies_allowed);
}

// Prepare the language tab
async function setLanguageSettings(search_lang, page_lang) {
    // Set search_lang
    document.querySelectorAll("#search-lang-select > .choices__item--choice").forEach((e) => {
        if (e.dataset.value == search_lang) {
            let choicesInner = e.parentElement.parentElement.parentElement.children[0].children;
             
            choicesInner[0].children[0].innerHTML = e.innerHTML;
            choicesInner[1].children[0].innerHTML = e.innerHTML;
        }
    });

    // Set page_lang
    document.querySelectorAll("#page-lang-select > .choices__item--choice").forEach((e) => {
        if (e.dataset.value == page_lang) {
            let choicesInner = e.parentElement.parentElement.parentElement.children[0].children;
            
            choicesInner[0].children[0].innerHTML = e.innerHTML;
            choicesInner[1].children[0].innerHTML = e.innerHTML;
        }
    });
}

// Prepare the search tab
async function setSearchSettings(english_always, english_on_top, example_sentences, focus_searchbar, items_per_page, kanji_per_page) {
    // Set checkboxes
    Util.setMdlCheckboxState("show_eng_settings", english_always);
    Util.setMdlCheckboxState("show_eng_on_top_settings", english_on_top);
    Util.setMdlCheckboxState("show_example_sentences_settings", example_sentences);
    Util.setMdlCheckboxState("focus_search_bar_settings", focus_searchbar);

    // Hide english_on_top if not english_always
    if (!english_always) {
        $('#eng_on_top_parent').addClass("hidden");
    } else {
        $('#eng_on_top_parent').removeClass("hidden");
    }

    // Default items val
    if (items_per_page == undefined) {
       items_per_page = 10;
    }

    // Set items val
    let itemsInput = $('#items_per_page_input');
    itemsInput.val(items_per_page);
    itemsInput.parent().addClass("is-dirty")

    // Default kanji val
    if (kanji_per_page == undefined) {
        kanji_per_page = 4;
    }

    // Set kanji val
    let kanjiInput =  $('#kanji_per_page_input');
    kanjiInput.val(kanji_per_page);
    kanjiInput.parent().addClass("is-dirty")
}

// Prepare the display tab
async function setDisplaySettings(theme, kanji_speed) {
    // Light / Dark Mode toggle
    Util.setMdlCheckboxState("use_dark_mode_settings", theme === "dark");

    // Kanji speed
    if (kanji_speed === undefined) {
        kanji_speed = 1;
    }
    $('#show_anim_speed_settings').val(kanji_speed);
    $('#show_anim_speed_settings_slider').html(kanji_speed);
}

// Prepare the others tab
async function setOtherSettings(allow_cookies) {
    if (allow_cookies === undefined) {
        Cookies.set("allow_cookies", false);
    } 

    Util.setMdlCheckboxState("cookie_settings", allow_cookies);
}

// Handles an event caused by an input field
function onInputSettingsChange(relatedCookie, event) {
    let value = event.target.value;

    if (value > 0 && value < 101) {
        Cookies.set(relatedCookie, event.target.value);
    } else {
        event.target.value = Cookies.get(relatedCookie);
        $(event.target).parent().addClass("is-dirty");
    }
}

// Handles an event caused by a settings-btn
function onBtnSettingsChange(relatedCookie, event) {
    Cookies.set(relatedCookie, event.target.checked);
}

// Special handling for english_always
function onBtnSettingsChange_englishAlways(event) {
    // Hide english_on_top if not english_always
    if (!event.target.checked) {
        $('#eng_on_top_parent').addClass("hidden");
    } else {
        $('#eng_on_top_parent').removeClass("hidden");
    }

    onBtnSettingsChange("show_english", event);
}

// Special handling for use_darkmode
function onBtnSettingsChange_darkTheme(event) {
    if (event.target.checked) {
        setTheme("dark");
    } else {
        setTheme("light");
    }
}

// Special handling for allow_cookies
function onCookiesAcceptChange(event) {
    if (event.target.checked) {
        cookiesAccepted(true);
    } else {
        revokeCookieAgreement(true);
    }
}

// Changes the Default Language to search for
function onSettingsChange_DefaultLanguage(html, value) {
    Cookies.set('default_lang', value, {path: '/'});
    if (window.location.href.includes("/search")) {
        location.reload();
    }
}

// Changes the Page's UI Language
function onSettingsChange_PageLanguage(html, value) {
    Cookies.set('page_lang', value, {path: '/'});
    location.reload();
}

// Sets the default kanji animation speed
function onSettingsChange_AnimationSpeed(event) {
    $('#show_anim_speed_settings_slider').html(event.target.value);
    localStorage.setItem('kanji_speed', event.target.value);
}
