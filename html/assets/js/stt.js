/**
 * This JS-File implements the Speech to Text functionality for text inpu
 */

var SpeechRecognition = SpeechRecognition || webkitSpeechRecognition;
var recognition = new SpeechRecognition();

// Settings
recognition.lang = 'en-US';
recognition.continuous = false;
recognition.interimResults = false;
recognition.maxAlternatives = 1;

// On recognition start
recognition.onstart = function() {
    $('#currentlyListening').html("Yes");
};

// On recognition error
recognition.onerror  = function(event) { 
    console.log(event.error);
    switch(event.error) {
        case "not-allowed":
            showMessage("error", "Need permissions to perform speech recognition!");
            break;
        case "aborted":
        case "no-speech":
            break;
        default:
            showMessage("error", "Your browser does not support speech recognition!");
    }
    $('#currentlyListening').html("No");
}

// On speech end
recognition.onspeechend = function() {
    recognition.stop();
    $('#currentlyListening').html("No");
}

// On recognition result
recognition.onresult = function(event) {
    let transcript = event.results[0][0].transcript;
    $('#search').val(transcript);
};

// Toggles the overlay on and off
function toggleSpeakOverlay() {
    let overlay = $('.speech-overlay');
    overlay.toggleClass('hidden');

    if (overlay.hasClass("hidden")) {
        recognition.stop();
    }
}

// Activate the given language for speech recognition TODO save in cookie
function setRecognitionLang(lang) {
    recognition.abort();

    switch(lang) {
        case "jap":
            recognition.lang = "ja";
            $('#currentSpeechLang').html("Japanese");
            break
        case "ger":
            recognition.lang = "de-DE";
            $('#currentSpeechLang').html("German");
            break
        case "eng":
            recognition.lang = "en-US";
            $('#currentSpeechLang').html("English");
            break
        case "spa":
            recognition.lang = "es-ES";
            $('#currentSpeechLang').html("Spanish");
            break
        case "swe":
            recognition.lang = "sv-SE";
            $('#currentSpeechLang').html("Swedish");
            break
        case "fre":
            recognition.lang = "fr-FR";
            $('#currentSpeechLang').html("French");
            break
        case "dut":
            recognition.lang = "nl-NL";
            $('#currentSpeechLang').html("Dutch");
            break
        case "hun":
            recognition.lang = "hu";
            $('#currentSpeechLang').html("Hungarian");
            break
        case "slv":
            recognition.lang = "sl-SI";
            $('#currentSpeechLang').html("Slovenian");
            break
    }

    setTimeout(function(){ recognition.start(); }, 400);
}