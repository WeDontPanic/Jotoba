/**
 * This JS-File contains some Improvements specifically for mobile views
 */

// Toggles the options for different input and page jumping on / off
function toggleMobileNav() {
    $('.mobile-nav').toggleClass('hidden');
}

// On Start, check if mobile view is enabled. If yes, activate the btn
Util.awaitDocumentReady(prepareMobilePageBtn);

// Variables used in mobiles' easy-use btn
var jmpBtn;
var kanjiDiv;
var jmpBtnPointsTop;

// Prepares the easy-use Btn for mobile devices
function prepareMobilePageBtn() {
    // The Jmp Btn and Kanji elements
    jmpBtn = $("#jmp-btn");
    kanjiDiv = document.getElementById("secondaryInfo"); 

    // Variables used in the following two functions
    jmpBtnPointsTop = false;

    if (kanjiDiv !== null) {
        // Prepare the Kanji jmp and its button
        var kanjiPos = kanjiDiv.offsetTop; 
        jmpBtn.removeClass("hidden");

        // Window Scroll checks
        window.onscroll = function() {
            if (Util.getBrowserWidth() < 600 && (document.body.scrollTop > kanjiPos - 500 || document.documentElement.scrollTop > kanjiPos - 500)) {
                jmpBtn.css("transform", "rotate(0deg)");
                jmpBtnPointsTop = true;
            } else {
                jmpBtn.css("transform", "rotate(180deg)");
                jmpBtnPointsTop = false;
            }
        }
    }
}

// Jumps to the top or kanji part
function jumpToTop() {
    if (jmpBtnPointsTop) {
        (!window.requestAnimationFrame) ? window.scrollTo(0, 0) : Util.scrollTo(0, 400);
    } else {
        let topOffset = kanjiDiv.offsetTop; 
        (!window.requestAnimationFrame) ? window.scrollTo(0, topOffset) : Util.scrollTo(topOffset, 400);
    }
}
