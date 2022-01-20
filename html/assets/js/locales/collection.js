const locales = {
    "en-US": {
        "LANG_JAP": "Japanese",
        "LANG_GER": "German",
        "LANG_ENG": "English",
        "LANG_RUS": "Russian",
        "LANG_SPA": "Spanish",
        "LANG_SWE": "Swedish",
        "LANG_FRE": "French",
        "LANG_DUT": "Dutch",
        "LANG_HUN": "Hungarian",
        "LANG_SLV": "Slovenian",
        "SPEECH_LISTEN_YES": "Yes",
        "SPEECH_LISTEN_NO": "No",
        "SPEECH_NO_PERMISSION": "Need permissions to perform speech recognition!",
        "SPEECH_ABORT": "Speech recognition aborted.",
        "SPEECH_NO_VOICE": "No voice input received!",
        "SPEECH_NOT_SUPPORTED": "Your browser does not support speech recognition!"
    },
    "de-DE": {
        "LANG_JAP": "Japanisch",
        "LANG_GER": "Deutsch",
        "LANG_ENG": "Englisch",
        "LANG_RUS": "Russisch",
        "LANG_SPA": "Spanisch",
        "LANG_SWE": "Schwedisch",
        "LANG_FRE": "Französisch",
        "LANG_DUT": "Niederländisch",
        "LANG_HUN": "Ungarisch",
        "LANG_SLV": "Slowenisch",
        "SPEECH_LISTEN_YES": "Ja",
        "SPEECH_LISTEN_NO": "Nein",
        "SPEECH_NO_PERMISSION": "Jotoba benötigt Berechtigungen für die Spracherkennung!",
        "SPEECH_ABORT": "Spracherkennung abgebrochen.",
        "SPEECH_NO_VOICE": "Wir konnten Deine Stimme nicht hören!",
        "SPEECH_NOT_SUPPORTED": "Dein Browser unterstützt dieses Feature leider nicht!"
    }
};

// Returns the text with the given identifier from the currently selected language
function getText(identifier) {
    try {
        return locales[Cookies.get("page_lang")][identifier];
    } catch {
        try {
            return locales["en-US"][identifier];
        } catch {
            return identifier;
        }
    }    
}