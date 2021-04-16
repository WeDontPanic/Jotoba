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