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

// Changes the state of an MDL checkbox
Util.setMdlCheckboxState = function(id, state) {
  if (state === undefined) {
    return;
  }

  let element = $('label[for='+id+']');

  // Only attempt to apply change if element exists.
  if (element[0]){
    if(state) {
      element[0].MaterialCheckbox.check();
    } else {
      element[0].MaterialCheckbox.uncheck();
    }
  }
}