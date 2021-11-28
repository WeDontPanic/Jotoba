/**
 * This JS-File implements the Speech to Text functionality for text input
 */

var SpeechRecognition, recognition;

try {
    SpeechRecognition = SpeechRecognition || webkitSpeechRecognition;
    recognition = new SpeechRecognition();

    recognitionSetup();
} catch (e) {}

// Handles the initial setup of the recognition lib 
function recognitionSetup() {
    recognition.lang = 'en-US';
    recognition.continuous = false;
    recognition.interimResults = false;
    recognition.maxAlternatives = 1;
    
    // On recognition start
    recognition.onstart = function() {
        $('#currentlyListening').html("Yes");
        $('.voiceSvg').toggleClass("active");
    };
    
    // On recognition error
    recognition.onerror  = function(event) { 
        console.log(event.error);
        switch(event.error) {
            case "not-allowed":
                Util.showMessage("error", "Need permissions to perform speech recognition!");
                break;
            case "aborted":
                Util.showMessage("info", "Speech recognition aborted.");
                break;
            case "no-speech":
                Util.showMessage("info", "No voice input received!");
                break;
            default:
                Util.showMessage("error", "Your browser does not support speech recognition!");
        }
        $('#currentlyListening').html("No");
        $('.voiceSvg').toggleClass("active");
    }
    
    // On speech end
    recognition.onspeechend = function() {
        recognition.stop();
        $('#currentlyListening').html("No");
        $('.voiceSvg').toggleClass("active");
    }
    
    // On recognition result
    recognition.onresult = function(event) {
        let transcript = event.results[0][0].transcript;
        $('#search').val(transcript);
    };
}

// Toggles the overlay on and off
function toggleSpeakOverlay() {
    if (recognition == undefined) {
        Util.showMessage("error", "This feature is not supported in your browser.");
        return;
    }

    closeAllSubSearchbarOverlays("speech");

    let overlay = $('.overlay.speech');
    overlay.toggleClass('hidden');

    if (overlay.hasClass("hidden")) {
        recognition.abort();
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
        case "rus":
            recognition.lang = "ru";
            $('#currentSpeechLang').html("Russian");
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