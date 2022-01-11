/**
 * This JS-File contains some functions that are commonly used
 * This file is supposed to be loaded asynchronously. Jotoba needs some things directly so they are located in a different file.
 */

// Displays the given message of type "succes", "error" or "info"
Util.showMessage = function(type, message) {
    switch (type) {
        case "success":
            alertify.success(message);
            break;
        case "error":
            alertify.error(message);
            break;
        case "info":
            alertify.warning(message);
    }
}

// Copies the given string to clipboard
Util.copyToClipboard = function(text) {
    const el = document.createElement('textarea');
    el.value = text;
    el.setAttribute('readonly', '');
    el.style.position = 'absolute';
    el.style.left = '-9999px';
    document.body.appendChild(el);
    el.select();
    document.execCommand('copy');
    document.body.removeChild(el);
}

// Convert a single 0-F to 0-15
Util.hex2num_single = function(hex) {
    if (hex < 10)
        return hex;
    switch(hex.toUpperCase()) {
        case "A":
            return 10;
        case "B":
            return 11;
        case "C":
            return 12;
        case "D":
            return 13;
        case "E":
            return 14;
        case "F":
            return 15;
    }
}

// Convert a single 0-15 to 0-F
Util.num2hex_single = function(num) {
    if (num < 10)
        return num;
    switch(num) {
        case 10:
            return "A";
        case 11:
            return "B";
        case 12:
            return "C";
        case 13:
            return "D";
        case 14:
            return "E";
        case 15:
            return "F";
    }
}

// Returns the browsers true width
Util.getBrowserWidth = function() {
    return Math.max(
      document.body.scrollWidth,
      document.documentElement.scrollWidth,
      document.body.offsetWidth,
      document.documentElement.offsetWidth,
      document.documentElement.clientWidth
    );
}
  

// Removes any current drag selection (not supported on IE)
Util.deleteSelection = function() {
    if (window.getSelection) {
        var selection = window.getSelection();
        selection.empty();
    }
}

// Scrolls to the destination in x miliseconds
Util.scrollTo = function (final, duration) {
    var start = window.scrollY || document.documentElement.scrollTop,
        currentTime = null;
        
    var animateScroll = function(timestamp) {
        if (!currentTime) {
            currentTime = timestamp;  
        }      

        let progress = timestamp - currentTime;

        if(progress > duration) {
            progress = duration;
        }

        let val = Math.easeInOutQuad(progress, start, final-start, duration);
        window.scrollTo(0, val);

        if(progress < duration) {
            window.requestAnimationFrame(animateScroll);
        }
    };
  
    window.requestAnimationFrame(animateScroll);
};
  
// Checks if child is contained in parent
Util.isChildOf = function (parent, child) {
    var node = child.parentNode;
    while (node != null) {
        if (node == parent) {
            return true;
        }
        node = node.parentNode;
    }
    return false;
}


// Splits the input by " " and returns the last result
Util.getLastWordOfString = function(s) {
    let inputSplit = s.split(" ");
    return inputSplit[inputSplit.length-1];
}

// Converts a Base64 Url to a JS File
Util.convertDataURLtoFile = function(dataUrl, fileName) {
    var arr = dataUrl.split(','),
            mime = arr[0].match(/:(.*?);/)[1],
            bstr = atob(arr[1]), 
            n = bstr.length, 
            u8arr = new Uint8Array(n);
            
    while(n--){
        u8arr[n] = bstr.charCodeAt(n);
    }
        
    return new File([u8arr], fileName, {type:mime});
}

// Sends a file to the given API endpoint; callback => function
Util.sendFilePostRequest = function(file, api, callback) {
    var formData = new FormData();
    formData.append(file.name, file);

    var xhr = new XMLHttpRequest();
    xhr.onreadystatechange = function() {
        if (xhr.readyState == XMLHttpRequest.DONE) {
            callback(xhr.responseText); 
        }
    }

    xhr.open("POST", api);
    xhr.send(formData);
}

// Checks if a given URL contains an image and call the corresponding callback function
Util.checkUrlIsImage = function(url, successCallback, errorCallback) {
    var image = new Image();
    image.onload = function() {
      if (this.width > 0) {
        successCallback();
      }
    }
    image.onerror = function() {
        errorCallback();
    }
    image.src = url;
}

// Used for animation curves
Math.easeInOutQuad = function (t, b, c, d) {
    t /= d/2;
    if (t < 1) return c/2*t*t + b;
    t--;
    return -c/2 * (t*(t-2) - 1) + b;
};

// Returns the modulo of n and m but always makes them positive (-6, 4) = 2
Math.positiveMod = function(n, m) {
    return ((n % m) + m) % m;
}

// Opens the given URL in the current tab
Util.loadUrl = function(url) {
    window.location = url;
}

// Tries to open URL in a new tab and keep focussed on current. Doesnt work in all browsers
Util.loadUrlInNewTab = function(url) {
    window.open(url, '_blank').blur();
    window.focus();
}

// Tries to find the given parameter in the url and returns its value
Util.getPageParameter = function(paramName) {
    var url_string = window.location.href;
    var url = new URL(url_string);
    var p = url.searchParams.get(paramName);
    return p;
}

// Sets a text field's cursor to the given position. -1 -> last position
Util.setCaretPosition = function(elemId, caretPos) {
    var elem = document.getElementById(elemId);
    if (caretPos == -1) {
        caretPos = elem.value.length;
    }
    
    if(elem != null) {
        if(elem.createTextRange !== undefined) {
            var range = elem.createTextRange();
            range.move('character', caretPos);
            range.select();
        }
        else {
            if(elem.selectionStart !== undefined) {
                elem.setSelectionRange(caretPos, caretPos);
            }
            else
                elem.focus();
        }
    }
}

// Check if the current browsers doesn't want the user to be tracked
Util.checkTrackingAllowed = function() {
    try {
        if (window.doNotTrack || navigator.doNotTrack || navigator.msDoNotTrack || 'msTrackingProtectionEnabled' in window.external) {
            if (window.doNotTrack == "1" || navigator.doNotTrack == "yes" || navigator.doNotTrack == "1" || navigator.msDoNotTrack == "1") {
                return false;
            } else {
                return true;
            }
        } else {
            return true;
        }
    } catch (e) {
        return true;
    }
}

// MDL doesn't show the scroll-arrows on start. This should help.
Util.mdlScrollFix = function(){
    $(".mdl-layout__tab-bar-right-button").addClass("is-active");
}

// Deletes all cookies whose names are within the given array
Util.deleteSelectedCookies = function(cookieArray) {
    var allCookies = document.cookie.split(';');
                
    for (var i = 0; i < allCookies.length; i++) {
        if (cookieArray.includes(allCookies[i])) {
            document.cookie = allCookies[i] + "=;expires="+ new Date(0).toUTCString()+";path=/;";
        } else {
            document.cookie = allCookies[i];
        }
    }
}

// Deletes all stored cookies
Util.deleteAllCookies = function() {
    var allCookies = document.cookie.split(';');
                
    for (var i = 0; i < allCookies.length; i++) {
        document.cookie = allCookies[i] + "=;expires="+ new Date(0).toUTCString()+";path=/;";
    }
}

// Parses the given value into a boolean
Util.toBoolean = function(value, inverseDefault) {
    switch (value) {
        case 0:
        case "0":
        case "false":
        case false:
            return false;
        case 1:
        case "1":
        case "true":
        case true:
            return true;
        default:
            if (inverseDefault)
                return true;
            return false;
    }
}