/**
 *  Used to handle the radical search
*/

var radicals = [
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


function toggleRadicalOverlay() {
    $('.overlay.speech').addClass('hidden');

    let overlay = $('.overlay.radical');
    overlay.toggleClass('hidden');

    if (overlay.hasClass("hidden")) {
        recognition.stop();
    }
}

function loadRadicals() {
    for (let i = 0; i < radicals.length; i++) {

        // There are no radicals with 15 or 16 strokes
        if (i == 14) {
            $('.rad-picker').append('<div class="rad-btn picker num noselect">'+17+'</div>');
        } else {
            $('.rad-picker').append('<div class="rad-btn picker num noselect">'+(i+1)+'</div>');
        }
        
        // Add the radical
        for (let ri = 0; ri < radicals[i].length; ri++) {
            $('.rad-picker').append('<div class="rad-btn picker noselect">'+radicals[i][ri]+'</div>');            
        }
    }
}

loadRadicals();

function loadRadicalResults() {

    // <div class="rad-btn result num noselect">1</div>
    // <div class="rad-btn result noselect">一</div>

}

