//! <https://adventofcode.com/2021/day/1>

const INPUT: [u32; 2000] = [
    193,
    197,
    188,
    170,
    162,
    180,
    183,
    211,
    213,
    235,
    238,
    237,
    234,
    227,
    222,
    221,
    223,
    224,
    223,
    216,
    217,
    226,
    234,
    228,
    230,
    237,
    239,
    242,
    244,
    246,
    242,
    258,
    269,
    270,
    255,
    257,
    261,
    262,
    258,
    257,
    262,
    261,
    260,
    263,
    266,
    282,
    281,
    290,
    299,
    300,
    302,
    288,
    286,
    296,
    293,
    292,
    272,
    270,
    283,
    294,
    311,
    310,
    309,
    310,
    312,
    314,
    318,
    330,
    331,
    332,
    344,
    342,
    328,
    317,
    314,
    310,
    306,
    310,
    351,
    353,
    352,
    354,
    355,
    349,
    350,
    354,
    353,
    352,
    357,
    362,
    358,
    346,
    329,
    322,
    320,
    294,
    277,
    270,
    273,
    275,
    283,
    279,
    277,
    271,
    273,
    246,
    247,
    248,
    255,
    258,
    243,
    267,
    266,
    282,
    281,
    282,
    284,
    288,
    290,
    289,
    288,
    298,
    302,
    299,
    300,
    303,
    297,
    324,
    325,
    329,
    332,
    313,
    309,
    307,
    299,
    289,
    315,
    320,
    311,
    295,
    303,
    307,
    329,
    330,
    329,
    317,
    318,
    334,
    335,
    310,
    312,
    296,
    297,
    296,
    299,
    300,
    301,
    284,
    300,
    283,
    278,
    286,
    281,
    276,
    266,
    278,
    279,
    268,
    261,
    274,
    281,
    282,
    299,
    298,
    299,
    297,
    296,
    295,
    286,
    283,
    282,
    283,
    298,
    308,
    310,
    287,
    280,
    281,
    280,
    282,
    275,
    247,
    244,
    245,
    240,
    236,
    241,
    250,
    252,
    247,
    244,
    246,
    250,
    251,
    259,
    272,
    270,
    269,
    277,
    279,
    280,
    297,
    298,
    299,
    301,
    315,
    322,
    323,
    326,
    328,
    322,
    323,
    316,
    315,
    293,
    303,
    334,
    344,
    345,
    346,
    358,
    359,
    360,
    361,
    374,
    380,
    378,
    379,
    377,
    382,
    383,
    384,
    388,
    389,
    396,
    408,
    409,
    410,
    401,
    405,
    400,
    404,
    403,
    424,
    444,
    450,
    440,
    430,
    412,
    415,
    436,
    430,
    445,
    450,
    472,
    477,
    480,
    464,
    470,
    462,
    456,
    463,
    467,
    466,
    465,
    466,
    473,
    474,
    470,
    471,
    479,
    481,
    501,
    500,
    499,
    502,
    493,
    485,
    498,
    493,
    505,
    506,
    494,
    498,
    501,
    515,
    517,
    523,
    520,
    517,
    511,
    510,
    522,
    517,
    523,
    524,
    520,
    518,
    517,
    527,
    525,
    536,
    510,
    511,
    526,
    516,
    522,
    524,
    543,
    539,
    540,
    547,
    560,
    577,
    591,
    592,
    593,
    596,
    597,
    598,
    581,
    575,
    576,
    586,
    585,
    598,
    599,
    603,
    600,
    602,
    603,
    604,
    595,
    600,
    621,
    601,
    604,
    607,
    621,
    618,
    621,
    619,
    620,
    617,
    615,
    614,
    628,
    630,
    627,
    635,
    634,
    629,
    650,
    663,
    665,
    669,
    670,
    682,
    662,
    677,
    679,
    669,
    677,
    676,
    692,
    693,
    682,
    697,
    691,
    702,
    718,
    721,
    725,
    753,
    752,
    751,
    746,
    741,
    737,
    765,
    780,
    806,
    805,
    803,
    786,
    787,
    801,
    812,
    818,
    797,
    794,
    792,
    798,
    800,
    799,
    803,
    788,
    799,
    804,
    805,
    812,
    814,
    805,
    803,
    804,
    791,
    796,
    786,
    785,
    797,
    791,
    792,
    811,
    809,
    800,
    818,
    824,
    825,
    824,
    823,
    841,
    835,
    848,
    843,
    846,
    828,
    811,
    812,
    806,
    795,
    798,
    802,
    809,
    827,
    838,
    837,
    836,
    833,
    812,
    814,
    817,
    816,
    827,
    826,
    829,
    831,
    829,
    831,
    832,
    817,
    830,
    827,
    828,
    829,
    832,
    836,
    835,
    837,
    840,
    833,
    831,
    833,
    832,
    833,
    835,
    840,
    868,
    882,
    886,
    883,
    873,
    893,
    891,
    897,
    898,
    897,
    898,
    895,
    889,
    890,
    892,
    880,
    901,
    890,
    889,
    891,
    894,
    899,
    927,
    928,
    927,
    918,
    937,
    935,
    931,
    929,
    935,
    939,
    937,
    936,
    935,
    944,
    928,
    930,
    926,
    927,
    933,
    935,
    959,
    961,
    947,
    955,
    961,
    960,
    956,
    968,
    973,
    978,
    977,
    980,
    994,
    996,
    974,
    978,
    997,
    1000,
    1009,
    1021,
    1038,
    1006,
    1009,
    997,
    999,
    1008,
    1004,
    1008,
    1010,
    1011,
    1010,
    1024,
    999,
    1000,
    999,
    1001,
    1011,
    1010,
    996,
    1024,
    1028,
    1021,
    1022,
    1038,
    1042,
    1044,
    1043,
    1081,
    1075,
    1072,
    1068,
    1069,
    1068,
    1067,
    1068,
    1070,
    1057,
    1054,
    1053,
    1038,
    1039,
    1064,
    1063,
    1064,
    1063,
    1061,
    1068,
    1053,
    1034,
    1056,
    1054,
    1055,
    1050,
    1040,
    1038,
    1036,
    1030,
    1011,
    1033,
    1035,
    1036,
    1049,
    1044,
    1038,
    1054,
    1062,
    1096,
    1097,
    1091,
    1092,
    1091,
    1090,
    1093,
    1102,
    1126,
    1124,
    1125,
    1160,
    1159,
    1162,
    1161,
    1145,
    1136,
    1137,
    1133,
    1134,
    1141,
    1143,
    1142,
    1122,
    1103,
    1099,
    1100,
    1097,
    1096,
    1107,
    1105,
    1103,
    1111,
    1150,
    1138,
    1140,
    1119,
    1088,
    1091,
    1089,
    1075,
    1079,
    1083,
    1085,
    1082,
    1076,
    1088,
    1086,
    1088,
    1083,
    1082,
    1088,
    1091,
    1093,
    1076,
    1078,
    1082,
    1081,
    1083,
    1085,
    1087,
    1086,
    1076,
    1095,
    1113,
    1109,
    1111,
    1110,
    1118,
    1119,
    1126,
    1117,
    1111,
    1129,
    1124,
    1131,
    1135,
    1134,
    1128,
    1141,
    1130,
    1121,
    1117,
    1113,
    1105,
    1097,
    1114,
    1115,
    1114,
    1119,
    1116,
    1115,
    1113,
    1112,
    1121,
    1124,
    1127,
    1126,
    1131,
    1134,
    1129,
    1131,
    1137,
    1139,
    1143,
    1145,
    1146,
    1135,
    1137,
    1142,
    1140,
    1141,
    1144,
    1142,
    1141,
    1138,
    1144,
    1141,
    1143,
    1128,
    1104,
    1112,
    1096,
    1101,
    1100,
    1099,
    1086,
    1071,
    1058,
    1057,
    1056,
    1047,
    1040,
    1036,
    1031,
    1032,
    1038,
    1043,
    1046,
    1035,
    1051,
    1049,
    1050,
    1054,
    1042,
    1043,
    1046,
    1034,
    1031,
    1016,
    1020,
    1019,
    1010,
    1011,
    1009,
    1010,
    1007,
    1010,
    1011,
    1022,
    1044,
    1043,
    1044,
    1048,
    1030,
    1028,
    1029,
    1054,
    1048,
    1069,
    1083,
    1082,
    1092,
    1121,
    1143,
    1144,
    1145,
    1146,
    1160,
    1165,
    1172,
    1169,
    1178,
    1181,
    1180,
    1181,
    1186,
    1198,
    1199,
    1190,
    1195,
    1227,
    1223,
    1222,
    1224,
    1237,
    1235,
    1242,
    1254,
    1256,
    1263,
    1260,
    1259,
    1270,
    1269,
    1264,
    1271,
    1272,
    1270,
    1273,
    1287,
    1304,
    1291,
    1287,
    1294,
    1291,
    1292,
    1296,
    1300,
    1301,
    1296,
    1327,
    1320,
    1341,
    1346,
    1345,
    1324,
    1328,
    1314,
    1317,
    1314,
    1300,
    1303,
    1296,
    1299,
    1326,
    1325,
    1343,
    1342,
    1338,
    1330,
    1329,
    1323,
    1331,
    1349,
    1348,
    1349,
    1350,
    1341,
    1353,
    1352,
    1353,
    1354,
    1356,
    1361,
    1357,
    1384,
    1389,
    1397,
    1400,
    1399,
    1409,
    1410,
    1415,
    1431,
    1432,
    1434,
    1421,
    1423,
    1385,
    1378,
    1387,
    1385,
    1378,
    1383,
    1411,
    1422,
    1421,
    1423,
    1429,
    1430,
    1434,
    1433,
    1432,
    1423,
    1387,
    1390,
    1392,
    1387,
    1388,
    1410,
    1407,
    1410,
    1408,
    1412,
    1418,
    1424,
    1422,
    1426,
    1420,
    1422,
    1423,
    1424,
    1444,
    1445,
    1442,
    1463,
    1459,
    1463,
    1468,
    1470,
    1475,
    1473,
    1462,
    1461,
    1458,
    1469,
    1466,
    1451,
    1453,
    1461,
    1474,
    1477,
    1475,
    1476,
    1470,
    1469,
    1467,
    1476,
    1484,
    1507,
    1505,
    1501,
    1481,
    1482,
    1483,
    1477,
    1488,
    1516,
    1505,
    1514,
    1504,
    1503,
    1487,
    1505,
    1513,
    1515,
    1512,
    1504,
    1506,
    1513,
    1521,
    1563,
    1562,
    1567,
    1568,
    1558,
    1557,
    1539,
    1514,
    1516,
    1524,
    1523,
    1524,
    1525,
    1516,
    1515,
    1517,
    1510,
    1488,
    1468,
    1467,
    1474,
    1473,
    1474,
    1478,
    1477,
    1476,
    1503,
    1529,
    1536,
    1560,
    1570,
    1581,
    1600,
    1601,
    1602,
    1605,
    1625,
    1629,
    1620,
    1609,
    1583,
    1599,
    1590,
    1592,
    1590,
    1592,
    1557,
    1554,
    1557,
    1535,
    1519,
    1529,
    1521,
    1520,
    1527,
    1537,
    1543,
    1539,
    1538,
    1511,
    1504,
    1523,
    1524,
    1529,
    1532,
    1530,
    1529,
    1536,
    1537,
    1511,
    1512,
    1536,
    1535,
    1526,
    1517,
    1515,
    1502,
    1503,
    1506,
    1479,
    1482,
    1480,
    1486,
    1488,
    1486,
    1477,
    1493,
    1489,
    1477,
    1478,
    1473,
    1472,
    1473,
    1472,
    1464,
    1466,
    1467,
    1468,
    1469,
    1470,
    1485,
    1483,
    1485,
    1475,
    1480,
    1481,
    1465,
    1474,
    1477,
    1478,
    1479,
    1478,
    1471,
    1470,
    1502,
    1518,
    1534,
    1497,
    1496,
    1490,
    1500,
    1504,
    1515,
    1516,
    1517,
    1540,
    1538,
    1537,
    1541,
    1528,
    1522,
    1526,
    1528,
    1529,
    1523,
    1497,
    1503,
    1492,
    1491,
    1497,
    1486,
    1514,
    1510,
    1514,
    1510,
    1503,
    1497,
    1490,
    1493,
    1517,
    1518,
    1517,
    1519,
    1520,
    1522,
    1523,
    1540,
    1544,
    1543,
    1545,
    1544,
    1517,
    1516,
    1531,
    1530,
    1526,
    1536,
    1533,
    1538,
    1565,
    1578,
    1577,
    1538,
    1541,
    1530,
    1528,
    1535,
    1552,
    1554,
    1557,
    1566,
    1577,
    1588,
    1614,
    1626,
    1616,
    1625,
    1627,
    1626,
    1627,
    1639,
    1644,
    1643,
    1645,
    1652,
    1618,
    1610,
    1586,
    1592,
    1587,
    1588,
    1589,
    1586,
    1585,
    1612,
    1611,
    1622,
    1644,
    1634,
    1638,
    1629,
    1625,
    1627,
    1620,
    1621,
    1622,
    1621,
    1623,
    1624,
    1622,
    1616,
    1598,
    1619,
    1620,
    1627,
    1628,
    1627,
    1605,
    1604,
    1605,
    1610,
    1617,
    1611,
    1618,
    1621,
    1620,
    1625,
    1624,
    1623,
    1622,
    1619,
    1620,
    1619,
    1622,
    1624,
    1631,
    1632,
    1634,
    1631,
    1632,
    1643,
    1648,
    1649,
    1651,
    1653,
    1642,
    1660,
    1672,
    1696,
    1694,
    1699,
    1688,
    1711,
    1728,
    1726,
    1725,
    1751,
    1750,
    1751,
    1758,
    1778,
    1771,
    1774,
    1804,
    1823,
    1803,
    1804,
    1807,
    1804,
    1795,
    1773,
    1775,
    1783,
    1773,
    1769,
    1770,
    1771,
    1772,
    1775,
    1778,
    1777,
    1782,
    1790,
    1788,
    1792,
    1790,
    1786,
    1788,
    1795,
    1793,
    1803,
    1804,
    1807,
    1811,
    1812,
    1836,
    1838,
    1829,
    1827,
    1830,
    1831,
    1830,
    1842,
    1851,
    1853,
    1850,
    1838,
    1839,
    1840,
    1841,
    1842,
    1839,
    1840,
    1834,
    1822,
    1823,
    1822,
    1821,
    1846,
    1851,
    1855,
    1853,
    1859,
    1858,
    1846,
    1848,
    1850,
    1882,
    1885,
    1894,
    1892,
    1903,
    1896,
    1875,
    1872,
    1877,
    1878,
    1872,
    1870,
    1875,
    1876,
    1877,
    1876,
    1873,
    1871,
    1881,
    1875,
    1888,
    1879,
    1877,
    1870,
    1871,
    1874,
    1879,
    1873,
    1872,
    1882,
    1884,
    1883,
    1892,
    1905,
    1911,
    1912,
    1911,
    1926,
    1930,
    1948,
    1949,
    1950,
    1949,
    1945,
    1949,
    1950,
    1961,
    1959,
    1957,
    1979,
    1980,
    1979,
    1963,
    1962,
    1964,
    1973,
    1978,
    1975,
    1966,
    1965,
    1964,
    1957,
    1958,
    1959,
    1960,
    1991,
    1997,
    1975,
    1986,
    1985,
    1982,
    1979,
    1978,
    1983,
    1989,
    1996,
    1976,
    1991,
    1996,
    2002,
    2004,
    2001,
    1998,
    2000,
    1992,
    2006,
    2007,
    2008,
    2010,
    2008,
    2010,
    2003,
    2004,
    2000,
    2002,
    2006,
    2000,
    2015,
    2021,
    2018,
    2001,
    1990,
    1993,
    1987,
    1990,
    1982,
    1992,
    1991,
    1993,
    1994,
    1995,
    1996,
    1989,
    1984,
    1989,
    1988,
    1987,
    1998,
    1990,
    1989,
    1991,
    1988,
    1993,
    1988,
    2000,
    2016,
    2024,
    2021,
    2029,
    2025,
    2019,
    2009,
    2008,
    2001,
    2009,
    2008,
    2009,
    1993,
    1990,
    1983,
    1982,
    1990,
    1991,
    1990,
    1989,
    1970,
    1962,
    1961,
    1963,
    1965,
    1969,
    1968,
    1988,
    1994,
    1995,
    2016,
    2015,
    2018,
    2017,
    2023,
    2027,
    2030,
    2020,
    2021,
    2022,
    2008,
    2010,
    1991,
    1995,
    1996,
    1995,
    1990,
    1991,
    1982,
    1986,
    1994,
    1991,
    1990,
    2005,
    2003,
    2004,
    2000,
    2008,
    2002,
    1994,
    1989,
    1992,
    2006,
    1997,
    1986,
    1982,
    1983,
    1986,
    1977,
    1974,
    1958,
    1962,
    1967,
    1970,
    1998,
    2011,
    2008,
    2009,
    2006,
    1998,
    1992,
    1997,
    1996,
    1989,
    1984,
    1990,
    1989,
    1997,
    1996,
    1966,
    1965,
    1960,
    1959,
    1928,
    1930,
    1931,
    1935,
    1929,
    1928,
    1930,
    1942,
    1929,
    1932,
    1915,
    1914,
    1888,
    1886,
    1896,
    1898,
    1886,
    1885,
    1897,
    1899,
    1910,
    1919,
    1921,
    1911,
    1904,
    1905,
    1897,
    1913,
    1925,
    1930,
    1938,
    1944,
    1946,
    1966,
    1979,
    1981,
    1984,
    1982,
    1985,
    2000,
    1992,
    1991,
    2007,
    2006,
    2008,
    2009,
    2005,
    2012,
    2008,
    2009,
    2010,
    2003,
    2007,
    2017,
    2016,
    2030,
    2028,
    2030,
    2035,
    2036,
    2033,
    2031,
    2037,
    2044,
    2042,
    2040,
    2038,
    2040,
    2031,
    2020,
    2043,
    2044,
    2061,
    2063,
    2044,
    2041,
    2043,
    2044,
    2071,
    2075,
    2081,
    2071,
    2077,
    2079,
    2080,
    2056,
    2047,
    2046,
    2060,
    2059,
    2061,
    2062,
    2082,
    2084,
    2085,
    2079,
    2090,
    2080,
    2079,
    2081,
    2071,
    2073,
    2074,
    2078,
    2086,
    2094,
    2112,
    2129,
    2125,
    2136,
    2138,
    2144,
    2142,
    2140,
    2158,
    2161,
    2149,
    2158,
    2168,
    2169,
    2165,
    2154,
    2168,
    2170,
    2163,
    2178,
    2186,
    2182,
    2189,
    2193,
    2204,
    2206,
    2219,
    2221,
    2219,
    2237,
    2247,
    2248,
    2246,
    2245,
    2269,
    2262,
    2263,
    2267,
    2271,
    2268,
    2270,
    2300,
    2305,
    2310,
    2309,
    2303,
    2306,
    2307,
    2308,
    2302,
    2301,
    2321,
    2311,
    2312,
    2314,
    2320,
    2319,
    2321,
    2326,
    2327,
    2328,
    2327,
    2311,
    2315,
    2314,
    2317,
    2327,
    2326,
    2320,
    2321,
    2319,
    2311,
    2304,
    2306,
    2314,
    2318,
    2319,
    2321,
    2307,
    2286,
    2289,
    2290,
    2294,
    2293,
    2294,
    2291,
    2290,
    2317,
    2322,
    2323,
    2329,
    2350,
    2356,
    2361,
    2376,
    2375,
    2379,
    2353,
    2347,
    2339,
    2340,
    2331,
    2332,
    2330,
    2317,
    2293,
    2296,
    2301,
    2289,
    2281,
    2280,
    2293,
    2296,
    2295,
    2302,
    2307,
    2337,
    2327,
    2344,
    2331,
    2341,
    2344,
    2335,
    2345,
    2344,
    2348,
    2352,
    2351,
    2340,
    2331,
    2332,
    2327,
    2316,
    2334,
    2335,
    2336,
    2343,
    2344,
    2349,
    2345,
    2353,
    2338,
    2339,
    2332,
    2330,
    2340,
    2337,
    2347,
    2345,
    2351,
    2352,
    2344,
    2350,
    2351,
    2356,
    2371,
    2370,
    2384,
    2383,
    2372,
    2382,
    2375,
    2376,
    2353,
    2360,
    2366,
    2344,
    2346,
    2365,
    2371,
    2378,
    2362,
    2354,
    2361,
    2356,
    2357,
    2358,
    2315,
    2324,
    2310,
    2311,
    2312,
    2291,
    2296,
    2295,
    2307,
    2304,
    2303,
    2308,
    2309,
    2299,
    2319,
    2322,
    2325,
    2327,
    2326,
    2319,
    2322,
    2319,
    2320,
    2322,
    2321,
    2323,
    2318,
    2325,
    2326,
    2312,
    2313,
    2316,
    2317,
    2338,
    2349,
    2347,
    2351,
    2352,
    2333,
    2340,
    2365,
    2357,
    2362,
    2372,
    2375,
    2376,
    2392,
    2395,
    2399,
    2407,
    2396,
    2391,
    2394,
    2401,
    2406,
    2399,
    2409,
    2421,
    2423,
    2400,
    2398,
    2399,
    2393,
    2392,
    2390,
    2382,
    2381,
    2379,
    2381,
    2390,
    2387,
    2389,
    2391,
    2393,
    2394,
    2397,
    2398,
    2399,
    2396,
    2394,
    2392,
    2400,
    2396,
    2397,
    2425,
    2416,
    2442,
    2443,
    2445,
    2442,
    2445,
    2434,
    2439,
    2440,
    2448,
    2447,
    2451,
    2442,
    2446,
    2451,
    2452,
    2453,
    2463,
    2460,
    2461,
    2466,
    2450,
    2460,
    2464,
    2465,
    2466,
    2468,
    2470,
    2477,
    2473,
    2469,
    2471,
    2476,
    2475,
    2476,
    2488,
    2489,
    2491,
    2490,
    2515,
    2533,
    2531,
    2532,
    2531,
    2533,
    2537,
    2539,
    2537,
    2533,
    2532,
    2529,
    2536,
    2535,
    2518,
    2520,
    2519,
    2517,
    2548,
    2549,
    2520,
    2521,
    2523,
    2546,
    2538,
    2543,
    2552,
    2561,
    2579,
    2581,
    2564,
    2567,
    2569,
    2570,
    2569,
    2577,
    2586,
    2587,
    2584,
    2601,
    2572,
    2579,
    2593,
    2586,
    2606,
    2604,
    2606,
    2600,
    2599,
    2602,
    2603,
    2621,
    2622,
    2623,
    2622,
    2623,
    2634,
    2646,
    2649,
    2677,
    2678,
    2679,
    2640,
    2642,
    2640,
    2648,
    2649,
    2639,
    2633,
    2636,
    2635,
    2642,
    2644,
    2643,
    2644,
    2636,
    2641,
    2642,
    2648,
    2647,
    2646,
    2658,
];

pub fn solution_1() -> String {
    let mut num_depth_increases: u32 = 0;
    for (i, &depth) in INPUT.iter().enumerate().skip(1) {
        let previous_depth = INPUT[i - 1];
        if depth > previous_depth {
            num_depth_increases += 1;
        }
    }
    num_depth_increases.to_string()
}

pub fn solution_2() -> String {
    String::from("TODO")
}
