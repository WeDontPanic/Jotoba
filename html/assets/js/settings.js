/**
 * This JS-File handles saving and loading from browser cookies
 */

// Array containing all ids of color settings
let colorIdentifiers = [
    "background_value", "overlay_value", "primaryColor_value", "primaryColor_hover_value", "secondaryColor_value",
    "primaryTextColor_value", "secondaryTextColor_value", "searchBackground_value", "searchTextColor_value", "tagColor_value", 
    "scrollBG_value"
];

// Arrays for color coding
let colorCodings = [
    "0gú*+q", "1hó&-r", "2ií$%s", "3jté.M", "4ku~,N", "5lv\\§O", "6mw}/P", "7nx]:Q", "8oy[;R", "9pz{?S",
    "AaG@)T", "BbH°(U", "CdI^_V", "DYJ´!W", "EeK'#X", "FfL=áZ",
]

/* ------------------------------------------------------------------- */

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
    setColorFromCookie();
}

// Loads all colors form the cookie from a given array of identifiers
function setColorFromCookie() {

    // Iterate all entries
    colorIdentifiers.forEach(id => {
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

// Loads all colors form the given array
function setColorFromArray(array) {

    // Iterate all entries
    colorIdentifiers.forEach((id, index) => {
        // Get the cookie's data
        let color = array[index];
        let cssName = "--" + id.split("_value")[0]

        // Set all the Color informations
        if (color === undefined)
            color = getCssValue(cssName);
        $('#'+id).val(color);

        // Set Property and Cookie
        document.documentElement.style.setProperty(cssName, color);
        Cookies.set(id, color);
    });

}

// Calculates a code from all identifiers
function createSchemeCode() {
    let colorCode = "";

    let currentNum = -1;
    let count = 0;

    // Iterate all entries
    colorIdentifiers.forEach(id => {
        let color = $('#'+id).val().substring(1);
        console.log(color);
        for (var i = 0; i < color.length; i++) {
            
            let num = hex2num_single(color.charAt(i))

            // Count appearance time
            if (currentNum == num) {
                count++;
            } else {
                // Add to colorCode
                if (currentNum !== -1) {
                    colorCode += colorCodings[currentNum].charAt(count);
                }
                
                currentNum = num;
                count = 0;
            }
        }

        if (count === 5) {
            colorCode += colorCodings[currentNum].charAt(count);
            currentNum = -1;
            count = 0;
        }
    });

    $("#scheme_input").val(colorCode);
}

function parseSchemeCode() {

    // Get color code
    let colorCode = $("#scheme_input").val().toUpperChase();

    // Error check
    if (colorCode.length % 6 !== 0) {
        showMessage("error", "Please enter a valid code.");
        return;
    }

    // A string containing all hex values in a row
    let allHex = "";

    // Iterate the colorCode's parts
    for (var i = 0; i < colorCode.length; i++) {    
        // Find where the code appears in
        let arrayIndex = -1;
        let entryIndex = -1;
        for (var j = 0; j < colorCodings.length; j++) {
            entryIndex = colorCodings[j].indexOf(colorCode[i]);
            if (entryIndex != -1) {
                arrayIndex = j;
                break;
            }
        }

        // Code Error
        if (arrayIndex === -1) {
            showMessage("error", "Please enter a valid code.");
            return;
        }

        // Add Hex
        for (var j = 0; j <= entryIndex; j++) {
            allHex += num2hex_single(arrayIndex);
        }
    }

    // Parse Hex String into respective single ones
    let parsedHex = [];
    for (var i = 0; i < allHex.length; i += 6) {   
        parsedHex.push("#"+allHex.substring(i, i+6));
    }

    setColorFromArray(parsedHex);
}
