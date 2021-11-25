// [news] is declared directly in the html

prepareNews();

function prepareNews() {
    let list = document.getElementById("news-list");
    
    for (info of news) {
        list.innerHTML += '<div class="news-container"><div class="news-head"><span></span></div><div class="news-body"></div></div>';
        list.lastChild.lastChild.innerHTML = Util.decodeHtml(info.html);
        list.lastChild.firstChild.firstChild.innerHTML = info.title;
    }
}