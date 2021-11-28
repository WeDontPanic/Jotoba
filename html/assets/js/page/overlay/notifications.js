// On Start -> Try and load the latest data
requestShortData();

// Start a query to receive current notifications
async function requestShortData() {
    if (!localStorage) { return; }

    var data = {"after": parseInt(localStorage.getItem("notification_timestamp") || 00000000)};
    $.ajax({ 
        type : "POST", 
        url : "/api/news/short", 
        data: JSON.stringify(data),
        headers: {
            'Content-Type': 'application/json'
         },
        success : function(result) { 
            parseShortNotificationResults(result);
        }, 
        error : function(result) { 
            console.log(result);
        } 
    });
}

// Parses the results of /api/news/short API calls and displays them
async function parseShortNotificationResults(results) {
    
    // If nothing was received, show a message that there are no new updates
    if (results.entries.length == 0) {
        $("#no-result").removeClass("hidden");
        return;
    }

    // Else, show the results
    let notifiContent = document.getElementById("notification-content");
    for (let result of results.entries) {
        let creationDateString = Util.toLocaleDateString(result.creation_time * 1000);

        var entryHtml = '<div class="notification-entry" onclick="requestLongData(event,'+result.id+');">'
                            + '<div class="entry-title">' + result.title + '</div>'
                            + '<div class="date-tag">' + creationDateString + '</div>'
                            + '<div class="content">' + result.html + '</div>'
                        +'</div>';

        notifiContent.innerHTML = entryHtml + notifiContent.innerHTML;
        document.getElementsByClassName("notificationBtn")[0].classList.add("update");
    }
}

// Shows the detailed information of the target element using its ID
function requestLongData(event, id) {
    if (event.target.nodeName === "IMG") {
        return;
    }

    var data = {"id": id};
    
    $.ajax({ 
        type : "POST", 
        url : "/api/news/detailed", 
        data: JSON.stringify(data),
        headers: {
            'Content-Type': 'application/json'
         },
        success : function(result) { 
            parseDetailedNotificationResults(result);
        }, 
        error : function(result) { 
            console.log(result);
        } 
    });
}

// Parses the results of /api/news/detailed API calls and displays them
async function parseDetailedNotificationResults(result) {
    $("#notification-detail-head").html(result.entry.title);
    $("#notification-detail-body").html(result.entry.html);

    $("#notificationModal").modal('show');
}

// Opens the short-informations for notifications
function toggleNotifications(event) {
    let container = $('#notifications-container');
    
    // Check if notification is opened already
    if (!container.hasClass("hidden")) {
        closeNotifications();
        return;
    }

    // Prevent click event to pass through to the body
    event.stopPropagation();    

    // Set the timestamp
    localStorage.setItem("notification_timestamp", Math.floor(Date.now() / 1000));
    container[0].classList.remove("hidden");
    
    // Make clicks outside the element close it 
    $(document).one("click", function() {
        closeNotifications();
        container.off("click");
    });
    container.click(function(event){
        event.stopPropagation();
    });
}

// Closes the short-informations for notifications
function closeNotifications() {
    document.getElementById("notifications-container").classList.add("hidden");
    document.getElementsByClassName("notificationBtn")[0].classList.remove("update");
}

// Calls a page that displays (more-or-less) all past notifications
function showAllNotifications() {
    Util.loadUrl(JotoTools.getPageUrl("news"));
}
