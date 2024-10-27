pub struct Address<'a>{
    pub id:&'a str,
    pub address:u16,
    pub mask:u16,
    pub shift:u16
}
pub const HUD_ATT_SW: Address = Address{
    id: "HUD_ATT_SW",
    address: 29742,
    mask: 768,
    shift: 8
};
pub const HUD_SYM_BRT: Address = Address{
    id: "HUD_SYM_BRT",
    address: 29784,
    mask: 65535,
    shift: 0
};
pub const HUD_SYM_REJ_SW: Address = Address{
    id: "HUD_SYM_REJ_SW",
    address: 29740,
    mask: 1536,
    shift: 9
};
pub const HUD_AOA_INDEXER: Address = Address{
    id: "HUD_AOA_INDEXER",
    address: 29790,
    mask: 65535,
    shift: 0
};
pub const HUD_VIDEO_CONTROL_SW: Address = Address{
    id: "HUD_VIDEO_CONTROL_SW",
    address: 29740,
    mask: 12288,
    shift: 12
};
pub const HUD_BLACK_LVL: Address = Address{
    id: "HUD_BLACK_LVL",
    address: 29786,
    mask: 65535,
    shift: 0
};
pub const HUD_ALT_SW: Address = Address{
    id: "HUD_ALT_SW",
    address: 29740,
    mask: 16384,
    shift: 14
};
pub const HUD_BALANCE: Address = Address{
    id: "HUD_BALANCE",
    address: 29788,
    mask: 65535,
    shift: 0
};
pub const HUD_SYM_BRT_SELECT: Address = Address{
    id: "HUD_SYM_BRT_SELECT",
    address: 29740,
    mask: 2048,
    shift: 11
};
pub const AV_COOL_SW: Address = Address{
    id: "AV_COOL_SW",
    address: 29858,
    mask: 8192,
    shift: 13
};
pub const RADALT_ALT_PTR: Address = Address{
    id: "RADALT_ALT_PTR",
    address: 29962,
    mask: 65535,
    shift: 0
};
pub const LOW_ALT_WARN_LT: Address = Address{
    id: "LOW_ALT_WARN_LT",
    address: 29854,
    mask: 16384,
    shift: 14
};
pub const RADALT_OFF_FLAG: Address = Address{
    id: "RADALT_OFF_FLAG",
    address: 29964,
    mask: 65535,
    shift: 0
};
pub const RADALT_GREEN_LAMP: Address = Address{
    id: "RADALT_GREEN_LAMP",
    address: 29854,
    mask: 32768,
    shift: 15
};
pub const RADALT_HEIGHT: Address = Address{
    id: "RADALT_HEIGHT",
    address: 29958,
    mask: 65535,
    shift: 0
};
pub const RADALT_MIN_HEIGHT_PTR: Address = Address{
    id: "RADALT_MIN_HEIGHT_PTR",
    address: 29960,
    mask: 65535,
    shift: 0
};
pub const RADALT_TEST_SW: Address = Address{
    id: "RADALT_TEST_SW",
    address: 29854,
    mask: 8192,
    shift: 13
};
pub const CANOPY_SW: Address = Address{
    id: "CANOPY_SW",
    address: 29896,
    mask: 3072,
    shift: 10
};
pub const CANOPY_POS: Address = Address{
    id: "CANOPY_POS",
    address: 30018,
    mask: 65535,
    shift: 0
};
pub const RIGHT_FIRE_BTN_COVER: Address = Address{
    id: "RIGHT_FIRE_BTN_COVER",
    address: 29708,
    mask: 64,
    shift: 6
};
pub const RIGHT_FIRE_BTN: Address = Address{
    id: "RIGHT_FIRE_BTN",
    address: 29708,
    mask: 32,
    shift: 5
};
pub const FIRE_RIGHT_LT: Address = Address{
    id: "FIRE_RIGHT_LT",
    address: 29708,
    mask: 16,
    shift: 4
};
pub const PRESSURE_ALT: Address = Address{
    id: "PRESSURE_ALT",
    address: 29956,
    mask: 65535,
    shift: 0
};
pub const LEFT_LOUVER: Address = Address{
    id: "LEFT_LOUVER",
    address: 29938,
    mask: 65535,
    shift: 0
};
pub const RIGHT_LOUVER: Address = Address{
    id: "RIGHT_LOUVER",
    address: 29940,
    mask: 65535,
    shift: 0
};
pub const EXT_PWR_SW: Address = Address{
    id: "EXT_PWR_SW",
    address: 29872,
    mask: 768,
    shift: 8
};
pub const GND_PWR_3_SW: Address = Address{
    id: "GND_PWR_3_SW",
    address: 29872,
    mask: 49152,
    shift: 14
};
pub const GND_PWR_1_SW: Address = Address{
    id: "GND_PWR_1_SW",
    address: 29872,
    mask: 3072,
    shift: 10
};
pub const GND_PWR_4_SW: Address = Address{
    id: "GND_PWR_4_SW",
    address: 29874,
    mask: 768,
    shift: 8
};
pub const GND_PWR_2_SW: Address = Address{
    id: "GND_PWR_2_SW",
    address: 29872,
    mask: 12288,
    shift: 12
};
pub const LH_ADV_L_BAR_GREEN: Address = Address{
    id: "LH_ADV_L_BAR_GREEN",
    address: 29706,
    mask: 2,
    shift: 1
};
pub const LH_ADV_L_BLEED: Address = Address{
    id: "LH_ADV_L_BLEED",
    address: 29704,
    mask: 2048,
    shift: 11
};
pub const LH_ADV_NO_GO: Address = Address{
    id: "LH_ADV_NO_GO",
    address: 29706,
    mask: 32,
    shift: 5
};
pub const LH_ADV_SPD_BRK: Address = Address{
    id: "LH_ADV_SPD_BRK",
    address: 29704,
    mask: 8192,
    shift: 13
};
pub const LH_ADV_L_BAR_RED: Address = Address{
    id: "LH_ADV_L_BAR_RED",
    address: 29704,
    mask: 32768,
    shift: 15
};
pub const LH_ADV_ASPJ_OH: Address = Address{
    id: "LH_ADV_ASPJ_OH",
    address: 29706,
    mask: 8,
    shift: 3
};
pub const LH_ADV_R_BLEED: Address = Address{
    id: "LH_ADV_R_BLEED",
    address: 29704,
    mask: 4096,
    shift: 12
};
pub const LH_ADV_STBY: Address = Address{
    id: "LH_ADV_STBY",
    address: 29704,
    mask: 16384,
    shift: 14
};
pub const LH_ADV_GO: Address = Address{
    id: "LH_ADV_GO",
    address: 29706,
    mask: 16,
    shift: 4
};
pub const LH_ADV_XMIT: Address = Address{
    id: "LH_ADV_XMIT",
    address: 29706,
    mask: 4,
    shift: 2
};
pub const LH_ADV_REC: Address = Address{
    id: "LH_ADV_REC",
    address: 29706,
    mask: 1,
    shift: 0
};
pub const GEAR_DOWNLOCK_OVERRIDE_BTN: Address = Address{
    id: "GEAR_DOWNLOCK_OVERRIDE_BTN",
    address: 29816,
    mask: 16384,
    shift: 14
};
pub const GEAR_LEVER: Address = Address{
    id: "GEAR_LEVER",
    address: 29816,
    mask: 4096,
    shift: 12
};
pub const GEAR_SILENCE_BTN: Address = Address{
    id: "GEAR_SILENCE_BTN",
    address: 29816,
    mask: 32768,
    shift: 15
};
pub const LANDING_GEAR_HANDLE_LT: Address = Address{
    id: "LANDING_GEAR_HANDLE_LT",
    address: 29816,
    mask: 2048,
    shift: 11
};
pub const EMERGENCY_GEAR_ROTATE: Address = Address{
    id: "EMERGENCY_GEAR_ROTATE",
    address: 29816,
    mask: 8192,
    shift: 13
};
pub const COM_ILS_CHANNEL_SW: Address = Address{
    id: "COM_ILS_CHANNEL_SW",
    address: 29880,
    mask: 31744,
    shift: 10
};
pub const COM_IFF_MASTER_SW: Address = Address{
    id: "COM_IFF_MASTER_SW",
    address: 29878,
    mask: 4096,
    shift: 12
};
pub const COM_MIDS_A: Address = Address{
    id: "COM_MIDS_A",
    address: 29986,
    mask: 65535,
    shift: 0
};
pub const COM_MIDS_B: Address = Address{
    id: "COM_MIDS_B",
    address: 29988,
    mask: 65535,
    shift: 0
};
pub const COM_CRYPTO_SW: Address = Address{
    id: "COM_CRYPTO_SW",
    address: 29880,
    mask: 768,
    shift: 8
};
pub const COM_ILS_UFC_MAN_SW: Address = Address{
    id: "COM_ILS_UFC_MAN_SW",
    address: 29878,
    mask: 32768,
    shift: 15
};
pub const COM_COMM_G_XMT_SW: Address = Address{
    id: "COM_COMM_G_XMT_SW",
    address: 29878,
    mask: 3072,
    shift: 10
};
pub const COM_TACAN: Address = Address{
    id: "COM_TACAN",
    address: 29990,
    mask: 65535,
    shift: 0
};
pub const COM_IFF_MODE4_SW: Address = Address{
    id: "COM_IFF_MODE4_SW",
    address: 29878,
    mask: 24576,
    shift: 13
};
pub const COM_ICS: Address = Address{
    id: "COM_ICS",
    address: 29980,
    mask: 65535,
    shift: 0
};
pub const COM_COMM_RELAY_SW: Address = Address{
    id: "COM_COMM_RELAY_SW",
    address: 29878,
    mask: 768,
    shift: 8
};
pub const COM_RWR: Address = Address{
    id: "COM_RWR",
    address: 29982,
    mask: 65535,
    shift: 0
};
pub const COM_AUX: Address = Address{
    id: "COM_AUX",
    address: 29992,
    mask: 65535,
    shift: 0
};
pub const COM_WPN: Address = Address{
    id: "COM_WPN",
    address: 29984,
    mask: 65535,
    shift: 0
};
pub const COM_VOX: Address = Address{
    id: "COM_VOX",
    address: 29978,
    mask: 65535,
    shift: 0
};
pub const HUD_VIDEO_BIT: Address = Address{
    id: "HUD_VIDEO_BIT",
    address: 29706,
    mask: 64,
    shift: 6
};
pub const FIRE_EXT_BTN: Address = Address{
    id: "FIRE_EXT_BTN",
    address: 29710,
    mask: 1,
    shift: 0
};
pub const RH_ADV_AI: Address = Address{
    id: "RH_ADV_AI",
    address: 29706,
    mask: 1024,
    shift: 10
};
pub const RH_ADV_SAM: Address = Address{
    id: "RH_ADV_SAM",
    address: 29706,
    mask: 512,
    shift: 9
};
pub const RH_ADV_SPARE_RH3: Address = Address{
    id: "RH_ADV_SPARE_RH3",
    address: 29706,
    mask: 32768,
    shift: 15
};
pub const RH_ADV_SPARE_RH4: Address = Address{
    id: "RH_ADV_SPARE_RH4",
    address: 29708,
    mask: 1,
    shift: 0
};
pub const RH_ADV_SPARE_RH2: Address = Address{
    id: "RH_ADV_SPARE_RH2",
    address: 29706,
    mask: 16384,
    shift: 14
};
pub const RH_ADV_SPARE_RH1: Address = Address{
    id: "RH_ADV_SPARE_RH1",
    address: 29706,
    mask: 8192,
    shift: 13
};
pub const RH_ADV_SPARE_RH5: Address = Address{
    id: "RH_ADV_SPARE_RH5",
    address: 29708,
    mask: 2,
    shift: 1
};
pub const RH_ADV_AAA: Address = Address{
    id: "RH_ADV_AAA",
    address: 29706,
    mask: 2048,
    shift: 11
};
pub const RH_ADV_DISP: Address = Address{
    id: "RH_ADV_DISP",
    address: 29706,
    mask: 256,
    shift: 8
};
pub const RH_ADV_RCDR_ON: Address = Address{
    id: "RH_ADV_RCDR_ON",
    address: 29706,
    mask: 128,
    shift: 7
};
pub const RH_ADV_CW: Address = Address{
    id: "RH_ADV_CW",
    address: 29706,
    mask: 4096,
    shift: 12
};
pub const CLIP_SPARE_CTN3_LT: Address = Address{
    id: "CLIP_SPARE_CTN3_LT",
    address: 29870,
    mask: 512,
    shift: 9
};
pub const CLIP_BATT_SW_LT: Address = Address{
    id: "CLIP_BATT_SW_LT",
    address: 29862,
    mask: 256,
    shift: 8
};
pub const CLIP_APU_ACC_LT: Address = Address{
    id: "CLIP_APU_ACC_LT",
    address: 29858,
    mask: 32768,
    shift: 15
};
pub const CLIP_GEN_TIE_LT: Address = Address{
    id: "CLIP_GEN_TIE_LT",
    address: 29862,
    mask: 1024,
    shift: 10
};
pub const CLIP_CK_SEAT_LT: Address = Address{
    id: "CLIP_CK_SEAT_LT",
    address: 29858,
    mask: 16384,
    shift: 14
};
pub const CLIP_FUEL_LO_LT: Address = Address{
    id: "CLIP_FUEL_LO_LT",
    address: 29862,
    mask: 4096,
    shift: 12
};
pub const CLIP_R_GEN_LT: Address = Address{
    id: "CLIP_R_GEN_LT",
    address: 29870,
    mask: 256,
    shift: 8
};
pub const CLIP_SPARE_CTN1_LT: Address = Address{
    id: "CLIP_SPARE_CTN1_LT",
    address: 29862,
    mask: 2048,
    shift: 11
};
pub const CLIP_SPARE_CTN2_LT: Address = Address{
    id: "CLIP_SPARE_CTN2_LT",
    address: 29862,
    mask: 16384,
    shift: 14
};
pub const CLIP_FCS_HOT_LT: Address = Address{
    id: "CLIP_FCS_HOT_LT",
    address: 29862,
    mask: 512,
    shift: 9
};
pub const CLIP_L_GEN_LT: Address = Address{
    id: "CLIP_L_GEN_LT",
    address: 29862,
    mask: 32768,
    shift: 15
};
pub const CLIP_FCES_LT: Address = Address{
    id: "CLIP_FCES_LT",
    address: 29862,
    mask: 8192,
    shift: 13
};
pub const MASTER_CAUTION_LT: Address = Address{
    id: "MASTER_CAUTION_LT",
    address: 29704,
    mask: 512,
    shift: 9
};
pub const MASTER_CAUTION_RESET_SW: Address = Address{
    id: "MASTER_CAUTION_RESET_SW",
    address: 29704,
    mask: 1024,
    shift: 10
};
pub const RIGHT_DDI_PB_15: Address = Address{
    id: "RIGHT_DDI_PB_15",
    address: 29736,
    mask: 8192,
    shift: 13
};
pub const RIGHT_DDI_BRT_SELECT: Address = Address{
    id: "RIGHT_DDI_BRT_SELECT",
    address: 29720,
    mask: 96,
    shift: 5
};
pub const RIGHT_DDI_PB_05: Address = Address{
    id: "RIGHT_DDI_PB_05",
    address: 29720,
    mask: 2048,
    shift: 11
};
pub const RIGHT_DDI_PB_04: Address = Address{
    id: "RIGHT_DDI_PB_04",
    address: 29720,
    mask: 1024,
    shift: 10
};
pub const RIGHT_DDI_PB_06: Address = Address{
    id: "RIGHT_DDI_PB_06",
    address: 29720,
    mask: 4096,
    shift: 12
};
pub const RIGHT_DDI_PB_10: Address = Address{
    id: "RIGHT_DDI_PB_10",
    address: 29736,
    mask: 256,
    shift: 8
};
pub const RIGHT_DDI_PB_19: Address = Address{
    id: "RIGHT_DDI_PB_19",
    address: 29738,
    mask: 512,
    shift: 9
};
pub const RIGHT_DDI_BRT_CTL: Address = Address{
    id: "RIGHT_DDI_BRT_CTL",
    address: 29778,
    mask: 65535,
    shift: 0
};
pub const RIGHT_DDI_PB_18: Address = Address{
    id: "RIGHT_DDI_PB_18",
    address: 29738,
    mask: 256,
    shift: 8
};
pub const RIGHT_DDI_PB_08: Address = Address{
    id: "RIGHT_DDI_PB_08",
    address: 29720,
    mask: 16384,
    shift: 14
};
pub const RIGHT_DDI_CONT_CTL: Address = Address{
    id: "RIGHT_DDI_CONT_CTL",
    address: 29780,
    mask: 65535,
    shift: 0
};
pub const RIGHT_DDI_PB_09: Address = Address{
    id: "RIGHT_DDI_PB_09",
    address: 29720,
    mask: 32768,
    shift: 15
};
pub const RIGHT_DDI_PB_02: Address = Address{
    id: "RIGHT_DDI_PB_02",
    address: 29720,
    mask: 256,
    shift: 8
};
pub const RIGHT_DDI_PB_07: Address = Address{
    id: "RIGHT_DDI_PB_07",
    address: 29720,
    mask: 8192,
    shift: 13
};
pub const RIGHT_DDI_PB_12: Address = Address{
    id: "RIGHT_DDI_PB_12",
    address: 29736,
    mask: 1024,
    shift: 10
};
pub const RIGHT_DDI_PB_14: Address = Address{
    id: "RIGHT_DDI_PB_14",
    address: 29736,
    mask: 4096,
    shift: 12
};
pub const RIGHT_DDI_PB_11: Address = Address{
    id: "RIGHT_DDI_PB_11",
    address: 29736,
    mask: 512,
    shift: 9
};
pub const RIGHT_DDI_PB_03: Address = Address{
    id: "RIGHT_DDI_PB_03",
    address: 29720,
    mask: 512,
    shift: 9
};
pub const RIGHT_DDI_PB_01: Address = Address{
    id: "RIGHT_DDI_PB_01",
    address: 29720,
    mask: 128,
    shift: 7
};
pub const RIGHT_DDI_PB_16: Address = Address{
    id: "RIGHT_DDI_PB_16",
    address: 29736,
    mask: 16384,
    shift: 14
};
pub const RIGHT_DDI_PB_17: Address = Address{
    id: "RIGHT_DDI_PB_17",
    address: 29736,
    mask: 32768,
    shift: 15
};
pub const RIGHT_DDI_PB_20: Address = Address{
    id: "RIGHT_DDI_PB_20",
    address: 29738,
    mask: 1024,
    shift: 10
};
pub const RIGHT_DDI_PB_13: Address = Address{
    id: "RIGHT_DDI_PB_13",
    address: 29736,
    mask: 2048,
    shift: 11
};
pub const APU_CONTROL_SW: Address = Address{
    id: "APU_CONTROL_SW",
    address: 29882,
    mask: 32768,
    shift: 15
};
pub const APU_READY_LT: Address = Address{
    id: "APU_READY_LT",
    address: 29884,
    mask: 1024,
    shift: 10
};
pub const ENGINE_CRANK_SW: Address = Address{
    id: "ENGINE_CRANK_SW",
    address: 29884,
    mask: 768,
    shift: 8
};
pub const RWR_OFFSET_BTN: Address = Address{
    id: "RWR_OFFSET_BTN",
    address: 29826,
    mask: 16384,
    shift: 14
};
pub const RWR_BIT_BTN: Address = Address{
    id: "RWR_BIT_BTN",
    address: 29826,
    mask: 32768,
    shift: 15
};
pub const RWR_DMR_CTRL: Address = Address{
    id: "RWR_DMR_CTRL",
    address: 29944,
    mask: 65535,
    shift: 0
};
pub const RWR_LIMIT_LT: Address = Address{
    id: "RWR_LIMIT_LT",
    address: 29850,
    mask: 4096,
    shift: 12
};
pub const RWR_SPECIAL_LT: Address = Address{
    id: "RWR_SPECIAL_LT",
    address: 29850,
    mask: 32768,
    shift: 15
};
pub const RWR_LOWER_LT: Address = Address{
    id: "RWR_LOWER_LT",
    address: 29850,
    mask: 2048,
    shift: 11
};
pub const RWR_POWER_BTN: Address = Address{
    id: "RWR_POWER_BTN",
    address: 29826,
    mask: 2048,
    shift: 11
};
pub const RWR_DISPLAY_LT: Address = Address{
    id: "RWR_DISPLAY_LT",
    address: 29850,
    mask: 8192,
    shift: 13
};
pub const RWR_DISPLAY_BTN: Address = Address{
    id: "RWR_DISPLAY_BTN",
    address: 29826,
    mask: 4096,
    shift: 12
};
pub const RWR_ENABLE_LT: Address = Address{
    id: "RWR_ENABLE_LT",
    address: 29854,
    mask: 256,
    shift: 8
};
pub const RWR_FAIL_LT: Address = Address{
    id: "RWR_FAIL_LT",
    address: 29854,
    mask: 1024,
    shift: 10
};
pub const RWR_SPECIAL_BTN: Address = Address{
    id: "RWR_SPECIAL_BTN",
    address: 29826,
    mask: 8192,
    shift: 13
};
pub const RWR_BIT_LT: Address = Address{
    id: "RWR_BIT_LT",
    address: 29854,
    mask: 2048,
    shift: 11
};
pub const RWR_OFFSET_LT: Address = Address{
    id: "RWR_OFFSET_LT",
    address: 29854,
    mask: 512,
    shift: 9
};
pub const RWR_DIS_TYPE_SW: Address = Address{
    id: "RWR_DIS_TYPE_SW",
    address: 29850,
    mask: 1792,
    shift: 8
};
pub const RWR_SPECIAL_EN_LT: Address = Address{
    id: "RWR_SPECIAL_EN_LT",
    address: 29850,
    mask: 16384,
    shift: 14
};
pub const RWR_RWR_INTESITY: Address = Address{
    id: "RWR_RWR_INTESITY",
    address: 29946,
    mask: 65535,
    shift: 0
};
pub const AMPCD_PB_17: Address = Address{
    id: "AMPCD_PB_17",
    address: 29814,
    mask: 8192,
    shift: 13
};
pub const AMPCD_PB_16: Address = Address{
    id: "AMPCD_PB_16",
    address: 29814,
    mask: 4096,
    shift: 12
};
pub const AMPCD_PB_18: Address = Address{
    id: "AMPCD_PB_18",
    address: 29814,
    mask: 16384,
    shift: 14
};
pub const AMPCD_PB_05: Address = Address{
    id: "AMPCD_PB_05",
    address: 29812,
    mask: 512,
    shift: 9
};
pub const AMPCD_PB_08: Address = Address{
    id: "AMPCD_PB_08",
    address: 29812,
    mask: 4096,
    shift: 12
};
pub const AMPCD_PB_13: Address = Address{
    id: "AMPCD_PB_13",
    address: 29814,
    mask: 512,
    shift: 9
};
pub const AMPCD_BRT_CTL: Address = Address{
    id: "AMPCD_BRT_CTL",
    address: 29904,
    mask: 65535,
    shift: 0
};
pub const AMPCD_PB_06: Address = Address{
    id: "AMPCD_PB_06",
    address: 29812,
    mask: 1024,
    shift: 10
};
pub const AMPCD_CONT_SW: Address = Address{
    id: "AMPCD_CONT_SW",
    address: 29798,
    mask: 3072,
    shift: 10
};
pub const AMPCD_PB_15: Address = Address{
    id: "AMPCD_PB_15",
    address: 29814,
    mask: 2048,
    shift: 11
};
pub const AMPCD_PB_02: Address = Address{
    id: "AMPCD_PB_02",
    address: 29798,
    mask: 16384,
    shift: 14
};
pub const AMPCD_PB_09: Address = Address{
    id: "AMPCD_PB_09",
    address: 29812,
    mask: 8192,
    shift: 13
};
pub const AMPCD_PB_07: Address = Address{
    id: "AMPCD_PB_07",
    address: 29812,
    mask: 2048,
    shift: 11
};
pub const AMPCD_PB_10: Address = Address{
    id: "AMPCD_PB_10",
    address: 29812,
    mask: 16384,
    shift: 14
};
pub const AMPCD_PB_01: Address = Address{
    id: "AMPCD_PB_01",
    address: 29792,
    mask: 32768,
    shift: 15
};
pub const AMPCD_PB_04: Address = Address{
    id: "AMPCD_PB_04",
    address: 29812,
    mask: 256,
    shift: 8
};
pub const AMPCD_GAIN_SW: Address = Address{
    id: "AMPCD_GAIN_SW",
    address: 29798,
    mask: 12288,
    shift: 12
};
pub const AMPCD_NIGHT_DAY: Address = Address{
    id: "AMPCD_NIGHT_DAY",
    address: 29792,
    mask: 24576,
    shift: 13
};
pub const AMPCD_PB_03: Address = Address{
    id: "AMPCD_PB_03",
    address: 29798,
    mask: 32768,
    shift: 15
};
pub const AMPCD_PB_11: Address = Address{
    id: "AMPCD_PB_11",
    address: 29812,
    mask: 32768,
    shift: 15
};
pub const AMPCD_PB_14: Address = Address{
    id: "AMPCD_PB_14",
    address: 29814,
    mask: 1024,
    shift: 10
};
pub const AMPCD_PB_19: Address = Address{
    id: "AMPCD_PB_19",
    address: 29814,
    mask: 32768,
    shift: 15
};
pub const AMPCD_SYM_SW: Address = Address{
    id: "AMPCD_SYM_SW",
    address: 29798,
    mask: 768,
    shift: 8
};
pub const AMPCD_PB_12: Address = Address{
    id: "AMPCD_PB_12",
    address: 29814,
    mask: 256,
    shift: 8
};
pub const AMPCD_PB_20: Address = Address{
    id: "AMPCD_PB_20",
    address: 29816,
    mask: 256,
    shift: 8
};
pub const VSI: Address = Address{
    id: "VSI",
    address: 29936,
    mask: 65535,
    shift: 0
};
pub const RUDDER_PEDAL_ADJUST: Address = Address{
    id: "RUDDER_PEDAL_ADJUST",
    address: 29854,
    mask: 4096,
    shift: 12
};
pub const HYD_IND_BRAKE: Address = Address{
    id: "HYD_IND_BRAKE",
    address: 29942,
    mask: 65535,
    shift: 0
};
pub const LDG_TAXI_SW: Address = Address{
    id: "LDG_TAXI_SW",
    address: 29818,
    mask: 32768,
    shift: 15
};
pub const SEL_JETT_BTN: Address = Address{
    id: "SEL_JETT_BTN",
    address: 29818,
    mask: 256,
    shift: 8
};
pub const HOOK_BYPASS_SW: Address = Address{
    id: "HOOK_BYPASS_SW",
    address: 29818,
    mask: 16384,
    shift: 14
};
pub const FLAP_SW: Address = Address{
    id: "FLAP_SW",
    address: 29822,
    mask: 768,
    shift: 8
};
pub const ANTI_SKID_SW: Address = Address{
    id: "ANTI_SKID_SW",
    address: 29818,
    mask: 4096,
    shift: 12
};
pub const LAUNCH_BAR_SW: Address = Address{
    id: "LAUNCH_BAR_SW",
    address: 29818,
    mask: 8192,
    shift: 13
};
pub const SEL_JETT_KNOB: Address = Address{
    id: "SEL_JETT_KNOB",
    address: 29818,
    mask: 3584,
    shift: 9
};
pub const AOA_INDEXER_HIGH: Address = Address{
    id: "AOA_INDEXER_HIGH",
    address: 29704,
    mask: 8,
    shift: 3
};
pub const AOA_INDEXER_LOW: Address = Address{
    id: "AOA_INDEXER_LOW",
    address: 29704,
    mask: 32,
    shift: 5
};
pub const AOA_INDEXER_NORMAL: Address = Address{
    id: "AOA_INDEXER_NORMAL",
    address: 29704,
    mask: 16,
    shift: 4
};
pub const HYD_IND_LEFT: Address = Address{
    id: "HYD_IND_LEFT",
    address: 29966,
    mask: 65535,
    shift: 0
};
pub const HYD_IND_RIGHT: Address = Address{
    id: "HYD_IND_RIGHT",
    address: 29968,
    mask: 65535,
    shift: 0
};
pub const CMSD_JET_SEL_BTN: Address = Address{
    id: "CMSD_JET_SEL_BTN",
    address: 29822,
    mask: 16384,
    shift: 14
};
pub const ECM_MODE_SW: Address = Address{
    id: "ECM_MODE_SW",
    address: 29826,
    mask: 1792,
    shift: 8
};
pub const AUX_REL_SW: Address = Address{
    id: "AUX_REL_SW",
    address: 29822,
    mask: 32768,
    shift: 15
};
pub const CMSD_DISPENSE_SW: Address = Address{
    id: "CMSD_DISPENSE_SW",
    address: 29822,
    mask: 12288,
    shift: 12
};
pub const FLP_LG_LEFT_GEAR_LT: Address = Address{
    id: "FLP_LG_LEFT_GEAR_LT",
    address: 29744,
    mask: 4096,
    shift: 12
};
pub const FLP_LG_FULL_FLAPS_LT: Address = Address{
    id: "FLP_LG_FULL_FLAPS_LT",
    address: 29744,
    mask: 32768,
    shift: 15
};
pub const FLP_LG_RIGHT_GEAR_LT: Address = Address{
    id: "FLP_LG_RIGHT_GEAR_LT",
    address: 29744,
    mask: 8192,
    shift: 13
};
pub const FLP_LG_NOSE_GEAR_LT: Address = Address{
    id: "FLP_LG_NOSE_GEAR_LT",
    address: 29744,
    mask: 2048,
    shift: 11
};
pub const FLP_LG_HALF_FLAPS_LT: Address = Address{
    id: "FLP_LG_HALF_FLAPS_LT",
    address: 29744,
    mask: 16384,
    shift: 14
};
pub const FLP_LG_FLAPS_LT: Address = Address{
    id: "FLP_LG_FLAPS_LT",
    address: 29792,
    mask: 1,
    shift: 0
};
pub const IFEI_R50_TEXTURE: Address = Address{
    id: "IFEI_R50_TEXTURE",
    address: 29890,
    mask: 0,
    shift: 0
};
pub const IFEI_LPOINTER_TEXTURE: Address = Address{
    id: "IFEI_LPOINTER_TEXTURE",
    address: 29896,
    mask: 0,
    shift: 0
};
pub const IFEI_TIMER_H: Address = Address{
    id: "IFEI_TIMER_H",
    address: 29806,
    mask: 0,
    shift: 0
};
pub const IFEI_ZONE_BTN: Address = Address{
    id: "IFEI_ZONE_BTN",
    address: 29792,
    mask: 32,
    shift: 5
};
pub const IFEI_DWN_BTN: Address = Address{
    id: "IFEI_DWN_BTN",
    address: 29792,
    mask: 16,
    shift: 4
};
pub const IFEI_CODES: Address = Address{
    id: "IFEI_CODES",
    address: 29856,
    mask: 0,
    shift: 0
};
pub const IFEI_UP_BTN: Address = Address{
    id: "IFEI_UP_BTN",
    address: 29792,
    mask: 8,
    shift: 3
};
pub const IFEI_RPM_L: Address = Address{
    id: "IFEI_RPM_L",
    address: 29844,
    mask: 0,
    shift: 0
};
pub const IFEI_BINGO_TEXTURE: Address = Address{
    id: "IFEI_BINGO_TEXTURE",
    address: 29878,
    mask: 0,
    shift: 0
};
pub const IFEI_L0_TEXTURE: Address = Address{
    id: "IFEI_L0_TEXTURE",
    address: 29886,
    mask: 0,
    shift: 0
};
pub const IFEI_FUEL_UP: Address = Address{
    id: "IFEI_FUEL_UP",
    address: 29834,
    mask: 0,
    shift: 0
};
pub const IFEI_FF_L: Address = Address{
    id: "IFEI_FF_L",
    address: 29820,
    mask: 0,
    shift: 0
};
pub const IFEI_TEMP_TEXTURE: Address = Address{
    id: "IFEI_TEMP_TEXTURE",
    address: 29872,
    mask: 0,
    shift: 0
};
pub const IFEI_RPOINTER_TEXTURE: Address = Address{
    id: "IFEI_RPOINTER_TEXTURE",
    address: 29898,
    mask: 0,
    shift: 0
};
pub const IFEI_TIMER_M: Address = Address{
    id: "IFEI_TIMER_M",
    address: 29808,
    mask: 0,
    shift: 0
};
pub const IFEI_CLOCK_M: Address = Address{
    id: "IFEI_CLOCK_M",
    address: 29802,
    mask: 0,
    shift: 0
};
pub const IFEI_TIMER_S: Address = Address{
    id: "IFEI_TIMER_S",
    address: 29810,
    mask: 0,
    shift: 0
};
pub const IFEI_Z_TEXTURE: Address = Address{
    id: "IFEI_Z_TEXTURE",
    address: 29900,
    mask: 0,
    shift: 0
};
pub const IFEI_DD_1: Address = Address{
    id: "IFEI_DD_1",
    address: 29812,
    mask: 0,
    shift: 0
};
pub const IFEI_QTY_BTN: Address = Address{
    id: "IFEI_QTY_BTN",
    address: 29792,
    mask: 4,
    shift: 2
};
pub const IFEI_L100_TEXTURE: Address = Address{
    id: "IFEI_L100_TEXTURE",
    address: 29892,
    mask: 0,
    shift: 0
};
pub const IFEI_TIME_SET_MODE: Address = Address{
    id: "IFEI_TIME_SET_MODE",
    address: 29864,
    mask: 0,
    shift: 0
};
pub const IFEI_CLOCK_H: Address = Address{
    id: "IFEI_CLOCK_H",
    address: 29800,
    mask: 0,
    shift: 0
};
pub const IFEI_RPM_R: Address = Address{
    id: "IFEI_RPM_R",
    address: 29846,
    mask: 0,
    shift: 0
};
pub const IFEI_OIL_TEXTURE: Address = Address{
    id: "IFEI_OIL_TEXTURE",
    address: 29876,
    mask: 0,
    shift: 0
};
pub const IFEI_MODE_BTN: Address = Address{
    id: "IFEI_MODE_BTN",
    address: 29792,
    mask: 2,
    shift: 1
};
pub const IFEI_L50_TEXTURE: Address = Address{
    id: "IFEI_L50_TEXTURE",
    address: 29888,
    mask: 0,
    shift: 0
};
pub const IFEI_RSCALE_TEXTURE: Address = Address{
    id: "IFEI_RSCALE_TEXTURE",
    address: 29882,
    mask: 0,
    shift: 0
};
pub const IFEI_DD_4: Address = Address{
    id: "IFEI_DD_4",
    address: 29818,
    mask: 0,
    shift: 0
};
pub const IFEI_TEMP_R: Address = Address{
    id: "IFEI_TEMP_R",
    address: 29852,
    mask: 0,
    shift: 0
};
pub const IFEI_BINGO: Address = Address{
    id: "IFEI_BINGO",
    address: 29794,
    mask: 0,
    shift: 0
};
pub const IFEI_OIL_PRESS_L: Address = Address{
    id: "IFEI_OIL_PRESS_L",
    address: 29840,
    mask: 0,
    shift: 0
};
pub const IFEI_LSCALE_TEXTURE: Address = Address{
    id: "IFEI_LSCALE_TEXTURE",
    address: 29880,
    mask: 0,
    shift: 0
};
pub const IFEI_RPM_TEXTURE: Address = Address{
    id: "IFEI_RPM_TEXTURE",
    address: 29870,
    mask: 0,
    shift: 0
};
pub const IFEI_DD_3: Address = Address{
    id: "IFEI_DD_3",
    address: 29816,
    mask: 0,
    shift: 0
};
pub const IFEI_CLOCK_S: Address = Address{
    id: "IFEI_CLOCK_S",
    address: 29804,
    mask: 0,
    shift: 0
};
pub const IFEI_R100_TEXTURE: Address = Address{
    id: "IFEI_R100_TEXTURE",
    address: 29894,
    mask: 0,
    shift: 0
};
pub const IFEI_DD_2: Address = Address{
    id: "IFEI_DD_2",
    address: 29814,
    mask: 0,
    shift: 0
};
pub const IFEI_FF_R: Address = Address{
    id: "IFEI_FF_R",
    address: 29824,
    mask: 0,
    shift: 0
};
pub const IFEI_FUEL_DOWN: Address = Address{
    id: "IFEI_FUEL_DOWN",
    address: 29828,
    mask: 0,
    shift: 0
};
pub const IFEI_OIL_PRESS_R: Address = Address{
    id: "IFEI_OIL_PRESS_R",
    address: 29842,
    mask: 0,
    shift: 0
};
pub const IFEI_FF_TEXTURE: Address = Address{
    id: "IFEI_FF_TEXTURE",
    address: 29874,
    mask: 0,
    shift: 0
};
pub const IFEI_ET_BTN: Address = Address{
    id: "IFEI_ET_BTN",
    address: 29792,
    mask: 64,
    shift: 6
};
pub const IFEI_SP: Address = Address{
    id: "IFEI_SP",
    address: 29860,
    mask: 0,
    shift: 0
};
pub const IFEI_TEMP_L: Address = Address{
    id: "IFEI_TEMP_L",
    address: 29848,
    mask: 0,
    shift: 0
};
pub const LEFT_DDI_PB_20: Address = Address{
    id: "LEFT_DDI_PB_20",
    address: 29716,
    mask: 64,
    shift: 6
};
pub const LEFT_DDI_PB_04: Address = Address{
    id: "LEFT_DDI_PB_04",
    address: 29710,
    mask: 64,
    shift: 6
};
pub const LEFT_DDI_PB_18: Address = Address{
    id: "LEFT_DDI_PB_18",
    address: 29716,
    mask: 16,
    shift: 4
};
pub const LEFT_DDI_PB_08: Address = Address{
    id: "LEFT_DDI_PB_08",
    address: 29710,
    mask: 1024,
    shift: 10
};
pub const LEFT_DDI_PB_06: Address = Address{
    id: "LEFT_DDI_PB_06",
    address: 29710,
    mask: 256,
    shift: 8
};
pub const LEFT_DDI_PB_15: Address = Address{
    id: "LEFT_DDI_PB_15",
    address: 29716,
    mask: 2,
    shift: 1
};
pub const LEFT_DDI_PB_10: Address = Address{
    id: "LEFT_DDI_PB_10",
    address: 29710,
    mask: 4096,
    shift: 12
};
pub const LEFT_DDI_PB_12: Address = Address{
    id: "LEFT_DDI_PB_12",
    address: 29710,
    mask: 16384,
    shift: 14
};
pub const LEFT_DDI_PB_01: Address = Address{
    id: "LEFT_DDI_PB_01",
    address: 29710,
    mask: 8,
    shift: 3
};
pub const LEFT_DDI_PB_09: Address = Address{
    id: "LEFT_DDI_PB_09",
    address: 29710,
    mask: 2048,
    shift: 11
};
pub const LEFT_DDI_PB_07: Address = Address{
    id: "LEFT_DDI_PB_07",
    address: 29710,
    mask: 512,
    shift: 9
};
pub const LEFT_DDI_PB_13: Address = Address{
    id: "LEFT_DDI_PB_13",
    address: 29710,
    mask: 32768,
    shift: 15
};
pub const LEFT_DDI_PB_11: Address = Address{
    id: "LEFT_DDI_PB_11",
    address: 29710,
    mask: 8192,
    shift: 13
};
pub const LEFT_DDI_BRT_SELECT: Address = Address{
    id: "LEFT_DDI_BRT_SELECT",
    address: 29710,
    mask: 6,
    shift: 1
};
pub const LEFT_DDI_PB_05: Address = Address{
    id: "LEFT_DDI_PB_05",
    address: 29710,
    mask: 128,
    shift: 7
};
pub const LEFT_DDI_PB_14: Address = Address{
    id: "LEFT_DDI_PB_14",
    address: 29716,
    mask: 1,
    shift: 0
};
pub const LEFT_DDI_PB_17: Address = Address{
    id: "LEFT_DDI_PB_17",
    address: 29716,
    mask: 8,
    shift: 3
};
pub const LEFT_DDI_PB_19: Address = Address{
    id: "LEFT_DDI_PB_19",
    address: 29716,
    mask: 32,
    shift: 5
};
pub const LEFT_DDI_BRT_CTL: Address = Address{
    id: "LEFT_DDI_BRT_CTL",
    address: 29712,
    mask: 65535,
    shift: 0
};
pub const LEFT_DDI_PB_02: Address = Address{
    id: "LEFT_DDI_PB_02",
    address: 29710,
    mask: 16,
    shift: 4
};
pub const LEFT_DDI_PB_03: Address = Address{
    id: "LEFT_DDI_PB_03",
    address: 29710,
    mask: 32,
    shift: 5
};
pub const LEFT_DDI_PB_16: Address = Address{
    id: "LEFT_DDI_PB_16",
    address: 29716,
    mask: 4,
    shift: 2
};
pub const LEFT_DDI_CONT_CTL: Address = Address{
    id: "LEFT_DDI_CONT_CTL",
    address: 29714,
    mask: 65535,
    shift: 0
};
pub const WING_FOLD_PULL: Address = Address{
    id: "WING_FOLD_PULL",
    address: 29858,
    mask: 1024,
    shift: 10
};
pub const WING_FOLD_ROTATE: Address = Address{
    id: "WING_FOLD_ROTATE",
    address: 29858,
    mask: 6144,
    shift: 11
};
pub const MC_SW: Address = Address{
    id: "MC_SW",
    address: 29882,
    mask: 768,
    shift: 8
};
pub const HYD_ISOLATE_OVERRIDE_SW: Address = Address{
    id: "HYD_ISOLATE_OVERRIDE_SW",
    address: 29882,
    mask: 1024,
    shift: 10
};
pub const TO_TRIM_BTN: Address = Address{
    id: "TO_TRIM_BTN",
    address: 29876,
    mask: 4096,
    shift: 12
};
pub const FCS_RESET_BTN: Address = Address{
    id: "FCS_RESET_BTN",
    address: 29876,
    mask: 8192,
    shift: 13
};
pub const GAIN_SWITCH_COVER: Address = Address{
    id: "GAIN_SWITCH_COVER",
    address: 29876,
    mask: 16384,
    shift: 14
};
pub const RUD_TRIM: Address = Address{
    id: "RUD_TRIM",
    address: 29976,
    mask: 65535,
    shift: 0
};
pub const GAIN_SWITCH: Address = Address{
    id: "GAIN_SWITCH",
    address: 29876,
    mask: 32768,
    shift: 15
};
pub const LIGHTS_TEST_SW: Address = Address{
    id: "LIGHTS_TEST_SW",
    address: 29890,
    mask: 1024,
    shift: 10
};
pub const CONSOLES_DIMMER: Address = Address{
    id: "CONSOLES_DIMMER",
    address: 30004,
    mask: 65535,
    shift: 0
};
pub const CHART_DIMMER: Address = Address{
    id: "CHART_DIMMER",
    address: 30010,
    mask: 65535,
    shift: 0
};
pub const COCKKPIT_LIGHT_MODE_SW: Address = Address{
    id: "COCKKPIT_LIGHT_MODE_SW",
    address: 29890,
    mask: 768,
    shift: 8
};
pub const FLOOD_DIMMER: Address = Address{
    id: "FLOOD_DIMMER",
    address: 30008,
    mask: 65535,
    shift: 0
};
pub const WARN_CAUTION_DIMMER: Address = Address{
    id: "WARN_CAUTION_DIMMER",
    address: 30012,
    mask: 65535,
    shift: 0
};
pub const INST_PNL_DIMMER: Address = Address{
    id: "INST_PNL_DIMMER",
    address: 30006,
    mask: 65535,
    shift: 0
};
pub const HOOK_LEVER: Address = Address{
    id: "HOOK_LEVER",
    address: 29858,
    mask: 256,
    shift: 8
};
pub const ARRESTING_HOOK_LT: Address = Address{
    id: "ARRESTING_HOOK_LT",
    address: 29858,
    mask: 512,
    shift: 9
};
pub const FIRE_APU_LT: Address = Address{
    id: "FIRE_APU_LT",
    address: 29708,
    mask: 4,
    shift: 2
};
pub const APU_FIRE_BTN: Address = Address{
    id: "APU_FIRE_BTN",
    address: 29708,
    mask: 8,
    shift: 3
};
pub const RIGHT_VIDEO_BIT: Address = Address{
    id: "RIGHT_VIDEO_BIT",
    address: 29898,
    mask: 32768,
    shift: 15
};
pub const LEFT_VIDEO_BIT: Address = Address{
    id: "LEFT_VIDEO_BIT",
    address: 29898,
    mask: 16384,
    shift: 14
};
pub const CB_FCS_CHAN1: Address = Address{
    id: "CB_FCS_CHAN1",
    address: 29884,
    mask: 16384,
    shift: 14
};
pub const CB_FCS_CHAN2: Address = Address{
    id: "CB_FCS_CHAN2",
    address: 29884,
    mask: 32768,
    shift: 15
};
pub const CB_LAUNCH_BAR: Address = Address{
    id: "CB_LAUNCH_BAR",
    address: 29886,
    mask: 512,
    shift: 9
};
pub const CB_SPD_BRK: Address = Address{
    id: "CB_SPD_BRK",
    address: 29886,
    mask: 256,
    shift: 8
};
pub const STBY_ASI_AIRSPEED: Address = Address{
    id: "STBY_ASI_AIRSPEED",
    address: 29920,
    mask: 65535,
    shift: 0
};
pub const SAI_BANK: Address = Address{
    id: "SAI_BANK",
    address: 29910,
    mask: 65535,
    shift: 0
};
pub const SAI_PITCH: Address = Address{
    id: "SAI_PITCH",
    address: 29908,
    mask: 65535,
    shift: 0
};
pub const SAI_TEST_BTN: Address = Address{
    id: "SAI_TEST_BTN",
    address: 29816,
    mask: 512,
    shift: 9
};
pub const SAI_SET: Address = Address{
    id: "SAI_SET",
    address: 29906,
    mask: 65535,
    shift: 0
};
pub const SAI_SLIP_BALL: Address = Address{
    id: "SAI_SLIP_BALL",
    address: 29916,
    mask: 65535,
    shift: 0
};
pub const SAI_MAN_PITCH_ADJ: Address = Address{
    id: "SAI_MAN_PITCH_ADJ",
    address: 29914,
    mask: 65535,
    shift: 0
};
pub const SAI_CAGE: Address = Address{
    id: "SAI_CAGE",
    address: 29816,
    mask: 1024,
    shift: 10
};
pub const SAI_RATE_OF_TURN: Address = Address{
    id: "SAI_RATE_OF_TURN",
    address: 29918,
    mask: 65535,
    shift: 0
};
pub const SAI_ATT_WARNING_FLAG: Address = Address{
    id: "SAI_ATT_WARNING_FLAG",
    address: 29912,
    mask: 65535,
    shift: 0
};
pub const THROTTLE_FRICTION: Address = Address{
    id: "THROTTLE_FRICTION",
    address: 29970,
    mask: 65535,
    shift: 0
};
pub const L_GEN_SW: Address = Address{
    id: "L_GEN_SW",
    address: 29886,
    mask: 4096,
    shift: 12
};
pub const VOLT_E: Address = Address{
    id: "VOLT_E",
    address: 29998,
    mask: 65535,
    shift: 0
};
pub const BATTERY_SW: Address = Address{
    id: "BATTERY_SW",
    address: 29886,
    mask: 3072,
    shift: 10
};
pub const VOLT_U: Address = Address{
    id: "VOLT_U",
    address: 29996,
    mask: 65535,
    shift: 0
};
pub const R_GEN_SW: Address = Address{
    id: "R_GEN_SW",
    address: 29886,
    mask: 8192,
    shift: 13
};
pub const LEFT_FIRE_BTN: Address = Address{
    id: "LEFT_FIRE_BTN",
    address: 29704,
    mask: 128,
    shift: 7
};
pub const LEFT_FIRE_BTN_COVER: Address = Address{
    id: "LEFT_FIRE_BTN_COVER",
    address: 29704,
    mask: 256,
    shift: 8
};
pub const FIRE_LEFT_LT: Address = Address{
    id: "FIRE_LEFT_LT",
    address: 29704,
    mask: 64,
    shift: 6
};
pub const GEN_TIE_COVER: Address = Address{
    id: "GEN_TIE_COVER",
    address: 29884,
    mask: 2048,
    shift: 11
};
pub const GEN_TIE_SW: Address = Address{
    id: "GEN_TIE_SW",
    address: 29884,
    mask: 4096,
    shift: 12
};
pub const EJECTION_HANDLE_SW: Address = Address{
    id: "EJECTION_HANDLE_SW",
    address: 29896,
    mask: 32768,
    shift: 15
};
pub const EJECTION_SEAT_ARMED: Address = Address{
    id: "EJECTION_SEAT_ARMED",
    address: 29898,
    mask: 256,
    shift: 8
};
pub const SEAT_HEIGHT_SW: Address = Address{
    id: "SEAT_HEIGHT_SW",
    address: 29898,
    mask: 6144,
    shift: 11
};
pub const SHLDR_HARNESS_SW: Address = Address{
    id: "SHLDR_HARNESS_SW",
    address: 29898,
    mask: 1024,
    shift: 10
};
pub const EJECTION_SEAT_MNL_OVRD: Address = Address{
    id: "EJECTION_SEAT_MNL_OVRD",
    address: 29898,
    mask: 512,
    shift: 9
};
pub const HIDE_STICK_TOGGLE: Address = Address{
    id: "HIDE_STICK_TOGGLE",
    address: 29898,
    mask: 8192,
    shift: 13
};
pub const FCS_BIT_SW: Address = Address{
    id: "FCS_BIT_SW",
    address: 29896,
    mask: 16384,
    shift: 14
};
pub const CB_LG: Address = Address{
    id: "CB_LG",
    address: 29896,
    mask: 8192,
    shift: 13
};
pub const CB_FCS_CHAN3: Address = Address{
    id: "CB_FCS_CHAN3",
    address: 29892,
    mask: 32768,
    shift: 15
};
pub const CB_FCS_CHAN4: Address = Address{
    id: "CB_FCS_CHAN4",
    address: 29894,
    mask: 32768,
    shift: 15
};
pub const CB_HOOOK: Address = Address{
    id: "CB_HOOOK",
    address: 29896,
    mask: 4096,
    shift: 12
};
pub const IFF_ANT_SELECT_SW: Address = Address{
    id: "IFF_ANT_SELECT_SW",
    address: 29882,
    mask: 24576,
    shift: 13
};
pub const COMM1_ANT_SELECT_SW: Address = Address{
    id: "COMM1_ANT_SELECT_SW",
    address: 29882,
    mask: 6144,
    shift: 11
};
pub const CANOPY_JETT_HANDLE_UNLOCK: Address = Address{
    id: "CANOPY_JETT_HANDLE_UNLOCK",
    address: 29708,
    mask: 128,
    shift: 7
};
pub const CANOPY_JETT_HANDLE_PULL: Address = Address{
    id: "CANOPY_JETT_HANDLE_PULL",
    address: 29708,
    mask: 256,
    shift: 8
};
pub const FIRE_TEST_SW: Address = Address{
    id: "FIRE_TEST_SW",
    address: 29870,
    mask: 49152,
    shift: 14
};
pub const CMSD_DISPENSE_BTN: Address = Address{
    id: "CMSD_DISPENSE_BTN",
    address: 29884,
    mask: 8192,
    shift: 13
};
pub const EMERGENCY_PARKING_BRAKE_PULL: Address = Address{
    id: "EMERGENCY_PARKING_BRAKE_PULL",
    address: 29822,
    mask: 1024,
    shift: 10
};
pub const EMERGENCY_PARKING_BRAKE_ROTATE: Address = Address{
    id: "EMERGENCY_PARKING_BRAKE_ROTATE",
    address: 29822,
    mask: 2048,
    shift: 11
};
pub const BLEED_AIR_KNOB: Address = Address{
    id: "BLEED_AIR_KNOB",
    address: 29886,
    mask: 49152,
    shift: 14
};
pub const CABIN_PRESS_SW: Address = Address{
    id: "CABIN_PRESS_SW",
    address: 29888,
    mask: 6144,
    shift: 11
};
pub const SUIT_TEMP: Address = Address{
    id: "SUIT_TEMP",
    address: 30002,
    mask: 65535,
    shift: 0
};
pub const ECS_MODE_SW: Address = Address{
    id: "ECS_MODE_SW",
    address: 29888,
    mask: 1536,
    shift: 9
};
pub const BLEED_AIR_PULL: Address = Address{
    id: "BLEED_AIR_PULL",
    address: 29888,
    mask: 256,
    shift: 8
};
pub const CABIN_TEMP: Address = Address{
    id: "CABIN_TEMP",
    address: 30000,
    mask: 65535,
    shift: 0
};
pub const ENG_ANTIICE_SW: Address = Address{
    id: "ENG_ANTIICE_SW",
    address: 29888,
    mask: 24576,
    shift: 13
};
pub const PITOT_HEAT_SW: Address = Address{
    id: "PITOT_HEAT_SW",
    address: 29888,
    mask: 32768,
    shift: 15
};
pub const COMM2_FREQ: Address = Address{
    id: "COMM2_FREQ",
    address: 29698,
    mask: 65535,
    shift: 0
};
pub const COMM1_FREQ: Address = Address{
    id: "COMM1_FREQ",
    address: 29696,
    mask: 65535,
    shift: 0
};
pub const COMM2_CHANNEL_NUMERIC: Address = Address{
    id: "COMM2_CHANNEL_NUMERIC",
    address: 29702,
    mask: 65535,
    shift: 0
};
pub const COMM1_CHANNEL_NUMERIC: Address = Address{
    id: "COMM1_CHANNEL_NUMERIC",
    address: 29700,
    mask: 65535,
    shift: 0
};
pub const OBOGS_SW: Address = Address{
    id: "OBOGS_SW",
    address: 29880,
    mask: 32768,
    shift: 15
};
pub const OXY_FLOW: Address = Address{
    id: "OXY_FLOW",
    address: 29994,
    mask: 65535,
    shift: 0
};
pub const EXT_FORMATION_LIGHTS: Address = Address{
    id: "EXT_FORMATION_LIGHTS",
    address: 30022,
    mask: 65535,
    shift: 0
};
pub const EXT_POSITION_LIGHT_LEFT: Address = Address{
    id: "EXT_POSITION_LIGHT_LEFT",
    address: 29900,
    mask: 256,
    shift: 8
};
pub const EXT_SPEED_BRAKE: Address = Address{
    id: "EXT_SPEED_BRAKE",
    address: 30020,
    mask: 65535,
    shift: 0
};
pub const EXT_POSITION_LIGHT_RIGHT: Address = Address{
    id: "EXT_POSITION_LIGHT_RIGHT",
    address: 29900,
    mask: 512,
    shift: 9
};
pub const EXT_STROBE_LIGHTS: Address = Address{
    id: "EXT_STROBE_LIGHTS",
    address: 29900,
    mask: 1024,
    shift: 10
};
pub const UFC_7: Address = Address{
    id: "UFC_7",
    address: 29720,
    mask: 1,
    shift: 0
};
pub const UFC_OPTION_CUEING_1: Address = Address{
    id: "UFC_OPTION_CUEING_1",
    address: 29736,
    mask: 0,
    shift: 0
};
pub const UFC_BCN: Address = Address{
    id: "UFC_BCN",
    address: 29716,
    mask: 4096,
    shift: 12
};
pub const UFC_COMM2_VOL: Address = Address{
    id: "UFC_COMM2_VOL",
    address: 29724,
    mask: 65535,
    shift: 0
};
pub const UFC_OPTION_CUEING_2: Address = Address{
    id: "UFC_OPTION_CUEING_2",
    address: 29738,
    mask: 0,
    shift: 0
};
pub const UFC_OS4: Address = Address{
    id: "UFC_OS4",
    address: 29718,
    mask: 8,
    shift: 3
};
pub const UFC_TCN: Address = Address{
    id: "UFC_TCN",
    address: 29716,
    mask: 512,
    shift: 9
};
pub const UFC_OPTION_DISPLAY_3: Address = Address{
    id: "UFC_OPTION_DISPLAY_3",
    address: 29754,
    mask: 0,
    shift: 0
};
pub const UFC_OS3: Address = Address{
    id: "UFC_OS3",
    address: 29718,
    mask: 4,
    shift: 2
};
pub const UFC_CLR: Address = Address{
    id: "UFC_CLR",
    address: 29720,
    mask: 8,
    shift: 3
};
pub const UFC_OPTION_DISPLAY_4: Address = Address{
    id: "UFC_OPTION_DISPLAY_4",
    address: 29758,
    mask: 0,
    shift: 0
};
pub const UFC_COMM1_DISPLAY: Address = Address{
    id: "UFC_COMM1_DISPLAY",
    address: 29732,
    mask: 0,
    shift: 0
};
pub const UFC_1: Address = Address{
    id: "UFC_1",
    address: 29718,
    mask: 1024,
    shift: 10
};
pub const UFC_IFF: Address = Address{
    id: "UFC_IFF",
    address: 29716,
    mask: 256,
    shift: 8
};
pub const UFC_ONOFF: Address = Address{
    id: "UFC_ONOFF",
    address: 29716,
    mask: 8192,
    shift: 13
};
pub const UFC_OPTION_CUEING_3: Address = Address{
    id: "UFC_OPTION_CUEING_3",
    address: 29740,
    mask: 0,
    shift: 0
};
pub const UFC_COMM1_VOL: Address = Address{
    id: "UFC_COMM1_VOL",
    address: 29722,
    mask: 65535,
    shift: 0
};
pub const UFC_OPTION_CUEING_4: Address = Address{
    id: "UFC_OPTION_CUEING_4",
    address: 29742,
    mask: 0,
    shift: 0
};
pub const UFC_OS2: Address = Address{
    id: "UFC_OS2",
    address: 29718,
    mask: 2,
    shift: 1
};
pub const UFC_ILS: Address = Address{
    id: "UFC_ILS",
    address: 29716,
    mask: 1024,
    shift: 10
};
pub const UFC_ENT: Address = Address{
    id: "UFC_ENT",
    address: 29720,
    mask: 16,
    shift: 4
};
pub const UFC_6: Address = Address{
    id: "UFC_6",
    address: 29718,
    mask: 32768,
    shift: 15
};
pub const UFC_5: Address = Address{
    id: "UFC_5",
    address: 29718,
    mask: 16384,
    shift: 14
};
pub const UFC_9: Address = Address{
    id: "UFC_9",
    address: 29720,
    mask: 4,
    shift: 2
};
pub const UFC_ADF: Address = Address{
    id: "UFC_ADF",
    address: 29718,
    mask: 192,
    shift: 6
};
pub const UFC_8: Address = Address{
    id: "UFC_8",
    address: 29720,
    mask: 2,
    shift: 1
};
pub const UFC_BRT: Address = Address{
    id: "UFC_BRT",
    address: 29726,
    mask: 65535,
    shift: 0
};
pub const UFC_DL: Address = Address{
    id: "UFC_DL",
    address: 29716,
    mask: 2048,
    shift: 11
};
pub const UFC_0: Address = Address{
    id: "UFC_0",
    address: 29718,
    mask: 512,
    shift: 9
};
pub const UFC_EMCON: Address = Address{
    id: "UFC_EMCON",
    address: 29718,
    mask: 256,
    shift: 8
};
pub const UFC_AP: Address = Address{
    id: "UFC_AP",
    address: 29716,
    mask: 128,
    shift: 7
};
pub const UFC_OPTION_CUEING_5: Address = Address{
    id: "UFC_OPTION_CUEING_5",
    address: 29744,
    mask: 0,
    shift: 0
};
pub const UFC_COMM1_PULL: Address = Address{
    id: "UFC_COMM1_PULL",
    address: 29716,
    mask: 16384,
    shift: 14
};
pub const UFC_COMM2_DISPLAY: Address = Address{
    id: "UFC_COMM2_DISPLAY",
    address: 29734,
    mask: 0,
    shift: 0
};
pub const UFC_OPTION_DISPLAY_2: Address = Address{
    id: "UFC_OPTION_DISPLAY_2",
    address: 29750,
    mask: 0,
    shift: 0
};
pub const UFC_2: Address = Address{
    id: "UFC_2",
    address: 29718,
    mask: 2048,
    shift: 11
};
pub const UFC_OPTION_DISPLAY_5: Address = Address{
    id: "UFC_OPTION_DISPLAY_5",
    address: 29762,
    mask: 0,
    shift: 0
};
pub const UFC_COMM2_PULL: Address = Address{
    id: "UFC_COMM2_PULL",
    address: 29716,
    mask: 32768,
    shift: 15
};
pub const UFC_OS1: Address = Address{
    id: "UFC_OS1",
    address: 29718,
    mask: 1,
    shift: 0
};
pub const UFC_OS5: Address = Address{
    id: "UFC_OS5",
    address: 29718,
    mask: 16,
    shift: 4
};
pub const UFC_SCRATCHPAD_NUMBER_DISPLAY: Address = Address{
    id: "UFC_SCRATCHPAD_NUMBER_DISPLAY",
    address: 29766,
    mask: 0,
    shift: 0
};
pub const UFC_SCRATCHPAD_STRING_1_DISPLAY: Address = Address{
    id: "UFC_SCRATCHPAD_STRING_1_DISPLAY",
    address: 29774,
    mask: 0,
    shift: 0
};
pub const UFC_IP: Address = Address{
    id: "UFC_IP",
    address: 29718,
    mask: 32,
    shift: 5
};
pub const UFC_OPTION_DISPLAY_1: Address = Address{
    id: "UFC_OPTION_DISPLAY_1",
    address: 29746,
    mask: 0,
    shift: 0
};
pub const UFC_3: Address = Address{
    id: "UFC_3",
    address: 29718,
    mask: 4096,
    shift: 12
};
pub const UFC_4: Address = Address{
    id: "UFC_4",
    address: 29718,
    mask: 8192,
    shift: 13
};
pub const UFC_SCRATCHPAD_STRING_2_DISPLAY: Address = Address{
    id: "UFC_SCRATCHPAD_STRING_2_DISPLAY",
    address: 29776,
    mask: 0,
    shift: 0
};
pub const EMER_JETT_BTN: Address = Address{
    id: "EMER_JETT_BTN",
    address: 29740,
    mask: 256,
    shift: 8
};
pub const IR_COOL_SW: Address = Address{
    id: "IR_COOL_SW",
    address: 29738,
    mask: 49152,
    shift: 14
};
pub const SPIN_LT: Address = Address{
    id: "SPIN_LT",
    address: 29738,
    mask: 2048,
    shift: 11
};
pub const SPIN_RECOVERY_COVER: Address = Address{
    id: "SPIN_RECOVERY_COVER",
    address: 29738,
    mask: 4096,
    shift: 12
};
pub const SPIN_RECOVERY_SW: Address = Address{
    id: "SPIN_RECOVERY_SW",
    address: 29738,
    mask: 8192,
    shift: 13
};
pub const HMD_OFF_BRT: Address = Address{
    id: "HMD_OFF_BRT",
    address: 29782,
    mask: 65535,
    shift: 0
};
pub const CLOCK_ELAPSED_SECONDS: Address = Address{
    id: "CLOCK_ELAPSED_SECONDS",
    address: 29954,
    mask: 65535,
    shift: 0
};
pub const CLOCK_MINUTES: Address = Address{
    id: "CLOCK_MINUTES",
    address: 29950,
    mask: 65535,
    shift: 0
};
pub const CLOCK_ELAPSED_MINUTES: Address = Address{
    id: "CLOCK_ELAPSED_MINUTES",
    address: 29952,
    mask: 65535,
    shift: 0
};
pub const CLOCK_HOURS: Address = Address{
    id: "CLOCK_HOURS",
    address: 29948,
    mask: 65535,
    shift: 0
};
pub const STBY_PRESS_ALT: Address = Address{
    id: "STBY_PRESS_ALT",
    address: 29922,
    mask: 65535,
    shift: 0
};
pub const STBY_ALT_1000_FT_CNT: Address = Address{
    id: "STBY_ALT_1000_FT_CNT",
    address: 29928,
    mask: 65535,
    shift: 0
};
pub const STBY_ALT_100_FT_PTR: Address = Address{
    id: "STBY_ALT_100_FT_PTR",
    address: 29924,
    mask: 65535,
    shift: 0
};
pub const STBY_PRESS_SET_0: Address = Address{
    id: "STBY_PRESS_SET_0",
    address: 29930,
    mask: 65535,
    shift: 0
};
pub const STBY_PRESS_SET_1: Address = Address{
    id: "STBY_PRESS_SET_1",
    address: 29932,
    mask: 65535,
    shift: 0
};
pub const STBY_ALT_10000_FT_CNT: Address = Address{
    id: "STBY_ALT_10000_FT_CNT",
    address: 29926,
    mask: 65535,
    shift: 0
};
pub const STBY_PRESS_SET_2: Address = Address{
    id: "STBY_PRESS_SET_2",
    address: 29934,
    mask: 65535,
    shift: 0
};
pub const SELECT_HMD_LDDI_RDDI: Address = Address{
    id: "SELECT_HMD_LDDI_RDDI",
    address: 29792,
    mask: 384,
    shift: 7
};
pub const SELECT_HUD_LDDI_RDDI: Address = Address{
    id: "SELECT_HUD_LDDI_RDDI",
    address: 29792,
    mask: 1536,
    shift: 9
};
pub const IFEI: Address = Address{
    id: "IFEI",
    address: 29902,
    mask: 65535,
    shift: 0
};
pub const MODE_SELECTOR_SW: Address = Address{
    id: "MODE_SELECTOR_SW",
    address: 29792,
    mask: 6144,
    shift: 11
};
pub const DEFOG_HANDLE: Address = Address{
    id: "DEFOG_HANDLE",
    address: 30016,
    mask: 65535,
    shift: 0
};
pub const WSHIELD_ANTI_ICE_SW: Address = Address{
    id: "WSHIELD_ANTI_ICE_SW",
    address: 29896,
    mask: 768,
    shift: 8
};
pub const MASTER_MODE_AA: Address = Address{
    id: "MASTER_MODE_AA",
    address: 29708,
    mask: 2048,
    shift: 11
};
pub const MASTER_MODE_AG: Address = Address{
    id: "MASTER_MODE_AG",
    address: 29708,
    mask: 4096,
    shift: 12
};
pub const MC_READY: Address = Address{
    id: "MC_READY",
    address: 29708,
    mask: 32768,
    shift: 15
};
pub const MASTER_ARM_SW: Address = Address{
    id: "MASTER_ARM_SW",
    address: 29708,
    mask: 8192,
    shift: 13
};
pub const MASTER_MODE_AA_LT: Address = Address{
    id: "MASTER_MODE_AA_LT",
    address: 29708,
    mask: 512,
    shift: 9
};
pub const MC_DISCH: Address = Address{
    id: "MC_DISCH",
    address: 29708,
    mask: 16384,
    shift: 14
};
pub const MASTER_MODE_AG_LT: Address = Address{
    id: "MASTER_MODE_AG_LT",
    address: 29708,
    mask: 1024,
    shift: 10
};
pub const SJ_LI: Address = Address{
    id: "SJ_LI",
    address: 29742,
    mask: 1024,
    shift: 10
};
pub const SJ_LO_LT: Address = Address{
    id: "SJ_LO_LT",
    address: 29744,
    mask: 256,
    shift: 8
};
pub const SJ_LI_LT: Address = Address{
    id: "SJ_LI_LT",
    address: 29742,
    mask: 32768,
    shift: 15
};
pub const SJ_CTR: Address = Address{
    id: "SJ_CTR",
    address: 29740,
    mask: 32768,
    shift: 15
};
pub const SJ_RI_LT: Address = Address{
    id: "SJ_RI_LT",
    address: 29744,
    mask: 512,
    shift: 9
};
pub const SJ_LO: Address = Address{
    id: "SJ_LO",
    address: 29742,
    mask: 2048,
    shift: 11
};
pub const SJ_RI: Address = Address{
    id: "SJ_RI",
    address: 29742,
    mask: 4096,
    shift: 12
};
pub const SJ_RO: Address = Address{
    id: "SJ_RO",
    address: 29742,
    mask: 8192,
    shift: 13
};
pub const SJ_CTR_LT: Address = Address{
    id: "SJ_CTR_LT",
    address: 29742,
    mask: 16384,
    shift: 14
};
pub const SJ_RO_LT: Address = Address{
    id: "SJ_RO_LT",
    address: 29744,
    mask: 1024,
    shift: 10
};
pub const LTD_R_ARM: Address = Address{
    id: "LTD_R_ARM",
    address: 29892,
    mask: 256,
    shift: 8
};
pub const FLIR_SW: Address = Address{
    id: "FLIR_SW",
    address: 29890,
    mask: 6144,
    shift: 11
};
pub const INS_SW: Address = Address{
    id: "INS_SW",
    address: 29892,
    mask: 28672,
    shift: 12
};
pub const LTD_R_SW: Address = Address{
    id: "LTD_R_SW",
    address: 29890,
    mask: 24576,
    shift: 13
};
pub const RADAR_SW: Address = Address{
    id: "RADAR_SW",
    address: 29892,
    mask: 1536,
    shift: 9
};
pub const LST_NFLR_SW: Address = Address{
    id: "LST_NFLR_SW",
    address: 29890,
    mask: 32768,
    shift: 15
};
pub const RADAR_SW_PULL: Address = Address{
    id: "RADAR_SW_PULL",
    address: 29892,
    mask: 2048,
    shift: 11
};
pub const KY58_VOLUME: Address = Address{
    id: "KY58_VOLUME",
    address: 30014,
    mask: 65535,
    shift: 0
};
pub const KY58_FILL_SELECT: Address = Address{
    id: "KY58_FILL_SELECT",
    address: 29894,
    mask: 7168,
    shift: 10
};
pub const KY58_MODE_SELECT: Address = Address{
    id: "KY58_MODE_SELECT",
    address: 29894,
    mask: 768,
    shift: 8
};
pub const KY58_POWER_SELECT: Address = Address{
    id: "KY58_POWER_SELECT",
    address: 29894,
    mask: 24576,
    shift: 13
};
pub const LS_SHOOT: Address = Address{
    id: "LS_SHOOT",
    address: 29704,
    mask: 2,
    shift: 1
};
pub const LS_LOCK: Address = Address{
    id: "LS_LOCK",
    address: 29704,
    mask: 1,
    shift: 0
};
pub const LS_SHOOT_STROBE: Address = Address{
    id: "LS_SHOOT_STROBE",
    address: 29704,
    mask: 4,
    shift: 2
};
pub const FORMATION_DIMMER: Address = Address{
    id: "FORMATION_DIMMER",
    address: 29974,
    mask: 65535,
    shift: 0
};
pub const POSITION_DIMMER: Address = Address{
    id: "POSITION_DIMMER",
    address: 29972,
    mask: 65535,
    shift: 0
};
pub const INT_WNG_TANK_SW: Address = Address{
    id: "INT_WNG_TANK_SW",
    address: 29874,
    mask: 4096,
    shift: 12
};
pub const STROBE_SW: Address = Address{
    id: "STROBE_SW",
    address: 29874,
    mask: 3072,
    shift: 10
};
pub const LEFT_DDI_CRS_SW: Address = Address{
    id: "LEFT_DDI_CRS_SW",
    address: 29870,
    mask: 12288,
    shift: 12
};
pub const LEFT_DDI_HDG_SW: Address = Address{
    id: "LEFT_DDI_HDG_SW",
    address: 29870,
    mask: 3072,
    shift: 10
};
pub const EXT_WNG_TANK_SW: Address = Address{
    id: "EXT_WNG_TANK_SW",
    address: 29876,
    mask: 3072,
    shift: 10
};
pub const FUEL_DUMP_SW: Address = Address{
    id: "FUEL_DUMP_SW",
    address: 29874,
    mask: 32768,
    shift: 15
};
pub const EXT_CNT_TANK_SW: Address = Address{
    id: "EXT_CNT_TANK_SW",
    address: 29876,
    mask: 768,
    shift: 8
};
pub const PROBE_SW: Address = Address{
    id: "PROBE_SW",
    address: 29874,
    mask: 24576,
    shift: 13
};
