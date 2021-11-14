/* Current Version of Jotoba */
const jotobaVersion = "1.1";

if (localStorage != null) {

    /* Check for Updates since the last release */
    let storedVersion = localStorage.getItem("joto_version");
    if (storedVersion < jotobaVersion) {
        document.getElementsByClassName("notificationBtn")[0].classList.add("update");
    }

    /* Check if user is the first time on Jotoba */
    if (localStorage.length == 0) {
        document.getElementsByClassName("infoBtn")[0].classList.add("new");
    }
}

// Opens the Patch Notes of the latest release
function openHelpPage() {
    document.getElementsByClassName("infoBtn")[0].classList.remove("new");
    if (localStorage != null)
        localStorage.setItem("first_time", "false");
    Util.loadUrl("/help");
}

// Opens the Patch Notes of the latest release
function openNotifications() {
    if (localStorage != null)
        localStorage.setItem("joto_version", jotobaVersion);
    document.getElementsByClassName("notificationBtn")[0].classList.remove("update");
}