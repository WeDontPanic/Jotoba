/**
 *  Used to handle the radical search
*/
const radicals = [
    ["一", "｜", "丶", "ノ", "乙", "亅"],
    ["二", "亠", "人", "⺅", "𠆢", "儿", "入", "ハ", "丷", "冂", "冖", "冫", "几", "凵", "刀", "⺉", "力", "勹", "匕", "匚", "十", "卜", "卩", "厂", "厶", "又", "マ", "九", "ユ", "乃", "𠂉"],
    ["⻌", "口", "囗", "土", "士", "夂", "夕", "大", "女", "子", "宀", "寸", "小", "⺌", "尢", "尸", "屮", "山", "川", "巛", "工", "已", "巾", "干", "幺", "广", "廴", "廾", "弋", "弓", "ヨ", "彑", "彡", "彳", "⺖", "⺘", "⺡", "⺨", "⺾", "⻏", "⻖", "也", "亡", "及", "久"],
    ["⺹", "心", "戈", "戸", "手", "支", "攵", "文", "斗", "斤", "方", "无", "日", "曰", "月", "木", "欠", "止", "歹", "殳", "比", "毛", "氏", "气", "水", "火", "⺣", "爪", "父", "爻", "爿", "片", "牛", "犬", "⺭", "王", "元", "井", "勿", "尤", "五", "屯", "巴", "毋"],
    ["玄", "瓦", "甘", "生", "用", "田", "疋", "疒", "癶", "白", "皮", "皿", "目", "矛", "矢", "石", "示", "禸", "禾", "穴", "立", "⻂", "世", "巨", "冊", "母", "⺲", "牙"],
    ["瓜", "竹", "米", "糸", "缶", "羊", "羽", "而", "耒", "耳", "聿", "肉", "自", "至", "臼", "舌", "舟", "艮", "色", "虍", "虫", "血", "行", "衣", "西"],
    ["臣", "見", "角", "言", "谷", "豆", "豕", "豸", "貝", "赤", "走", "足", "身", "車", "辛", "辰", "酉", "釆", "里", "舛", "麦"],
    ["金", "長", "門", "隶", "隹", "雨", "青", "非", "奄", "岡", "免", "斉"],
    ["面", "革", "韭", "音", "頁", "風", "飛", "食", "首", "香", "品"],
    ["馬", "骨", "高", "髟", "鬥", "鬯", "鬲", "鬼", "竜", "韋"],
    ["魚", "鳥", "鹵", "鹿", "麻", "亀", "啇", "黄", "黒"],
    ["黍", "黹", "無", "歯"],
    ["黽", "鼎", "鼓", "鼠"],
    ["鼻", "齊"],
    ["龠"]
];

var radicalMask = [
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0],
    [0]
];

var baseRadResult;
var currentSearchInput;
var lastRadicalSearchResult;

Util.awaitDocumentReady(() => {
    baseRadResult = $('.rad-results')[0].innerHTML;
    loadRadicals(0);

    // Used to re-focus searchbar upon using Radical Btns
    $("#kanji-search").focus(e => {
        currentSearchInput = $("#kanji-search");
    });
    $("#search").focus(e => {
        currentSearchInput = $("#search");
    });
});

// Opens | Closes the Radical overlay
function toggleRadicalOverlay() {
    closeAllSubSearchbarOverlays("radical");

    let overlay = $('.overlay.radical');
    overlay.toggleClass('hidden');
    sContainer.parentElement.classList.add("hidden");

    // Reset on close
    if (overlay.hasClass("hidden")) {
        resetRadPicker()
        rContainer.classList.add("hidden");
        Suggestions.overlay.show();
    } else {
        $('.rad-results').html(baseRadResult);
        $('.rad-results').removeClass("hidden");
        Suggestions.updateSuggestions();
        scrollSearchIntoView();
        $('#kanji-search').focus();
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

    iterateMaskAsync((i, j) => {
        radicalMask[i][j] = 0;
    });

    $('.rad-results').html(baseRadResult);
    resetAllTabs();

    currentSearchInput.focus();
}

// Adds the selected Kanji to the search bar
function handleKanjiSelect(event) {
    // Insert Kanji in search bar
    $('#search').val($('#search').val() + event.target.innerHTML);

    // Update search bar
    Suggestions.updateSuggestions();
    toggleSearchIcon(200);

    // Focus the last search bar
    currentSearchInput.focus();
}

// Toggles Radicals on Input and loads the results
function handleRadicalSelect(event) {
    let target = $(event.target);

    // Dont do anything if disabled
    if (target.hasClass("disabled")) {
        return;
    }

    // Update Radical Map
    if (target.hasClass("selected")) {
        radicalMask[target.attr("index")][target.attr("position")] = 0;
    } else {
        radicalMask[target.attr("index")][target.attr("position")] = 1;
    }
        
    // Make results visible again if they were hidden
    $('.rad-results').removeClass("hidden");

    // Toggle the "selected" class
    target.toggleClass('selected');

    // Get possible Kanji / Radicals from selection
    getRadicalInfo();
    
    // Focus the last search bar
    currentSearchInput.focus();

    // Update search bar
    Suggestions.updateSuggestions(getSelectedRadicalArray());
}

// Opens the Radical Page at the given index
let lastRadicalPage;
function openRadicalPage(index) {
    // Handle special pages
    if (index == -1) {
        if (lastRadicalPage !== undefined) {
            index = lastRadicalPage;
        } 
        else {
            openRadicalPage(0);
            return;
        }
    }

    // Iterate buttons and update whether to hightlight them or not
    $(".rad-page-toggle > span").each((i, e) => {
        if (i == index) {
            e.classList.add("selected");
            lastRadicalPage = index;
        } else
            e.classList.remove("selected");
    });
    
    // Load Radicals of new page
    loadRadicals(index);

     // Focus the last search bar
     currentSearchInput.focus();
}

// Clears the shown Radical list
function clearRadicals() {
    // Clear Radicals
    $(".rad-btn.picker:not(.num)").each((i, e) => {
        if (e.classList.contains("selected")) {
            radicalMask[e.getAttribute("index")][e.getAttribute("position")] = 1;
        }
    });
    $(".rad-picker").html("");
}

// Loads the Radicals of the specific tab
function loadRadicals(tabIndex) {
    // Clear Radicals
    clearRadicals();

    // Add Radicals
    if (tabIndex == 0) {
        addRadicals(0);
        addRadicals(1);
    }
    else if (tabIndex == 9) {
        for (let i = 10; i < radicals.length; i++) {
            addRadicals(i);
        }
    }
    else if (tabIndex == 10) {
        loadRadicalSearchResults(lastRadicalSearchResult);
    }
    else {
        addRadicals(tabIndex+1);
    }
}

// Loads the given Array into the Select Radicals Tab
function addRadicals(arrayIndex) {
    let html = $(".rad-picker").html();
    html += '<span class="rad-btn picker num">'+(arrayIndex+1)+'</span>';

    for (let i = 0; i < radicals[arrayIndex].length; i++) {
        html += '<span class="rad-btn picker'+(radicalMask[arrayIndex][i] == 1 ? " selected" : "")+(radicalMask[arrayIndex][i] == -1 ? " disabled" : "")+'" index='+arrayIndex+' position='+i+' onClick="handleRadicalSelect(event)">'+radicals[arrayIndex][i]+'</span>';
    }

    $(".rad-picker").html(html);
}

// Appends radicals contained in an array
function addRadicalsFromArray(index, array) {
    let html = $(".rad-picker").html();
    html += '<span class="rad-btn picker num">'+index+'</span>';
    index -= 1;

    for (let a = 0; a < array.length; a++) {
        for (let j = 0; j < radicals[index].length; j++) {
            if (radicals[index][j] == array[a].l) {
                html += '<span class="rad-btn picker'+(radicalMask[index][j] == 1 ? " selected" : "")+(radicalMask[index][j] == -1 ? " disabled" : "")+'" index='+index+' position='+j+' onClick="handleRadicalSelect(event)">'+radicals[index][j]+'</span>';
            }
        }
    }

    $(".rad-picker").html(html);
}

// Loads Kanji / Radical result from API into frontend
function loadRadicalResults(info) {
    var rrHtml = "";

    // Get and Iterate Kanji Keys
    let kanjiKeys = Object.keys(info.kanji)

    // Iterate all and add
    for (let i = 0; i < kanjiKeys.length; i++) {

        // Get the data
        let key = kanjiKeys[i];
        let possibleKanji = info.kanji[key];

        // Create the stroke-count btn
        rrHtml += '<span class="rad-btn result num noselect">' + key + '</span>';

        let kanjiBtns = "";

        // Create the btn for each entry
        for (let j = 0; j < possibleKanji.length; j++) {
            kanjiBtns += '<span class="rad-btn result noselect" onClick="handleKanjiSelect(event)">' + possibleKanji[j] + '</span>';
        }

        rrHtml += kanjiBtns;
    }

    $('.rad-results').html(rrHtml);

    // Only activate possible radicals
    let currentRadicals = $('.rad-btn.picker:not(.num)').toArray();
    for (let i = 0; i < currentRadicals.length; i++) {
        let rad = $(currentRadicals[i]);
        if (info.possible_radicals.includes(rad.html()) || rad.hasClass("selected")) {
            rad.removeClass("disabled");
        } else {
            rad.addClass("disabled");
        }
    }

    // Apply changes to mask
    iterateMaskAsync((i, j) => {
        if (!info.possible_radicals.includes(radicals[i][j])) {
            radicalMask[i][j] = -1;
        } else if (radicalMask[i][j] == -1) {
            radicalMask[i][j] = 0;
        }
    });

}

// Calls the given function on every iteration of the array. Passes i (outer) and j (inner) as params.
function iterateMaskAsync(functionToCall, startIndex, endIndex) {
    if (startIndex == undefined) {
        let middle = Math.floor(radicals.length / 2); 
        iterateMaskAsync(functionToCall, middle, radicals.length);
        startIndex = 0;
        endIndex = middle;
    }

    for (let i = startIndex; i < endIndex; i++) {
        for (let j = 0; j < radicals[i].length; j++) {
           functionToCall(i, j);
        }
    }

    updateTabVisuals();
}

// Checks whether the Page-Tabs have to be colored in a specific way (None possible, element selected...)
async function updateTabVisuals() {
    for (let i = 0; i < 10; i++) {
        let tabStatus = -1;

        // First Tab
        if (i == 0) {
            tabStatus = checkRadicalsInTab(0);
            let tabStatus2 = checkRadicalsInTab(1);
            if (tabStatus2 == 0 && tabStatus == -1)
                tabStatus = 0;
            else if (tabStatus2 == 1)
                tabStatus = 1;
        }
        // Last Tab
        else if (i == 9) {
            for (let j = 10; j < radicals.length; j++) {
                tabStatus = checkRadicalsInTab(j);
                $("#r-t"+j).toggleClass("disabled", tabStatus == -1);
                $("#r-t"+j).toggleClass("highlighted", tabStatus == 1);
            }
            break;
        }
        // Any other Tab
        else {
            tabStatus = checkRadicalsInTab(i+1);
        }

        $("#r-t"+i).toggleClass("disabled", tabStatus == -1);
        $("#r-t"+i).toggleClass("highlighted", tabStatus == 1);
    }
}

// Called by updateTabVisuals. Checks for tabDisabled (-1) | normal (0) | highlighted (1)
function checkRadicalsInTab(arrayIndex) {
    let status = -1;

    for (let i = 0; i < radicals[arrayIndex].length; i++) {
        if (radicalMask[arrayIndex][i] == 0) {
            status = 0;
        } else if (radicalMask[arrayIndex][i] == 1) {
            status = 1; 
            break;
        }
    }

    return status;
}

// Resets all Radical-Tabs by removing class-modifiers
function resetAllTabs() {
    for (let i = 0; i < 10; i++) {
        $("#r-t"+i).removeClass("disabled");
        $("#r-t"+i).removeClass("highlighted");
    }
}

// Resets all Radical-Tabs by removing class-modifiers including the selected tab
function closeAllTabs() {
    for (let i = 0; i < 10; i++) {
        $("#r-t"+i).removeClass("disabled");
        $("#r-t"+i).removeClass("selected");
    }
}

// Returns an array only containing selected radicals
function getSelectedRadicalArray() {
    let arr = [];

    // Populate radicals within JSON with all selected radicals
    for (let i = 0; i < radicalMask.length; i++) {
        for (let j = 0; j < radicalMask[i].length; j++) {
            if (radicalMask[i][j] == 1) {
                arr.push(radicals[i][j]);
            }
        }
    }

    return arr;
}

// Calls the API to get all kanji and radicals that are still possible
function getRadicalInfo() {
    // Create the JSON
    let radicalJSON = {
        "radicals": getSelectedRadicalArray()
    }

    // No Radicals selected, Reset
    if (radicalJSON.radicals.length == 0) {
        $('.rad-btn.disabled').each((i, e) => {
            $(e).removeClass("disabled");
        });
        iterateMaskAsync((i, j) => {
            if (radicalMask[i][j] == -1) {
                radicalMask[i][j] = 0;
            }
        });
        resetAllTabs();

        return;
    }

    // Send Request to backend
    $.ajax({
        type: "POST",
        url: "/api/kanji/by_radical",
        data: JSON.stringify(radicalJSON),
        headers: {
            'Content-Type': 'application/json'
        },
        success: function (result) {
            // Load the results into frontend
            loadRadicalResults(result);
        },
        error: function (result) {
            // Print Error
            Util.showMessage("error", getText("RADICAL_API_UNREACHABLE"))
        }
    });
}

// Calls the API to get input suggestions
var lastRadRequest;
function getRadicalSearchResults() {

    // Get value for the input
    let query = $("#kanji-search").val();
    if (query.length == 0) {
        return;
    }

    // Create the JSON
    let inputJSON = {
        "query": query
    }

    // Abort any requests sent earlier
    if (lastRadRequest !== undefined) {
        lastRadRequest.abort();
    }

    // Send Request to backend
    lastRadRequest = $.ajax({ 
        type : "POST", 
        url : "/api/radical/search", 
        data: JSON.stringify(inputJSON),
        headers: {
            'Content-Type': 'application/json'
        },
        success : function(result) { 
            // Load the results into frontend
            loadRadicalSearchResults(result);
            lastRadicalSearchResult = result;
        }, 
        error : function(result) { 
            $("#r-tc").removeClass("show");
            $("#r-tc").removeClass("selected");
        } 
    }); 
}

// Visualizes the results of getRadicalSearchResults
function loadRadicalSearchResults(results) {
    let firstFound = false;

    for (let i = 1; i <= 15; i++) {
        if (results.radicals[i] !== undefined) {
            if (!firstFound) {
                firstFound = true;
                
                clearRadicals();
                closeAllTabs();

                $("#r-tc").addClass("show");
                $("#r-tc").addClass("selected");
            }

            addRadicalsFromArray(i, results.radicals[i]);
        }
    }

    if (!firstFound) {
        $("#r-tc").removeClass("show")
        openRadicalPage(-1);
    }
}
