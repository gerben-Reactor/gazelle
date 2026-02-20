#[doc(hidden)]
mod __table {
    use super::gazelle;
    pub static ACTION_DATA: &[u32] = &[
        20u32, 4294967295u32, 4294967286u32, 46u32, 4294967288u32, 4294967288u32,
        4294967287u32, 4294967287u32, 4294967275u32, 4294967275u32, 4294967282u32,
        4294967282u32, 4294967274u32, 4294967273u32, 16u32, 16u32, 4294967283u32,
        4294967270u32, 4294967265u32, 4294967274u32, 4294967273u32, 38u32, 4294967274u32,
        4294967273u32, 4294967270u32, 4294967265u32, 4294967266u32, 4294967270u32,
        4294967265u32, 4294967274u32, 4294967273u32, 4294967267u32, 4294967268u32,
        4294967266u32, 4294967270u32, 4294967265u32, 4294967266u32, 45u32, 4294967267u32,
        4294967268u32, 4294967269u32, 4294967267u32, 4294967268u32, 4294967266u32, 15u32,
        14u32, 13u32, 4294967269u32, 4294967267u32, 4294967268u32, 4294967269u32, 48u32,
        52u32, 53u32, 54u32, 55u32, 12u32, 4294967269u32, 12u32, 46u32, 12u32, 21u32,
        4294967291u32, 11u32, 4294967291u32, 11u32, 23u32, 11u32, 23u32, 4294967293u32,
        23u32, 4294967293u32, 22u32, 19u32, 4294967292u32, 4294967284u32, 4294967292u32,
        4294967284u32, 4294967294u32, 4294967285u32, 4294967294u32, 4294967285u32, 37u32,
        41u32, 28u32, 6u32, 4294967280u32, 4294967279u32, 4294967281u32, 36u32,
        4294967280u32, 4294967279u32, 4294967281u32, 4294967278u32, 4294967290u32,
        4294967289u32, 27u32, 4294967278u32, 4294967290u32, 4294967289u32, 7u32, 10u32,
        25u32, 4294967272u32, 4294967272u32, 4294967271u32, 4294967271u32, 4294967277u32,
        4294967277u32, 4294967276u32, 4294967276u32, 26u32, 5u32, 29u32, 32u32, 34u32,
        18u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
        0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
        0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
        0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
        0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
        0u32, 0u32, 0u32,
    ];
    pub static ACTION_BASE: &[i32] = &[
        -3i32,
        1i32,
        2i32,
        4i32,
        6i32,
        8i32,
        9i32,
        10i32,
        11i32,
        12i32,
        16i32,
        17i32,
        25i32,
        30i32,
        31i32,
        39i32,
        15i32,
        20i32,
        36i32,
        50i32,
        51i32,
        52i32,
        53i32,
        54i32,
        55i32,
        57i32,
        59i32,
        58i32,
        59i32,
        65i32,
        58i32,
        70i32,
        71i32,
        74i32,
        75i32,
        78i32,
        75i32,
        76i32,
        76i32,
        77i32,
        78i32,
        83i32,
        84i32,
        85i32,
        86i32,
        89i32,
        87i32,
        87i32,
        89i32,
        91i32,
        93i32,
        95i32,
        96i32,
        97i32,
        98i32,
        94i32,
    ];
    pub static ACTION_CHECK: &[u32] = &[
        0u32, 1u32, 2u32, 2u32, 3u32, 3u32, 4u32, 4u32, 5u32, 5u32, 6u32, 7u32, 8u32,
        9u32, 6u32, 7u32, 16u32, 10u32, 11u32, 8u32, 9u32, 17u32, 8u32, 9u32, 10u32,
        11u32, 12u32, 10u32, 11u32, 8u32, 9u32, 13u32, 14u32, 12u32, 10u32, 11u32, 12u32,
        18u32, 13u32, 14u32, 15u32, 13u32, 14u32, 12u32, 12u32, 12u32, 12u32, 15u32,
        13u32, 14u32, 15u32, 19u32, 20u32, 21u32, 22u32, 23u32, 24u32, 15u32, 25u32,
        27u32, 26u32, 28u32, 30u32, 24u32, 30u32, 25u32, 24u32, 26u32, 25u32, 29u32,
        26u32, 29u32, 29u32, 24u32, 31u32, 32u32, 31u32, 32u32, 33u32, 34u32, 33u32,
        34u32, 35u32, 36u32, 35u32, 37u32, 38u32, 39u32, 40u32, 38u32, 38u32, 39u32,
        40u32, 41u32, 42u32, 43u32, 44u32, 41u32, 42u32, 43u32, 44u32, 45u32, 46u32,
        47u32, 47u32, 48u32, 48u32, 49u32, 49u32, 50u32, 50u32, 51u32, 51u32, 52u32,
        53u32, 54u32, 55u32, 4294967295u32, 4294967295u32, 4294967295u32, 4294967295u32,
        4294967295u32, 4294967295u32, 4294967295u32, 4294967295u32, 4294967295u32,
        4294967295u32, 4294967295u32, 4294967295u32, 4294967295u32, 4294967295u32,
        4294967295u32, 4294967295u32, 4294967295u32, 4294967295u32, 4294967295u32,
        4294967295u32, 4294967295u32, 4294967295u32, 4294967295u32, 4294967295u32,
        4294967295u32, 4294967295u32, 4294967295u32, 4294967295u32, 4294967295u32,
        4294967295u32, 4294967295u32, 4294967295u32, 4294967295u32, 4294967295u32,
        4294967295u32, 4294967295u32, 4294967295u32, 4294967295u32, 4294967295u32,
        4294967295u32, 4294967295u32, 4294967295u32, 4294967295u32, 4294967295u32,
        4294967295u32, 4294967295u32, 4294967295u32, 4294967295u32, 4294967295u32,
        4294967295u32, 4294967295u32, 4294967295u32, 4294967295u32, 4294967295u32,
        4294967295u32, 4294967295u32, 4294967295u32, 4294967295u32, 4294967295u32,
        4294967295u32, 4294967295u32, 4294967295u32, 4294967295u32, 4294967295u32,
        4294967295u32, 4294967295u32, 4294967295u32,
    ];
    pub static GOTO_DATA: &[u32] = &[
        1u32, 3u32, 43u32, 42u32, 47u32, 8u32, 50u32, 49u32, 9u32, 9u32, 44u32, 4u32,
        17u32, 17u32, 33u32, 51u32, 24u32, 24u32, 2u32, 35u32, 31u32, 40u32, 30u32, 0u32,
        0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 39u32, 0u32, 0u32, 0u32, 0u32,
    ];
    pub static GOTO_BASE: &[i32] = &[
        0i32,
        0i32,
        -4i32,
        0i32,
        0i32,
        0i32,
        -1i32,
        0i32,
        0i32,
        0i32,
        0i32,
        0i32,
        0i32,
        0i32,
        0i32,
        0i32,
        0i32,
        0i32,
        0i32,
        0i32,
        0i32,
        0i32,
        0i32,
        0i32,
        -3i32,
        0i32,
        1i32,
        6i32,
        0i32,
        13i32,
        9i32,
        0i32,
        0i32,
        0i32,
        0i32,
        18i32,
        0i32,
        0i32,
        17i32,
        0i32,
        0i32,
        0i32,
        0i32,
        0i32,
        0i32,
        0i32,
        0i32,
        0i32,
        0i32,
        0i32,
        0i32,
        0i32,
        0i32,
        0i32,
        0i32,
        0i32,
    ];
    pub static GOTO_CHECK: &[u32] = &[
        0u32, 2u32, 6u32, 7u32, 24u32, 24u32, 25u32, 26u32, 25u32, 26u32, 6u32, 27u32,
        6u32, 7u32, 29u32, 25u32, 25u32, 26u32, 27u32, 30u32, 35u32, 38u32, 29u32,
        4294967295u32, 4294967295u32, 4294967295u32, 4294967295u32, 4294967295u32,
        4294967295u32, 4294967295u32, 4294967295u32, 38u32, 4294967295u32, 4294967295u32,
        4294967295u32, 4294967295u32,
    ];
    pub static RULES: &[(u32, u8)] = &[
        (40u32, 1u8),
        (32u32, 1u8),
        (32u32, 0u8),
        (33u32, 2u8),
        (33u32, 0u8),
        (34u32, 3u8),
        (34u32, 1u8),
        (35u32, 2u8),
        (35u32, 1u8),
        (23u32, 10u8),
        (24u32, 3u8),
        (25u32, 4u8),
        (36u32, 1u8),
        (36u32, 0u8),
        (37u32, 1u8),
        (37u32, 0u8),
        (26u32, 3u8),
        (27u32, 2u8),
        (38u32, 3u8),
        (38u32, 1u8),
        (28u32, 4u8),
        (39u32, 2u8),
        (39u32, 1u8),
        (29u32, 2u8),
        (30u32, 2u8),
        (31u32, 5u8),
        (31u32, 2u8),
        (31u32, 2u8),
        (31u32, 2u8),
        (31u32, 1u8),
        (31u32, 1u8),
    ];
    pub static STATE_SYMBOL: &[u32] = &[
        0u32, 23u32, 35u32, 28u32, 28u32, 17u32, 9u32, 14u32, 31u32, 31u32, 12u32, 8u32,
        1u32, 21u32, 20u32, 19u32, 5u32, 36u32, 22u32, 18u32, 3u32, 2u32, 7u32, 11u32,
        39u32, 15u32, 16u32, 10u32, 6u32, 17u32, 32u32, 25u32, 17u32, 24u32, 17u32,
        33u32, 13u32, 4u32, 1u32, 37u32, 27u32, 8u32, 26u32, 26u32, 34u32, 1u32, 1u32,
        30u32, 1u32, 29u32, 29u32, 38u32, 1u32, 1u32, 1u32, 1u32,
    ];
    pub const NUM_STATES: usize = 56usize;
    pub const NUM_TERMINALS: u32 = 23u32;
    #[allow(dead_code)]
    pub const NUM_NON_TERMINALS: u32 = 18u32;
    pub static SYMBOL_NAMES: &[&str] = &[
        "$",
        "IDENT",
        "NUM",
        "KW_START",
        "KW_TERMINALS",
        "KW_PREC",
        "KW_EXPECT",
        "KW_MODE",
        "UNDERSCORE",
        "LBRACE",
        "RBRACE",
        "LPAREN",
        "RPAREN",
        "COLON",
        "COMMA",
        "EQ",
        "PIPE",
        "SEMI",
        "FAT_ARROW",
        "QUESTION",
        "STAR",
        "PLUS",
        "PERCENT",
        "grammar_def",
        "mode_decl",
        "expect_decl",
        "terminal_item",
        "type_annot",
        "rule",
        "alt",
        "variant",
        "term",
        "__mode_decl_opt",
        "__expect_decl_star",
        "__terminal_item_sep_comma",
        "__rule_plus",
        "__kw_prec_opt",
        "__type_annot_opt",
        "__alt_sep_pipe",
        "__term_plus",
        "__start",
    ];
    static STATE_ITEMS_0: &[(u16, u8)] = &[(0u16, 0u8), (9u16, 0u8)];
    static STATE_ITEMS_1: &[(u16, u8)] = &[(0u16, 1u8)];
    static STATE_ITEMS_2: &[(u16, u8)] = &[
        (9u16, 10u8),
        (7u16, 1u8),
        (20u16, 0u8),
        (7u16, 1u8),
        (20u16, 0u8),
    ];
    static STATE_ITEMS_3: &[(u16, u8)] = &[(7u16, 2u8), (7u16, 2u8)];
    static STATE_ITEMS_4: &[(u16, u8)] = &[(8u16, 1u8), (8u16, 1u8)];
    static STATE_ITEMS_5: &[(u16, u8)] = &[(20u16, 4u8), (20u16, 4u8)];
    static STATE_ITEMS_6: &[(u16, u8)] = &[
        (9u16, 7u8),
        (5u16, 0u8),
        (6u16, 0u8),
        (5u16, 0u8),
        (6u16, 0u8),
        (16u16, 0u8),
        (16u16, 0u8),
        (12u16, 0u8),
        (13u16, 0u8),
    ];
    static STATE_ITEMS_7: &[(u16, u8)] = &[
        (16u16, 0u8),
        (5u16, 2u8),
        (16u16, 0u8),
        (12u16, 0u8),
        (13u16, 0u8),
        (5u16, 2u8),
    ];
    static STATE_ITEMS_8: &[(u16, u8)] = &[
        (21u16, 2u8),
        (21u16, 2u8),
        (21u16, 2u8),
        (21u16, 2u8),
    ];
    static STATE_ITEMS_9: &[(u16, u8)] = &[
        (22u16, 1u8),
        (22u16, 1u8),
        (22u16, 1u8),
        (22u16, 1u8),
    ];
    static STATE_ITEMS_10: &[(u16, u8)] = &[
        (25u16, 5u8),
        (25u16, 5u8),
        (25u16, 5u8),
        (25u16, 5u8),
    ];
    static STATE_ITEMS_11: &[(u16, u8)] = &[
        (30u16, 1u8),
        (30u16, 1u8),
        (30u16, 1u8),
        (30u16, 1u8),
    ];
    static STATE_ITEMS_12: &[(u16, u8)] = &[
        (26u16, 1u8),
        (27u16, 1u8),
        (28u16, 1u8),
        (29u16, 1u8),
        (26u16, 1u8),
        (27u16, 1u8),
        (28u16, 1u8),
        (29u16, 1u8),
        (26u16, 1u8),
        (27u16, 1u8),
        (28u16, 1u8),
        (29u16, 1u8),
        (26u16, 1u8),
        (27u16, 1u8),
        (28u16, 1u8),
        (29u16, 1u8),
    ];
    static STATE_ITEMS_13: &[(u16, u8)] = &[
        (28u16, 2u8),
        (28u16, 2u8),
        (28u16, 2u8),
        (28u16, 2u8),
    ];
    static STATE_ITEMS_14: &[(u16, u8)] = &[
        (27u16, 2u8),
        (27u16, 2u8),
        (27u16, 2u8),
        (27u16, 2u8),
    ];
    static STATE_ITEMS_15: &[(u16, u8)] = &[
        (26u16, 2u8),
        (26u16, 2u8),
        (26u16, 2u8),
        (26u16, 2u8),
    ];
    static STATE_ITEMS_16: &[(u16, u8)] = &[(12u16, 1u8)];
    static STATE_ITEMS_17: &[(u16, u8)] = &[(16u16, 1u8), (16u16, 1u8)];
    static STATE_ITEMS_18: &[(u16, u8)] = &[
        (25u16, 3u8),
        (25u16, 3u8),
        (25u16, 3u8),
        (25u16, 3u8),
    ];
    static STATE_ITEMS_19: &[(u16, u8)] = &[(24u16, 1u8), (24u16, 1u8)];
    static STATE_ITEMS_20: &[(u16, u8)] = &[(9u16, 1u8)];
    static STATE_ITEMS_21: &[(u16, u8)] = &[(11u16, 2u8), (11u16, 2u8)];
    static STATE_ITEMS_22: &[(u16, u8)] = &[(10u16, 1u8), (10u16, 1u8)];
    static STATE_ITEMS_23: &[(u16, u8)] = &[
        (25u16, 1u8),
        (25u16, 1u8),
        (25u16, 1u8),
        (25u16, 1u8),
    ];
    static STATE_ITEMS_24: &[(u16, u8)] = &[
        (23u16, 1u8),
        (23u16, 1u8),
        (24u16, 0u8),
        (21u16, 1u8),
        (25u16, 0u8),
        (26u16, 0u8),
        (27u16, 0u8),
        (28u16, 0u8),
        (29u16, 0u8),
        (30u16, 0u8),
        (24u16, 0u8),
        (21u16, 1u8),
        (21u16, 1u8),
        (21u16, 1u8),
        (25u16, 0u8),
        (26u16, 0u8),
        (27u16, 0u8),
        (28u16, 0u8),
        (29u16, 0u8),
        (30u16, 0u8),
        (25u16, 0u8),
        (26u16, 0u8),
        (27u16, 0u8),
        (28u16, 0u8),
        (29u16, 0u8),
        (30u16, 0u8),
        (25u16, 0u8),
        (26u16, 0u8),
        (27u16, 0u8),
        (28u16, 0u8),
        (29u16, 0u8),
        (30u16, 0u8),
    ];
    static STATE_ITEMS_25: &[(u16, u8)] = &[
        (20u16, 2u8),
        (20u16, 2u8),
        (18u16, 0u8),
        (19u16, 0u8),
        (18u16, 0u8),
        (19u16, 0u8),
        (23u16, 0u8),
        (23u16, 0u8),
        (21u16, 0u8),
        (22u16, 0u8),
        (21u16, 0u8),
        (21u16, 0u8),
        (21u16, 0u8),
        (22u16, 0u8),
        (22u16, 0u8),
        (22u16, 0u8),
        (25u16, 0u8),
        (26u16, 0u8),
        (27u16, 0u8),
        (28u16, 0u8),
        (29u16, 0u8),
        (30u16, 0u8),
        (25u16, 0u8),
        (26u16, 0u8),
        (27u16, 0u8),
        (28u16, 0u8),
        (29u16, 0u8),
        (30u16, 0u8),
        (25u16, 0u8),
        (26u16, 0u8),
        (27u16, 0u8),
        (28u16, 0u8),
        (29u16, 0u8),
        (30u16, 0u8),
        (25u16, 0u8),
        (26u16, 0u8),
        (27u16, 0u8),
        (28u16, 0u8),
        (29u16, 0u8),
        (30u16, 0u8),
    ];
    static STATE_ITEMS_26: &[(u16, u8)] = &[
        (23u16, 0u8),
        (18u16, 2u8),
        (23u16, 0u8),
        (21u16, 0u8),
        (22u16, 0u8),
        (18u16, 2u8),
        (21u16, 0u8),
        (21u16, 0u8),
        (21u16, 0u8),
        (22u16, 0u8),
        (22u16, 0u8),
        (22u16, 0u8),
        (25u16, 0u8),
        (26u16, 0u8),
        (27u16, 0u8),
        (28u16, 0u8),
        (29u16, 0u8),
        (30u16, 0u8),
        (25u16, 0u8),
        (26u16, 0u8),
        (27u16, 0u8),
        (28u16, 0u8),
        (29u16, 0u8),
        (30u16, 0u8),
        (25u16, 0u8),
        (26u16, 0u8),
        (27u16, 0u8),
        (28u16, 0u8),
        (29u16, 0u8),
        (30u16, 0u8),
        (25u16, 0u8),
        (26u16, 0u8),
        (27u16, 0u8),
        (28u16, 0u8),
        (29u16, 0u8),
        (30u16, 0u8),
    ];
    static STATE_ITEMS_27: &[(u16, u8)] = &[
        (9u16, 9u8),
        (7u16, 0u8),
        (8u16, 0u8),
        (7u16, 0u8),
        (8u16, 0u8),
        (20u16, 0u8),
        (20u16, 0u8),
    ];
    static STATE_ITEMS_28: &[(u16, u8)] = &[(11u16, 1u8), (11u16, 1u8)];
    static STATE_ITEMS_29: &[(u16, u8)] = &[
        (9u16, 3u8),
        (1u16, 0u8),
        (1u16, 0u8),
        (2u16, 0u8),
        (2u16, 0u8),
        (10u16, 0u8),
        (10u16, 0u8),
    ];
    static STATE_ITEMS_30: &[(u16, u8)] = &[
        (9u16, 4u8),
        (3u16, 0u8),
        (4u16, 0u8),
        (3u16, 0u8),
        (4u16, 0u8),
    ];
    static STATE_ITEMS_31: &[(u16, u8)] = &[(3u16, 2u8), (3u16, 2u8)];
    static STATE_ITEMS_32: &[(u16, u8)] = &[(11u16, 4u8), (11u16, 4u8)];
    static STATE_ITEMS_33: &[(u16, u8)] = &[(1u16, 1u8), (1u16, 1u8)];
    static STATE_ITEMS_34: &[(u16, u8)] = &[(10u16, 3u8), (10u16, 3u8)];
    static STATE_ITEMS_35: &[(u16, u8)] = &[
        (9u16, 5u8),
        (3u16, 1u8),
        (11u16, 0u8),
        (3u16, 1u8),
        (11u16, 0u8),
    ];
    static STATE_ITEMS_36: &[(u16, u8)] = &[(17u16, 1u8), (17u16, 1u8)];
    static STATE_ITEMS_37: &[(u16, u8)] = &[(9u16, 6u8)];
    static STATE_ITEMS_38: &[(u16, u8)] = &[
        (16u16, 2u8),
        (16u16, 2u8),
        (14u16, 0u8),
        (15u16, 0u8),
        (14u16, 0u8),
        (15u16, 0u8),
        (17u16, 0u8),
        (17u16, 0u8),
    ];
    static STATE_ITEMS_39: &[(u16, u8)] = &[(16u16, 3u8), (16u16, 3u8)];
    static STATE_ITEMS_40: &[(u16, u8)] = &[(14u16, 1u8), (14u16, 1u8)];
    static STATE_ITEMS_41: &[(u16, u8)] = &[(17u16, 2u8), (17u16, 2u8)];
    static STATE_ITEMS_42: &[(u16, u8)] = &[(5u16, 3u8), (5u16, 3u8)];
    static STATE_ITEMS_43: &[(u16, u8)] = &[(6u16, 1u8), (6u16, 1u8)];
    static STATE_ITEMS_44: &[(u16, u8)] = &[(9u16, 8u8), (5u16, 1u8), (5u16, 1u8)];
    static STATE_ITEMS_45: &[(u16, u8)] = &[
        (25u16, 4u8),
        (25u16, 4u8),
        (25u16, 4u8),
        (25u16, 4u8),
    ];
    static STATE_ITEMS_46: &[(u16, u8)] = &[(20u16, 1u8), (20u16, 1u8)];
    static STATE_ITEMS_47: &[(u16, u8)] = &[(23u16, 2u8), (23u16, 2u8)];
    static STATE_ITEMS_48: &[(u16, u8)] = &[(24u16, 2u8), (24u16, 2u8)];
    static STATE_ITEMS_49: &[(u16, u8)] = &[(18u16, 3u8), (18u16, 3u8)];
    static STATE_ITEMS_50: &[(u16, u8)] = &[(19u16, 1u8), (19u16, 1u8)];
    static STATE_ITEMS_51: &[(u16, u8)] = &[
        (20u16, 3u8),
        (20u16, 3u8),
        (18u16, 1u8),
        (18u16, 1u8),
    ];
    static STATE_ITEMS_52: &[(u16, u8)] = &[(9u16, 2u8)];
    static STATE_ITEMS_53: &[(u16, u8)] = &[(11u16, 3u8), (11u16, 3u8)];
    static STATE_ITEMS_54: &[(u16, u8)] = &[(10u16, 2u8), (10u16, 2u8)];
    static STATE_ITEMS_55: &[(u16, u8)] = &[
        (25u16, 2u8),
        (25u16, 2u8),
        (25u16, 2u8),
        (25u16, 2u8),
    ];
    pub static STATE_ITEMS: &[&[(u16, u8)]] = &[
        STATE_ITEMS_0,
        STATE_ITEMS_1,
        STATE_ITEMS_2,
        STATE_ITEMS_3,
        STATE_ITEMS_4,
        STATE_ITEMS_5,
        STATE_ITEMS_6,
        STATE_ITEMS_7,
        STATE_ITEMS_8,
        STATE_ITEMS_9,
        STATE_ITEMS_10,
        STATE_ITEMS_11,
        STATE_ITEMS_12,
        STATE_ITEMS_13,
        STATE_ITEMS_14,
        STATE_ITEMS_15,
        STATE_ITEMS_16,
        STATE_ITEMS_17,
        STATE_ITEMS_18,
        STATE_ITEMS_19,
        STATE_ITEMS_20,
        STATE_ITEMS_21,
        STATE_ITEMS_22,
        STATE_ITEMS_23,
        STATE_ITEMS_24,
        STATE_ITEMS_25,
        STATE_ITEMS_26,
        STATE_ITEMS_27,
        STATE_ITEMS_28,
        STATE_ITEMS_29,
        STATE_ITEMS_30,
        STATE_ITEMS_31,
        STATE_ITEMS_32,
        STATE_ITEMS_33,
        STATE_ITEMS_34,
        STATE_ITEMS_35,
        STATE_ITEMS_36,
        STATE_ITEMS_37,
        STATE_ITEMS_38,
        STATE_ITEMS_39,
        STATE_ITEMS_40,
        STATE_ITEMS_41,
        STATE_ITEMS_42,
        STATE_ITEMS_43,
        STATE_ITEMS_44,
        STATE_ITEMS_45,
        STATE_ITEMS_46,
        STATE_ITEMS_47,
        STATE_ITEMS_48,
        STATE_ITEMS_49,
        STATE_ITEMS_50,
        STATE_ITEMS_51,
        STATE_ITEMS_52,
        STATE_ITEMS_53,
        STATE_ITEMS_54,
        STATE_ITEMS_55,
    ];
    static RULE_RHS_0: &[u32] = &[23u32];
    static RULE_RHS_1: &[u32] = &[24u32];
    static RULE_RHS_2: &[u32] = &[];
    static RULE_RHS_3: &[u32] = &[33u32, 25u32];
    static RULE_RHS_4: &[u32] = &[];
    static RULE_RHS_5: &[u32] = &[34u32, 14u32, 26u32];
    static RULE_RHS_6: &[u32] = &[26u32];
    static RULE_RHS_7: &[u32] = &[35u32, 28u32];
    static RULE_RHS_8: &[u32] = &[28u32];
    static RULE_RHS_9: &[u32] = &[
        3u32, 1u32, 17u32, 32u32, 33u32, 4u32, 9u32, 34u32, 10u32, 35u32,
    ];
    static RULE_RHS_10: &[u32] = &[7u32, 1u32, 17u32];
    static RULE_RHS_11: &[u32] = &[6u32, 2u32, 1u32, 17u32];
    static RULE_RHS_12: &[u32] = &[5u32];
    static RULE_RHS_13: &[u32] = &[];
    static RULE_RHS_14: &[u32] = &[27u32];
    static RULE_RHS_15: &[u32] = &[];
    static RULE_RHS_16: &[u32] = &[36u32, 1u32, 37u32];
    static RULE_RHS_17: &[u32] = &[13u32, 8u32];
    static RULE_RHS_18: &[u32] = &[38u32, 16u32, 29u32];
    static RULE_RHS_19: &[u32] = &[29u32];
    static RULE_RHS_20: &[u32] = &[1u32, 15u32, 38u32, 17u32];
    static RULE_RHS_21: &[u32] = &[39u32, 31u32];
    static RULE_RHS_22: &[u32] = &[31u32];
    static RULE_RHS_23: &[u32] = &[39u32, 30u32];
    static RULE_RHS_24: &[u32] = &[18u32, 1u32];
    static RULE_RHS_25: &[u32] = &[11u32, 1u32, 22u32, 1u32, 12u32];
    static RULE_RHS_26: &[u32] = &[1u32, 19u32];
    static RULE_RHS_27: &[u32] = &[1u32, 20u32];
    static RULE_RHS_28: &[u32] = &[1u32, 21u32];
    static RULE_RHS_29: &[u32] = &[1u32];
    static RULE_RHS_30: &[u32] = &[8u32];
    pub static RULE_RHS: &[&[u32]] = &[
        RULE_RHS_0,
        RULE_RHS_1,
        RULE_RHS_2,
        RULE_RHS_3,
        RULE_RHS_4,
        RULE_RHS_5,
        RULE_RHS_6,
        RULE_RHS_7,
        RULE_RHS_8,
        RULE_RHS_9,
        RULE_RHS_10,
        RULE_RHS_11,
        RULE_RHS_12,
        RULE_RHS_13,
        RULE_RHS_14,
        RULE_RHS_15,
        RULE_RHS_16,
        RULE_RHS_17,
        RULE_RHS_18,
        RULE_RHS_19,
        RULE_RHS_20,
        RULE_RHS_21,
        RULE_RHS_22,
        RULE_RHS_23,
        RULE_RHS_24,
        RULE_RHS_25,
        RULE_RHS_26,
        RULE_RHS_27,
        RULE_RHS_28,
        RULE_RHS_29,
        RULE_RHS_30,
    ];
    pub fn symbol_id(name: &str) -> gazelle::SymbolId {
        match name {
            "IDENT" => gazelle::SymbolId(1u32),
            "NUM" => gazelle::SymbolId(2u32),
            "KW_START" => gazelle::SymbolId(3u32),
            "KW_TERMINALS" => gazelle::SymbolId(4u32),
            "KW_PREC" => gazelle::SymbolId(5u32),
            "KW_EXPECT" => gazelle::SymbolId(6u32),
            "KW_MODE" => gazelle::SymbolId(7u32),
            "UNDERSCORE" => gazelle::SymbolId(8u32),
            "LBRACE" => gazelle::SymbolId(9u32),
            "RBRACE" => gazelle::SymbolId(10u32),
            "LPAREN" => gazelle::SymbolId(11u32),
            "RPAREN" => gazelle::SymbolId(12u32),
            "COLON" => gazelle::SymbolId(13u32),
            "COMMA" => gazelle::SymbolId(14u32),
            "EQ" => gazelle::SymbolId(15u32),
            "PIPE" => gazelle::SymbolId(16u32),
            "SEMI" => gazelle::SymbolId(17u32),
            "FAT_ARROW" => gazelle::SymbolId(18u32),
            "QUESTION" => gazelle::SymbolId(19u32),
            "STAR" => gazelle::SymbolId(20u32),
            "PLUS" => gazelle::SymbolId(21u32),
            "PERCENT" => gazelle::SymbolId(22u32),
            "grammar_def" => gazelle::SymbolId(23u32),
            "mode_decl" => gazelle::SymbolId(24u32),
            "expect_decl" => gazelle::SymbolId(25u32),
            "terminal_item" => gazelle::SymbolId(26u32),
            "type_annot" => gazelle::SymbolId(27u32),
            "rule" => gazelle::SymbolId(28u32),
            "alt" => gazelle::SymbolId(29u32),
            "variant" => gazelle::SymbolId(30u32),
            "term" => gazelle::SymbolId(31u32),
            "__mode_decl_opt" => gazelle::SymbolId(32u32),
            "__expect_decl_star" => gazelle::SymbolId(33u32),
            "__terminal_item_sep_comma" => gazelle::SymbolId(34u32),
            "__rule_plus" => gazelle::SymbolId(35u32),
            "__kw_prec_opt" => gazelle::SymbolId(36u32),
            "__type_annot_opt" => gazelle::SymbolId(37u32),
            "__alt_sep_pipe" => gazelle::SymbolId(38u32),
            "__term_plus" => gazelle::SymbolId(39u32),
            "__start" => gazelle::SymbolId(40u32),
            _ => panic!("unknown symbol: {}", name),
        }
    }
    pub static TABLE: gazelle::ParseTable<'static> = gazelle::ParseTable::new(
        ACTION_DATA,
        ACTION_BASE,
        ACTION_CHECK,
        GOTO_DATA,
        GOTO_BASE,
        GOTO_CHECK,
        RULES,
        NUM_TERMINALS,
    );
    pub static ERROR_INFO: gazelle::ErrorInfo<'static> = gazelle::ErrorInfo {
        symbol_names: SYMBOL_NAMES,
        state_items: STATE_ITEMS,
        rule_rhs: RULE_RHS,
        state_symbols: STATE_SYMBOL,
    };
}
/// Terminal symbols for the parser.
pub enum Terminal<A: Types> {
    Ident(A::Ident),
    Num(A::Num),
    KwStart,
    KwTerminals,
    KwPrec,
    KwExpect,
    KwMode,
    Underscore,
    Lbrace,
    Rbrace,
    Lparen,
    Rparen,
    Colon,
    Comma,
    Eq,
    Pipe,
    Semi,
    FatArrow,
    Question,
    Star,
    Plus,
    Percent,
    #[doc(hidden)]
    __Phantom(std::marker::PhantomData<A>),
}
impl<A: Types> Terminal<A> {
    /// Get the symbol ID for this terminal.
    pub fn symbol_id(&self) -> gazelle::SymbolId {
        match self {
            Self::Ident(_) => gazelle::SymbolId(1u32),
            Self::Num(_) => gazelle::SymbolId(2u32),
            Self::KwStart => gazelle::SymbolId(3u32),
            Self::KwTerminals => gazelle::SymbolId(4u32),
            Self::KwPrec => gazelle::SymbolId(5u32),
            Self::KwExpect => gazelle::SymbolId(6u32),
            Self::KwMode => gazelle::SymbolId(7u32),
            Self::Underscore => gazelle::SymbolId(8u32),
            Self::Lbrace => gazelle::SymbolId(9u32),
            Self::Rbrace => gazelle::SymbolId(10u32),
            Self::Lparen => gazelle::SymbolId(11u32),
            Self::Rparen => gazelle::SymbolId(12u32),
            Self::Colon => gazelle::SymbolId(13u32),
            Self::Comma => gazelle::SymbolId(14u32),
            Self::Eq => gazelle::SymbolId(15u32),
            Self::Pipe => gazelle::SymbolId(16u32),
            Self::Semi => gazelle::SymbolId(17u32),
            Self::FatArrow => gazelle::SymbolId(18u32),
            Self::Question => gazelle::SymbolId(19u32),
            Self::Star => gazelle::SymbolId(20u32),
            Self::Plus => gazelle::SymbolId(21u32),
            Self::Percent => gazelle::SymbolId(22u32),
            Self::__Phantom(_) => unreachable!(),
        }
    }
    /// Convert to a gazelle Token for parsing.
    pub fn to_token(
        &self,
        symbol_ids: &impl Fn(&str) -> gazelle::SymbolId,
    ) -> gazelle::Token {
        match self {
            Self::Ident(_) => gazelle::Token::new(symbol_ids("IDENT")),
            Self::Num(_) => gazelle::Token::new(symbol_ids("NUM")),
            Self::KwStart => gazelle::Token::new(symbol_ids("KW_START")),
            Self::KwTerminals => gazelle::Token::new(symbol_ids("KW_TERMINALS")),
            Self::KwPrec => gazelle::Token::new(symbol_ids("KW_PREC")),
            Self::KwExpect => gazelle::Token::new(symbol_ids("KW_EXPECT")),
            Self::KwMode => gazelle::Token::new(symbol_ids("KW_MODE")),
            Self::Underscore => gazelle::Token::new(symbol_ids("UNDERSCORE")),
            Self::Lbrace => gazelle::Token::new(symbol_ids("LBRACE")),
            Self::Rbrace => gazelle::Token::new(symbol_ids("RBRACE")),
            Self::Lparen => gazelle::Token::new(symbol_ids("LPAREN")),
            Self::Rparen => gazelle::Token::new(symbol_ids("RPAREN")),
            Self::Colon => gazelle::Token::new(symbol_ids("COLON")),
            Self::Comma => gazelle::Token::new(symbol_ids("COMMA")),
            Self::Eq => gazelle::Token::new(symbol_ids("EQ")),
            Self::Pipe => gazelle::Token::new(symbol_ids("PIPE")),
            Self::Semi => gazelle::Token::new(symbol_ids("SEMI")),
            Self::FatArrow => gazelle::Token::new(symbol_ids("FAT_ARROW")),
            Self::Question => gazelle::Token::new(symbol_ids("QUESTION")),
            Self::Star => gazelle::Token::new(symbol_ids("STAR")),
            Self::Plus => gazelle::Token::new(symbol_ids("PLUS")),
            Self::Percent => gazelle::Token::new(symbol_ids("PERCENT")),
            Self::__Phantom(_) => unreachable!(),
        }
    }
    /// Get precedence for runtime precedence comparison.
    pub fn precedence(&self) -> Option<gazelle::Precedence> {
        match self {
            Self::Ident(_) => None,
            Self::Num(_) => None,
            Self::KwStart => None,
            Self::KwTerminals => None,
            Self::KwPrec => None,
            Self::KwExpect => None,
            Self::KwMode => None,
            Self::Underscore => None,
            Self::Lbrace => None,
            Self::Rbrace => None,
            Self::Lparen => None,
            Self::Rparen => None,
            Self::Colon => None,
            Self::Comma => None,
            Self::Eq => None,
            Self::Pipe => None,
            Self::Semi => None,
            Self::FatArrow => None,
            Self::Question => None,
            Self::Star => None,
            Self::Plus => None,
            Self::Percent => None,
            Self::__Phantom(_) => unreachable!(),
        }
    }
}
pub enum Alt<A: Types> {
    Alt(Vec<A::Term>, A::Variant),
}
impl<A: Types> std::fmt::Debug for Alt<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Alt(f0, f1) => f.debug_tuple("Alt").field(f0).field(f1).finish(),
        }
    }
}
pub enum ExpectDecl<A: Types> {
    ExpectDecl(A::Num, A::Ident),
}
impl<A: Types> std::fmt::Debug for ExpectDecl<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExpectDecl(f0, f1) => {
                f.debug_tuple("ExpectDecl").field(f0).field(f1).finish()
            }
        }
    }
}
pub enum GrammarDef<A: Types> {
    GrammarDef(
        A::Ident,
        Option<A::ModeDecl>,
        Vec<A::ExpectDecl>,
        Vec<A::TerminalItem>,
        Vec<A::Rule>,
    ),
}
impl<A: Types> std::fmt::Debug for GrammarDef<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GrammarDef(f0, f1, f2, f3, f4) => {
                f.debug_tuple("GrammarDef")
                    .field(f0)
                    .field(f1)
                    .field(f2)
                    .field(f3)
                    .field(f4)
                    .finish()
            }
        }
    }
}
pub enum ModeDecl<A: Types> {
    ModeDecl(A::Ident),
}
impl<A: Types> std::fmt::Debug for ModeDecl<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ModeDecl(f0) => f.debug_tuple("ModeDecl").field(f0).finish(),
        }
    }
}
pub enum Rule<A: Types> {
    Rule(A::Ident, Vec<A::Alt>),
}
impl<A: Types> std::fmt::Debug for Rule<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rule(f0, f1) => f.debug_tuple("Rule").field(f0).field(f1).finish(),
        }
    }
}
pub enum Term<A: Types> {
    SymSep(A::Ident, A::Ident),
    SymOpt(A::Ident),
    SymStar(A::Ident),
    SymPlus(A::Ident),
    SymPlain(A::Ident),
    SymEmpty,
}
impl<A: Types> std::fmt::Debug for Term<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SymSep(f0, f1) => f.debug_tuple("SymSep").field(f0).field(f1).finish(),
            Self::SymOpt(f0) => f.debug_tuple("SymOpt").field(f0).finish(),
            Self::SymStar(f0) => f.debug_tuple("SymStar").field(f0).finish(),
            Self::SymPlus(f0) => f.debug_tuple("SymPlus").field(f0).finish(),
            Self::SymPlain(f0) => f.debug_tuple("SymPlain").field(f0).finish(),
            Self::SymEmpty => f.write_str("SymEmpty"),
        }
    }
}
pub enum TerminalItem<A: Types> {
    TerminalItem(Option<()>, A::Ident, Option<A::TypeAnnot>),
}
impl<A: Types> std::fmt::Debug for TerminalItem<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TerminalItem(f0, f1, f2) => {
                f.debug_tuple("TerminalItem").field(f0).field(f1).field(f2).finish()
            }
        }
    }
}
pub enum TypeAnnot<A: Types> {
    TypeAnnot,
    #[doc(hidden)]
    _Phantom(std::convert::Infallible, std::marker::PhantomData<A>),
}
impl<A: Types> std::fmt::Debug for TypeAnnot<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TypeAnnot => f.write_str("TypeAnnot"),
            _ => unreachable!(),
        }
    }
}
pub enum Variant<A: Types> {
    Variant(A::Ident),
}
impl<A: Types> std::fmt::Debug for Variant<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Variant(f0) => f.debug_tuple("Variant").field(f0).finish(),
        }
    }
}
/// Associated types for parser symbols.
pub trait Types: Sized {
    type Error: From<gazelle::ParseError>;
    type Ident: std::fmt::Debug;
    type Num: std::fmt::Debug;
    type GrammarDef: std::fmt::Debug;
    type ModeDecl: std::fmt::Debug;
    type ExpectDecl: std::fmt::Debug;
    type TerminalItem: std::fmt::Debug;
    type TypeAnnot: std::fmt::Debug;
    type Rule: std::fmt::Debug;
    type Alt: std::fmt::Debug;
    type Variant: std::fmt::Debug;
    type Term: std::fmt::Debug;
    /// Called before each reduction with the token range `[start..end)`.
    /// Override to track source spans. Default is no-op.
    #[allow(unused_variables)]
    fn set_token_range(&mut self, start: usize, end: usize) {}
}
impl<A: Types> gazelle::AstNode for GrammarDef<A> {
    type Output = A::GrammarDef;
    type Error = A::Error;
}
impl<A: Types> gazelle::AstNode for ModeDecl<A> {
    type Output = A::ModeDecl;
    type Error = A::Error;
}
impl<A: Types> gazelle::AstNode for ExpectDecl<A> {
    type Output = A::ExpectDecl;
    type Error = A::Error;
}
impl<A: Types> gazelle::AstNode for TerminalItem<A> {
    type Output = A::TerminalItem;
    type Error = A::Error;
}
impl<A: Types> gazelle::AstNode for TypeAnnot<A> {
    type Output = A::TypeAnnot;
    type Error = A::Error;
}
impl<A: Types> gazelle::AstNode for Rule<A> {
    type Output = A::Rule;
    type Error = A::Error;
}
impl<A: Types> gazelle::AstNode for Alt<A> {
    type Output = A::Alt;
    type Error = A::Error;
}
impl<A: Types> gazelle::AstNode for Variant<A> {
    type Output = A::Variant;
    type Error = A::Error;
}
impl<A: Types> gazelle::AstNode for Term<A> {
    type Output = A::Term;
    type Error = A::Error;
}
#[doc(hidden)]
union __Value<A: Types> {
    __ident: std::mem::ManuallyDrop<A::Ident>,
    __num: std::mem::ManuallyDrop<A::Num>,
    __grammar_def: std::mem::ManuallyDrop<A::GrammarDef>,
    __mode_decl: std::mem::ManuallyDrop<A::ModeDecl>,
    __expect_decl: std::mem::ManuallyDrop<A::ExpectDecl>,
    __terminal_item: std::mem::ManuallyDrop<A::TerminalItem>,
    __type_annot: std::mem::ManuallyDrop<A::TypeAnnot>,
    __rule: std::mem::ManuallyDrop<A::Rule>,
    __alt: std::mem::ManuallyDrop<A::Alt>,
    __variant: std::mem::ManuallyDrop<A::Variant>,
    __term: std::mem::ManuallyDrop<A::Term>,
    ____mode_decl_opt: std::mem::ManuallyDrop<Option<A::ModeDecl>>,
    ____expect_decl_star: std::mem::ManuallyDrop<Vec<A::ExpectDecl>>,
    ____terminal_item_sep_comma: std::mem::ManuallyDrop<Vec<A::TerminalItem>>,
    ____rule_plus: std::mem::ManuallyDrop<Vec<A::Rule>>,
    ____kw_prec_opt: std::mem::ManuallyDrop<Option<()>>,
    ____type_annot_opt: std::mem::ManuallyDrop<Option<A::TypeAnnot>>,
    ____alt_sep_pipe: std::mem::ManuallyDrop<Vec<A::Alt>>,
    ____term_plus: std::mem::ManuallyDrop<Vec<A::Term>>,
    __unit: (),
    __phantom: std::mem::ManuallyDrop<std::marker::PhantomData<A>>,
}
/// Type-safe LR parser.
pub struct Parser<A: Types> {
    parser: gazelle::Parser<'static>,
    value_stack: Vec<__Value<A>>,
}
impl<A: Types> Parser<A> {
    /// Create a new parser instance.
    pub fn new() -> Self {
        Self {
            parser: gazelle::Parser::new(__table::TABLE),
            value_stack: Vec::new(),
        }
    }
    /// Get the current parser state.
    pub fn state(&self) -> usize {
        self.parser.state()
    }
    /// Format a parse error message.
    pub fn format_error(&self, err: &gazelle::ParseError) -> String {
        self.parser.format_error(err, &__table::ERROR_INFO)
    }
    /// Format a parse error with display names and token texts.
    pub fn format_error_with(
        &self,
        err: &gazelle::ParseError,
        display_names: &std::collections::HashMap<&str, &str>,
        tokens: &[&str],
    ) -> String {
        self.parser.format_error_with(err, &__table::ERROR_INFO, display_names, tokens)
    }
    /// Get the error info for custom error formatting.
    pub fn error_info() -> &'static gazelle::ErrorInfo<'static> {
        &__table::ERROR_INFO
    }
    /// Recover from a parse error by searching for minimum-cost repairs.
    ///
    /// Drops the value stack before running recovery on the state
    /// machine. The parser should be discarded afterwards.
    pub fn recover(&mut self, buffer: &[gazelle::Token]) -> Vec<gazelle::RecoveryInfo> {
        self.drain_values();
        self.parser.recover(buffer)
    }
    fn drain_values(&mut self) {
        for i in (0..self.value_stack.len()).rev() {
            let union_val = self.value_stack.pop().unwrap();
            let sym_id = __table::STATE_SYMBOL[self.parser.state_at(i)];
            unsafe {
                match sym_id {
                    1u32 => {
                        std::mem::ManuallyDrop::into_inner(union_val.__ident);
                    }
                    2u32 => {
                        std::mem::ManuallyDrop::into_inner(union_val.__num);
                    }
                    23u32 => {
                        std::mem::ManuallyDrop::into_inner(union_val.__grammar_def);
                    }
                    24u32 => {
                        std::mem::ManuallyDrop::into_inner(union_val.__mode_decl);
                    }
                    25u32 => {
                        std::mem::ManuallyDrop::into_inner(union_val.__expect_decl);
                    }
                    26u32 => {
                        std::mem::ManuallyDrop::into_inner(union_val.__terminal_item);
                    }
                    27u32 => {
                        std::mem::ManuallyDrop::into_inner(union_val.__type_annot);
                    }
                    28u32 => {
                        std::mem::ManuallyDrop::into_inner(union_val.__rule);
                    }
                    29u32 => {
                        std::mem::ManuallyDrop::into_inner(union_val.__alt);
                    }
                    30u32 => {
                        std::mem::ManuallyDrop::into_inner(union_val.__variant);
                    }
                    31u32 => {
                        std::mem::ManuallyDrop::into_inner(union_val.__term);
                    }
                    32u32 => {
                        std::mem::ManuallyDrop::into_inner(union_val.____mode_decl_opt);
                    }
                    33u32 => {
                        std::mem::ManuallyDrop::into_inner(
                            union_val.____expect_decl_star,
                        );
                    }
                    34u32 => {
                        std::mem::ManuallyDrop::into_inner(
                            union_val.____terminal_item_sep_comma,
                        );
                    }
                    35u32 => {
                        std::mem::ManuallyDrop::into_inner(union_val.____rule_plus);
                    }
                    36u32 => {
                        std::mem::ManuallyDrop::into_inner(union_val.____kw_prec_opt);
                    }
                    37u32 => {
                        std::mem::ManuallyDrop::into_inner(union_val.____type_annot_opt);
                    }
                    38u32 => {
                        std::mem::ManuallyDrop::into_inner(union_val.____alt_sep_pipe);
                    }
                    39u32 => {
                        std::mem::ManuallyDrop::into_inner(union_val.____term_plus);
                    }
                    _ => {}
                }
            }
        }
    }
}
#[allow(clippy::result_large_err)]
impl<
    A: Types + gazelle::Action<GrammarDef<A>> + gazelle::Action<ModeDecl<A>>
        + gazelle::Action<ExpectDecl<A>> + gazelle::Action<TerminalItem<A>>
        + gazelle::Action<TypeAnnot<A>> + gazelle::Action<Rule<A>>
        + gazelle::Action<Alt<A>> + gazelle::Action<Variant<A>>
        + gazelle::Action<Term<A>>,
> Parser<A> {
    /// Push a terminal, performing any reductions.
    pub fn push(
        &mut self,
        terminal: Terminal<A>,
        actions: &mut A,
    ) -> Result<(), A::Error> {
        let token = gazelle::Token {
            terminal: terminal.symbol_id(),
            prec: terminal.precedence(),
        };
        while let Some((rule, _, start_idx)) = self.parser.maybe_reduce(Some(token))? {
            self.do_reduce(rule, start_idx, actions)?;
        }
        self.parser.shift(token);
        match terminal {
            Terminal::Ident(v) => {
                self.value_stack
                    .push(__Value {
                        __ident: std::mem::ManuallyDrop::new(v),
                    });
            }
            Terminal::Num(v) => {
                self.value_stack
                    .push(__Value {
                        __num: std::mem::ManuallyDrop::new(v),
                    });
            }
            Terminal::KwStart => {
                self.value_stack.push(__Value { __unit: () });
            }
            Terminal::KwTerminals => {
                self.value_stack.push(__Value { __unit: () });
            }
            Terminal::KwPrec => {
                self.value_stack.push(__Value { __unit: () });
            }
            Terminal::KwExpect => {
                self.value_stack.push(__Value { __unit: () });
            }
            Terminal::KwMode => {
                self.value_stack.push(__Value { __unit: () });
            }
            Terminal::Underscore => {
                self.value_stack.push(__Value { __unit: () });
            }
            Terminal::Lbrace => {
                self.value_stack.push(__Value { __unit: () });
            }
            Terminal::Rbrace => {
                self.value_stack.push(__Value { __unit: () });
            }
            Terminal::Lparen => {
                self.value_stack.push(__Value { __unit: () });
            }
            Terminal::Rparen => {
                self.value_stack.push(__Value { __unit: () });
            }
            Terminal::Colon => {
                self.value_stack.push(__Value { __unit: () });
            }
            Terminal::Comma => {
                self.value_stack.push(__Value { __unit: () });
            }
            Terminal::Eq => {
                self.value_stack.push(__Value { __unit: () });
            }
            Terminal::Pipe => {
                self.value_stack.push(__Value { __unit: () });
            }
            Terminal::Semi => {
                self.value_stack.push(__Value { __unit: () });
            }
            Terminal::FatArrow => {
                self.value_stack.push(__Value { __unit: () });
            }
            Terminal::Question => {
                self.value_stack.push(__Value { __unit: () });
            }
            Terminal::Star => {
                self.value_stack.push(__Value { __unit: () });
            }
            Terminal::Plus => {
                self.value_stack.push(__Value { __unit: () });
            }
            Terminal::Percent => {
                self.value_stack.push(__Value { __unit: () });
            }
            Terminal::__Phantom(_) => unreachable!(),
        }
        Ok(())
    }
    /// Finish parsing and return the result.
    pub fn finish(mut self, actions: &mut A) -> Result<A::GrammarDef, (Self, A::Error)> {
        loop {
            match self.parser.maybe_reduce(None) {
                Ok(Some((0, _, _))) => {
                    let union_val = self.value_stack.pop().unwrap();
                    return Ok(unsafe {
                        std::mem::ManuallyDrop::into_inner(union_val.__grammar_def)
                    });
                }
                Ok(Some((rule, _, start_idx))) => {
                    if let Err(e) = self.do_reduce(rule, start_idx, actions) {
                        return Err((self, e));
                    }
                }
                Ok(None) => unreachable!(),
                Err(e) => return Err((self, e.into())),
            }
        }
    }
    fn do_reduce(
        &mut self,
        rule: usize,
        start_idx: usize,
        actions: &mut A,
    ) -> Result<(), A::Error> {
        if rule == 0 {
            return Ok(());
        }
        actions.set_token_range(start_idx, self.parser.token_count());
        let original_rule_idx = rule - 1;
        let value = match original_rule_idx {
            0usize => {
                let v0 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().__mode_decl,
                    )
                };
                __Value {
                    ____mode_decl_opt: std::mem::ManuallyDrop::new(Some(v0)),
                }
            }
            1usize => {
                __Value {
                    ____mode_decl_opt: std::mem::ManuallyDrop::new(None),
                }
            }
            2usize => {
                let v1 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().__expect_decl,
                    )
                };
                let v0 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().____expect_decl_star,
                    )
                };
                {
                    let mut v0 = v0;
                    v0.push(v1);
                    __Value {
                        ____expect_decl_star: std::mem::ManuallyDrop::new(v0),
                    }
                }
            }
            3usize => {
                __Value {
                    ____expect_decl_star: std::mem::ManuallyDrop::new(Vec::new()),
                }
            }
            4usize => {
                let v2 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().__terminal_item,
                    )
                };
                let _ = self.value_stack.pop().unwrap();
                let v0 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().____terminal_item_sep_comma,
                    )
                };
                {
                    let mut v0 = v0;
                    v0.push(v2);
                    __Value {
                        ____terminal_item_sep_comma: std::mem::ManuallyDrop::new(v0),
                    }
                }
            }
            5usize => {
                let v0 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().__terminal_item,
                    )
                };
                __Value {
                    ____terminal_item_sep_comma: std::mem::ManuallyDrop::new(vec![v0]),
                }
            }
            6usize => {
                let v1 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().__rule,
                    )
                };
                let v0 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().____rule_plus,
                    )
                };
                {
                    let mut v0 = v0;
                    v0.push(v1);
                    __Value {
                        ____rule_plus: std::mem::ManuallyDrop::new(v0),
                    }
                }
            }
            7usize => {
                let v0 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().__rule,
                    )
                };
                __Value {
                    ____rule_plus: std::mem::ManuallyDrop::new(vec![v0]),
                }
            }
            8usize => {
                let v9 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().____rule_plus,
                    )
                };
                let _ = self.value_stack.pop().unwrap();
                let v7 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().____terminal_item_sep_comma,
                    )
                };
                let _ = self.value_stack.pop().unwrap();
                let _ = self.value_stack.pop().unwrap();
                let v4 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().____expect_decl_star,
                    )
                };
                let v3 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().____mode_decl_opt,
                    )
                };
                let _ = self.value_stack.pop().unwrap();
                let v1 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().__ident,
                    )
                };
                let _ = self.value_stack.pop().unwrap();
                __Value {
                    __grammar_def: std::mem::ManuallyDrop::new(
                        gazelle::Action::build(
                            actions,
                            GrammarDef::GrammarDef(v1, v3, v4, v7, v9),
                        )?,
                    ),
                }
            }
            9usize => {
                let _ = self.value_stack.pop().unwrap();
                let v1 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().__ident,
                    )
                };
                let _ = self.value_stack.pop().unwrap();
                __Value {
                    __mode_decl: std::mem::ManuallyDrop::new(
                        gazelle::Action::build(actions, ModeDecl::ModeDecl(v1))?,
                    ),
                }
            }
            10usize => {
                let _ = self.value_stack.pop().unwrap();
                let v2 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().__ident,
                    )
                };
                let v1 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().__num,
                    )
                };
                let _ = self.value_stack.pop().unwrap();
                __Value {
                    __expect_decl: std::mem::ManuallyDrop::new(
                        gazelle::Action::build(actions, ExpectDecl::ExpectDecl(v1, v2))?,
                    ),
                }
            }
            11usize => {
                let _ = self.value_stack.pop().unwrap();
                __Value {
                    ____kw_prec_opt: std::mem::ManuallyDrop::new(Some(())),
                }
            }
            12usize => {
                __Value {
                    ____kw_prec_opt: std::mem::ManuallyDrop::new(None),
                }
            }
            13usize => {
                let v0 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().__type_annot,
                    )
                };
                __Value {
                    ____type_annot_opt: std::mem::ManuallyDrop::new(Some(v0)),
                }
            }
            14usize => {
                __Value {
                    ____type_annot_opt: std::mem::ManuallyDrop::new(None),
                }
            }
            15usize => {
                let v2 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().____type_annot_opt,
                    )
                };
                let v1 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().__ident,
                    )
                };
                let v0 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().____kw_prec_opt,
                    )
                };
                __Value {
                    __terminal_item: std::mem::ManuallyDrop::new(
                        gazelle::Action::build(
                            actions,
                            TerminalItem::TerminalItem(v0, v1, v2),
                        )?,
                    ),
                }
            }
            16usize => {
                let _ = self.value_stack.pop().unwrap();
                let _ = self.value_stack.pop().unwrap();
                __Value {
                    __type_annot: std::mem::ManuallyDrop::new(
                        gazelle::Action::build(actions, TypeAnnot::TypeAnnot)?,
                    ),
                }
            }
            17usize => {
                let v2 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().__alt,
                    )
                };
                let _ = self.value_stack.pop().unwrap();
                let v0 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().____alt_sep_pipe,
                    )
                };
                {
                    let mut v0 = v0;
                    v0.push(v2);
                    __Value {
                        ____alt_sep_pipe: std::mem::ManuallyDrop::new(v0),
                    }
                }
            }
            18usize => {
                let v0 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().__alt,
                    )
                };
                __Value {
                    ____alt_sep_pipe: std::mem::ManuallyDrop::new(vec![v0]),
                }
            }
            19usize => {
                let _ = self.value_stack.pop().unwrap();
                let v2 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().____alt_sep_pipe,
                    )
                };
                let _ = self.value_stack.pop().unwrap();
                let v0 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().__ident,
                    )
                };
                __Value {
                    __rule: std::mem::ManuallyDrop::new(
                        gazelle::Action::build(actions, Rule::Rule(v0, v2))?,
                    ),
                }
            }
            20usize => {
                let v1 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().__term,
                    )
                };
                let v0 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().____term_plus,
                    )
                };
                {
                    let mut v0 = v0;
                    v0.push(v1);
                    __Value {
                        ____term_plus: std::mem::ManuallyDrop::new(v0),
                    }
                }
            }
            21usize => {
                let v0 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().__term,
                    )
                };
                __Value {
                    ____term_plus: std::mem::ManuallyDrop::new(vec![v0]),
                }
            }
            22usize => {
                let v1 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().__variant,
                    )
                };
                let v0 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().____term_plus,
                    )
                };
                __Value {
                    __alt: std::mem::ManuallyDrop::new(
                        gazelle::Action::build(actions, Alt::Alt(v0, v1))?,
                    ),
                }
            }
            23usize => {
                let v1 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().__ident,
                    )
                };
                let _ = self.value_stack.pop().unwrap();
                __Value {
                    __variant: std::mem::ManuallyDrop::new(
                        gazelle::Action::build(actions, Variant::Variant(v1))?,
                    ),
                }
            }
            24usize => {
                let _ = self.value_stack.pop().unwrap();
                let v3 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().__ident,
                    )
                };
                let _ = self.value_stack.pop().unwrap();
                let v1 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().__ident,
                    )
                };
                let _ = self.value_stack.pop().unwrap();
                __Value {
                    __term: std::mem::ManuallyDrop::new(
                        gazelle::Action::build(actions, Term::SymSep(v1, v3))?,
                    ),
                }
            }
            25usize => {
                let _ = self.value_stack.pop().unwrap();
                let v0 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().__ident,
                    )
                };
                __Value {
                    __term: std::mem::ManuallyDrop::new(
                        gazelle::Action::build(actions, Term::SymOpt(v0))?,
                    ),
                }
            }
            26usize => {
                let _ = self.value_stack.pop().unwrap();
                let v0 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().__ident,
                    )
                };
                __Value {
                    __term: std::mem::ManuallyDrop::new(
                        gazelle::Action::build(actions, Term::SymStar(v0))?,
                    ),
                }
            }
            27usize => {
                let _ = self.value_stack.pop().unwrap();
                let v0 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().__ident,
                    )
                };
                __Value {
                    __term: std::mem::ManuallyDrop::new(
                        gazelle::Action::build(actions, Term::SymPlus(v0))?,
                    ),
                }
            }
            28usize => {
                let v0 = unsafe {
                    std::mem::ManuallyDrop::into_inner(
                        self.value_stack.pop().unwrap().__ident,
                    )
                };
                __Value {
                    __term: std::mem::ManuallyDrop::new(
                        gazelle::Action::build(actions, Term::SymPlain(v0))?,
                    ),
                }
            }
            29usize => {
                let _ = self.value_stack.pop().unwrap();
                __Value {
                    __term: std::mem::ManuallyDrop::new(
                        gazelle::Action::build(actions, Term::SymEmpty)?,
                    ),
                }
            }
            _ => return Ok(()),
        };
        self.value_stack.push(value);
        Ok(())
    }
}
impl<A: Types> Default for Parser<A> {
    fn default() -> Self {
        Self::new()
    }
}
impl<A: Types> Drop for Parser<A> {
    fn drop(&mut self) {
        self.drain_values();
    }
}

