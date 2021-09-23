/**
 *  This file handles everything related to image-search requests
 */ 

// Prepare to correctly close the Modal when clicking on the darkened parts
Util.awaitDocumentReady(() => {
    var target = $("#imageCroppingModal")[0];
    $(target).on("click", (e) => {
        if (e.target == target) {
            resetUploadUrlInput();
        }
    });
});

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

function resetUploadUrlInput() {
    let urlInput = document.getElementById("imgUploadUrl")
    urlInput.classList.remove("disabled");
    urlInput.disabled = false;
    urlInputDisabled = false;
    urlInput.placeholder = originalMsg; 

    document.getElementById("imgUploadFile").value = null;

    if (croppedImage !== null) {
        document.getElementById("croppingTarget").innerHTML = "";
        croppedImage = null;
    }
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
var croppedImage = null;
function openImageCropOverlay() {
    var selectedFiles = document.getElementById("imgUploadFile").files;
    var inputUrl = document.getElementById("imgUploadUrl").value;

    if (selectedFiles.length > 0) {
        let reader = new FileReader();
        reader.onload = function(e) {
            croppedImage = new ImageCropper("#croppingTarget", e.target.result, {max_width:300,max_height:300, min_crop_width:10, min_crop_height:10}) 
        }
        reader.readAsDataURL(selectedFiles[0]);
        $("#imageCroppingModal").modal();
    }
    else if (inputUrl.length > 0 && /\.(jpg|jpeg|png)/.test(inputUrl)) {  
        croppedImage = new ImageCropper("#croppingTarget", document.getElementById("imgUploadUrl").value, {max_width:300,max_height:300, min_crop_width:10, min_crop_height:10}) 
        $("#imageCroppingModal").modal();
    } else {
        Util.showMessage("error", "You need to enter a URL or upload a file!");
    }
}

function uploadCroppedImage() {
    if (croppedImage !== null) {
        // Generate a file from the Base64 String
        let generatedFile = Util.convertDataURLtoFile(croppedImage.crop("image/png", 1), "upload.png");
        
        // Send the Request and handle it
        Util.sendFilePostRequest(generatedFile, "/api/img_scan", function(responseText) {
            let response = JSON.parse(responseText);
            if (response.code !== undefined) { // JSON doesnt have a code when the text is given
                Util.showMessage("error", response.message);
                $("#loading-screen").toggleClass("show", false);
            } else {
                if (response.text.length == 1 && response.text.match(kanjiRegEx)) {
                    window.location = window.location.origin + "/search/" + encodeURIComponent(response.text) + "?t=1"
                } else {
                    window.location = window.location.origin + "/search/" + encodeURIComponent(response.text)
                }
                
            }
        });

        // Block Screen until Server responded
        $("#loading-screen").toggleClass("show", true);

    }

    resetUploadUrlInput();
}