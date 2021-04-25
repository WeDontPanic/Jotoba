/**
 * This JS-File handles saving and loading from browser cookies
 */

// On load, get all the cookie's data
loadCookieData();

// Changes the color that the caller represents
function onSettingsChange_Color(event) {
    // Find the input div
    let input = $(event.target.parentElement).children('input');
    let id = input.attr("id");
    let cssName = "--" + id.split("_value")[0];
    let color = input[0].value;

    // Reset if clicked
    if (event.target.classList.contains("clickable")) {
        document.documentElement.style.removeProperty(cssName);
        $(input[0]).val(getCssValue(cssName));
    } else { // Set the selected color if not
        document.documentElement.style.setProperty(cssName, color);
    }

    // Set the Cookie
    Cookies.set(id, color);
}

// Returns the value of the given CSS-Variable's name
function getCssValue(cssName) {
    return  getComputedStyle(document.documentElement).getPropertyValue(cssName).trim();
}

// Changes the Default Language to search for
function onSettingsChange_DefaultLanguage(event) {
    Cookies.set('default_lang', event.target.value);
}

// Changes whether english results should be shown
function onSettingsChange_ShowEnglish(event) {
    Cookies.set('show_english', event.target.checked);
}

// Returns the value of Cookie "show_english" if availiable
function showEnglish() {
    let show = Cookies.get('show_english');
    if (show === "true")
        return true;
    else
        return false;
}

// Load the cookie's data into important stuff
function loadCookieData() {
    // Load search data
    let default_lang = Cookies.get("default_lang");
    let show_english = Cookies.get("show_english");

    // Set Default_Lang 
    if (default_lang !== undefined)
        $('#default_lang_settings').val(default_lang);
    else 
        $('#default_lang_settings').val(navigator.language);

    // Set English results
    if (show_english === "false")
    $('#show_eng_settings').prop('checked', false);

    // Load all Color Data
    let colors = [
        "background_value", "overlay_value", "primaryColor_value", "primaryColor_hover_value", "secondaryColor_value",
        "primaryTextColor_value", "secondaryTextColor_value", "searchBackground_value", "searchTextColor_value", "tagColor_value", 
        "scrollBG_value"
    ];
    setColorFromCookie(colors);
}

// Loads all colors form the cookie from a given array of identifiers
function setColorFromCookie(identifiers) {

    // Iterate all entries
    identifiers.forEach(id => {
        // Get the cookie's data
        let color = Cookies.get(id);
        let cssName = "--" + id.split("_value")[0]

        // Set all the Color informations
        if (color === undefined)
            color = getCssValue(cssName);
        $('#'+id).val(color);

        document.documentElement.style.setProperty(cssName, color);
    });

}