/**
 *  Used to handle the radical search
*/
const radicals = [
    ["一","｜","丶","ノ","乙","亅"], 
    ["二","亠","人","⺅","𠆢","儿","入","ハ","丷","冂","冖","冫","几","凵","刀","⺉","力","勹","匕","匚","十","卜","卩","厂","厶","又","マ","九","ユ","乃", "𠂉"],
    ["⻌","口","囗","土","士","夂","夕","大","女","子","宀","寸","小","⺌","尢","尸","屮","山","川","巛","工","已","巾","干","幺","广","廴","廾","弋","弓","ヨ","彑","彡","彳","⺖","⺘","⺡","⺨","⺾","⻏","⻖","也","亡","及","久"],
    ["⺹","心","戈","戸","手","支","攵","文","斗","斤","方","无","日","曰","月","木","欠","止","歹","殳","比","毛","氏","气","水","火","⺣","爪","父","爻","爿","片","牛","犬","⺭","王","元","井","勿","尤","五","屯","巴","毋"],
    ["玄","瓦","甘","生","用","田","疋","疒","癶","白","皮","皿","目","矛","矢","石","示","禸","禾","穴","立","⻂","世","巨","冊","母","⺲","牙"],
    ["瓜","竹","米","糸","缶","羊","羽","而","耒","耳","聿","肉","自","至","臼","舌","舟","艮","色","虍","虫","血","行","衣","西"],
    ["臣","見","角","言","谷","豆","豕","豸","貝","赤","走","足","身","車","辛","辰","酉","釆","里","舛","麦"],
    ["金","長","門","隶","隹","雨","青","非","奄","岡","免","斉"],
    ["面","革","韭","音","頁","風","飛","食","首","香","品"],
    ["馬","骨","高","髟","鬥","鬯","鬲","鬼","竜","韋"],
    ["魚","鳥","鹵","鹿","麻","亀","啇","黄","黒"],
    ["黍","黹","無","歯"],
    ["黽","鼎","鼓","鼠"],
    ["鼻","齊"],
    ["龠"]
];

var baseRadResult = $('.rad-results')[0].innerHTML;

function toggleRadicalOverlay() {
    $('.overlay.speech').addClass('hidden');

    let overlay = $('.overlay.radical');
    overlay.toggleClass('hidden');

    if (overlay.hasClass("hidden")) {
        recognition.stop();
    }

    // Reset on close
    if (overlay.hasClass("hidden")) {
        resetRadPicker()
    } else {
        $('.rad-results').html(baseRadResult);
        $('.rad-results').removeClass("hidden");
        scrollSearchIntoView();
    }
}

// Called by reset btn. Deselects all
function resetRadPicker() {
    $('.rad-btn.selected').each((i, e) => {
        $(e).removeClass("selected");
    });

    $('.rad-btn.disabled').each((i, e) => {
        $(e).removeClass("disabled");
    });

    $('.rad-results').addClass("hidden");
}

// Adds the selected Kanji to the search bar
function handleKanjiSelect(event) {
    $('#search').val($('#search').val() + event.target.innerHTML);
}

// Toggles Radicals on Input and loads the results
function handleRadicalSelect(event) {
    let target = $(event.target);

    // Dont do anything if disabled
    if (target.hasClass("disabled")) {
        return;
    }

    // Make results visible again if they were hidden
    $('.rad-results').removeClass("hidden");
    
    // Toggle the "selected" class
    target.toggleClass('selected');

    // Get possible Kanji / Radicals from selection
    getRadicalInfo();
}

// Loads Kanji / Radical result from API into frontend
function loadRadicalResults(info) {
    var rrHtml = "";

    // Get and Iterate Kanji Keys
    let kanjiKeys =  Object.keys(info.kanji)

    // Iterate all and add
    for (let i = 0; i < kanjiKeys.length; i++) {

        // Get the data
        let key = kanjiKeys[i];
        let possibleKanji = info.kanji[key];

        // Create the stroke-count btn
        rrHtml += '<span class="rad-btn result num noselect">'+key+'</span>';

        let kanjiBtns = "";

        // Create the btn for each entry
        for (let j = 0; j < possibleKanji.length; j++) {
            kanjiBtns += '<span class="rad-btn result noselect" onClick="handleKanjiSelect(event)">'+possibleKanji[j]+'</span>';
        }

        //$('.rad-results').append(kanjiBtns);
        rrHtml += kanjiBtns;
    }


     $('.rad-results').html(rrHtml);

    // Only activate possible radicals
    let radicals = $('.rad-btn.picker:not(.num)').toArray();
    for (let i = 0; i < radicals.length; i++) {
        let rad = $(radicals[i]);
        if (info.possible_radicals.includes(rad.html()) || rad.hasClass("selected")) {
            rad.removeClass("disabled");
        } else {
            rad.addClass("disabled");
        }
    }

}

// Calls the API to get all kanji and radicals that are still possible
function getRadicalInfo() {
    // Create the JSON
    let radicalJSON = {
        "radicals": []
    }

    // Populate radicals within JSON with all selected radicals
    let rads = $('.rad-btn.selected').toArray();
    for (let i = 0; i < rads.length; i++) {
        radicalJSON.radicals.push(rads[i].innerHTML);
    }

    // No Radicals selected, Reset
    if (radicalJSON.radicals.length == 0) { 
        $('.rad-btn.disabled').each((i, e) => {
            $(e).removeClass("disabled");
        });
        return;
    }

    // Send Request to backend
    $.ajax({ 
        type : "POST", 
        url : "/api/kanji/by_radical", 
        data: JSON.stringify(radicalJSON),
        headers: {
            'Content-Type': 'application/json'
        },
        success : function(result) { 
            // Load the results into frontend
            loadRadicalResults(result);
        }, 
        error : function(result) { 
           // Print Error
           Util.showMessage("error", "Could not reach Radical API.")
        } 
    }); 
}
