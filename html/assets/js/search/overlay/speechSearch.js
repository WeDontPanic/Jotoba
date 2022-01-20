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
        $('#currentlyListening').html(getText("SPEECH_LISTEN_YES"));
        $('.voiceSvg').toggleClass("active");
    };
    
    // On recognition error
    recognition.onerror  = function(event) { 
        console.log(event.error);
        switch(event.error) {
            case "not-allowed":
                Util.showMessage("error", getText("SPEECH_NO_PERMISSION"));
                break;
            case "aborted":
                Util.showMessage("info", getText("SPEECH_ABORT"));
                break;
            case "no-speech":
                Util.showMessage("info", getText("SPEECH_NO_VOICE"));
                break;
            default:
                Util.showMessage("error", getText("SPEECH_NOT_SUPPORTED"));
        }
        $('#currentlyListening').html(getText("SPEECH_LISTEN_NO"));
        $('.voiceSvg').toggleClass("active");
    }
    
    // On speech end
    recognition.onspeechend = function() {
        recognition.stop();
        $('#currentlyListening').html(getText("SPEECH_LISTEN_NO"));
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
        Util.showMessage("error", getText("SPEECH_NOT_SUPPORTED"));
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
            $('#currentSpeechLang').html(getText("LANG_JAP"));
            break
        case "ger":
            recognition.lang = "de-DE";
            $('#currentSpeechLang').html(getText("LANG_GER"));
            break
        case "eng":
            recognition.lang = "en-US";
            $('#currentSpeechLang').html(getText("LANG_ENG"));
            break
        case "rus":
            recognition.lang = "ru";
            $('#currentSpeechLang').html(getText("LANG_RUS"));
            break
        case "spa":
            recognition.lang = "es-ES";
            $('#currentSpeechLang').html(getText("LANG_SPA"));
            break
        case "swe":
            recognition.lang = "sv-SE";
            $('#currentSpeechLang').html(getText("LANG_SWE"));
            break
        case "fre":
            recognition.lang = "fr-FR";
            $('#currentSpeechLang').html(getText("LANG_FRE"));
            break
        case "dut":
            recognition.lang = "nl-NL";
            $('#currentSpeechLang').html(getText("LANG_DUT"));
            break
        case "hun":
            recognition.lang = "hu";
            $('#currentSpeechLang').html(getText("LANG_HUN"));
            break
        case "slv":
            recognition.lang = "sl-SI";
            $('#currentSpeechLang').html(getText("LANG_SLV"));
            break
    }

    setTimeout(function(){ recognition.start(); }, 400);
}