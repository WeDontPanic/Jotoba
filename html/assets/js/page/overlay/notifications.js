// On load -> Start a query to receive current notifications
var data = {"after": parseInt(localStorage.getItem("notification_timestamp") || 00000000)};
var request = $.ajax({ 
        type : "POST", 
        url : "/api/news/short", 
        data: JSON.stringify(data),
        headers: {
            'Content-Type': 'application/json'
        },
        success : function(result) { 
            console.log(result);
            parseNotificationResults(result);
        }, 
        error : function(result) { 
            console.log(result);
        } 
});
localStorage.setItem("notification_timestamp", + new Date());

// Parses the results of /api/news/short API calls and displays them
async function parseNotificationResults(results) {
    
    // If nothing was received, show a message that there are no new updates
    if (results.entries.length == 0) {
        $("#no-result").removeClass("hidden");
        return;
    }

    // Else, show the results
    let notifiContent = document.getElementById("notification-content");
    for (let result of results.entries) {
        var entry = document.createElement("div"); 
        var title = document.createElement("div"); 
        var date = document.createElement("div"); 
        var content = document.createElement("div"); 
    
        entry.classList.add("notification-entry");
        title.classList.add("entry-title");
        date.classList.add("date-tag");
        content.classList.add("content");
    
        title.innerHTML = result.title;
    
            var creationDate = new Date(result.creation_time * 1000);
            const options = { year: 'numeric', month: 'numeric', day: 'numeric' };
        date.innerHTML = creationDate.toLocaleDateString(Cookies.get("page_lang") || "en-US", options);
        
        content.innerHTML = result.html;
    
        entry.appendChild(title);
        entry.appendChild(date);
        entry.appendChild(content);
    
        notifiContent.insertBefore(entry, notifiContent.firstChild);
    }
}






/*
<div class="notification-entry">
                        <div class="title">V 1.2</div>
                        <div class="date-tag">24.11.2021</div>
                        <div class="content"></div>
                     </div>
*/