/**
 *  This file handles everything related to image-search requests
 */ 

// Shows / Hides the image search overlay
function toggleImageSearchOverlay() {
    let overlay = $('.overlay.image');
    overlay.toggleClass('hidden');

    // Reset on close
    if (urlInputDisabled) {
        document.getElementById("imgUploadFile").value = null;
        resetUploadUrlInput();
    }
    
    closeAllSubSearchbarOverlays("image");
}

// Clicks on the upload SVG should trigger the underlying function
function imgUploadAltClick() {
    document.getElementById("imgUploadFile").click();    
}

// Blocks the URL input upon file selection
function imgSearchFileSelected() {
    let fileInput = document.getElementById("imgUploadFile").files[0];
    disableUploadUrlInput(fileInput.name);
}

// Toggles the URL input active / disabled
var urlInputDisabled = false;
var originalMsg = document.getElementById("imgUploadUrl").placeholder;

function resetUploadUrlInput() {
    let urlInput = document.getElementById("imgUploadUrl")
    urlInput.classList.remove("disabled");
    urlInput.disabled = false;
    urlInputDisabled = false;

    urlInput.placeholder = originalMsg; 
}

function disableUploadUrlInput(newMessage) {
    let urlInput = document.getElementById("imgUploadUrl")
    urlInput.classList.add('disabled');
    urlInput.disabled = true;
    urlInputDisabled = true;

    urlInput.value = null;
    urlInput.placeholder = newMessage; 
}

// Opens the Image Cropping Overlay
function openImageCropOverlay() {
    // ImageCropper(parent, targetFile, settings) -> targetFile kann direkt eine URL aus dem Browser sein!
    // new ImageCropper(".image-upload-parent", todo,{max_width:300,max_height:300})

    var selectedFiles = document.getElementById("imgUploadFile").files;

    if (selectedFiles.length > 0) {
        var reader  = new FileReader();
        reader.onload = function(e) {
            new ImageCropper("#croppingTarget", e.target.result, {max_width:300,max_height:300}) 
        }
        reader.readAsDataURL(selectedFiles[0]);
    }
    else {
        new ImageCropper("#croppingTarget", document.getElementById("imgUploadUrl").value, {max_width:300,max_height:300}) 
    }

    // Modal + #croppingTarget => TODO
}
