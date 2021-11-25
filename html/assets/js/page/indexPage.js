// Opens the Help Page
function openHelpPage() {
    document.getElementsByClassName("infoBtn")[0].classList.remove("new");
    if (localStorage != null)
        localStorage.setItem("first_time", "false");
    Util.loadUrl("/help");
}