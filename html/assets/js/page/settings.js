/**
 * This JS-File handles saving and loading from browser cookies
 */

// Cookies that track the user
const trackingCookies = [
    "allow_cookies"
];

// Analytics. Use your own or leave empty
var analyticsUrl = '';
var analyticsAttributes = null;

/* ------------------------------------------------------------------- */

// On load, get all the cookie's data
loadCookieData();

// Deletes all stored cookies
function deleteCookies(deleteAll) {
    var allCookies = document.cookie.split(';');
                
    for (var i = 0; i < allCookies.length; i++) {
        if (deleteAll || (!deleteAll && !trackingCookies.includes(allCookies[i]))) {
            document.cookie = allCookies[i] + "=;expires="+ new Date(0).toUTCString()+";path=/;";
        }
    }
}

// Handle Cookie stuff on load
function prepareCookieSettings(allow_cookies) {
    if (allow_cookies == undefined) {
        $('#cookie-footer').removeClass("hidden");
        $('#cookie-agreement-accept').removeClass("hidden");
    } else if (allow_cookies == "1") {
        $('#cookie-agreement-revoke').removeClass("hidden");
    } else {
        $('#cookie-agreement-accept').removeClass("hidden");
    }
}

// Opens the Settings Overlay and accepts cookie usage
function cookiesAccepted() {
    Cookies.set("allow_cookies", "1", {path: '/'});
    Util.showMessage("success", "Cookies accepted!");

    $('#cookie-footer').addClass("hidden");
    $('#cookie-agreement-accept').addClass("hidden");
    $('#cookie-agreement-revoke').removeClass("hidden");

    Util.loadScript(analyticsUrl, true, analyticsAttributes);
}

// Revokes the right to store user Cookies
function revokeCookieAgreement(manuallyCalled) {
    $('#cookie-footer').addClass("hidden");
    $('#cookie-agreement-accept').removeClass("hidden");
    $('#cookie-agreement-revoke').addClass("hidden");

    if (manuallyCalled) {
        Util.showMessage("success", "Successfully deleted your cookie data.");
        deleteCookies(true);
    } else {
        Util.showMessage("success", "Your personal data will not be saved.");
        deleteCookies(false);
    }

    Cookies.set("allow_cookies", "0", {path: '/'});
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

// Changes whether english results should be shown
function onSettingsChange_ShowEnglish(event) {
    Cookies.set('show_english', event.target.checked, {path: '/'});
    if (!event.target.checked)
        $('#show_eng_on_top_settings_parent').addClass("hidden");
    else
        $('#show_eng_on_top_settings_parent').removeClass("hidden");
    
}

// Changes whether english results should be shown on top
function onSettingsChange_ShowEnglishOnTop(event) {
    Cookies.set('show_english_on_top', event.target.checked, {path: '/'});
}

// Sets the default kanji animation speed
function onSettingsChange_AnimationSpeed(event) {
    $('#show_anim_speed_settings_slider').html(event.target.value);
    Cookies.set('anim_speed', event.target.value, {path: '/'});
}

// Load the cookie's data into important stuff
function loadCookieData() {
    // User agreement on using Cookies
    let allow_cookies = Cookies.get("allow_cookies");
    if (!checkTrackingAllowed()) {
        allow_cookies = "0";
        Cookies.set("allow_cookies", 0);
    }
    prepareCookieSettings(allow_cookies);

    // Load search language
    let default_lang = Cookies.get("default_lang");
    let page_lang = Cookies.get ("page_lang");

    // Load result settings
    let show_english = Cookies.get("show_english");
    let show_english_on_top = Cookies.get("show_english_on_top");

    // Load display settings
    let anim_speed = Cookies.get("anim_speed");

    // Set Default_Lang 
    let userLang = default_lang || navigator.language || navigator.userLanguage || "en-US";
    if (!isSupportedSearchLang(userLang)) {
         userLang = "en-US";
    }

    // Activate by finding the correct 
    Util.awaitDocumentReady(() => {
        
        document.querySelectorAll("#search-lang-select > .choices__item--choice").forEach((e) => {
            if (e.dataset.value == userLang) {
                let choicesInner = e.parentElement.parentElement.parentElement.children[0].children;
                
                choicesInner[0].children[0].innerHTML = e.innerHTML;
                choicesInner[1].children[0].innerHTML = e.innerHTML;
            }
        });
    });

    // Set in cookie selected language
    document.querySelectorAll("#page-lang-select > .choices__item--choice").forEach((e) => {

        if (e.dataset.value == page_lang) {
            let choicesInner = e.parentElement.parentElement.parentElement.children[0].children;
            
            choicesInner[0].children[0].innerHTML = e.innerHTML;
            choicesInner[1].children[0].innerHTML = e.innerHTML;
        }
    });
       
    // Set English results
    if (show_english === "false") {
        $('#show_eng_settings').prop('checked', false);
        $('#show_eng_on_top_settings_parent').addClass("hidden");
    } else {
        $('#show_eng_on_top_settings_parent').removeClass("hidden");
    }
    if (show_english_on_top === "true") {
        $('#show_eng_on_top_settings').prop('checked', true);
    }

    // Load anim speed
    $('#show_anim_speed_settings').val(anim_speed);
    $('#show_anim_speed_settings_slider').html(anim_speed);
}

// Check if the current browsers doesn't want the user to be tracked
function checkTrackingAllowed() {
    try {
        if (window.doNotTrack || navigator.doNotTrack || navigator.msDoNotTrack || 'msTrackingProtectionEnabled' in window.external) {
            if (window.doNotTrack == "1" || navigator.doNotTrack == "yes" || navigator.doNotTrack == "1" || navigator.msDoNotTrack == "1") {
                return false;
            } else {
                return true;
            }
        } else {
            return true;
        }
    } catch (e) {
        return true;
    }
}

// Returns the Kanji's default speed
function getDefaultAnimSpeed() {
    let speed = Cookies.get("anim_speed");
    if (speed === undefined) {
        speed = 1;
    }

    return speed;
}

// Checks if a given language code is supported as a search lang
function isSupportedSearchLang(code) {
    switch (code) {
        case "en-US":
        case "de-DE":
        case "es-ES":
        case "fr-FR":
        case "nl-NL":
        case "sv-SE":
        case "ru":
        case "hu":
        case "sl-SI":
            return true;
        default:
            return false;
    }
}

// Set all sliders (if any) to their default value
var sliders = $('.speedSlider');
sliders.each(function() {
    this.value = getDefaultAnimSpeed();
    if (this.textField !== undefined) {
        this.textField.innerHTML = "Animation speed: "+ this.value;
    }
});

// Set all kanji animation's initial speed
var kanjis = $('.kanjisvg');
kanjis.each(function() {
    restartAnimation(this, getDefaultAnimSpeed());
});

