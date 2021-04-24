/**
 * This JS-File handles saving and loading from browser cookies
 */

// On load, get all the cookie's data
loadCookieData();

// Changes the Background Color
function onSettingsChange_BackgroundColor(event) {
    let color = "#f2f1f0";
    if (event !== undefined)
        color = event.target.value;
    else 
        $('#bg_col_settings').val(color);

    Cookies.set('bg_color', color);
    document.documentElement.style.setProperty('--background', color);
}

// Changes the Primary Page Color (--green)
function onSettingsChange_PrimaryColor(event) {
    let color = "#34a83c";
    if (event !== undefined)
        color = event.target.value;
    else 
        $('#prim_col_settings').val(color);

    Cookies.set('prim_color', color);
    document.documentElement.style.setProperty('--primaryColor', color);
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
    let bg_color = Cookies.get("bg_color");
    let prim_color = Cookies.get("prim_color");
    let default_lang = Cookies.get("default_lang");
    let show_english = Cookies.get("show_english");

    // Background
    if (bg_color === undefined)
        bg_color = "#f2f1f0";

    $('#bg_col_settings').val(bg_color);
    document.documentElement.style.setProperty('--background', bg_color);

    // Primary 
    if (prim_color === undefined)
        prim_color = "#34a83c";

    $('#bg_col_settings').val(prim_color);
    document.documentElement.style.setProperty('--primaryColor', prim_color);

    // Default_Lang 
    if (default_lang !== undefined)
        $('#default_lang_settings').val(default_lang);
    else 
        $('#default_lang_settings').val(navigator.language);

    // English results
    if (show_english === "true")
        $('#show_eng_settings').val(true);
    else 
        $('#show_eng_settings').val(false);
}