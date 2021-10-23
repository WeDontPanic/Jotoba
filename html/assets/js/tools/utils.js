/**
 * This JS-File contains some functions that are commonly used
 */

// The util "parent"
function Util () {};

// Runs callback fn on document ready
Util.awaitDocumentReady = function(callback) {
    let readyWait = window.setInterval(() => {
        if (document.readyState == "complete") {
            callback();
            window.clearInterval(readyWait);
        }
    }, 10);
}

// Loads a script dynamically
Util.loadScript = function(url, async, attributes, callback) {
    // Called without url? Return.
    if (url.length == 0) {
        return;
    }

    // Create the element
    var s = document.createElement('script');
    s.setAttribute('src', url);
    s.onload = callback;
    if (async) {
        s.async = true;
    }
    
    // Add specific attributes
    for (let i = 0; i < attributes.length; i++) {
        s.setAttribute(attributes[i][0], attributes[i][1]);
    }

    // Append and load
    document.head.appendChild(s);
}

// Converts a hex value to rgb
Util.hexToRgb = function(hex) {
    var result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
    return result ? {
        r: parseInt(result[1], 16),
        g: parseInt(result[2], 16),
        b: parseInt(result[3], 16)
    } : null;
}