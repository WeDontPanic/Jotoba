/**
 * This JS-File contains some functions that are commonly used
 */


// Displays the given message of type "succes", "error" or "info"
function showMessage(type, message) {
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
function copyToClipboard(text) {
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
function hex2num_single(hex) {
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
function num2hex_single(num) {
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