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
  
// Checks if a given element is overflown
Util.checkOverflow = function checkOverflow(el) {
   var curOverflow = el.style.overflow;

   if (!curOverflow || curOverflow === "visible")
      el.style.overflow = "hidden";

   var isOverflowing = el.clientWidth < el.scrollWidth || el.clientHeight < el.scrollHeight;

   el.style.overflow = curOverflow;

   return isOverflowing;
}

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

Util.loadUrl = function(url) {
    window.location = url;
}

Util.loadUrlInNewTab = function(url) {
    window.open(url, '_blank');
}