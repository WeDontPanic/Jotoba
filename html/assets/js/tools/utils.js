/**
 * This JS-File contains some functions that are commonly used
 */

// Constants
const dateSettings = { year: 'numeric', month: 'short', day: 'numeric' };

// The util "parent"
function Util () {};

// Runs callback fn when document is done loading DOM-Elements
Util.awaitDocumentInteractive = function(callback) {
  let readyWait = window.setInterval(() => {
      if (document.readyState == "interactive" || document.readyState == "complete") {
          callback();
          window.clearInterval(readyWait);
      }
  }, 10);
}

// Runs callback fn when document is done loading
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

// Checks if a given element is overflown
Util.checkOverflow = function(el) {
  var curOverflow = el.style.overflow;

  if (!curOverflow || curOverflow === "visible")
     el.style.overflow = "hidden";

  var isOverflowing = el.clientWidth < el.scrollWidth || el.clientHeight < el.scrollHeight;

  el.style.overflow = curOverflow;

  return isOverflowing;
}

// Re-Encodes a decoded HTML
Util.decodeHtml = function(html) {
  var doc = new DOMParser().parseFromString(html, "text/html");
  return doc.documentElement.textContent;
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

// Parses the given Unix time to a date of the given language
Util.toLocaleDateString = function(unixTime) {
  return new Date(unixTime).toLocaleDateString("de-DE", dateSettings);
}   

// Returns whether the current page is index or not
Util.isIndexPage = function() {
  return window.location.origin+"/" == document.location.href;
}

// Returns whether the current page is listed under {index}/{path}
Util.isInPath = function(path) {
  return document.location.href.startsWith(window.location.origin+"/"+path);
}