//! <https://adventofcode.com/2021/day/19>
//!
//! Runs very slowly without optimizations. (With optimizations it takes an acceptable few seconds.)

use lazy_static::lazy_static;
use nalgebra::{IsometryMatrix3, Point3, Rotation3, Translation3, Vector3};
use std::collections::{HashMap, HashSet};

const INPUT: [&str; 777] = [
    "--- scanner 0 ---",
    "806,301,-633",
    "-501,-417,319",
    "699,-593,593",
    "599,-948,-962",
    "666,506,480",
    "657,-639,635",
    "-363,523,-711",
    "-689,-409,-863",
    "523,-841,-961",
    "-466,-444,389",
    "-551,-478,293",
    "682,294,-783",
    "637,-604,531",
    "628,429,346",
    "-557,720,459",
    "698,394,-619",
    "-727,-433,-850",
    "-299,531,-672",
    "-405,725,448",
    "-418,610,-699",
    "544,505,321",
    "12,-5,-66",
    "534,-850,-930",
    "-659,-479,-863",
    "-481,563,432",
    "",
    "--- scanner 1 ---",
    "-567,723,271",
    "396,736,775",
    "-333,391,-925",
    "879,-437,-483",
    "435,619,-826",
    "391,674,792",
    "-362,-472,-516",
    "818,-588,563",
    "-675,800,276",
    "841,-447,-497",
    "-392,-344,-501",
    "-699,-555,431",
    "558,488,-853",
    "97,11,24",
    "883,-354,-438",
    "-369,353,-837",
    "-400,-372,-455",
    "777,-696,687",
    "537,540,-764",
    "390,680,786",
    "-305,394,-763",
    "745,-688,659",
    "-52,18,-125",
    "-754,-476,397",
    "-652,-554,484",
    "-503,789,349",
    "",
    "--- scanner 2 ---",
    "715,-612,-331",
    "545,344,802",
    "-544,-764,-423",
    "605,551,-225",
    "-484,815,-437",
    "-501,-701,621",
    "-547,683,-535",
    "646,-699,-245",
    "461,319,726",
    "615,-621,837",
    "592,631,-352",
    "674,-712,-435",
    "12,16,66",
    "-488,446,506",
    "-595,701,-474",
    "-446,-713,708",
    "-514,-703,-390",
    "583,478,725",
    "723,-635,930",
    "-382,-674,700",
    "-522,-922,-413",
    "-564,430,668",
    "-640,457,511",
    "-74,-142,125",
    "599,-785,937",
    "468,608,-260",
    "",
    "--- scanner 3 ---",
    "711,-748,-788",
    "-428,630,-632",
    "590,-772,-913",
    "605,-418,728",
    "-514,718,-615",
    "-815,-855,649",
    "-541,714,303",
    "441,331,-954",
    "109,-112,-17",
    "846,546,409",
    "-13,1,-175",
    "-481,598,-593",
    "-416,646,368",
    "860,485,221",
    "-793,-799,727",
    "592,-779,-647",
    "-648,-558,-530",
    "414,368,-881",
    "-391,664,243",
    "645,-341,704",
    "-680,-765,-573",
    "403,509,-949",
    "-840,-850,672",
    "466,-343,676",
    "934,609,281",
    "-674,-569,-489",
    "",
    "--- scanner 4 ---",
    "428,-524,476",
    "466,-579,-849",
    "-769,-857,-356",
    "406,-703,-775",
    "553,-542,501",
    "812,511,472",
    "-447,-558,513",
    "-430,-470,599",
    "705,539,591",
    "-755,469,-592",
    "-696,-820,-329",
    "865,811,-327",
    "79,-43,-21",
    "594,-642,-761",
    "-712,564,-682",
    "978,793,-405",
    "-693,607,426",
    "-764,-679,-318",
    "950,755,-407",
    "-740,539,490",
    "488,-474,626",
    "-408,-590,566",
    "844,420,607",
    "-770,435,-728",
    "-668,460,484",
    "",
    "--- scanner 5 ---",
    "-770,670,763",
    "812,733,-782",
    "-382,460,-566",
    "864,-289,801",
    "380,-691,-662",
    "-709,738,832",
    "553,686,617",
    "-386,599,-494",
    "528,604,507",
    "-495,-654,-628",
    "-352,-408,571",
    "-478,-568,-716",
    "-393,-434,560",
    "126,151,86",
    "804,591,-833",
    "809,-242,852",
    "-748,662,687",
    "786,-267,857",
    "413,640,643",
    "56,-22,-18",
    "-456,-601,-596",
    "-404,652,-635",
    "-436,-402,767",
    "773,636,-858",
    "505,-666,-644",
    "440,-541,-712",
    "",
    "--- scanner 6 ---",
    "415,738,-730",
    "-509,-231,-485",
    "-566,-398,-446",
    "895,-614,-866",
    "-673,871,-512",
    "-478,-737,417",
    "329,617,-773",
    "-435,-703,499",
    "425,-435,792",
    "835,-555,-763",
    "381,-366,613",
    "-2,82,-131",
    "488,849,633",
    "-547,-310,-637",
    "-436,812,529",
    "-304,875,431",
    "814,-461,-928",
    "-662,719,-517",
    "639,762,677",
    "-689,795,-674",
    "495,666,-823",
    "421,-468,663",
    "-477,-650,423",
    "538,758,741",
    "-418,764,468",
    "",
    "--- scanner 7 ---",
    "816,594,927",
    "764,-673,507",
    "-700,-812,470",
    "-509,776,-400",
    "-775,-530,-277",
    "-651,-775,612",
    "474,893,-700",
    "535,-581,-321",
    "-739,-564,-332",
    "-602,-628,-266",
    "-698,-780,644",
    "137,144,40",
    "826,-635,367",
    "-779,893,786",
    "947,459,955",
    "562,-418,-377",
    "-572,882,-393",
    "-617,891,703",
    "462,890,-494",
    "-608,806,812",
    "898,456,894",
    "-638,796,-428",
    "856,-752,466",
    "510,-495,-311",
    "413,960,-544",
    "26,4,-20",
    "",
    "--- scanner 8 ---",
    "-2,-27,-81",
    "279,771,385",
    "-722,552,475",
    "-883,-705,621",
    "418,-693,422",
    "673,-491,-462",
    "527,810,-417",
    "339,-785,381",
    "-572,-771,-592",
    "-730,327,449",
    "311,769,559",
    "-538,-703,-736",
    "-860,-496,609",
    "-649,732,-830",
    "-652,793,-712",
    "510,-464,-406",
    "561,846,-362",
    "-869,-653,584",
    "596,-439,-396",
    "463,673,-351",
    "456,-755,282",
    "-596,761,-615",
    "316,722,527",
    "-785,447,522",
    "-561,-867,-645",
    "",
    "--- scanner 9 ---",
    "534,387,316",
    "609,-822,763",
    "-598,-439,-626",
    "478,-699,790",
    "-779,659,516",
    "-722,660,680",
    "-837,742,607",
    "-457,-416,434",
    "726,-861,-838",
    "-425,588,-715",
    "41,-108,-42",
    "-447,-378,420",
    "-488,580,-642",
    "670,-867,-849",
    "791,664,-548",
    "796,474,-521",
    "664,-866,-855",
    "-364,-374,375",
    "-359,597,-749",
    "-698,-512,-722",
    "564,438,294",
    "779,574,-496",
    "530,-720,701",
    "536,308,410",
    "-706,-480,-750",
    "-74,50,-5",
    "",
    "--- scanner 10 ---",
    "-406,581,-714",
    "722,-721,317",
    "802,778,364",
    "-564,849,465",
    "-667,-365,-438",
    "880,877,-395",
    "-703,-353,-411",
    "663,-772,352",
    "-459,913,488",
    "365,-811,-472",
    "-585,-455,-439",
    "-46,127,15",
    "640,835,-400",
    "733,886,-350",
    "-476,497,-778",
    "71,-69,-135",
    "722,-803,507",
    "421,-731,-387",
    "-376,-800,384",
    "-365,-867,526",
    "-494,-863,468",
    "383,-733,-359",
    "741,836,476",
    "782,684,432",
    "-479,605,-874",
    "-582,767,515",
    "",
    "--- scanner 11 ---",
    "402,-784,-886",
    "783,741,-556",
    "-533,-704,643",
    "839,727,-764",
    "-821,-388,-547",
    "-465,784,451",
    "792,715,595",
    "664,690,666",
    "316,-559,712",
    "-683,-423,-526",
    "-440,637,393",
    "431,-649,-791",
    "-363,809,-761",
    "-378,775,-625",
    "740,735,-794",
    "-75,-30,-32",
    "-745,-486,-533",
    "-525,-622,485",
    "-421,680,-715",
    "406,-540,884",
    "-491,-643,554",
    "-393,705,457",
    "452,-813,-891",
    "65,119,-66",
    "456,-575,672",
    "567,766,593",
    "",
    "--- scanner 12 ---",
    "784,-492,511",
    "-295,-330,781",
    "884,816,628",
    "-346,698,653",
    "448,382,-760",
    "-344,-362,747",
    "933,886,678",
    "-350,-348,824",
    "872,819,834",
    "490,-826,-332",
    "399,385,-821",
    "811,-655,455",
    "582,-747,-373",
    "530,520,-793",
    "-308,-480,-589",
    "-547,403,-234",
    "520,-709,-233",
    "-587,379,-311",
    "-379,-467,-617",
    "85,84,166",
    "836,-569,445",
    "-332,886,592",
    "69,-48,4",
    "-258,826,716",
    "-330,-440,-596",
    "-726,431,-236",
    "",
    "--- scanner 13 ---",
    "411,707,-621",
    "-565,830,590",
    "-692,445,-432",
    "-739,-815,-383",
    "772,-378,-261",
    "801,-428,-256",
    "10,48,94",
    "-787,402,-522",
    "376,845,-600",
    "474,769,-614",
    "353,804,492",
    "-747,-716,-353",
    "-584,754,611",
    "-423,-607,800",
    "-508,703,701",
    "578,-458,360",
    "-753,-759,-591",
    "-821,387,-386",
    "323,770,583",
    "-649,-602,747",
    "-125,151,-15",
    "288,802,574",
    "618,-419,428",
    "-585,-660,787",
    "637,-407,-356",
    "496,-530,367",
    "",
    "--- scanner 14 ---",
    "608,-336,293",
    "752,444,-774",
    "-502,766,474",
    "-625,793,605",
    "-902,528,-904",
    "641,-374,-719",
    "-800,421,-990",
    "714,-534,-743",
    "-890,385,-949",
    "436,-409,304",
    "-89,60,-6",
    "644,-470,-827",
    "-641,-686,-497",
    "-457,-846,413",
    "-714,-625,-568",
    "717,476,-765",
    "-79,-90,-191",
    "238,703,699",
    "266,865,702",
    "687,442,-856",
    "-463,-835,357",
    "302,777,740",
    "-724,-581,-518",
    "500,-331,370",
    "-458,-852,419",
    "-604,704,553",
    "",
    "--- scanner 15 ---",
    "787,-650,573",
    "-404,-560,-455",
    "336,-785,-518",
    "797,-689,790",
    "-582,-698,615",
    "-67,-11,29",
    "785,770,-779",
    "-804,818,-693",
    "-792,863,-836",
    "520,834,444",
    "364,-788,-356",
    "642,895,555",
    "-673,-793,518",
    "850,688,-777",
    "775,-644,672",
    "-608,-826,595",
    "-711,854,745",
    "-527,-691,-428",
    "400,-826,-444",
    "560,902,483",
    "-861,886,833",
    "-506,-639,-517",
    "815,679,-745",
    "78,64,-58",
    "-702,904,-800",
    "-740,856,837",
    "",
    "--- scanner 16 ---",
    "768,605,-805",
    "-608,844,666",
    "347,571,569",
    "848,559,-637",
    "-611,783,784",
    "-475,498,-568",
    "563,-497,-533",
    "-538,603,-521",
    "798,-408,758",
    "810,-356,747",
    "545,610,547",
    "-699,-816,599",
    "678,-525,-466",
    "-722,-652,690",
    "502,545,513",
    "667,-496,-603",
    "-664,812,569",
    "-688,-720,546",
    "787,478,-779",
    "73,9,-106",
    "-400,-445,-655",
    "-79,60,53",
    "-530,378,-495",
    "-391,-602,-630",
    "-377,-654,-667",
    "742,-408,707",
    "",
    "--- scanner 17 ---",
    "-860,464,581",
    "404,733,736",
    "399,-669,688",
    "-865,422,341",
    "869,842,-527",
    "747,844,-651",
    "-882,-419,796",
    "564,-247,-439",
    "756,692,-561",
    "-513,-525,-716",
    "69,2,35",
    "350,-664,708",
    "601,-364,-349",
    "-896,-492,758",
    "480,712,717",
    "-493,-654,-688",
    "-84,119,78",
    "-407,-588,-723",
    "-765,-350,735",
    "531,622,793",
    "-396,458,-416",
    "-329,397,-572",
    "-399,446,-622",
    "-910,413,488",
    "389,-725,818",
    "661,-310,-316",
    "",
    "--- scanner 18 ---",
    "-544,-725,396",
    "699,-488,503",
    "-607,-772,501",
    "264,447,-773",
    "310,575,521",
    "-572,460,285",
    "-7,-109,20",
    "-460,-663,-704",
    "-469,-638,-547",
    "290,458,-950",
    "598,-690,-598",
    "-599,-798,276",
    "437,572,427",
    "-393,543,-957",
    "718,-514,376",
    "-187,20,-25",
    "779,-656,457",
    "-446,-626,-756",
    "326,596,469",
    "363,558,-826",
    "-736,444,385",
    "-722,336,292",
    "609,-827,-627",
    "494,-760,-599",
    "-425,608,-859",
    "-394,619,-850",
    "",
    "--- scanner 19 ---",
    "-792,-483,-620",
    "341,393,-874",
    "-563,907,-919",
    "-868,-534,-710",
    "-931,785,778",
    "700,-776,776",
    "-913,898,700",
    "547,396,-886",
    "-195,30,-19",
    "-811,-407,-599",
    "497,326,-935",
    "-589,-835,513",
    "486,-759,-640",
    "559,-844,787",
    "-40,53,-150",
    "522,514,344",
    "459,386,256",
    "452,-872,-490",
    "-656,857,-868",
    "-875,870,698",
    "-555,-850,377",
    "576,-750,-488",
    "-416,862,-910",
    "737,-849,749",
    "583,326,330",
    "-684,-792,423",
    "",
    "--- scanner 20 ---",
    "-407,576,646",
    "486,697,-523",
    "544,808,502",
    "-929,-530,-560",
    "542,-407,-580",
    "-821,-480,-469",
    "-656,426,-554",
    "585,-321,-701",
    "405,-675,559",
    "635,-487,-652",
    "620,783,586",
    "-661,-416,528",
    "543,649,624",
    "-442,519,509",
    "-534,496,633",
    "-865,-631,-556",
    "-741,-344,531",
    "329,670,-599",
    "-183,57,-25",
    "359,-719,448",
    "-717,444,-506",
    "320,-770,452",
    "-30,-75,-63",
    "-813,-345,599",
    "324,681,-447",
    "-619,312,-443",
    "",
    "--- scanner 21 ---",
    "429,874,443",
    "614,-831,729",
    "349,789,387",
    "807,850,-681",
    "-654,-676,-750",
    "-619,513,-843",
    "-565,-718,-689",
    "-468,-816,659",
    "427,902,405",
    "-450,736,687",
    "750,-846,763",
    "-585,768,778",
    "-633,426,-839",
    "-404,-769,591",
    "536,-388,-434",
    "704,-788,634",
    "35,16,-73",
    "-462,-744,738",
    "-555,442,-822",
    "-569,629,659",
    "705,822,-563",
    "795,823,-645",
    "-496,-684,-668",
    "-107,41,71",
    "522,-464,-532",
    "515,-526,-469",
    "",
    "--- scanner 22 ---",
    "365,-635,386",
    "581,917,409",
    "-8,123,57",
    "-370,-536,-733",
    "528,809,-505",
    "-334,618,-845",
    "787,-549,-660",
    "-614,794,504",
    "499,852,407",
    "590,882,441",
    "-648,-567,444",
    "-322,582,-872",
    "-642,-441,306",
    "837,-559,-616",
    "-424,700,-844",
    "406,-725,350",
    "-654,-331,419",
    "-607,731,442",
    "-593,829,366",
    "78,-10,-96",
    "-377,-454,-889",
    "636,806,-409",
    "-418,-429,-807",
    "451,-737,493",
    "612,879,-569",
    "858,-476,-721",
    "",
    "--- scanner 23 ---",
    "-851,915,696",
    "389,-697,-376",
    "281,-653,-479",
    "397,601,-595",
    "-534,868,-330",
    "-413,-663,-631",
    "-867,756,662",
    "376,684,-490",
    "630,-676,673",
    "709,-513,622",
    "-540,867,-324",
    "829,822,386",
    "-8,16,1",
    "447,611,-551",
    "814,731,514",
    "386,-751,-431",
    "740,-619,672",
    "-807,869,591",
    "-389,-373,604",
    "714,821,524",
    "-408,-473,-658",
    "-425,-573,-683",
    "-478,862,-341",
    "-397,-581,557",
    "-383,-375,578",
    "",
    "--- scanner 24 ---",
    "553,-531,-484",
    "515,901,443",
    "-488,-330,-877",
    "-395,-640,803",
    "-489,-468,-762",
    "719,-689,477",
    "666,-610,341",
    "-699,607,-495",
    "-554,-606,819",
    "676,-504,-338",
    "628,-731,322",
    "656,858,-646",
    "623,896,366",
    "451,823,377",
    "-579,-458,-862",
    "-626,546,-424",
    "-118,-62,-103",
    "-448,397,430",
    "729,698,-670",
    "-709,651,-445",
    "-443,380,492",
    "-441,-535,759",
    "-5,84,74",
    "659,762,-754",
    "571,-481,-402",
    "-442,386,486",
    "",
    "--- scanner 25 ---",
    "576,805,-368",
    "-522,908,464",
    "630,-324,-503",
    "735,983,767",
    "601,-613,565",
    "-656,770,-584",
    "-604,-268,-754",
    "452,655,-375",
    "659,984,874",
    "724,952,776",
    "621,-509,530",
    "559,-571,519",
    "560,641,-468",
    "20,5,45",
    "-746,-231,-803",
    "-510,931,673",
    "-849,-408,723",
    "592,-458,-580",
    "7,178,-115",
    "-609,833,-479",
    "-750,-291,753",
    "-682,778,-359",
    "-572,-263,-801",
    "-547,917,621",
    "-709,-370,661",
    "714,-299,-579",
    "",
    "--- scanner 26 ---",
    "843,-791,297",
    "921,-678,279",
    "755,637,-577",
    "520,957,655",
    "-471,-436,-492",
    "-449,574,-489",
    "-440,542,-454",
    "475,-712,-753",
    "-477,590,418",
    "748,575,-640",
    "149,105,-164",
    "566,899,526",
    "734,499,-542",
    "-767,-755,794",
    "-387,-372,-562",
    "-725,-750,554",
    "-567,641,342",
    "-456,514,399",
    "463,-739,-789",
    "-358,540,-519",
    "656,942,621",
    "-495,-276,-565",
    "841,-763,335",
    "-776,-800,647",
    "18,47,10",
    "565,-711,-671",
    "",
    "--- scanner 27 ---",
    "310,-373,899",
    "-770,469,775",
    "-603,-909,924",
    "408,454,706",
    "-534,339,-353",
    "550,-378,857",
    "-826,632,802",
    "-447,-842,863",
    "527,374,-750",
    "468,339,-699",
    "457,-328,908",
    "769,-365,-709",
    "-500,307,-304",
    "62,-61,94",
    "468,529,-781",
    "-689,-909,-341",
    "807,-337,-711",
    "-836,546,827",
    "449,498,858",
    "-653,-839,875",
    "-628,-721,-355",
    "-565,-849,-267",
    "825,-450,-789",
    "388,386,800",
    "-457,325,-359",
];

type ScannerId = u8;

lazy_static! {
    static ref DIRECTIONS: [Vector3<f64>; 6] = [
        Vector3::x(),
        -Vector3::x(),
        Vector3::y(),
        -Vector3::y(),
        Vector3::z(),
        -Vector3::z(),
    ];
}

fn parse_beacons(input_beacons: &[&str]) -> HashMap<ScannerId, Vec<Point3<f64>>> {
    let mut beacons_per_scanner = HashMap::new();
    let mut current_scanner = 0u8;
    for &row in input_beacons.iter().skip(1) {
        if row.is_empty() {
            continue;
        } else if row.contains("scanner") {
            current_scanner += 1;
        } else {
            let row_split: Vec<&str> = row.split(',').collect();
            beacons_per_scanner
                .entry(current_scanner)
                .or_insert_with(Vec::new)
                .push(Point3::new(
                    row_split[0].parse().unwrap(),
                    row_split[1].parse().unwrap(),
                    row_split[2].parse().unwrap(),
                ));
        }
    }
    beacons_per_scanner
}

/// Returns an isometry that maps at least 12 beacons in `beacons_relative` to beacons in
/// `beacons_absolute`; or returns [None] if no such isometry exists.
///
/// Only considers rotations of some integer number of 90-degree turns around all of the x, y, and
/// z axes, as per puzzle specification.
fn try_match(
    beacons_relative: &[Point3<f64>],
    beacons_absolute: &[Point3<f64>],
) -> Option<IsometryMatrix3<f64>> {
    // Try all possible rotations...
    for dir in DIRECTIONS.iter() {
        for up in DIRECTIONS.iter() {
            if up == dir || *up == (dir * -1.0) {
                // dir and up can't be collinear.
                continue;
            }

            let rotation = Rotation3::face_towards(dir, up);
            let rotated_beacons: Vec<Point3<f64>> =
                beacons_relative.iter().map(|p| rotation * p).collect();

            // ...and for each rotation, try all possible translations that result in at least one
            // pair of beacons matching...
            for head_beacon in beacons_absolute {
                for tail_beacon in &rotated_beacons {
                    let translation = Translation3::from(head_beacon - tail_beacon);
                    assert_eq!(translation * tail_beacon, *head_beacon);
                    let translated_rotated_beacons: Vec<Point3<f64>> =
                        rotated_beacons.iter().map(|p| translation * p).collect();

                    // ...and if at least 12 beacons match, return the isometry consisting of the
                    // successful rotation/translation combination.
                    let mut num_matching_beacons = 0;
                    for beacon_absolute in beacons_absolute {
                        if translated_rotated_beacons.contains(beacon_absolute) {
                            num_matching_beacons += 1;
                        }
                    }
                    if num_matching_beacons >= 12 {
                        return Some(IsometryMatrix3::from_parts(translation, rotation));
                    }
                }
            }
        }
    }
    None
}

pub fn solution_1() -> String {
    let beacons_per_scanner = parse_beacons(&INPUT);

    // Consider scanner 0's frame of reference to be the absolute frame of reference, and map
    // beacon coordinates from all other scanners to this frame of reference.
    let mut beacons_absolute_per_scanner = HashMap::from([(0u8, beacons_per_scanner[&0].clone())]);
    let mut mapped_scanners = HashSet::from([0u8]);
    let mut scanners_to_process = Vec::from([0u8]);
    while !(mapped_scanners.len() == beacons_per_scanner.len()) {
        let scanner_to_process = scanners_to_process.pop().unwrap();

        // Try to find a mapping into the absolute frame of reference for yet-unmapped scanners,
        // by matching against `scanner_to_process` (for which we have absolute beacon coordinates).
        for (&unmapped_scanner, beacons_relative) in &beacons_per_scanner {
            if mapped_scanners.contains(&unmapped_scanner) {
                continue;
            }

            if let Some(isometry) = try_match(
                beacons_relative,
                &beacons_absolute_per_scanner[&scanner_to_process],
            ) {
                let beacons_absolute = beacons_per_scanner[&unmapped_scanner]
                    .iter()
                    .map(|p| isometry * p)
                    .collect();
                beacons_absolute_per_scanner.insert(unmapped_scanner, beacons_absolute);
                mapped_scanners.insert(unmapped_scanner);
                scanners_to_process.push(unmapped_scanner);
            }
        }
    }

    let mut unique_beacons = Vec::new(); // Can't use HashSet, only have PartialEq.
    for beacons in beacons_absolute_per_scanner.values() {
        for beacon in beacons {
            if !unique_beacons.contains(beacon) {
                unique_beacons.push(*beacon);
            }
        }
    }

    unique_beacons.len().to_string()
}

fn manhattan_distance(p1: &Point3<f64>, p2: &Point3<f64>) -> f64 {
    (p1.coords.x - p2.coords.x).abs()
        + (p1.coords.y - p2.coords.y).abs()
        + (p1.coords.z - p2.coords.z).abs()
}

pub fn solution_2() -> String {
    let beacons_per_scanner = parse_beacons(&INPUT);

    // Consider scanner 0's frame of reference to be the absolute frame of reference, and map
    // beacon coordinates from all other scanners to this frame of reference; also determine
    // absolute coordinates of the scanners themselves in the process.
    let mut beacons_absolute_per_scanner = HashMap::from([(0u8, beacons_per_scanner[&0].clone())]);
    let mut scanner_coordinates = HashMap::from([(0u8, Point3::new(0.0, 0.0, 0.0))]);
    let mut mapped_scanners = HashSet::from([0u8]);
    let mut scanners_to_process = Vec::from([0u8]);
    while !(mapped_scanners.len() == beacons_per_scanner.len()) {
        let scanner_to_process = scanners_to_process.pop().unwrap();

        // Try to find a mapping into the absolute frame of reference for yet-unmapped scanners,
        // by matching against `scanner_to_process` (for which we have absolute beacon coordinates).
        for (&unmapped_scanner, beacons_relative) in &beacons_per_scanner {
            if mapped_scanners.contains(&unmapped_scanner) {
                continue;
            }

            if let Some(isometry) = try_match(
                beacons_relative,
                &beacons_absolute_per_scanner[&scanner_to_process],
            ) {
                let beacons_absolute = beacons_per_scanner[&unmapped_scanner]
                    .iter()
                    .map(|p| isometry * p)
                    .collect();
                beacons_absolute_per_scanner.insert(unmapped_scanner, beacons_absolute);
                scanner_coordinates.insert(unmapped_scanner, isometry * Point3::new(0.0, 0.0, 0.0));
                mapped_scanners.insert(unmapped_scanner);
                scanners_to_process.push(unmapped_scanner);
            }
        }
    }

    let mut max_distance = 0f64;
    for s1 in scanner_coordinates.values() {
        for s2 in scanner_coordinates.values() {
            max_distance = max_distance.max(manhattan_distance(s1, s2));
        }
    }
    max_distance.to_string()
}