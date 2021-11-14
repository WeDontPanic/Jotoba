
// On load, check if Shortcuts should be shown. They are useless for mobile devices
if( /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent) ) {
    document.getElementById("shortcutInfo").classList.add("hidden");
    document.getElementsByClassName("help-cat")[0].classList.remove("help-cat");
}