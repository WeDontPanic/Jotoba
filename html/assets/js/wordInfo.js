/**
 * This JS-File handles the 3-dot menu and related calls
 */

var lastCaller;

// openInfoMenu(this, "", "c23434552345") e.g
function openInfoMenu(caller, conjugationOverlayId, collocationOverlayId) {

    if (lastCaller !== undefined && lastCaller == caller) {
        $('#info-dropdown').addClass("hidden");
        lastCaller = null;
        return;
    }

    lastCaller = caller;

    let bodyRect = document.body.getBoundingClientRect();
    let callerRect = caller.getBoundingClientRect();

    offsetTop  = callerRect.top - bodyRect.top + 31;
    offsetLeft = callerRect.left - 153;

    $('#info-dropdown').css("top", offsetTop);
    $('#info-dropdown').css("left", offsetLeft);
    $('#info-dropdown').removeClass("hidden");

    if (conjugationOverlayId !== undefined && conjugationOverlayId !== "") {
        $('#conjugationBtn')[0].dataset.target = conjugationOverlayId;
        console.log(conjugationOverlayId);
    }

    if (collocationOverlayId !== undefined && collocationOverlayId !== "") {
        $('#collocationBtn')[0].dataset.target = collocationOverlayId;
    }
}