/**
 * This JS-File handles saving and loading from browser cookies
 */

// Array containing all ids of color settings
let colorIdentifiers = [
    "background_value", "overlay_value", "primaryColor_value", "bgPrimaryColor_value", "secondaryColor_value",
    "primaryTextColor_value", "secondaryTextColor_value", "searchBackground_value", "searchTextColor_value", "tagColor_value", 
    "itemBG_value"
];

// Arrays for color coding
let colorCodings = [
    "0gú*+q", "1hó&-r", "2ií$%s", "3jté.M", "4ku~,N", "5lv\\§O", "6mw}/P", "7nx]:Q", "8oy[;R", "9pz{?S",
    "AaG@)T", "BbH°(U", "CdI^_V", "DYJ´!W", "EeK'#X", "FfL=áZ",
]

/* ------------------------------------------------------------------- */

// On load, get all the cookie's data
parseSchemeCode("1A1A1C252527C3083F9407416Fi32ZZZOR6Fi32");
loadCookieData();

// Opens the Settings Overlay and accepts cookie usage
function cookiesAccepted() {
    Cookies.set("user_agreement", true);
    let settingsBtns =  $('.settingsBtn')
        settingsBtns.each((i, e) => {
            e.dataset.target = "#settingsModal";
        });

    setTimeout(function() {
        if (!$('#settingsModal').hasClass("show")) 
            settingsBtns[0].click();
    }, 400);
}

// Revokes the right to store user Cookies
function revokeCookieAgreement() {
    Util.deleteCookies();

    $('.settingsBtn').each((i, e) => {
        e.dataset.target = "#cookiesModal";
    });

    $('#settingsModal').modal('hide');

    showMessage("success", "Successfully deleted your cookie data.")
}

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

    setSpecialColorVars();
    
    let code = createSchemeCode()
    Cookies.set('scheme_code', code);
}

// Returns the value of the given CSS-Variable's name
function getCssValue(cssName) {
    return getComputedStyle(document.documentElement).getPropertyValue(cssName).trim();
}

// Changes the Default Language to search for
function onSettingsChange_DefaultLanguage(event) {
    Cookies.set('default_lang', event.target.value);
    if (window.location.href.includes("/search")) {
        location.reload();
    }
}

// Changes whether english results should be shown
function onSettingsChange_ShowEnglish(event) {
    Cookies.set('show_english', event.target.checked);
    if (!event.target.checked)
        $('#show_eng_on_top_settings_parent').addClass("hidden");
    else
        $('#show_eng_on_top_settings_parent').removeClass("hidden");
    
}

// Changes whether english results should be shown on top
function onSettingsChange_ShowEnglishOnTop(event) {
    Cookies.set('show_english_on_top', event.target.checked);
}

// Sets the default kanji animation speed
function onSettingsChange_AnimationSpeed(event) {
    $('#show_anim_speed_settings_slider').html(event.target.value);
    Cookies.set('anim_speed', event.target.value);
}

// Load the cookie's data into important stuff
function loadCookieData() {

    // User agreement on using Cookies
    let user_agreement = Cookies.get("user_agreement");

    // Load search language
    let default_lang = Cookies.get("default_lang");

    // Load result settings
    let show_english = Cookies.get("show_english");
    let show_english_on_top = Cookies.get("show_english_on_top");

    // Load display settings
    let anim_speed = Cookies.get("anim_speed");

    // Adjust settings btn if user already accepted cookies
    if (user_agreement !== undefined) {
        $('.settingsBtn').each((i, e) => {
            e.dataset.target = "#settingsModal";
        });
    }

    // Set Default_Lang 
    if (default_lang !== undefined)
        $('#default_lang_settings').val(default_lang);
    else 
        $('#default_lang_settings').val(navigator.language);

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

    // Load all Color Data
    let scheme_code = Cookies.get("scheme_code");

    if (scheme_code !== undefined) {
        parseSchemeCode(scheme_code)
        setSpecialColorVars();
    }
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
    });

    setSpecialColorVars();
}

// Sets variables with (e.g.) lower opacity
function setSpecialColorVars() {
    let hexVal_itemBg = getComputedStyle(document.documentElement).getPropertyValue("--itemBG").trim();
    let rgbVal_itemBg = hexToRgb(hexVal_itemBg);
    document.documentElement.style.setProperty("--itemBG_075", "rgba("+rgbVal_itemBg.r+","+rgbVal_itemBg.g+","+rgbVal_itemBg.b+",0.75)");

    let hexVal_lineColor = getComputedStyle(document.documentElement).getPropertyValue("--primaryTextColor").trim();
    let rgbVal_lineColor = hexToRgb(hexVal_lineColor);
    document.documentElement.style.setProperty("--lineColor", "rgba("+rgbVal_lineColor.r+","+rgbVal_lineColor.g+","+rgbVal_lineColor.b+",0.1)");

}

// Calculates a code from all identifiers
function createSchemeCode(showCode) {
    let colorCode = "";

    let currentNum = -1;
    let count = 0;

    // Iterate all entries
    colorIdentifiers.forEach((id, index) => {
        let color = $('#'+id).val().substring(1);
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

            // Handle last element
            if (index == colorIdentifiers.length - 1 && i == color.length - 1) {
                colorCode += colorCodings[num].charAt(count);
            }
        }

        // Handle max counts
        if (count === 5) {
            colorCode += colorCodings[currentNum].charAt(count);
            currentNum = -1;
            count = 0;
        }
    });

    if (showCode) {
       $("#scheme_input").val(colorCode);
    }
    return colorCode;
}

function parseSchemeCode(colorCode) {
    // Get color code
    if (colorCode === undefined) {
        colorCode = $("#scheme_input").val();
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
            showMessage("error", "Please enter a valid code. (Index = -1)");
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
    Cookies.set('scheme_code', colorCode);
}

// Returns the Kanji's default speed
function getDefaultAnimSpeed() {
    let speed = Cookies.get("anim_speed");
    if (speed === undefined) {
        speed = 1;
    }

    return speed;
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
