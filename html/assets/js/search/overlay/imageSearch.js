/**
 *  This file handles everything related to image-search requests
 */ 

// Quick image search for STRG + V
document.onpaste = (evt) => {
    let dT = evt.clipboardData || window.clipboardData;
    let file = dT.files[0];

    if (file !== undefined && file.name.includes(".png")) {
        disableUploadUrlInput(file.name);
        openImageCropOverlay(file);
    }
};

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
    if (fileInput !== undefined) {
        disableUploadUrlInput(fileInput.name);
        openImageCropOverlay();
    } else {
        resetUploadUrlInput();
    }
}

// Toggles the URL input active / disabled
var urlInputDisabled = false;
var originalMsg = document.getElementById("imgUploadUrl").placeholder;
var cropTarget;

function resetUploadUrlInput() {
    let urlInput = document.getElementById("imgUploadUrl")
    urlInput.classList.remove("disabled");
    urlInput.disabled = false;
    urlInputDisabled = false;
    urlInput.placeholder = originalMsg; 

    document.getElementById("imgUploadFile").value = null;

    if (cropTarget !== null) {
        cropTarget.croppie("destroy");
    }
    toggleCroppingModal();
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
function openImageCropOverlay(pastedFile) {
    var selectedFiles = document.getElementById("imgUploadFile").files;
    var inputUrl = document.getElementById("imgUploadUrl").value;

    if (selectedFiles.length > 0 || pastedFile !== undefined) {
        let reader = new FileReader();
        reader.onload = function(e) {
            initCroppie(e.target.result);
        }
        reader.readAsDataURL(selectedFiles[0] || pastedFile);
        toggleCroppingModal();
    }
    else if (inputUrl.length > 0) {  
        Util.checkUrlIsImage(inputUrl, () => {
            initCroppie(inputUrl);
        });
        toggleCroppingModal();
    } else {
        Util.showMessage("error", "You need to enter a URL or upload a file!");
    }
}

// Receives the image from Croppie, sends it to the server and starts the search
function uploadCroppedImage(dataUrl) {
    cropTarget.croppie('result', {
        type: 'canvas',
        size: 'viewport'
    }).then(function (resp) {
        // Generate a file from the Base64 String
        let generatedFile = Util.convertDataURLtoFile(resp);

        // Block Screen until Server responded
        $("#loading-screen").toggleClass("show", true);

        // Send the Request and handle it
        Util.sendFilePostRequest(generatedFile, "/api/img_scan", function(responseText) {
            let response = JSON.parse(responseText);
            if (response.code !== undefined) { // JSON doesnt have a code when the text is given
                Util.showMessage("error", response.message);
                $("#loading-screen").toggleClass("show", false);
            } else {
                if (response.text.length == 1 && response.text.match(kanjiRegEx)) {
                    Util.loadUrl(JotoTools.createUrl(response.text, 1));
                } else {
                    Util.loadUrl(JotoTools.createUrl(response.text));
                }
            }
        });
    });
    
    resetUploadUrlInput();
}

// Loads the Image Cropper
function initCroppie(inputUrl) {
    cropTarget = $('#croppingTarget').croppie({
    showZoomer: false,
    enableResize: true,
    enableOrientation: true,
    mouseWheelZoom: 'ctrl'
    });
    cropTarget.croppie('bind', {
        url: inputUrl,
    });

    cropTarget.croppie('result', 'html').then(function(html) { });
}

// Custom Modal Toggle function for the custom Modal
var modalIsVisible = false;
function toggleCroppingModal() {
    if (modalIsVisible) {
        $(".modal-backdrop").remove();
        $("#imageCroppingModal").css("display", "none");
    } else {
        $("body").append('<div class="modal-backdrop fade show"></div>');
        $("#imageCroppingModal").css("display", "block");
    }

    modalIsVisible = !modalIsVisible;
    $("#imageCroppingModal").modal();
    $("#imageCroppingModal").toggleClass("show");
}