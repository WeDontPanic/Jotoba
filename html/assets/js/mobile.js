/**
 * This JS-File contains some Improvements specifically for mobile views
 */

// Toggles the options for different input and page jumping on / off
function toggleMobileNav() {
    $('.mobile-nav').toggleClass('hidden');
}

// The Jmp Btn and Kanji elements
var jmpBtn = $("#jmp-btn");
var kanjiDiv = document.getElementById("secondaryInfo"); 

// Variables used in the following two functions
var jmpBtnPointsTop = false;
var kanjiPos = kanjiDiv.offsetTop; 

// Window Scroll checks
window.onscroll = function() {
    if (getBrowserWidth() < 600 && (document.body.scrollTop > kanjiPos - 500 || document.documentElement.scrollTop > kanjiPos - 500)) {
        jmpBtn.css("transform", "rotate(0deg)");
        jmpBtnPointsTop = true;
    } else {
        jmpBtn.css("transform", "rotate(180deg)");
        jmpBtnPointsTop = false;
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