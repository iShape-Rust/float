pub(crate) struct FixSin;

impl FixSin {
    const MAP: [u16; 129] = [
        0,    // 0
        12,   // 1
        25,   // 2
        37,   // 3
        50,   // 4
        62,   // 5
        75,   // 6
        87,   // 7
        100,  // 8
        112,  // 9
        125,  // 10
        137,  // 11
        150,  // 12
        162,  // 13
        175,  // 14
        187,  // 15
        199,  // 16
        212,  // 17
        224,  // 18
        236,  // 19
        248,  // 20
        260,  // 21
        273,  // 22
        285,  // 23
        297,  // 24
        309,  // 25
        321,  // 26
        333,  // 27
        344,  // 28
        356,  // 29
        368,  // 30
        380,  // 31
        391,  // 32
        403,  // 33
        414,  // 34
        426,  // 35
        437,  // 36
        449,  // 37
        460,  // 38
        471,  // 39
        482,  // 40
        493,  // 41
        504,  // 42
        515,  // 43
        526,  // 44
        537,  // 45
        547,  // 46
        558,  // 47
        568,  // 48
        579,  // 49
        589,  // 50
        599,  // 51
        609,  // 52
        620,  // 53
        629,  // 54
        639,  // 55
        649,  // 56
        659,  // 57
        668,  // 58
        678,  // 59
        687,  // 60
        696,  // 61
        706,  // 62
        715,  // 63
        724,  // 64
        732,  // 65
        741,  // 66
        750,  // 67
        758,  // 68
        767,  // 69
        775,  // 70
        783,  // 71
        791,  // 72
        799,  // 73
        807,  // 74
        814,  // 75
        822,  // 76
        829,  // 77
        837,  // 78
        844,  // 79
        851,  // 80
        858,  // 81
        865,  // 82
        871,  // 83
        878,  // 84
        884,  // 85
        890,  // 86
        897,  // 87
        903,  // 88
        908,  // 89
        914,  // 90
        920,  // 91
        925,  // 92
        930,  // 93
        936,  // 94
        941,  // 95
        946,  // 96
        950,  // 97
        955,  // 98
        959,  // 99
        964,  // 100
        968,  // 101
        972,  // 102
        976,  // 103
        979,  // 104
        983,  // 105
        986,  // 106
        990,  // 107
        993,  // 108
        996,  // 109
        999,  // 110
        1001, // 111
        1004, // 112
        1006, // 113
        1008, // 114
        1010, // 115
        1012, // 116
        1014, // 117
        1016, // 118
        1017, // 119
        1019, // 120
        1020, // 121
        1021, // 122
        1022, // 123
        1022, // 124
        1023, // 125
        1023, // 126
        1023, // 127
        1024  // 128
    ];

    pub(crate) fn value(index: usize) -> i64 {
        Self::MAP[index] as i64
    }

}