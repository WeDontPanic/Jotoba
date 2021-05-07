/**
 *  Used to handle the radical search
*/

var radicals = ["\u4e00","\uff5c","\u4e36","\u30ce","\u4e59","\u4e85","\u4e8c","\u4ea0","\u4eba","\u513f","\u5165","\u30cf","\u5182","\u5196","\u51ab","\u51e0","\u51f5","\u5200","\u529b","\u52f9","\u5315","\u531a","\u5341","\u535c","\u5369","\u5382","\u53b6","\u53c8","\u30de","\u4e5d","\u30e6","\u4e43","\u53e3","\u56d7","\u571f","\u58eb","\u5902","\u5915","\u5927","\u5973","\u5b50","\u5b80","\u5bf8","\u5c0f","\u5c22","\u5c38","\u5c6e","\u5c71","\u5ddd","\u5ddb","\u5de5","\u5df2","\u5dfe","\u5e72","\u5e7a","\u5e7f","\u5ef4","\u5efe","\u5f0b","\u5f13","\u30e8","\u5f51","\u5f61","\u5f73","\u4e5f","\u4ea1","\u53ca","\u4e45","\u5fc3","\u6208","\u6238","\u624b","\u652f","\u6535","\u6587","\u6597","\u65a4","\u65b9","\u65e0","\u65e5","\u66f0","\u6708","\u6728","\u6b20","\u6b62","\u6b79","\u6bb3","\u6bd4","\u6bdb","\u6c0f","\u6c14","\u6c34","\u706b","\u722a","\u7236","\u723b","\u723f","\u7247","\u725b","\u72ac","\u738b","\u5143","\u4e95","\u52ff","\u5c24","\u4e94","\u5c6f","\u5df4","\u6bcb","\u7384","\u74e6","\u7518","\u751f","\u7528","\u7530","\u758b","\u7676","\u767d","\u76ae","\u76bf","\u76ee","\u77db","\u77e2","\u77f3","\u793a","\u79be","\u7a74","\u7acb","\u4e16","\u5de8","\u518a","\u6bcd","\u7259","\u74dc","\u7af9","\u7c73","\u7cf8","\u7f36","\u7f8a","\u7fbd","\u800c","\u8012","\u8033","\u807f","\u8089","\u81ea","\u81f3","\u81fc","\u820c","\u821f","\u826e","\u8272","\u864d","\u866b","\u8840","\u884c","\u8863","\u897f","\u81e3","\u898b","\u89d2","\u8a00","\u8c37","\u8c46","\u8c55","\u8c78","\u8c9d","\u8d64","\u8d70","\u8db3","\u8eab","\u8eca","\u8f9b","\u8fb0","\u9149","\u91c6","\u91cc","\u821b","\u9ea6","\u91d1","\u9577","\u9580","\u96b6","\u96b9","\u96e8","\u9752","\u975e","\u5944","\u5ca1","\u514d","\u6589","\u9762","\u9769","\u97ed","\u97f3","\u9801","\u98a8","\u98db","\u98df","\u9996","\u9999","\u54c1","\u99ac","\u9aa8","\u9ad8","\u9adf","\u9b25","\u9b2f","\u9b32","\u9b3c","\u7adc","\u97cb","\u9b5a","\u9ce5","\u9e75","\u9e7f","\u9ebb","\u4e80","\u9ec4","\u9ed2","\u9ecd","\u9ef9","\u7121","\u6b6f","\u9efd","\u9f0e","\u9f13","\u9f20","\u9f3b","\u9f4a","\u9fa0"];

function toggleRadicalOverlay() {
    $('.overlay.speech').addClass('hidden');

    let overlay = $('.overlay.radical');
    overlay.toggleClass('hidden');

    if (overlay.hasClass("hidden")) {
        recognition.stop();
    }
}

function loadRadicals() {
    let strokeCounter = 1;

    for (let i = 0; i < radicals.length; i++) {
        if (i == 0 || i == 6 || i == 38 || i == 83 || i == 128 || i == 156 || i == 180 || i == 200 || i == 212 || i == 223 || i == 233 || i == 242 || i == 246 || i == 250 || i == 252) {
            $('.rad-picker').append('<div class="rad-btn picker num noselect">'+strokeCounter+'</div>');
            strokeCounter++;
            if (strokeCounter == 15) {
                strokeCounter = 17;
            }
        }
        $('.rad-picker').append('<div class="rad-btn picker noselect">'+radicals[i]+'</div>');
    }

    // <div class="rad-btn picker num noselect">1</div>
    // <div class="rad-btn picker noselect">一</div>
}

loadRadicals();

function loadRadicalResults() {

    // <div class="rad-btn result num noselect">1</div>
    // <div class="rad-btn result noselect">一</div>

}

