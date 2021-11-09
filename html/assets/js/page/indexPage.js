/* Current Version of Jotoba */
const jotobaVersion = "1.1";

/* Check for Updates since the last release */
let storedVersion = localStorage.getItem("joto_version");
if (storedVersion < jotobaVersion) {
    document.getElementsByClassName("notificationBtn")[0].classList.add("update");
}

// Opens the Patch Notes of the latest release
function openNotifications() {
    localStorage.setItem("joto_version", jotobaVersion);
    document.getElementsByClassName("notificationBtn")[0].classList.remove("update");
}