// [news] is declared directly in the html

prepareNews();

function prepareNews() {
    let list = document.getElementById("news-list");
    
    for (info of news) {
        list.innerHTML += '<div class="news-container"><div class="news-head"><span></span></div><div class="news-date"></div><div class="news-body"></div></div>';
        list.lastChild.firstChild.firstChild.innerHTML = info.title;
        list.lastChild.children[1].innerHTML =  Util.toLocaleDateString(info.creation_time * 1000);
        list.lastChild.lastChild.innerHTML = Util.decodeHtml(info.html);
    }
}