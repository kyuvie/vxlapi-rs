#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub mod vxlapi {
    use std::ffi::*;
    use std::fmt::Debug;

    // Bus types
    pub const XL_BUS_TYPE_NONE      : u32 = 0x00000000u32;
    pub const XL_BUS_TYPE_CAN       : u32 = 0x00000001u32;
    pub const XL_BUS_TYPE_LIN       : u32 = 0x00000002u32;
    pub const XL_BUS_TYPE_FLEXRAY   : u32 = 0x00000004u32;
    pub const XL_BUS_TYPE_AFDX      : u32 = 0x00000008u32; // former BUS_TYPE_BEAN
    pub const XL_BUS_TYPE_MOST      : u32 = 0x00000010u32;
    pub const XL_BUS_TYPE_DAIO      : u32 = 0x00000040u32; // IO cab/piggy
    pub const XL_BUS_TYPE_J1708     : u32 = 0x00000100u32;
    pub const XL_BUS_TYPE_KLINE     : u32 = 0x00000800u32;
    pub const XL_BUS_TYPE_ETHERNET  : u32 = 0x00001000u32;
    pub const XL_BUS_TYPE_A429      : u32 = 0x00002000u32;

    //------------------------------------------------------------------------------
    // Transceiver types
    //------------------------------------------------------------------------------
    // CAN Cab
    pub const XL_TRANSCEIVER_TYPE_NONE                      : u16 = 0x0000;
    pub const XL_TRANSCEIVER_TYPE_CAN_251                   : u16 = 0x0001;
    pub const XL_TRANSCEIVER_TYPE_CAN_252                   : u16 = 0x0002;
    pub const XL_TRANSCEIVER_TYPE_CAN_DNOPTO                : u16 = 0x0003;
    pub const XL_TRANSCEIVER_TYPE_CAN_SWC_PROTO             : u16 = 0x0005; // Prototype. Driver may latch-up.
    pub const XL_TRANSCEIVER_TYPE_CAN_SWC                   : u16 = 0x0006;
    pub const XL_TRANSCEIVER_TYPE_CAN_EVA                   : u16 = 0x0007;
    pub const XL_TRANSCEIVER_TYPE_CAN_FIBER                 : u16 = 0x0008;
    pub const XL_TRANSCEIVER_TYPE_CAN_1054_OPTO             : u16 = 0x000B; // 1054 with optical isolation
    pub const XL_TRANSCEIVER_TYPE_CAN_SWC_OPTO              : u16 = 0x000C; // SWC with optical isolation
    pub const XL_TRANSCEIVER_TYPE_CAN_B10011S               : u16 = 0x000D; // B10011S truck-and-trailer
    pub const XL_TRANSCEIVER_TYPE_CAN_1050                  : u16 = 0x000E; // 1050
    pub const XL_TRANSCEIVER_TYPE_CAN_1050_OPTO             : u16 = 0x000F; // 1050 with optical isolation
    pub const XL_TRANSCEIVER_TYPE_CAN_1041                  : u16 = 0x0010; // 1041
    pub const XL_TRANSCEIVER_TYPE_CAN_1041_OPTO             : u16 = 0x0011; // 1041 with optical isolation
    pub const XL_TRANSCEIVER_TYPE_CAN_VIRTUAL               : u16 = 0x0016; // Virtual CAN Trasceiver for Virtual CAN Bus Driver
    pub const XL_TRANSCEIVER_TYPE_LIN_6258_OPTO             : u16 = 0x0017; // Vector LINcab 6258opto with transceiver Infineon TLE6258
    pub const XL_TRANSCEIVER_TYPE_LIN_6259_OPTO             : u16 = 0x0019; // Vector LINcab 6259opto with transceiver Infineon TLE6259
    pub const XL_TRANSCEIVER_TYPE_DAIO_8444_OPTO            : u16 = 0x001D; // Vector IOcab 8444  (8 dig.Inp.; 4 dig.Outp.; 4 ana.Inp.; 4 ana.Outp.)
    pub const XL_TRANSCEIVER_TYPE_CAN_1041A_OPTO            : u16 = 0x0021; // 1041A with optical isolation
    pub const XL_TRANSCEIVER_TYPE_LIN_6259_MAG              : u16 = 0x0023; // LIN transceiver 6259, with transceiver Infineon TLE6259, magnetically isolated, stress functionality
    pub const XL_TRANSCEIVER_TYPE_LIN_7259_MAG              : u16 = 0x0025; // LIN transceiver 7259, with transceiver Infineon TLE7259, magnetically isolated, stress functionality
    pub const XL_TRANSCEIVER_TYPE_LIN_7269_MAG              : u16 = 0x0027; // LIN transceiver 7269, with transceiver Infineon TLE7269, magnetically isolated, stress functionality
    pub const XL_TRANSCEIVER_TYPE_CAN_1054_MAG              : u16 = 0x0033; // TJA1054, magnetically isolated, with selectable termination resistor (via 4th IO line)
    pub const XL_TRANSCEIVER_TYPE_CAN_251_MAG               : u16 = 0x0035; // 82C250/251 or equivalent, magnetically isolated
    pub const XL_TRANSCEIVER_TYPE_CAN_1050_MAG              : u16 = 0x0037; // TJA1050, magnetically isolated
    pub const XL_TRANSCEIVER_TYPE_CAN_1040_MAG              : u16 = 0x0039; // TJA1040, magnetically isolated
    pub const XL_TRANSCEIVER_TYPE_CAN_1041A_MAG             : u16 = 0x003B; // TJA1041A, magnetically isolated
    pub const XL_TRANSCEIVER_TYPE_TWIN_CAN_1041A_MAG        : u16 = 0x0080; // TWINcab with two TJA1041, magnetically isolated
    pub const XL_TRANSCEIVER_TYPE_TWIN_LIN_7269_MAG         : u16 = 0x0081; // TWINcab with two 7259, Infineon TLE7259, magnetically isolated, stress functionality
    pub const XL_TRANSCEIVER_TYPE_TWIN_CAN_1041AV2_MAG      : u16 = 0x0082; // TWINcab with two TJA1041, magnetically isolated
    pub const XL_TRANSCEIVER_TYPE_TWIN_CAN_1054_1041A_MAG   : u16 = 0x0083; // TWINcab with TJA1054A and TJA1041A with magnetic isolation

    // CAN PiggyBack
    pub const XL_TRANSCEIVER_TYPE_PB_CAN_251                    : u16 = 0x0101u16;
    pub const XL_TRANSCEIVER_TYPE_PB_CAN_1054                   : u16 = 0x0103u16;
    pub const XL_TRANSCEIVER_TYPE_PB_CAN_251_OPTO               : u16 = 0x0105u16;
    pub const XL_TRANSCEIVER_TYPE_PB_CAN_SWC                    : u16 = 0x010Bu16;
    // 0x010D not supported, 0x010F, 0x0111, 0x0113 reserved for future use!!
    pub const XL_TRANSCEIVER_TYPE_PB_CAN_1054_OPTO              : u16 = 0x0115;
    pub const XL_TRANSCEIVER_TYPE_PB_CAN_SWC_OPTO               : u16 = 0x0117;
    pub const XL_TRANSCEIVER_TYPE_PB_CAN_TT_OPTO                : u16 = 0x0119;
    pub const XL_TRANSCEIVER_TYPE_PB_CAN_1050                   : u16 = 0x011B;
    pub const XL_TRANSCEIVER_TYPE_PB_CAN_1050_OPTO              : u16 = 0x011D;
    pub const XL_TRANSCEIVER_TYPE_PB_CAN_1041                   : u16 = 0x011F;
    pub const XL_TRANSCEIVER_TYPE_PB_CAN_1041_OPTO              : u16 = 0x0121;
    pub const XL_TRANSCEIVER_TYPE_PB_LIN_6258_OPTO              : u16 = 0x0129; // LIN piggy back with transceiver Infineon TLE6258
    pub const XL_TRANSCEIVER_TYPE_PB_LIN_6259_OPTO              : u16 = 0x012B; // LIN piggy back with transceiver Infineon TLE6259
    pub const XL_TRANSCEIVER_TYPE_PB_LIN_6259_MAG               : u16 = 0x012D; // LIN piggy back with transceiver Infineon TLE6259, magnetically isolated, stress functionality
    pub const XL_TRANSCEIVER_TYPE_PB_CAN_1041A_OPTO             : u16 = 0x012F; // CAN transceiver 1041A
    pub const XL_TRANSCEIVER_TYPE_PB_LIN_7259_MAG               : u16 = 0x0131; // LIN piggy back with transceiver Infineon TLE7259, magnetically isolated, stress functionality
    pub const XL_TRANSCEIVER_TYPE_PB_LIN_7269_MAG               : u16 = 0x0133; // LIN piggy back with transceiver Infineon TLE7269, magnetically isolated, stress functionality
    pub const XL_TRANSCEIVER_TYPE_PB_CAN_251_MAG                : u16 = 0x0135; // 82C250/251 or compatible, magnetically isolated
    pub const XL_TRANSCEIVER_TYPE_PB_CAN_1050_MAG               : u16 = 0x0136; // TJA 1050, magnetically isolated
    pub const XL_TRANSCEIVER_TYPE_PB_CAN_1040_MAG               : u16 = 0x0137; // TJA 1040, magnetically isolated
    pub const XL_TRANSCEIVER_TYPE_PB_CAN_1041A_MAG              : u16 = 0x0138; // TJA 1041A, magnetically isolated
    pub const XL_TRANSCEIVER_TYPE_PB_DAIO_8444_OPTO             : u16 = 0x0139; // optically isolated IO piggy
    pub const XL_TRANSCEIVER_TYPE_PB_CAN_1054_MAG               : u16 = 0x013B; // TJA1054, magnetically isolated, with selectable termination resistor (via 4th IO line)
    pub const XL_TRANSCEIVER_TYPE_CAN_1051_CAP_FIX              : u16 = 0x013C; // TJA1051 - fixed transceiver on e.g. 16xx/8970
    pub const XL_TRANSCEIVER_TYPE_DAIO_1021_FIX                 : u16 = 0x013D; // Onboard IO of VN1630/VN1640
    pub const XL_TRANSCEIVER_TYPE_LIN_7269_CAP_FIX              : u16 = 0x013E; // TLE7269 - fixed transceiver on VN1611
    pub const XL_TRANSCEIVER_TYPE_PB_CAN_1051_CAP               : u16 = 0x013F; // TJA 1051, capacitive isolated
    pub const XL_TRANSCEIVER_TYPE_PB_CAN_SWC_7356_CAP           : u16 = 0x0140; // Single Wire NCV7356, capacitive isolated
    pub const XL_TRANSCEIVER_TYPE_PB_CAN_1055_CAP               : u16 = 0x0141; // TJA1055, capacitive isolated, with selectable termination resistor (via 4th IO line)
    pub const XL_TRANSCEIVER_TYPE_PB_CAN_1057_CAP               : u16 = 0x0142; // TJA 1057, capacitive isolated
    pub const XL_TRANSCEIVER_TYPE_A429_HOLT8596_FIX             : u16 = 0x0143; // Onboard HOLT 8596 TX transceiver on VN0601
    pub const XL_TRANSCEIVER_TYPE_A429_HOLT8455_FIX             : u16 = 0x0144; // Onboard HOLT 8455 RX transceiver on VN0601
    pub const XL_TRANSCEIVER_TYPE_PB_CAN_1051HG_CAP             : u16 = 0x0145; // TJA 1051HG, capacitive isolated
    pub const XL_TRANSCEIVER_TYPE_CAN_1057_FIX                  : u16 = 0x0146; // TJA 1057 - fixed transceiver on e.g. VN1530, VN4610
    pub const XL_TRANSCEIVER_TYPE_LIN_7269_FIX                  : u16 = 0x0147; // TLE7269 - fixed transceiver on VN1531
    pub const XL_TRANSCEIVER_TYPE_PB_CAN_1462BT                 : u16 = 0x0149;

    // FlexRay PiggyBacks
    pub const XL_TRANSCEIVER_TYPE_PB_FR_1080                    : u16 = 0x0201u16; // TJA 1080
    pub const XL_TRANSCEIVER_TYPE_PB_FR_1080_MAG                : u16 = 0x0202u16; // TJA 1080 magnetically isolated piggy
    pub const XL_TRANSCEIVER_TYPE_PB_FR_1080A_MAG               : u16 = 0x0203u16; // TJA 1080A magnetically isolated piggy
    pub const XL_TRANSCEIVER_TYPE_PB_FR_1082_CAP                : u16 = 0x0204u16; // TJA 1082 capacitive isolated piggy
    pub const XL_TRANSCEIVER_TYPE_PB_FRC_1082_CAP               : u16 = 0x0205u16; // TJA 1082 capacitive isolated piggy with CANpiggy form factor
    pub const XL_TRANSCEIVER_TYPE_FR_1082_CAP_FIX               : u16 = 0x0206u16; // TJA 1082 capacitive isolated piggy fixed transceiver - e.g. 7610

    pub const XL_TRANSCEIVER_TYPE_MOST150_ONBOARD               : u16 = 0x0220u16; // Onboard MOST150 transceiver of VN2640

    // Ethernet Phys
    pub const XL_TRANSCEIVER_TYPE_ETH_BCM54810_FIX              : u16 = 0x0230u16; // Onboard Broadcom Ethernet PHY on VN5610 and VX0312
    pub const XL_TRANSCEIVER_TYPE_ETH_AR8031_FIX                : u16 = 0x0231u16; // Onboard Atheros Ethernet PHY
    pub const XL_TRANSCEIVER_TYPE_ETH_BCM89810_FIX              : u16 = 0x0232u16; // Onboard Broadcom Ethernet PHY
    pub const XL_TRANSCEIVER_TYPE_ETH_TJA1100_FIX               : u16 = 0x0233u16; // Onboard NXP Ethernet PHY
    pub const XL_TRANSCEIVER_TYPE_ETH_BCM54810_89811_FIX        : u16 = 0x0234u16; // Onboard Broadcom Ethernet PHYs (e.g. VN5610A - BCM54810: RJ45, BCM89811: DSUB)
    pub const XL_TRANSCEIVER_TYPE_ETH_DP83XG710Q1_FIX           : u16 = 0x0235u16; // Onboard TI 1000BASE-T1 Eth PHY
    pub const XL_TRANSCEIVER_TYPE_ETH_BCM54811S_FIX             : u16 = 0x0236u16; // Onboard Broadcom Ethernet PHY on VN7640 and VH6501
    pub const XL_TRANSCEIVER_TYPE_ETH_RTL9000AA_FIX             : u16 = 0x0237u16; // Onboard Realtek Eth PHY
    pub const XL_TRANSCEIVER_TYPE_ETH_BCM89811_FIX              : u16 = 0x0238u16; // Onboard Broadcom Ethernet PHY
    pub const XL_TRANSCEIVER_TYPE_ETH_BCM54210_FIX              : u16 = 0x0239u16; // Onboard Broadcom BCM54210
    pub const XL_TRANSCEIVER_TYPE_ETH_88Q2112_FIX               : u16 = 0x023Au16; // Onboard Marvell 88Q2112
    pub const XL_TRANSCEIVER_TYPE_ETH_BCM84891_FIX              : u16 = 0x023Bu16; // Onboard Broadcom BCM84891

    // IOpiggy 8642
    pub const XL_TRANSCEIVER_TYPE_PB_DAIO_8642                  : u16 = 0x0280u16; // Iopiggy for VN8900
    pub const XL_TRANSCEIVER_TYPE_DAIO_AL_ONLY                  : u16 = 0x028fu16; // virtual piggy type for activation line only (e.g. VN8810ini)
    pub const XL_TRANSCEIVER_TYPE_DAIO_1021_FIX_WITH_AL         : u16 = 0x0290u16; // On board IO with Activation Line (e.g. VN5640)
    pub const XL_TRANSCEIVER_TYPE_DAIO_AL_WU                    : u16 = 0x0291u16; // virtual piggy type for activation line and WakeUp Line only (e.g. VN5610A/VN5620)
    pub const XL_TRANSCEIVER_TYPE_DAIO_1021_FIX_WITH_5V         : u16 = 0x0292u16; // On board IO with 2nd output (e.g. 5V CMOS @ VN4610)

    // Eth modules
    pub const XL_TRANSCEIVER_TYPE_ETH_MOD_BR_BCM89810           : u16 = 0x0300u16; // BroadR-Reach Module with 2x Broadcom BCM89810
    pub const XL_TRANSCEIVER_TYPE_ETH_MOD_IEEE_RGMII_AR8031     : u16 = 0x0301u16; // IEEE802.3 RGMII Module with 2x Atheros AR8031
    pub const XL_TRANSCEIVER_TYPE_ETH_MOD_IEEE_SGMII_AR8031     : u16 = 0x0302u16; // IEEE802.3 SGMII Module with 2x Atheros AR8031
    pub const XL_TRANSCEIVER_TYPE_ETH_MOD_BR_TJA1100            : u16 = 0x0303u16; // BroadR-Reach Module with 2x NXP TJA1100
    pub const XL_TRANSCEIVER_TYPE_ETH_MOD_BR_RTL9000AA          : u16 = 0x0304u16; // BroadR-Reach Module with 2x Realtek RTL9000-AA
    pub const XL_TRANSCEIVER_TYPE_ETH_MOD_BR_SGMII_DP83XG710Q1  : u16 = 0x0305u16; // 1Gbit (1000BASE-T1) SGMII Module with 2x TI DP83GX710-Q1
    pub const XL_TRANSCEIVER_TYPE_ETH_MOD_BR_88Q2112            : u16 = 0x0306u16; // BroadR-Reach Module with 2x Marvell 88Q2112
    pub const XL_TRANSCEIVER_TYPE_ETH_MOD_BR_BCM89811           : u16 = 0x0307u16; // BroadR-Reach Module with 2x Broadcom BCM89811
    pub const XL_TRANSCEIVER_TYPE_ETH_MOD_BR_TJA1101            : u16 = 0x0308u16; // 100BASE-T1 Module with 2x NXP TJA1101

    // AE modules
    pub const XL_TRANSCEIVER_TYPE_AE_MOD_BR_BCM89883            : u16 = 0x0401u16; // 100/1000BASE-T1 Module with 4x Broadcom BCM89883

    // VT Ethernet piggy
    pub const XL_TRANSCEIVER_TYPE_PB_ETH_100BASET1_TJA1101      : u16 = 0x1F82u16; // 100BASE-T1 piggy with 6x NXP TJA1101
    pub const XL_TRANSCEIVER_TYPE_PB_ETH_1000BASET1_88Q2112     : u16 = 0x1F83u16; // 1000BASE-T1 piggy with 6x Marvell 88Q2112

    //------------------------------------------------------------------------------
    // Transceiver Operation Modes
    //------------------------------------------------------------------------------
    pub const XL_TRANSCEIVER_LINEMODE_NA            : u32 = 0x0000u32;
    pub const XL_TRANSCEIVER_LINEMODE_TWO_LINE      : u32 = 0x0001u32;
    pub const XL_TRANSCEIVER_LINEMODE_CAN_H         : u32 = 0x0002u32;
    pub const XL_TRANSCEIVER_LINEMODE_CAN_L         : u32 = 0x0003u32;
    pub const XL_TRANSCEIVER_LINEMODE_SWC_SLEEP     : u32 = 0x0004u32; // SWC Sleep Mode.
    pub const XL_TRANSCEIVER_LINEMODE_SWC_NORMAL    : u32 = 0x0005u32; // SWC Normal Mode.
    pub const XL_TRANSCEIVER_LINEMODE_SWC_FAST      : u32 = 0x0006u32; // SWC High-Speed Mode.
    pub const XL_TRANSCEIVER_LINEMODE_SWC_WAKEUP    : u32 = 0x0007u32; // SWC Wakeup Mode.
    pub const XL_TRANSCEIVER_LINEMODE_SLEEP         : u32 = 0x0008u32;
    pub const XL_TRANSCEIVER_LINEMODE_NORMAL        : u32 = 0x0009u32;
    pub const XL_TRANSCEIVER_LINEMODE_STDBY         : u32 = 0x000au32; // Standby for those who support it
    pub const XL_TRANSCEIVER_LINEMODE_TT_CAN_H      : u32 = 0x000bu32; // truck & trailer: operating mode single wire using CAN high
    pub const XL_TRANSCEIVER_LINEMODE_TT_CAN_L      : u32 = 0x000cu32; // truck & trailer: operating mode single wire using CAN low
    pub const XL_TRANSCEIVER_LINEMODE_EVA_00        : u32 = 0x000du32; // CANcab Eva
    pub const XL_TRANSCEIVER_LINEMODE_EVA_01        : u32 = 0x000eu32; // CANcab Eva
    pub const XL_TRANSCEIVER_LINEMODE_EVA_10        : u32 = 0x000fu32; // CANcab Eva
    pub const XL_TRANSCEIVER_LINEMODE_EVA_11        : u32 = 0x0010u32; // CANcab Eva

    //------------------------------------------------------------------------------
    // Transceiver Status Flags
    //------------------------------------------------------------------------------
    // (not all used, but for compatibility reasons)
    pub const XL_TRANSCEIVER_STATUS_PRESENT         : u32 = 0x0001u32;
    pub const XL_TRANSCEIVER_STATUS_POWER_GOOD      : u32 = 0x0010u32;
    pub const XL_TRANSCEIVER_STATUS_EXT_POWER_GOOD  : u32 = 0x0020u32;
    pub const XL_TRANSCEIVER_STATUS_NOT_SUPPORTED   : u32 = 0x0040u32;

    ////////////////////////////////////////////////////////////////////////////////
    // driver status
    pub const XL_SUCCESS                                : i16 = 0i16; //=0x0000
    pub const XL_PENDING                                : i16 = 1i16; //=0x0001

    pub const XL_ERR_QUEUE_IS_EMPTY                     : i16 = 10i16; //=0x000A
    pub const XL_ERR_QUEUE_IS_FULL                      : i16 = 11i16; //=0x000B
    pub const XL_ERR_TX_NOT_POSSIBLE                    : i16 = 12i16; //=0x000C
    pub const XL_ERR_NO_LICENSE                         : i16 = 14i16; //=0x000E
    pub const XL_ERR_WRONG_PARAMETER                    : i16 = 101i16; //=0x0065
    pub const XL_ERR_TWICE_REGISTER                     : i16 = 110i16; //=0x006E
    pub const XL_ERR_INVALID_CHAN_INDEX                 : i16 = 111i16; //=0x006F
    pub const XL_ERR_INVALID_ACCESS                     : i16 = 112i16; //=0x0070
    pub const XL_ERR_PORT_IS_OFFLINE                    : i16 = 113i16; //=0x0071
    pub const XL_ERR_CHAN_IS_ONLINE                     : i16 = 116i16; //=0x0074
    pub const XL_ERR_NOT_IMPLEMENTED                    : i16 = 117i16; //=0x0075
    pub const XL_ERR_INVALID_PORT                       : i16 = 118i16; //=0x0076
    pub const XL_ERR_HW_NOT_READY                       : i16 = 120i16; //=0x0078
    pub const XL_ERR_CMD_TIMEOUT                        : i16 = 121i16; //=0x0079
    pub const XL_ERR_CMD_HANDLING                       : i16 = 122i16; //=0x007A
    pub const XL_ERR_HW_NOT_PRESENT                     : i16 = 129i16; //=0x0081
    pub const XL_ERR_NOTIFY_ALREADY_ACTIVE              : i16 = 131i16; //=0x0083
    pub const XL_ERR_INVALID_TAG                        : i16 = 132i16; //=0x0084
    pub const XL_ERR_INVALID_RESERVED_FLD               : i16 = 133i16; //=0x0085
    pub const XL_ERR_INVALID_SIZE                       : i16 = 134i16; //=0x0086
    pub const XL_ERR_INSUFFICIENT_BUFFER                : i16 = 135i16; //=0x0087
    pub const XL_ERR_ERROR_CRC                          : i16 = 136i16; //=0x0088
    pub const XL_ERR_BAD_EXE_FORMAT                     : i16 = 137i16; //=0x0089
    pub const XL_ERR_NO_SYSTEM_RESOURCES                : i16 = 138i16; //=0x008A
    pub const XL_ERR_NOT_FOUND                          : i16 = 139i16; //=0x008B
    pub const XL_ERR_INVALID_ADDRESS                    : i16 = 140i16; //=0x008C
    pub const XL_ERR_REQ_NOT_ACCEP                      : i16 = 141i16; //=0x008D
    pub const XL_ERR_INVALID_LEVEL                      : i16 = 142i16; //=0x008E
    pub const XL_ERR_NO_DATA_DETECTED                   : i16 = 143i16; //=0x008F
    pub const XL_ERR_INTERNAL_ERROR                     : i16 = 144i16; //=0x0090
    pub const XL_ERR_UNEXP_NET_ERR                      : i16 = 145i16; //=0x0091
    pub const XL_ERR_INVALID_USER_BUFFER                : i16 = 146i16; //=0x0092
    pub const XL_ERR_INVALID_PORT_ACCESS_TYPE           : i16 = 147i16; //=0x0093
    pub const XL_ERR_NO_RESOURCES                       : i16 = 152i16; //=0x0098
    pub const XL_ERR_WRONG_CHIP_TYPE                    : i16 = 153i16; //=0x0099
    pub const XL_ERR_WRONG_COMMAND                      : i16 = 154i16; //=0x009A
    pub const XL_ERR_INVALID_HANDLE                     : i16 = 155i16; //=0x009B
    pub const XL_ERR_RESERVED_NOT_ZERO                  : i16 = 157i16; //=0x009D
    pub const XL_ERR_INIT_ACCESS_MISSING                : i16 = 158i16; //=0x009E
    pub const XL_ERR_WRONG_VERSION                      : i16 = 160i16; //=0x00A0
    pub const XL_ERR_CANNOT_OPEN_DRIVER                 : i16 = 201i16; //=0x00C9
    pub const XL_ERR_WRONG_BUS_TYPE                     : i16 = 202i16; //=0x00CA
    pub const XL_ERR_DLL_NOT_FOUND                      : i16 = 203i16; //=0x00CB
    pub const XL_ERR_INVALID_CHANNEL_MASK               : i16 = 204i16; //=0x00CC
    pub const XL_ERR_NOT_SUPPORTED                      : i16 = 205i16; //=0x00CD
    // special stream defines
    pub const XL_ERR_CONNECTION_BROKEN                  : i16 = 210i16; //=0x00D2
    pub const XL_ERR_CONNECTION_CLOSED                  : i16 = 211i16; //=0x00D3
    pub const XL_ERR_INVALID_STREAM_NAME                : i16 = 212i16; //=0x00D4
    pub const XL_ERR_CONNECTION_FAILED                  : i16 = 213i16; //=0x00D5
    pub const XL_ERR_STREAM_NOT_FOUND                   : i16 = 214i16; //=0x00D6
    pub const XL_ERR_STREAM_NOT_CONNECTED               : i16 = 215i16; //=0x00D7
    pub const XL_ERR_QUEUE_OVERRUN                      : i16 = 216i16; //=0x00D8
    pub const XL_ERROR                                  : i16 = 255i16; //=0x00FF

    ////////////////////////////////////////////////////////////////////////////////
    // Extended error codes
    pub const XL_ERR_PDU_OUT_OF_MEMORY                  : i16 = 0x0104i16; // Too many PDUs configured or too less system memory free
    pub const XL_ERR_FR_CLUSTERCONFIG_MISSING           : i16 = 0x0105i16; // No cluster configuration has been sent to the driver but is needed for the command which failed
    pub const XL_ERR_PDU_OFFSET_REPET_INVALID           : i16 = 0x0106i16; // Invalid offset and/or repetition value specified
    pub const XL_ERR_PDU_PAYLOAD_SIZE_INVALID           : i16 = 0x0107i16; // Specified PDU payload size is invalid (e.g. size is too large) Frame-API: size is different than static payload length configured in cluster config
    pub const XL_ERR_FR_NBR_FRAMES_OVERFLOW             : i16 = 0x0109i16; // Too many frames specified in parameter
    pub const XL_ERR_FR_SLOT_ID_INVALID                 : i16 = 0x010Bi16; // Specified slot-ID exceeds biggest possible ID specified by the cluster configuration
    pub const XL_ERR_FR_SLOT_ALREADY_OCCUPIED_BY_ERAY   : i16 = 0x010Ci16; // Specified slot cannot be used by Coldstart-Controller because it's already in use by the eRay
    pub const XL_ERR_FR_SLOT_ALREADY_OCCUPIED_BY_COLDC  : i16 = 0x010Di16; // Specified slot cannot be used by eRay because it's already in use by the Coldstart-Controller
    pub const XL_ERR_FR_SLOT_OCCUPIED_BY_OTHER_APP      : i16 = 0x010Ei16; // Specified slot cannot be used because it's already in use by another application
    pub const XL_ERR_FR_SLOT_IN_WRONG_SEGMENT           : i16 = 0x010Fi16; // Specified slot is not in correct segment. E.g.: A dynamic slot was specified for startup&sync
    pub const XL_ERR_FR_FRAME_CYCLE_MULTIPLEX_ERROR     : i16 = 0x0110i16; // The given frame-multiplexing rule (specified by offset and repetition) cannot be done because some of the slots are already in use
    pub const XL_ERR_PDU_NO_UNMAP_OF_SYNCFRAME          : i16 = 0x0116i16; // Unmapping of eRay startup/sync frames is not allowed
    pub const XL_ERR_SYNC_FRAME_MODE                    : i16 = 0x0123i16; // Wrong txMode in sync frame
    pub const XL_ERR_INVALID_DLC                        : i16 = 0x0201i16; // DLC with invalid value
    pub const XL_ERR_INVALID_CANID                      : i16 = 0x0202i16; // CAN Id has invalid bits set
    pub const XL_ERR_INVALID_FDFLAG_MODE20              : i16 = 0x0203i16; // flag set that must not be set when configured for CAN20 (e.g. EDL)
    pub const XL_ERR_EDL_RTR                            : i16 = 0x0204i16; // RTR must not be set in combination with EDL
    pub const XL_ERR_EDL_NOT_SET                        : i16 = 0x0205i16; // EDL is not set but BRS and/or ESICTRL is
    pub const XL_ERR_UNKNOWN_FLAG                       : i16 = 0x0206i16; // unknown bit in flags field is set
    ///////////////////////////////////////////////////////////////////////////////
    // Ethernet API error code (range: 0x1100..0x11FF)
    pub const XL_ERR_ETH_PHY_ACTIVATION_FAILED          : i16 = 0x1100i16;
    pub const XL_ERR_ETH_PHY_CONFIG_ABORTED             : i16 = 0x1103i16;
    pub const XL_ERR_ETH_RESET_FAILED                   : i16 = 0x1104i16;
    pub const XL_ERR_ETH_SET_CONFIG_DELAYED             : i16 = 0x1105i16; //Requested config was stored but could not be immediately activated
    pub const XL_ERR_ETH_UNSUPPORTED_FEATURE            : i16 = 0x1106i16; //Requested feature/function not supported by device
    pub const XL_ERR_ETH_MAC_ACTIVATION_FAILED          : i16 = 0x1107i16;
    pub const XL_ERR_NET_ETH_SWITCH_IS_ONLINE           : i16 = 0x110Ci16; //Switch has already been activated

    // enum e_XLevent_type
    pub const XL_NO_COMMAND                 : u8 = 0u8;
    pub const XL_RECEIVE_MSG                : u8 = 1u8;
    pub const XL_CHIP_STATE                 : u8 = 4u8;
    pub const XL_TRANSCEIVER                : u8 = 6u8;
    pub const XL_TIMER                      : u8 = 8u8;
    pub const XL_TRANSMIT_MSG               : u8 = 10u8;
    pub const XL_SYNC_PULSE                 : u8 = 11u8;
    pub const XL_APPLICATION_NOTIFICATION   : u8 = 15u8;

    //for LIN we have special events
    pub const XL_LIN_MSG                    : u8 = 20u8;
    pub const XL_LIN_ERRMSG                 : u8 = 21u8;
    pub const XL_LIN_SYNCERR                : u8 = 22u8;
    pub const XL_LIN_NOANS                  : u8 = 23u8;
    pub const XL_LIN_WAKEUP                 : u8 = 24u8;
    pub const XL_LIN_SLEEP                  : u8 = 25u8;
    pub const XL_LIN_CRCINFO                : u8 = 26u8;

    // for D/A IO bus
    pub const XL_RECEIVE_DAIO_DATA          : u8 = 32u8; // D/A IO data message

    pub const XL_RECEIVE_DAIO_PIGGY         : u8 = 34u8; // D/A IO Piggy data message
    pub const XL_KLINE_MSG                  : u8 = 36u8;

    // };

    //
    // common event tags
    //
    // pub const XL_RECEIVE_MSG              : u16 = 0x0001u16; // equals to e_XLevent_type's member
    // pub const XL_CHIP_STATE               : u16 = 0x0004u16; // equals to e_XLevent_type's member
    pub const XL_TRANSCEIVER_INFO            : u16 = 0x0006u16;
    // pub const XL_TRANSCEIVER              : u16 = XL_TRANSCEIVER_INFO; // equals to e_XLevent_type's member
    pub const XL_TIMER_EVENT                 : u16 = 0x0008u16;
    // pub const XL_TIMER                    : u16 = XL_TIMER_EVENT; // equals to e_XLevent_type's member
    // pub const XL_TRANSMIT_MSG             : u16 = 0x000Au16; // equals to e_XLevent_type's member
    // pub const XL_SYNC_PULSE               : u16 = 0x000Bu16; // equals to e_XLevent_type's member
    // pub const XL_APPLICATION_NOTIFICATION : u16 = 0x000Fu16; // equals to e_XLevent_type's member

    //
    // LIN event tags
    //
    pub const LIN_MSG       : u16 = 0x0014u16;
    pub const LIN_ERRMSG    : u16 = 0x0015u16;
    pub const LIN_SYNCERR   : u16 = 0x0016u16;
    pub const LIN_NOANS     : u16 = 0x0017u16;
    pub const LIN_WAKEUP    : u16 = 0x0018u16;
    pub const LIN_SLEEP     : u16 = 0x0019u16;
    pub const LIN_CRCINFO   : u16 = 0x001Au16;

    //
    // DAIO event tags
    //
    pub const RECEIVE_DAIO_DATA: u16 = 0x0020u16; // D/A IO data message

    pub const KLINE_MSG: u16 = 0x0024u16;

    //
    // FlexRay event tags
    //
    pub const XL_FR_START_CYCLE                 : u16 = 0x0080u16;
    pub const XL_FR_RX_FRAME                    : u16 = 0x0081u16;
    pub const XL_FR_TX_FRAME                    : u16 = 0x0082u16;
    pub const XL_FR_TXACK_FRAME                 : u16 = 0x0083u16;
    pub const XL_FR_INVALID_FRAME               : u16 = 0x0084u16;
    pub const XL_FR_WAKEUP                      : u16 = 0x0085u16;
    pub const XL_FR_SYMBOL_WINDOW               : u16 = 0x0086u16;
    pub const XL_FR_ERROR                       : u16 = 0x0087u16;
    pub const XL_FR_ERROR_POC_MODE              : u8 = 0x01u8;
    pub const XL_FR_ERROR_SYNC_FRAMES_BELOWMIN  : u8 = 0x02u8;
    pub const XL_FR_ERROR_SYNC_FRAMES_OVERLOAD  : u8 = 0x03u8;
    pub const XL_FR_ERROR_CLOCK_CORR_FAILURE    : u8 = 0x04u8;
    pub const XL_FR_ERROR_NIT_FAILURE           : u8 = 0x05u8;
    pub const XL_FR_ERROR_CC_ERROR              : u8 = 0x06u8;
    pub const XL_FR_STATUS                      : u16 = 0x0088u16;
    pub const XL_FR_NM_VECTOR                   : u16 = 0x008Au16;
    pub const XL_FR_TRANCEIVER_STATUS           : u16 = 0x008Bu16;
    pub const XL_FR_SPY_FRAME                   : u16 = 0x008Eu16;
    pub const XL_FR_SPY_SYMBOL                  : u16 = 0x008Fu16;

    //
    // CAPL-On-Board event tags
    //

    //
    // MOST25 event tags
    //
    pub const XL_MOST_START                 : u16 = 0x0101u16;
    pub const XL_MOST_STOP                  : u16 = 0x0102u16;
    pub const XL_MOST_EVENTSOURCES          : u16 = 0x0103u16;
    pub const XL_MOST_ALLBYPASS             : u16 = 0x0107u16;
    pub const XL_MOST_TIMINGMODE            : u16 = 0x0108u16;
    pub const XL_MOST_FREQUENCY             : u16 = 0x0109u16;
    pub const XL_MOST_REGISTER_BYTES        : u16 = 0x010au16;
    pub const XL_MOST_REGISTER_BITS         : u16 = 0x010bu16;
    pub const XL_MOST_SPECIAL_REGISTER      : u16 = 0x010cu16;
    pub const XL_MOST_CTRL_RX_SPY           : u16 = 0x010du16;
    pub const XL_MOST_CTRL_RX_OS8104        : u16 = 0x010eu16;
    pub const XL_MOST_CTRL_TX               : u16 = 0x010fu16;
    pub const XL_MOST_ASYNC_MSG             : u16 = 0x0110u16;
    pub const XL_MOST_ASYNC_TX              : u16 = 0x0111u16;
    pub const XL_MOST_SYNC_ALLOCTABLE       : u16 = 0x0112u16;
    pub const XL_MOST_SYNC_VOLUME_STATUS    : u16 = 0x0116u16;
    pub const XL_MOST_RXLIGHT               : u16 = 0x0117u16;
    pub const XL_MOST_TXLIGHT               : u16 = 0x0118u16;
    pub const XL_MOST_LOCKSTATUS            : u16 = 0x0119u16;
    pub const XL_MOST_ERROR                 : u16 = 0x011au16;
    pub const XL_MOST_CTRL_RXBUFFER         : u16 = 0x011cu16;
    pub const XL_MOST_SYNC_TX_UNDERFLOW     : u16 = 0x011du16;
    pub const XL_MOST_SYNC_RX_OVERFLOW      : u16 = 0x011eu16;
    pub const XL_MOST_CTRL_SYNC_AUDIO       : u16 = 0x011fu16;
    pub const XL_MOST_SYNC_MUTE_STATUS      : u16 = 0x0120u16;
    pub const XL_MOST_GENLIGHTERROR         : u16 = 0x0121u16;
    pub const XL_MOST_GENLOCKERROR          : u16 = 0x0122u16;
    pub const XL_MOST_TXLIGHT_POWER         : u16 = 0x0123u16;
    pub const XL_MOST_CTRL_BUSLOAD          : u16 = 0x0126u16;
    pub const XL_MOST_ASYNC_BUSLOAD         : u16 = 0x0127u16;
    pub const XL_MOST_CTRL_SYNC_AUDIO_EX    : u16 = 0x012au16;
    pub const XL_MOST_TIMINGMODE_SPDIF      : u16 = 0x012bu16;
    pub const XL_MOST_STREAM_STATE          : u16 = 0x012cu16;
    pub const XL_MOST_STREAM_BUFFER         : u16 = 0x012du16;

    //
    // MOST150 event tags
    //
    pub const XL_START                          : u16 = 0x0200u16;
    pub const XL_STOP                           : u16 = 0x0201u16;
    pub const XL_MOST150_EVENT_SOURCE           : u16 = 0x0203u16;
    pub const XL_MOST150_DEVICE_MODE            : u16 = 0x0204u16;
    pub const XL_MOST150_SYNC_ALLOC_INFO        : u16 = 0x0205u16;
    pub const XL_MOST150_FREQUENCY              : u16 = 0x0206u16;
    pub const XL_MOST150_SPECIAL_NODE_INFO      : u16 = 0x0207u16;
    pub const XL_MOST150_CTRL_RX                : u16 = 0x0208u16;
    pub const XL_MOST150_CTRL_TX_ACK            : u16 = 0x0209u16;
    pub const XL_MOST150_ASYNC_SPY              : u16 = 0x020Au16;
    pub const XL_MOST150_ASYNC_RX               : u16 = 0x020Bu16;
    pub const XL_MOST150_SYNC_VOLUME_STATUS     : u16 = 0x020Du16;
    pub const XL_MOST150_TX_LIGHT               : u16 = 0x020Eu16;
    pub const XL_MOST150_RXLIGHT_LOCKSTATUS     : u16 = 0x020Fu16;
    pub const XL_MOST150_ERROR                  : u16 = 0x0210u16;
    pub const XL_MOST150_CONFIGURE_RX_BUFFER    : u16 = 0x0211u16;
    pub const XL_MOST150_CTRL_SYNC_AUDIO        : u16 = 0x0212u16;
    pub const XL_MOST150_SYNC_MUTE_STATUS       : u16 = 0x0213u16;
    pub const XL_MOST150_LIGHT_POWER            : u16 = 0x0214u16;
    pub const XL_MOST150_GEN_LIGHT_ERROR        : u16 = 0x0215u16;
    pub const XL_MOST150_GEN_LOCK_ERROR         : u16 = 0x0216u16;
    pub const XL_MOST150_CTRL_BUSLOAD           : u16 = 0x0217u16;
    pub const XL_MOST150_ASYNC_BUSLOAD          : u16 = 0x0218u16;
    pub const XL_MOST150_ETHERNET_RX            : u16 = 0x0219u16;
    pub const XL_MOST150_SYSTEMLOCK_FLAG        : u16 = 0x021Au16;
    pub const XL_MOST150_SHUTDOWN_FLAG          : u16 = 0x021Bu16;
    pub const XL_MOST150_CTRL_SPY               : u16 = 0x021Cu16;
    pub const XL_MOST150_ASYNC_TX_ACK           : u16 = 0x021Du16;
    pub const XL_MOST150_ETHERNET_SPY           : u16 = 0x021Eu16;
    pub const XL_MOST150_ETHERNET_TX_ACK        : u16 = 0x021Fu16;
    pub const XL_MOST150_SPDIFMODE              : u16 = 0x0220u16;
    pub const XL_MOST150_ECL_LINE_CHANGED       : u16 = 0x0222u16;
    pub const XL_MOST150_ECL_TERMINATION_CHANGED: u16 = 0x0223u16;
    pub const XL_MOST150_NW_STARTUP             : u16 = 0x0224u16;
    pub const XL_MOST150_NW_SHUTDOWN            : u16 = 0x0225u16;
    pub const XL_MOST150_STREAM_STATE           : u16 = 0x0226u16;
    pub const XL_MOST150_STREAM_TX_BUFFER       : u16 = 0x0227u16;
    pub const XL_MOST150_STREAM_RX_BUFFER       : u16 = 0x0228u16;
    pub const XL_MOST150_STREAM_TX_LABEL        : u16 = 0x0229u16;
    pub const XL_MOST150_STREAM_TX_UNDERFLOW    : u16 = 0x022Bu16;
    pub const XL_MOST150_GEN_BYPASS_STRESS      : u16 = 0x022Cu16;
    pub const XL_MOST150_ECL_SEQUENCE           : u16 = 0x022Du16;
    pub const XL_MOST150_ECL_GLITCH_FILTER      : u16 = 0x022Eu16;
    pub const XL_MOST150_SSO_RESULT             : u16 = 0x022Fu16;

    //
    // CAN/CAN-FD event tags
    // Rx
    pub const XL_CAN_EV_TAG_RX_OK       : u16 = 0x0400u16;
    pub const XL_CAN_EV_TAG_RX_ERROR    : u16 = 0x0401u16;
    pub const XL_CAN_EV_TAG_TX_ERROR    : u16 = 0x0402u16;
    pub const XL_CAN_EV_TAG_TX_REQUEST  : u16 = 0x0403u16;
    pub const XL_CAN_EV_TAG_TX_OK       : u16 = 0x0404u16;
    pub const XL_CAN_EV_TAG_CHIP_STATE  : u16 = 0x0409u16;

    // CAN/CAN-FD event tags
    // Tx
    pub const XL_CAN_EV_TAG_TX_MSG      : u16 = 0x0440u16;
    //
    // Ethernet event tags
    //
    pub const XL_ETH_EVENT_TAG_FRAMERX                  : u16 = 0x0500u16; // Event data type T_XL_ETH_DATAFRAME_RX
    pub const XL_ETH_EVENT_TAG_FRAMERX_ERROR            : u16 = 0x0501u16; // Event data type T_XL_ETH_DATAFRAME_RX_ERROR
    pub const XL_ETH_EVENT_TAG_FRAMETX_ERROR            : u16 = 0x0506u16; // Event data type T_XL_ETH_DATAFRAME_TX_ERROR
    pub const XL_ETH_EVENT_TAG_FRAMETX_ERROR_SWITCH     : u16 = 0x0507u16; // Event data type T_XL_ETH_DATAFRAME_TX_ERR_SW
    pub const XL_ETH_EVENT_TAG_FRAMETX_ACK              : u16 = 0x0510u16; // Event data type T_XL_ETH_DATAFRAME_TXACK
    pub const XL_ETH_EVENT_TAG_FRAMETX_ACK_SWITCH       : u16 = 0x0511u16; // Event data type T_XL_ETH_DATAFRAME_TXACK_SW
    pub const XL_ETH_EVENT_TAG_FRAMETX_ACK_OTHER_APP    : u16 = 0x0513u16; // Event data type T_XL_ETH_DATAFRAME_TXACK_OTHERAPP
    pub const XL_ETH_EVENT_TAG_FRAMETX_ERROR_OTHER_APP  : u16 = 0x0514u16; // Event data type T_XL_ETH_DATAFRAME_TX_ERR_OTHERAPP
    pub const XL_ETH_EVENT_TAG_CHANNEL_STATUS           : u16 = 0x0520u16; // Event data type T_XL_ETH_CHANNEL_STATUS
    pub const XL_ETH_EVENT_TAG_CONFIGRESULT             : u16 = 0x0530u16; // Event data type T_XL_ETH_CONFIG_RESULT
    pub const XL_ETH_EVENT_TAG_FRAMERX_SIMULATION       : u16 = 0x0550u16; // Event data type T_XL_ETH_DATAFRAME_RX_SIMULATION  (with payload)
    pub const XL_ETH_EVENT_TAG_FRAMERX_ERROR_SIMULATION : u16 = 0x0551u16; // Event data type T_XL_ETH_DATAFRAME_RX_ERROR_SIMULATION (with payload)
    pub const XL_ETH_EVENT_TAG_FRAMETX_ACK_SIMULATION   : u16 = 0x0552u16; // Event data type T_XL_ETH_DATAFRAME_TX_SIMULATION (with payload)
    pub const XL_ETH_EVENT_TAG_FRAMETX_ERROR_SIMULATION : u16 = 0x0553u16; // Event data type T_XL_ETH_DATAFRAME_TX_ERROR_SIMULATION (with payload)

    pub const XL_ETH_EVENT_TAG_FRAMERX_MEASUREMENT      : u16 = 0x0560u16; // Event data type T_XL_ETH_DATAFRAME_RX_MEASUREMENT  (with payload)
    pub const XL_ETH_EVENT_TAG_FRAMERX_ERROR_MEASUREMENT: u16 = 0x0561u16; // Event data type T_XL_ETH_DATAFRAME_RX_ERROR_MEASUREMENT (with payload)
    pub const XL_ETH_EVENT_TAG_FRAMETX_MEASUREMENT      : u16 = 0x0562u16; // Event data type T_XL_ETH_DATAFRAME_TX_MEASUREMENT (with payload)
    pub const XL_ETH_EVENT_TAG_FRAMETX_ERROR_MEASUREMENT: u16 = 0x0563u16; // Event data type T_XL_ETH_DATAFRAME_TX_ERROR_MEASUREMENT (with payload)
    pub const XL_ETH_EVENT_TAG_LOSTEVENT                : u16 = 0x05feu16; // Indication that one or more intended events could not be generated. Event data type T_XL_ETH_LOSTEVENT
    pub const XL_ETH_EVENT_TAG_ERROR                    : u16 = 0x05ffu16; // Generic error

    //
    // ARINC429 event tags
    //
    pub const XL_A429_EV_TAG_TX_OK          : u16 = 0x0600u16;
    pub const XL_A429_EV_TAG_TX_ERR         : u16 = 0x0601u16;
    pub const XL_A429_EV_TAG_RX_OK          : u16 = 0x0608u16;
    pub const XL_A429_EV_TAG_RX_ERR         : u16 = 0x0609u16;
    pub const XL_A429_EV_TAG_BUS_STATISTIC  : u16 = 0x060Fu16;

    pub type XLuint64 = u64;

    // #ifndef XL_LONG_XL_ULONG_DEFINED
    // #define XL_LONG_XL_ULONG_DEFINED
    pub type XLlong = c_long;
    pub type XLulong = c_ulong;
    // #endif

    // defines for XL_APPLICATION_NOTIFICATION_EV::notifyReason
    pub const XL_NOTIFY_REASON_CHANNEL_ACTIVATION   : u8 = 1u8;
    pub const XL_NOTIFY_REASON_CHANNEL_DEACTIVATION : u8 = 2u8;
    pub const XL_NOTIFY_REASON_PORT_CLOSED          : u8 = 3u8;

    #[repr(C)]
    #[repr(align(8))] // is this right?
    pub struct s_xl_application_notification {
        notifyReason: c_uint,               // XL_NOTIFY_REASON_xxx
        reserved    : [c_uint; 7usize],
    }

    pub type XL_APPLICATION_NOTIFICATION_EV = s_xl_application_notification;

    // defines for XL_SYNC_PULSE_EV::triggerSource and s_xl_sync_pulse::pulseCode
    pub const XL_SYNC_PULSE_EXTERNAL    : u8 = 0x00u8;
    pub const XL_SYNC_PULSE_OUR         : u8 = 0x01u8;
    pub const XL_SYNC_PULSE_OUR_SHARED  : u8 = 0x02u8;

    // definition of the sync pulse event for xl interface versions V3 and higher
    // (XL_INTERFACE_VERSION_V3, XL_INTERFACE_VERSION_V4, ..)
    #[repr(C)]
    #[repr(align(8))] // is this right?
    #[derive(Clone, Copy)]
    pub struct s_xl_sync_pulse_ev {
        triggerSource   : c_uint,   // e.g. external or internal trigger source
        reserved        : c_uint,
        time            : XLuint64, // internally generated timestamp
    }

    pub type XL_SYNC_PULSE_EV = s_xl_sync_pulse_ev;

    // definition of the sync pulse event for xl interface versions V1 and V2
    // (XL_INTERFACE_VERSION_V1, XL_INTERFACE_VERSION_V2)
    #[repr(C)]
    #[repr(packed)] // is this right?
    #[derive(Clone, Copy)]
    struct s_xl_sync_pulse {
        pulseCode   : c_uchar,      // generated by us
        time        : XLuint64,     // 1 ns resolution
    }

    //------------------------------------------------------------------------------
    // defines for the supported hardware
    pub const XL_HWTYPE_NONE                    : u8 = 0u8;
    pub const XL_HWTYPE_VIRTUAL                 : u8 = 1u8;
    pub const XL_HWTYPE_CANCARDX                : u8 = 2u8;
    pub const XL_HWTYPE_CANAC2PCI               : u8 = 6u8;
    pub const XL_HWTYPE_CANCARDY                : u8 = 12u8;
    pub const XL_HWTYPE_CANCARDXL               : u8 = 15u8;
    pub const XL_HWTYPE_CANCASEXL               : u8 = 21u8;
    pub const XL_HWTYPE_CANCASEXL_LOG_OBSOLETE  : u8 = 23u8;
    pub const XL_HWTYPE_CANBOARDXL              : u8 = 25u8; // CANboardXL, CANboardXL PCIe
    pub const XL_HWTYPE_CANBOARDXL_PXI          : u8 = 27u8; // CANboardXL pxi
    pub const XL_HWTYPE_VN2600                  : u8 = 29u8;
    pub const XL_HWTYPE_VN2610                  : u8 = XL_HWTYPE_VN2600;
    pub const XL_HWTYPE_VN3300                  : u8 = 37u8;
    pub const XL_HWTYPE_VN3600                  : u8 = 39u8;
    pub const XL_HWTYPE_VN7600                  : u8 = 41u8;
    pub const XL_HWTYPE_CANCARDXLE              : u8 = 43u8;
    pub const XL_HWTYPE_VN8900                  : u8 = 45u8;
    pub const XL_HWTYPE_VN8950                  : u8 = 47u8;
    pub const XL_HWTYPE_VN2640                  : u8 = 53u8;
    pub const XL_HWTYPE_VN1610                  : u8 = 55u8;
    pub const XL_HWTYPE_VN1630                  : u8 = 57u8;
    pub const XL_HWTYPE_VN1640                  : u8 = 59u8;
    pub const XL_HWTYPE_VN8970                  : u8 = 61u8;
    pub const XL_HWTYPE_VN1611                  : u8 = 63u8;
    pub const XL_HWTYPE_VN5240                  : u8 = 64u8;
    pub const XL_HWTYPE_VN5610                  : u8 = 65u8;
    pub const XL_HWTYPE_VN5620                  : u8 = 66u8;
    pub const XL_HWTYPE_VN7570                  : u8 = 67u8;
    pub const XL_HWTYPE_VN5650                  : u8 = 68u8;
    pub const XL_HWTYPE_IPCLIENT                : u8 = 69u8;
    pub const XL_HWTYPE_IPSERVER                : u8 = 71u8;
    pub const XL_HWTYPE_VX1121                  : u8 = 73u8;
    pub const XL_HWTYPE_VX1131                  : u8 = 75u8;
    pub const XL_HWTYPE_VT6204                  : u8 = 77u8;

    pub const XL_HWTYPE_VN1630_LOG              : u8 = 79u8;
    pub const XL_HWTYPE_VN7610                  : u8 = 81u8;
    pub const XL_HWTYPE_VN7572                  : u8 = 83u8;
    pub const XL_HWTYPE_VN8972                  : u8 = 85u8;
    pub const XL_HWTYPE_VN0601                  : u8 = 87u8;
    pub const XL_HWTYPE_VN5640                  : u8 = 89u8;
    pub const XL_HWTYPE_VX0312                  : u8 = 91u8;
    pub const XL_HWTYPE_VH6501                  : u8 = 94u8;
    pub const XL_HWTYPE_VN8800                  : u8 = 95u8;
    pub const XL_HWTYPE_IPCL8800                : u8 = 96u8;
    pub const XL_HWTYPE_IPSRV8800               : u8 = 97u8;
    pub const XL_HWTYPE_CSMCAN                  : u8 = 98u8;
    pub const XL_HWTYPE_VN5610A                 : u8 = 101u8;
    pub const XL_HWTYPE_VN7640                  : u8 = 102u8;
    pub const XL_HWTYPE_VX1135                  : u8 = 104u8;
    pub const XL_HWTYPE_VN4610                  : u8 = 105u8;
    pub const XL_HWTYPE_VT6306                  : u8 = 107u8;
    pub const XL_HWTYPE_VT6104A                 : u8 = 108u8;
    pub const XL_HWTYPE_VN5430                  : u8 = 109u8;
    pub const XL_HWTYPE_VTSSERVICE              : u8 = 110u8;
    pub const XL_HWTYPE_VN1530                  : u8 = 112u8;
    pub const XL_HWTYPE_VN1531                  : u8 = 113u8;
    pub const XL_HWTYPE_VX1161A                 : u8 = 114u8;
    pub const XL_HWTYPE_VX1161B                 : u8 = 115u8;
    pub const XL_MAX_HWTYPE                     : u8 = 120u8;

    ////////////////////////////////////////////////////////////////////////////////
    pub type XLstringType = *const c_char;

    ////////////////////////////////////////////////////////////////////////////////
    // channel selector
    pub type XLaccess = XLuint64;
    pub const XL_INVALID_CHANNEL_INDEX  : c_uint = 0xFFFFFFFF;
    pub const XL_INVALID_DEVICE_INDEX   : c_uint = 0xFFFFFFFF;

    ////////////////////////////////////////////////////////////////////////////////
    // handle for xlSetNotification
    pub type XLhandle = isize;

    ////////////////////////////////////////////////////////////////////////////////
    // LIN lib
    //------------------------------------------------------------------------------
    // defines for LIN
    //------------------------------------------------------------------------------

    // defines for xlLinSetChannelParams
    pub const XL_LIN_MASTER     : c_uint = 01u32; // channel is a LIN master
    pub const XL_LIN_SLAVE      : c_uint = 02u32; // channel is a LIN slave
    pub const XL_LIN_VERSION_1_3: c_uint = 0x01u32; // LIN version 1.3
    pub const XL_LIN_VERSION_2_0: c_uint = 0x02u32; // LIN version 2.0
    pub const XL_LIN_VERSION_2_1: c_uint = 0x03u32; // LIN version 2.1

    // defines for xlLinSetSlave
    pub const XL_LIN_CALC_CHECKSUM          : c_ushort = 0x100u16; // flag for automatic 'classic' checksum calculation
    pub const XL_LIN_CALC_CHECKSUM_ENHANCED : c_ushort = 0x200u16; // flag for automatic 'enhanced' checksum calculation

    // defines for xlLinSetSleepMode
    pub const XL_LIN_SET_SILENT     : c_uint = 0x01u32; // set hardware into sleep mode
    pub const XL_LIN_SET_WAKEUPID   : c_uint = 0x03u32; // set hardware into sleep mode and send a request at wake-up

    // defines for xlLinSetChecksum. For LIN >= 2.0 there can be used two different Checksum models.
    pub const XL_LIN_CHECKSUM_CLASSIC   : c_uchar = 0x00u8; // Use classic CRC
    pub const XL_LIN_CHECKSUM_ENHANCED  : c_uchar = 0x01u8; // Use enhanced CRC
    pub const XL_LIN_CHECKSUM_UNDEFINED : c_uchar = 0xffu8; // Set the checksum calculation to undefined.

    // defines for the sleep mode event: XL_LIN_SLEEP
    pub const XL_LIN_STAYALIVE              : c_uchar = 0x00u8; // flag if nothing changes
    pub const XL_LIN_SET_SLEEPMODE          : c_uchar = 0x01u8; // flag if the hardware is set into the sleep mode
    pub const XL_LIN_COMESFROM_SLEEPMODE    : c_uchar = 0x02u8; // flag if the hardware comes from the sleep mode

    // defines for the wake up event: XL_LIN_WAKEUP
    pub const XL_LIN_WAKUP_INTERNAL: c_uchar = 0x01u8; // flag to signal a internal WAKEUP (event)

    // defines for xlLINSetDLC
    pub const XL_LIN_UNDEFINED_DLC: c_uchar = 0xffu8; // set the DLC to undefined

    // defines for xlLinSwitchSlave
    pub const XL_LIN_SLAVE_ON   : c_uchar = 0xffu8; // switch on the LIN slave
    pub const XL_LIN_SLAVE_OFF  : c_uchar = 0x00u8; // switch off the LIN slave

    //------------------------------------------------------------------------------
    // structures for LIN
    //------------------------------------------------------------------------------

    #[repr(C)]
    #[repr(packed)]
    pub struct XLlinStatPar {
        pub LINMode: c_uint,    // XL_LIN_SLAVE | XL_LIN_MASTER
        pub baudrate: c_int,    // the baudrate will be calculated within the API. Here: e.g. 9600, 19200
        pub LINVersion: c_uint, // define for the LIN version (actual V1.3 of V2.0)
        reserved: c_uint,       // for future use
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Defines
    //------------------------------------------------------------------------------
    // message flags
    pub const MAX_MSG_LEN: usize = 8usize;

    // interface version for our events
    pub const XL_INTERFACE_VERSION_V2: u32 = 2u32;
    pub const XL_INTERFACE_VERSION_V3: u32 = 3u32;
    pub const XL_INTERFACE_VERSION_V4: u32 = 4u32;
    //current version
    pub const XL_INTERFACE_VERSION: u32 = XL_INTERFACE_VERSION_V3;

    pub const XL_CAN_EXT_MSG_ID             : u32 = 0x80000000u32;

    pub const XL_CAN_MSG_FLAG_ERROR_FRAME   : u16 = 0x01u16;
    pub const XL_CAN_MSG_FLAG_OVERRUN       : u16 = 0x02u16; // Overrun in Driver or CAN Controller,
                                                             // previous msgs have been lost.
    pub const XL_CAN_MSG_FLAG_NERR          : u16 = 0x04u16; // Line Error on Lowspeed
    pub const XL_CAN_MSG_FLAG_WAKEUP        : u16 = 0x08u16; // High Voltage Message on Single Wire CAN
    pub const XL_CAN_MSG_FLAG_REMOTE_FRAME  : u16 = 0x10u16;
    pub const XL_CAN_MSG_FLAG_RESERVED_1    : u16 = 0x20u16;
    pub const XL_CAN_MSG_FLAG_TX_COMPLETED  : u16 = 0x40u16; // Message Transmitted
    pub const XL_CAN_MSG_FLAG_TX_REQUEST    : u16 = 0x80u16; // Transmit Message stored into Controller
    pub const XL_CAN_MSG_FLAG_SRR_BIT_DOM   : u16 = 0x0200u16; // SRR bit in CAN message is dominant

    pub const XL_EVENT_FLAG_OVERRUN         : u8 = 0x01u8; // Used in XLevent.flags

    // LIN flags
    pub const XL_LIN_MSGFLAG_TX             : u16 = XL_CAN_MSG_FLAG_TX_COMPLETED; // LIN TX flag
    pub const XL_LIN_MSGFLAG_CRCERROR       : u16 = 0x81; // Wrong LIN CRC

    //------------------------------------------------------------------------------
    // structure for XL_RECEIVE_MSG, XL_TRANSMIT_MSG
    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy, Debug)]
    pub struct s_xl_can_msg {
        /* 32 Bytes */
        pub id: c_uint,
        pub flags: c_ushort,
        pub dlc: c_ushort,
        res1: XLuint64,
        pub data: [c_uchar; MAX_MSG_LEN],
        res2: XLuint64,
    }

    //------------------------------------------------------------------------------
    // structure for XL_TRANSMIT_DAIO_DATA

    // flags masks
    pub const XL_DAIO_DATA_GET              : c_ushort = 0x8000u16;
    pub const XL_DAIO_DATA_VALUE_DIGITAL    : c_ushort = 0x0001u16;
    pub const XL_DAIO_DATA_VALUE_ANALOG     : c_ushort = 0x0002u16;
    pub const XL_DAIO_DATA_PWM              : c_ushort = 0x0010u16;

    // optional function flags
    pub const XL_DAIO_MODE_PULSE            : c_ushort = 0x0020u16; // generates pulse in values of PWM

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy, Default)]
    struct s_xl_daio_data {
        /* 32 Bytes */
        flags: c_ushort,              // 2
        timestamp_correction: c_uint, // 4
        mask_digital: c_uchar,        // 1
        value_digital: c_uchar,       // 1
        mask_analog: c_uchar,         // 1
        reserved0: c_uchar,           // 1
        value_analog: [c_ushort; 4],  // 8
        pwm_frequency: c_uint,        // 4
        pwm_value: c_ushort,          // 2
        reserved1: c_uint,            // 4
        reserved2: c_uint,            // 4
    }

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    pub struct s_xl_io_digital_data {
        pub digitalInputData: c_uint,
    }

    pub type XL_IO_DIGITAL_DATA = s_xl_io_digital_data;

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    pub struct s_xl_io_analog_data {
        pub measuredAnalogData0: c_uint,
        pub measuredAnalogData1: c_uint,
        pub measuredAnalogData2: c_uint,
        pub measuredAnalogData3: c_uint,
    }

    pub type XL_IO_ANALOG_DATA = s_xl_io_analog_data;

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    union s_xl_daio_piggy_data_inner_data {
        digital: XL_IO_DIGITAL_DATA,
        analog: XL_IO_ANALOG_DATA,
    }

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    struct s_xl_daio_piggy_data {
        daioEvtTag: c_uint,
        triggerType: c_uint,
        data: s_xl_daio_piggy_data_inner_data,
    }

    //------------------------------------------------------------------------------
    // structure for XL_CHIP_STATE

    pub const XL_CHIPSTAT_BUSOFF        : c_uchar = 0x01u8;
    pub const XL_CHIPSTAT_ERROR_PASSIVE : c_uchar = 0x02u8;
    pub const XL_CHIPSTAT_ERROR_WARNING : c_uchar = 0x04u8;
    pub const XL_CHIPSTAT_ERROR_ACTIVE  : c_uchar = 0x08u8;

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    struct s_xl_chip_state {
        busStatus: c_uchar,
        txErrorCounter: c_uchar,
        rxErrorCounter: c_uchar,
    }

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    struct s_xl_transceiver {
        event_reason: c_uchar, // reason for what was event sent
        is_present: c_uchar,   // allways valid transceiver presen ce flag
    }

    //------------------------------------------------------------------------------
    // structure and defines for XL_TRANSCEIVER
    pub const XL_TRANSCEIVER_EVENT_NONE         : u8 = 0u8;
    pub const XL_TRANSCEIVER_EVENT_INSERTED     : u8 = 1u8; // cable was inserted
    pub const XL_TRANSCEIVER_EVENT_REMOVED      : u8 = 2u8; // cable was removed
    pub const XL_TRANSCEIVER_EVENT_STATE_CHANGE : u8 = 3u8; // transceiver state changed

    //------------------------------------------------------------------------------
    // defines for SET_OUTPUT_MODE
    pub const XL_OUTPUT_MODE_SILENT         : u8 = 0u8; // switch CAN trx into default silent mode
    pub const XL_OUTPUT_MODE_NORMAL         : u8 = 1u8; // switch CAN trx into normal mode
    pub const XL_OUTPUT_MODE_TX_OFF         : u8 = 2u8; // switch CAN trx into silent mode with tx pin off
    pub const XL_OUTPUT_MODE_SJA_1000_SILENT: u8 = 3u8; // switch CAN trx into SJA1000 silent mode

    //------------------------------------------------------------------------------
    // Transceiver modes
    pub const XL_TRANSCEIVER_EVENT_ERROR    : u8 = 1u8;
    pub const XL_TRANSCEIVER_EVENT_CHANGED  : u8 = 2u8;

    ////////////////////////////////////////////////////////////////////////////////
    // LIN lib
    //------------------------------------------------------------------------------
    // LIN event structures

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    struct s_xl_lin_msg {
        id: c_uchar,
        dlc: c_uchar,
        flags: c_ushort,
        data: [c_uchar; 8],
        crc: c_uchar,
    }

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    struct s_xl_lin_sleep {
        flag: c_uchar,
    }

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    struct s_xl_lin_no_ans {
        id: c_uchar,
    }

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    struct s_xl_lin_wake_up {
        flag: c_uchar,
        unused: [c_uchar; 3],
        startOffs: c_uint, // spec >= 2.0 only, else 0
        width: c_uint,     // spec >= 2.0 only, else 0
    }

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    struct s_xl_lin_crc_info {
        id: c_uchar,
        flags: c_uchar,
    }

    //------------------------------------------------------------------------------
    // LIN messages structure

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    union s_xl_lin_msg_api {
        linMsg: s_xl_lin_msg,
        linNoAns: s_xl_lin_no_ans,
        linWakeUp: s_xl_lin_wake_up,
        linSleep: s_xl_lin_sleep,
        linCRCinfo: s_xl_lin_crc_info,
    }

    //------------------------------------------------------------------------------
    // K-Line messages structure

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    pub struct s_xl_kline_rx_data {
        pub timeDiff: c_uint,
        pub data: c_uint,
        pub error: c_uint,
    }

    pub type XL_KLINE_RX_DATA = s_xl_kline_rx_data;

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    pub struct s_xl_kline_tx_data {
        pub timeDiff: c_uint,
        pub data: c_uint,
        pub error: c_uint,
    }

    pub type XL_KLINE_TX_DATA = s_xl_kline_tx_data;

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    pub struct s_xl_kline_tester_5bd {
        pub tag5bd: c_uint,
        pub timeDiff: c_uint,
        pub data: c_uint,
    }

    pub type XL_KLINE_TESTER_5BD = s_xl_kline_tester_5bd;

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    pub struct s_xl_kline_ecu_5bd {
        pub tag5bd: c_uint,
        pub timeDiff: c_uint,
        pub data: c_uint,
    }

    pub type XL_KLINE_ECU_5BD = s_xl_kline_ecu_5bd;

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    pub struct s_xl_kline_tester_fastinit_wu_pattern {
        timeDiff: c_uint,
        fastInitEdgeTimeDiff: c_uint,
    }

    pub type XL_KLINE_TESTER_FI_WU_PATTERN = s_xl_kline_tester_fastinit_wu_pattern;

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    pub struct s_xl_kline_ecu_fastinit_wu_pattern {
        timeDiff: c_uint,
        fastInitEdgeTimeDiff: c_uint, // TiniL
    }

    pub type XL_KLINE_ECU_FI_WU_PATTERN = s_xl_kline_ecu_fastinit_wu_pattern;

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    pub struct s_xl_kline_confirmation {
        channel: c_uint,
        confTag: c_uint,
        result: c_uint,
    }

    pub type XL_KLINE_CONFIRMATION = s_xl_kline_confirmation;

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    struct s_xl_kline_error_rxtx {
        rxtxErrData: c_uint,
    }

    type XL_KLINE_ERROR_RXTX = s_xl_kline_error_rxtx;

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    struct s_xl_kline_error_5bd_tester {
        tester5BdErr: c_uint,
    }

    type XL_KLINE_ERROR_TESTER_5BD = s_xl_kline_error_5bd_tester;

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    struct s_xl_kline_error_5bd_ecu {
        ecu5BdErr: c_uint,
    }
    type XL_KLINE_ERROR_ECU_5BD = s_xl_kline_error_5bd_ecu;

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    struct s_xl_kline_error_ibs {
        ibsErr: c_uint,
        rxtxErrData: c_uint,
    }
    type XL_KLINE_ERROR_IBS = s_xl_kline_error_ibs;

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    pub union s_xl_kline_error_inner_union {
        rxtxErr: XL_KLINE_ERROR_RXTX, // klineErrorTag: XL_KLINE_ERROR_TYPE_RXTX_ERROR / XL_KLINE_ERROR_TYPE_FI
        tester5BdErr: XL_KLINE_ERROR_TESTER_5BD, // klineErrorTag: XL_KLINE_ERROR_TYPE_5BD_TESTER
        ecu5BdErr: XL_KLINE_ERROR_ECU_5BD, // klineErrorTag: XL_KLINE_ERROR_TYPE_5BD_ECU
        ibsErr: XL_KLINE_ERROR_IBS,   // klineErrorTag: XL_KLINE_ERROR_TYPE_IBS
        reserved: [c_uint; 4],
    }

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    pub struct s_xl_kline_error {
        pub klineErrorTag: c_uint,
        reserved: c_uint,
        pub data: s_xl_kline_error_inner_union,
    }

    pub type XL_KLINE_ERROR = s_xl_kline_error;

    //------------------------------------------------------------------------------
    // K-Line messages structure
    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    pub union s_xl_kline_data_inner_data {
        pub klineRx: XL_KLINE_RX_DATA,
        pub klineTx: XL_KLINE_TX_DATA,

        pub klineTester5Bd: XL_KLINE_TESTER_5BD,
        pub klineEcu5Bd: XL_KLINE_ECU_5BD,

        pub klineTesterFiWu: XL_KLINE_TESTER_FI_WU_PATTERN,
        pub klineEcuFiWu: XL_KLINE_ECU_FI_WU_PATTERN,
        pub klineConfirmation: XL_KLINE_CONFIRMATION,

        pub klineError: XL_KLINE_ERROR,
    }

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    pub struct s_xl_kline_data {
        pub klineEvtTag: c_uint,
        reserved: c_uint,
        pub data: s_xl_kline_data_inner_data,
    }

    pub type XL_KLINE_DATA = s_xl_kline_data;

    //------------------------------------------------------------------------------
    // BASIC bus message structure
    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    pub union s_xl_tag_data {
        pub msg: s_xl_can_msg,
        chipState: s_xl_chip_state,
        linMsgApi: s_xl_lin_msg_api,
        syncPulse: s_xl_sync_pulse,
        daioData: s_xl_daio_data,
        transceiver: s_xl_transceiver,
        daioPiggyData: s_xl_daio_piggy_data,
        klineData: s_xl_kline_data,
    }

    impl Default for s_xl_tag_data {
        fn default() -> Self {
            Self {
                daioData: Default::default(),
            }
        }
    }

    impl Debug for s_xl_tag_data {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("s_xl_tag_data")
                .field("msg", unsafe { &self.msg })
                .finish()
        }
    }

    type XLeventTag = c_uchar;

    //------------------------------------------------------------------------------
    // XL_EVENT structures
    // event type definition

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    pub struct s_xl_event {
        pub tag: XLeventTag,        // 1
        pub chanIndex: c_uchar,     // 1
        pub transId: c_ushort,      // 2
        pub portHandle: c_ushort,   // 2 internal use only !!!!
        pub flags: c_uchar,         // 1 (e.g. XL_EVENT_FLAG_OVERRUN)
        pub reserved: c_uchar,      // 1
        pub timeStamp: c_ulonglong, // 8
        pub tagData: s_xl_tag_data, // 32 Bytes
    }
    // --------
    // 48 Bytes
    pub type XLevent = s_xl_event;

    impl Default for XLevent {
        fn default() -> Self {
            Self {
                tag: Default::default(),
                chanIndex: Default::default(),
                transId: Default::default(),
                portHandle: Default::default(),
                flags: Default::default(),
                reserved: Default::default(),
                timeStamp: Default::default(),
                tagData: Default::default(),
            }
        }
    }

    impl Debug for XLevent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let transId = self.transId;
            let porthandle = self.portHandle;
            let timeStamp = self.timeStamp;
            f.debug_struct("s_xl_event")
                .field("tag", &self.tag)
                .field("chanIndex", &self.chanIndex)
                .field("transId", &transId)
                .field("portHandle", &porthandle)
                .field("flags", &self.flags)
                .field("reserved", &self.reserved)
                .field("timeStamp", &timeStamp)
                .field("tagData", &self.tagData)
                .finish()
        }
    }

    #[allow(non_upper_case_globals)]
    pub const DriverNotifyMessageName: &str = "VectorCanDriverChangeNotifyMessage";

    //------------------------------------------------------------------------------
    // build a channels mask from the channels index

    #[macro_export]
    macro_rules! XL_CHANNEL_MASK {
        ( $( $x:expr ),* ) => {{
            1i64 << $X
        }};
    }

    pub const XL_MAX_APPNAME: u8 = 32u8;

    //------------------------------------------------------------------------------
    // driver status
    pub type XLstatus = c_short;

    //defines for xlGetDriverConfig structures
    pub const XL_MAX_LENGTH             : usize = 31usize;
    pub const XL_CONFIG_MAX_CHANNELS    : usize = 64usize;
    pub const XL_MAX_NAME_LENGTH        : usize = 48usize;

    // defines for xlSet/GetApplConfig
    pub const XL_APPLCONFIG_MAX_CHANNELS: usize = 256usize;

    //activate - channel flags
    pub const XL_ACTIVATE_NONE          : u32 = 0u32;
    pub const XL_ACTIVATE_RESET_CLOCK   : u32 = 8u32; // using this flag with time synchronisation protocols supported by Vector Timesync Service is not recommended

    pub const XL_BUS_COMPATIBLE_CAN     : u32 = XL_BUS_TYPE_CAN;
    pub const XL_BUS_COMPATIBLE_LIN     : u32 = XL_BUS_TYPE_LIN;
    pub const XL_BUS_COMPATIBLE_FLEXRAY : u32 = XL_BUS_TYPE_FLEXRAY;
    pub const XL_BUS_COMPATIBLE_MOST    : u32 = XL_BUS_TYPE_MOST;
    pub const XL_BUS_COMPATIBLE_DAIO    : u32 = XL_BUS_TYPE_DAIO; //io cab/piggy
    pub const XL_BUS_COMPATIBLE_J1708   : u32 = XL_BUS_TYPE_J1708;
    pub const XL_BUS_COMPATIBLE_KLINE   : u32 = XL_BUS_TYPE_KLINE;
    pub const XL_BUS_COMPATIBLE_ETHERNET: u32 = XL_BUS_TYPE_ETHERNET;
    pub const XL_BUS_COMPATIBLE_A429    : u32 = XL_BUS_TYPE_A429;

    // the following bus types can be used with the current cab / piggy
    pub const XL_BUS_ACTIVE_CAP_CAN     : u32 = XL_BUS_COMPATIBLE_CAN << 16;
    pub const XL_BUS_ACTIVE_CAP_LIN     : u32 = XL_BUS_COMPATIBLE_LIN << 16;
    pub const XL_BUS_ACTIVE_CAP_FLEXRAY : u32 = XL_BUS_COMPATIBLE_FLEXRAY << 16;
    pub const XL_BUS_ACTIVE_CAP_MOST    : u32 = XL_BUS_COMPATIBLE_MOST << 16;
    pub const XL_BUS_ACTIVE_CAP_DAIO    : u32 = XL_BUS_COMPATIBLE_DAIO << 16;
    pub const XL_BUS_ACTIVE_CAP_J1708   : u32 = XL_BUS_COMPATIBLE_J1708 << 16;
    pub const XL_BUS_ACTIVE_CAP_KLINE   : u32 = XL_BUS_COMPATIBLE_KLINE << 16;
    pub const XL_BUS_ACTIVE_CAP_ETHERNET: u32 = XL_BUS_COMPATIBLE_ETHERNET << 16;
    pub const XL_BUS_ACTIVE_CAP_A429    : u32 = XL_BUS_COMPATIBLE_A429 << 16;

    pub const XL_BUS_NAME_NONE          : &str = "";
    pub const XL_BUS_NAME_CAN           : &str = "CAN";
    pub const XL_BUS_NAME_LIN           : &str = "LIN";
    pub const XL_BUS_NAME_FLEXRAY       : &str = "FlexRay";
    pub const XL_BUS_NAME_STREAM        : &str = "Stream";
    pub const XL_BUS_NAME_MOST          : &str = "MOST";
    pub const XL_BUS_NAME_DAIO          : &str = "DAIO";
    pub const XL_BUS_NAME_HWSYNC_KEYPAD : &str = "HWSYNC_KEYPAD";
    pub const XL_BUS_NAME_J1708         : &str = "J1708";
    pub const XL_BUS_NAME_KLINE         : &str = "K-Line";
    pub const XL_BUS_NAME_ETHERNET      : &str = "Ethernet";
    pub const XL_BUS_NAME_AFDX          : &str = "AFDX";
    pub const XL_BUS_NAME_A429          : &str = "ARINC429";

    //------------------------------------------------------------------------------
    // acceptance filter

    pub const XL_CAN_STD: u8 = 01u8; // flag for standard ID's
    pub const XL_CAN_EXT: u8 = 02u8; // flag for extended ID's

    //------------------------------------------------------------------------------
    // bit timing

    pub const CANFD_CONFOPT_NO_ISO: u8 = 0x08; // configuration option CANFD-BOSCH

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    pub struct XLcanFdConf {
        pub arbitrationBitRate: c_uint,
        pub sjwAbr: c_uint, // CAN bus timing for nominal / arbitration bit rate
        pub tseg1Abr: c_uint,
        pub tseg2Abr: c_uint,
        pub dataBitRate: c_uint,
        pub sjwDbr: c_uint, // CAN bus timing for data bit rate
        pub tseg1Dbr: c_uint,
        pub tseg2Dbr: c_uint,
        pub reserved: c_uchar,       // has to be zero
        pub options: c_uchar,        // CANFD_CONFOPT_
        pub reserved1: [c_uchar; 2], // has to be zero
        pub reserved2: c_uint,       // has to be zero
    }

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    struct XLchipParams {
        bitRate: c_uint,
        sjw: c_uchar,
        tseg1: c_uchar,
        tseg2: c_uchar,
        sam: c_uchar, // 1 or 3
    }

    // defines for XLbusParams::data::most::activeSpeedGrade and compatibleSpeedGrade
    pub const XL_BUS_PARAMS_MOST_SPEED_GRADE_25     : u8 = 0x01u8;
    pub const XL_BUS_PARAMS_MOST_SPEED_GRADE_150    : u8 = 0x02u8;

    //defines for XLbusParams::data::can/canFD::canOpMode
    pub const XL_BUS_PARAMS_CANOPMODE_CAN20         : u8 = 0x01u8; // channel operates in CAN20
    pub const XL_BUS_PARAMS_CANOPMODE_CANFD         : u8 = 0x02u8; // channel operates in CANFD
    pub const XL_BUS_PARAMS_CANOPMODE_CANFD_NO_ISO  : u8 = 0x08u8; // channel operates in CANFD_NO_ISO

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    union XlBusParamsUnion {
        can: XLbusParamsCAN,
        canFD: XLbusParamsCANFD,
        most: XLbusParamsMost,
        flexray: XLbusParamsFlexray,
        ethernet: XLbusParamsEthernet,
        a429: XLbusParamsa429,
        raw: [c_uchar; 28],
    }

    impl Default for XlBusParamsUnion {
        fn default() -> Self {
            Self {
                can: XLbusParamsCAN::default(),
            }
        }
    }

    impl Debug for XlBusParamsUnion {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("XlbusParamsUnion")
                .field("raw", unsafe { &self.raw })
                .finish()
        }
    }

    /// unnamed struct for unnamed Union in XLbusParams
    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy, Default)]
    struct XLbusParamsCAN {
        bitRate: c_uint,
        sjw: c_uchar,
        tseg1: c_uchar,
        tseg2: c_uchar,
        sam: c_uchar,
        outputMode: c_uchar,
        reserved1: [c_uchar; 7],
        canOpMode: c_uchar,
    }

    /// unnamed struct for unnamed Union in XLbusParams
    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy, Default)]
    struct XLbusParamsCANFD {
        arbitrationBitRate: c_uint, // CAN bus timing for nominal / arbitration bit rate
        sjwAbr: c_uchar,
        tseg1Abr: c_uchar,
        tseg2Abr: c_uchar,
        samAbr: c_uchar, // 1 or 3
        outputMode: c_uchar,
        sjwDbr: c_uchar, // CAN bus timing for data bit rate
        tseg1Dbr: c_uchar,
        tseg2Dbr: c_uchar,
        dataBitRate: c_int,
        canOpMode: c_uchar,
    }

    /// unnamed struct for unnamed Union in XLbusParams
    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy, Default)]
    struct XLbusParamsMost {
        activeSpeedGrade: c_uint,
        compatibleSpeedGrade: c_uint,
        inicFwVersion: c_uint,
    }

    /// unnamed struct for unnamed Union in XLbusParams
    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy, Default)]
    struct XLbusParamsFlexray {
        // status and cfg mode are part of xlFrGetChannelConfiguration, too
        status: c_uint,   // XL_FR_CHANNEL_CFG_STATUS_xxx
        cfgMode: c_uint,  // XL_FR_CHANNEL_CFG_MODE_xxx
        baudrate: c_uint, // FlexRay baudrate in kBaud
    }

    /// unnamed struct for unnamed Union in XLbusParams
    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy, Default)]
    struct XLbusParamsEthernet {
        macAddr: [c_uchar; 6], // MAC address (starting with MSB!)
        connector: c_uchar,    // XL_ETH_STATUS_CONNECTOR_xxx
        phy: c_uchar,          // XL_ETH_STATUS_PHY_xxx
        link: c_uchar,         // XL_ETH_STATUS_LINK_xxx
        speed: c_uchar,        // XL_ETH_STATUS_SPEED_xxx
        clockMode: c_uchar,    // XL_ETH_STATUS_CLOCK_xxx
        bypass: c_uchar,       // XL_ETH_BYPASS_xxx
    }

    /// unnamed struct for unnamed Union in XLbusParams
    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy, Default)]
    struct XLbusParamsa429 {
        channelDirection: c_ushort,
        res1: c_ushort,
        dir: XLbusParamss429UnnamedUnion,
    }

    /// unnamed union for XLbusParamsa429
    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    union XLbusParamss429UnnamedUnion {
        tx: A429Tx,
        rx: A429Rx,
        raw: [c_uchar; 24],
    }

    impl Default for XLbusParamss429UnnamedUnion {
        fn default() -> Self {
            Self {
                raw: [0 as c_uchar; 24],
            }
        }
    }

    /// unnamed struct for XLbusParamss429UnnamedUnion
    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy, Default)]
    struct A429Tx {
        bitrate: c_uint,
        parity: c_uint,
        minGap: c_uint,
    }

    /// unnamed struct for XLbusParamss429UnnamedUnion
    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy, Default)]
    struct A429Rx {
        bitrate: c_uint,
        minBitrate: c_uint,
        maxBitrate: c_uint,
        parity: c_uint,
        minGap: c_uint,
        autoBaudrate: c_uint,
    }

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy, Default, Debug)]
    struct XLbusParams {
        busType: c_uint,
        data: XlBusParamsUnion,
    }

    // porthandle
    pub const XL_INVALID_PORTHANDLE: XLportHandle = -1;
    pub type XLportHandle = XLlong;
    pub type pXLportHandle = *mut XLlong;

    pub const XL_CONNECTION_INFO_FAMILY_MASK: u32 = 0xff000000u32;
    pub const XL_CONNECTION_INFO_DETAIL_MASK: u32 = 0x00ffffffu32;

    // defines to select the connectionInfo family (most-significant-byte of connection_info)
    pub const XL_CONNECTION_INFO_FAMILY_USB     : u32 = 0 << 24; // USB devices
    pub const XL_CONNECTION_INFO_FAMILY_NETWORK : u32 = 1 << 24; // Ethernet and WiFi devices
    pub const XL_CONNECTION_INFO_FAMILY_PCIE    : u32 = 2 << 24; // PCI-Express devices

    // defines for the connectionInfo (only for the USB devices)
    pub const XL_CONNECTION_INFO_USB_UNKNOWN    : u8 = 0u8;
    pub const XL_CONNECTION_INFO_USB_FULLSPEED  : u8 = 1u8;
    pub const XL_CONNECTION_INFO_USB_HIGHSPEED  : u8 = 2u8;
    pub const XL_CONNECTION_INFO_USB_SUPERSPEED : u8 = 3u8;
    // defines for FPGA core types (fpgaCoreCapabilities)
    pub const XL_FPGA_CORE_TYPE_NONE    : u8 = 0u8;
    pub const XL_FPGA_CORE_TYPE_CAN     : u8 = 1u8;
    pub const XL_FPGA_CORE_TYPE_LIN     : u8 = 2u8;
    pub const XL_FPGA_CORE_TYPE_LIN_RX  : u8 = 3u8;

    //#defines for specialDeviceStatus
    pub const XL_SPECIAL_DEVICE_STAT_FPGA_UPDATE_DONE: u8 = 0x01u8; // automatic driver FPGA flashing done

    // structure for xlGetLicenseInfo function
    // This structure is returned as an array from the xlGetLicenseInfo. It contains all available licenses on
    // the queried channels. The position inside the array is defined by the license itself, e.g. the license for
    // the Advanced-Flexray-Library is always at the same array index.
    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy)]
    pub struct s_xl_license_info {
        pub bAvailable: c_uchar,        // License is available
        pub licName: [c_char; 65usize], // Name of the license as NULL-terminated string
    }

    pub type XL_LICENSE_INFO = s_xl_license_info;
    pub type XLlicenseInfo = XL_LICENSE_INFO;

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy, Default, Debug)]
    pub struct s_xl_channel_config {
        pub name: [c_char; XL_MAX_LENGTH + 1],
        pub hwType: c_uchar,
        hwIndex: c_uchar,
        hwChannel: c_uchar,
        pub transceiverType: c_ushort,
        tranaceiverState: c_ushort,
        configError: c_ushort,
        pub channelIndex: c_uchar,
        pub channelMask: c_ulonglong,
        pub channelCapabilities: c_uint,
        pub channelBusCapabilities: c_uint,

        // Channel
        isOnBus: c_uchar,
        connectedBusType: c_uint,
        busParams: XLbusParams,
        _doNotUse: c_uint,

        driverVersion: c_uint,
        interfaceVersion: c_uint,
        raw_data: [c_uint; 10],

        serialNumber: c_uint,
        articleNumber: c_uint,

        pub transceiverName: [c_char; XL_MAX_LENGTH + 1usize], // is different from the original type

        specialCabFlags: c_uint,
        dominantTimeout: c_uint,
        dominantRecessiveDelay: c_uchar,
        recessiveDominantDelay: c_uchar,
        connectionInfo: c_uchar,
        currentlyAvailableTimestamps: c_uchar,
        minimalSupplyVoltage: c_ushort,
        maximalSupplyVoltage: c_ushort,
        maximalBaudrate: c_uint,
        fpgaCoreCapabilities: c_uchar,
        specialDeviceStatus: c_uchar,
        channelBusActiveCapabilities: c_ushort,
        breakOffset: c_ushort,
        delimiterOffset: c_ushort,
        reserved: [c_uint; 3],
    }

    pub type XlChannelConfig = s_xl_channel_config;
    pub type pXLchannelConfig = *mut s_xl_channel_config;

    #[repr(C)]
    #[repr(packed)]
    #[derive(Clone, Copy, Debug)]
    pub struct XLdriverConfig {
        dllVersion: c_uint,
        pub channelCount: c_uint,
        reserved: [c_uint; 10],
        pub channel: [XlChannelConfig; XL_CONFIG_MAX_CHANNELS],
    }

    impl Default for XLdriverConfig {
        fn default() -> Self {
            Self {
                dllVersion: 0 as c_uint,
                channelCount: 0 as c_uint,
                reserved: [0; 10],
                channel: [XlChannelConfig::default(); XL_CONFIG_MAX_CHANNELS],
            }
        }
    }

    
    // flags for channelCapabilities
    // Time-Sync
    pub const XL_CHANNEL_FLAG_TIME_SYNC_RUNNING     : u32 = 0x00000001u32;
    pub const XL_CHANNEL_FLAG_NO_HWSYNC_SUPPORT     : u32 = 0x00000400u32; // Device is not capable of hardware-based time synchronization via Sync-line
                                                                           // used to distinguish between VN2600 (w/o SPDIF) and VN2610 (with S/PDIF)
    pub const XL_CHANNEL_FLAG_SPDIF_CAPABLE         : u32 = 0x00004000u32;
    pub const XL_CHANNEL_FLAG_CANFD_BOSCH_SUPPORT   : u32 = 0x20000000u32;
    pub const XL_CHANNEL_FLAG_CMACTLICENSE_SUPPORT  : u32 = 0x40000000u32;
    pub const XL_CHANNEL_FLAG_CANFD_ISO_SUPPORT     : u32 = 0x80000000u32;

    ///////////////////////////////////////////////////////////////////////
    // CAN / CAN-FD types and definitions
    ///////////////////////////////////////////////////////////////////////

    pub const XL_CAN_MAX_DATA_LEN           : usize = 64usize;
    pub const XL_CANFD_RX_EVENT_HEADER_SIZE : usize = 32usize;
    pub const XL_CANFD_MAX_EVENT_SIZE       : usize = 128usize;

    ////////////////////////////////////////////////////////////////////////
    // get the number of databytes from dlc/edl/rtr in received events
    #[macro_export]
    macro_rules! CANFD_GET_NUM_DATABYTES {
        ( $dlc:expr , $edl:expr, $rtr:expr ) => {{
            if $rtr == 0 {
                return 0;
            }

            if $dlc < 9 {
                return $dlc;
            }

            if $edl != 0 {
                return match $dlc {
                    9 => 12,
                    10 => 16,
                    11 => 20,
                    12 => 24,
                    13 => 32,
                    14 => 48,
                    _ => 64,
                };
            } else {
                return 8;
            }
        }};
    }

    // to be used with XLcanTxEvent::XL_CAN_TX_MSG::msgFlags
    pub const XL_CAN_TXMSG_FLAG_EDL     : c_uint = 0x0001u32; // extended data length
    pub const XL_CAN_TXMSG_FLAG_BRS     : c_uint = 0x0002u32; // baud rate switch
    pub const XL_CAN_TXMSG_FLAG_RTR     : c_uint = 0x0010u32; // remote transmission request
    pub const XL_CAN_TXMSG_FLAG_HIGHPRIO: c_uint = 0x0080u32; // high priority message - clears all send buffers - then transmits
    pub const XL_CAN_TXMSG_FLAG_WAKEUP  : c_uint = 0x0200u32; // generate a wakeup message

    ////////////////////////////////////////////////////////////////////////
    // CAN / CAN-FD tx event definitions
    ////////////////////////////////////////////////////////////////////////

    #[repr(C)]
    #[repr(align(8))]
    #[derive(Clone, Copy)]
    pub struct XL_CAN_TX_MSG {
        pub canId: c_uint,
        pub msgFlags: c_uint,
        pub dlc: c_uchar,
        reserved: [c_uchar; 7usize],
        pub data: [c_uchar; XL_CAN_MAX_DATA_LEN],
    }

    impl Default for XL_CAN_TX_MSG {
        fn default() -> Self {
            Self {
                canId: Default::default(),
                msgFlags: Default::default(),
                dlc: Default::default(),
                reserved: Default::default(),
                data: [0; XL_CAN_MAX_DATA_LEN],
            }
        }
    }

    #[repr(C)]
    #[repr(align(8))]
    #[derive(Clone, Copy)]
    pub union XLcanTxEventInnerUnion {
        pub canMsg: XL_CAN_TX_MSG,
    }

    impl Default for XLcanTxEventInnerUnion {
        fn default() -> Self {
            Self {
                canMsg: Default::default(),
            }
        }
    }

    #[repr(C)]
    #[repr(align(8))]
    #[derive(Clone, Copy)]
    pub struct XLcanTxEvent {
        pub tag: c_ushort,           //  2 - type of the event
        transId: c_ushort,           //  2
        channelIndex: c_uchar,       //  1 - internal has to be 0
        reserved: [c_uchar; 3usize], //  3 - has to be zero

        pub tagData: XLcanTxEventInnerUnion,
    }

    impl Default for XLcanTxEvent {
        fn default() -> Self {
            Self {
                tag: Default::default(),
                transId: Default::default(),
                channelIndex: Default::default(),
                reserved: Default::default(),
                tagData: Default::default(),
            }
        }
    }

    ////////////////////////////////////////////////////////////////////////
    // CAN / CAN-FD rx event definitions
    ////////////////////////////////////////////////////////////////////////

    // used with XL_CAN_EV_TAG_RX_OK, XL_CAN_EV_TAG_TX_OK
    #[repr(C)]
    #[repr(align(8))]
    #[derive(Clone, Copy)]
    struct XL_CAN_EV_RX_MSG {
        canId: c_uint,
        msgFlags: c_uint,
        crc: c_uint,
        reserved1: [c_uchar; 12usize],
        totalBitCnt: c_ushort,
        dlc: c_uchar,
        reserved: [c_uchar; 5usize],
        data: [c_uchar; XL_CAN_MAX_DATA_LEN],
    }

    #[repr(C)]
    #[repr(align(8))]
    #[derive(Clone, Copy)]
    pub struct XL_CAN_EV_TX_REQUEST {
        canId: c_uint,
        msgFlags: c_uint,
        dlc: c_uchar,
        reserved1: c_uchar,
        reserved: c_ushort,
        data: [c_uchar; XL_CAN_MAX_DATA_LEN],
    }

    #[repr(C)]
    #[repr(align(8))]
    #[derive(Clone, Copy)]
    // to be used with XL_CAN_EV_TAG_CHIP_STATE
    pub struct XL_CAN_EV_CHIP_STATE {
        busStatus: c_uchar,
        txErrorCounter: c_uchar,
        rxErrorCounter: c_uchar,
        reserved: c_uchar,
        reserved0: c_uint,
    }

    type XL_CAN_EV_SYNC_PULSE = XL_SYNC_PULSE_EV;

    // to be used with XL_CAN_EV_ERROR::errorCode
    pub const XL_CAN_ERRC_BIT_ERROR     : c_uchar = 1u8;
    pub const XL_CAN_ERRC_FORM_ERROR    : c_uchar = 2u8;
    pub const XL_CAN_ERRC_STUFF_ERROR   : c_uchar = 3u8;
    pub const XL_CAN_ERRC_OTHER_ERROR   : c_uchar = 4u8;
    pub const XL_CAN_ERRC_CRC_ERROR     : c_uchar = 5u8;
    pub const XL_CAN_ERRC_ACK_ERROR     : c_uchar = 6u8;
    pub const XL_CAN_ERRC_NACK_ERROR    : c_uchar = 7u8;
    pub const XL_CAN_ERRC_OVLD_ERROR    : c_uchar = 8u8;
    pub const XL_CAN_ERRC_EXCPT_ERROR   : c_uchar = 9u8;

    //to be used with XL_CAN_EV_TAG_RX_ERROR/XL_CAN_EV_TAG_TX_ERROR
    #[repr(C)]
    #[repr(align(8))]
    #[derive(Clone, Copy)]
    pub struct XL_CAN_EV_ERROR {
        errorCode: c_uchar,
        reserved: [c_uchar; 95],
    }

    // to be used with XLcanRxEvent::flagsChip
    pub const XL_CAN_QUEUE_OVERFLOW: u32 = 0x100;

    // max./min size of application rx fifo (bytes)
    pub const RX_FIFO_CANFD_QUEUE_SIZE_MAX: u32 = 524288u32; // 0,5 MByte
    pub const RX_FIFO_CANFD_QUEUE_SIZE_MIN: u32 = 8192u32; // 8 kByte

    //------------------------------------------------------------------------------
    // General RX Event
    #[repr(C)]
    #[repr(align(8))]
    #[derive(Clone, Copy)]
    pub struct XLcanRxEvent {
        pub size: c_uint,               // 4 - overall size of the complete event
        pub tag: c_ushort,              // 2 - type of the event
        pub channelIndex: c_ushort,     // 2
        pub userHandle: c_uint,         // 4 (lower 12 bit available for CAN)
        pub flagsChip: c_ushort,        // 2 queue overflow (upper 8bit)
        pub reserved0: c_ushort,        // 2
        pub reserved1: c_ulonglong,     // 8
        pub timeStampSync: c_ulonglong, // 8 - timestamp which is synchronized by the driver
        pub tagData: XLcanRxEventTagData,
    }

    impl Default for XLcanRxEvent {
        fn default() -> Self {
            Self {
                size: Default::default(),
                tag: Default::default(),
                channelIndex: Default::default(),
                userHandle: Default::default(),
                flagsChip: Default::default(),
                reserved0: Default::default(),
                reserved1: Default::default(),
                timeStampSync: Default::default(),
                tagData: Default::default(),
            }
        }
    }

    #[repr(C)]
    #[repr(align(8))]
    #[derive(Clone, Copy)]
    pub union XLcanRxEventTagData {
        raw: [c_uchar; XL_CANFD_MAX_EVENT_SIZE - XL_CANFD_RX_EVENT_HEADER_SIZE],
        canRxOkMsg: XL_CAN_EV_RX_MSG,
        canTxOkMsg: XL_CAN_EV_RX_MSG,
        canTxRequest: XL_CAN_EV_TX_REQUEST,

        canError: XL_CAN_EV_ERROR,
        canChipState: XL_CAN_EV_CHIP_STATE,
        canSyncPulse: XL_CAN_EV_SYNC_PULSE,
    }

    impl Default for XLcanRxEventTagData {
        fn default() -> Self {
            Self {
                raw: [0u8; XL_CANFD_MAX_EVENT_SIZE - XL_CANFD_RX_EVENT_HEADER_SIZE],
            }
        }
    }

    #[link(name = "vxlapi64")]
    extern "stdcall" {
        fn xlOpenDriver() -> XLstatus; // 3.2.1
        fn xlCloseDriver() -> XLstatus; // 3.2.2
        fn xlGetDriverConfig(pDriverConfig: *mut XLdriverConfig) -> XLstatus; // 3.2.5
        fn xlOpenPort(
            portHandle: *mut XLportHandle,
            userName: *const c_char,
            accessMask: XLaccess,
            permissionMask: *const XLaccess,
            rxQueueSize: c_uint,
            xlInterfaceVersion: c_uint,
            busType: c_uint,
        ) -> XLstatus; // 3.2.9
        fn xlClosePort(portHandle: XLportHandle) -> XLstatus; // 3.2.10
        fn xlSetTimerRate(portHandle: XLportHandle, timerRate: c_ulong) -> XLstatus; // 3.2.11
        fn xlSetTimerRateAndChannel(
            portHandle: XLportHandle,
            timerChannelMask: *const XLaccess,
            timerRate: *const c_ulong,
        ) -> XLstatus; // 3.2.12
        fn xlResetClock(portHandle: XLportHandle) -> XLstatus; // 3.2.13
        fn xlSetNotification(
            portHandle: XLportHandle,
            handle: *const XLhandle,
            queueLevel: c_uint,
        ) -> XLstatus; // 3.2.14
        fn xlActivateChannel(
            portHandle: XLportHandle,
            accessMask: XLaccess,
            busType: c_uint,
            flags: c_uint,
        ) -> XLstatus; // 3.2.27
        fn xlReceive(
            portHandle: XLportHandle,
            pEventCount: *mut c_int,
            pEventList: *mut XLevent,
        ) -> XLstatus; // 3.2.18
        fn xlGetEventString(ev: *const XLevent) -> XLstringType; //3.2.19
        fn xlGetErrorString(err: XLstatus) -> *const c_char; // 3.2.20
        fn xlGenerateSyncPulse(portHandle: XLportHandle, accessMask: XLaccess) -> XLstatus; // 3.2.23
        fn xlDeactivateChannel(portHandle: XLportHandle, accessMask: XLaccess) -> XLstatus; // 3.2.25
        fn xlCanSetChannelOutput (
            portHandle: XLportHandle,
            accessMask: XLaccess,
            mode: c_uchar) -> XLstatus; // 4.3.2
        fn xlCanSetChannelBitrate(
            portHandle: XLportHandle,
            accessMask: XLaccess,
            bitrate: c_ulong,
        ) -> XLstatus; // 4.3.7
        fn xlCanRequestChipState(portHandle: XLportHandle, accessMask: XLaccess) -> XLstatus; // 4.3.12
        fn xlCanTransmit(
            portHandle: XLportHandle,
            accessMask: XLaccess,
            messageCount: *const c_uint,
            pMessages: *const c_void,
        ) -> XLstatus; // 4.3.13
        fn xlCanFdSetConfiguration(
            portHandle: XLportHandle,
            accessMask: XLaccess,
            pCanFdConf: *const XLcanFdConf,
        ) -> XLstatus; // 5.3.1
        fn xlCanTransmitEx(
            portHandle: XLportHandle,
            accessMask: XLaccess,
            msgCnt: c_uint,
            pMsgCntSent: *mut c_uint,
            pXlCanTxEvt: *const XLcanTxEvent,
        ) -> XLstatus; // 5.3.2
        fn xlCanReceive(portHandle: XLportHandle, pXlCanRxEvt: *mut XLcanRxEvent) -> XLstatus; // 5.3.3
        fn xlCanGetEventString(pEv: *const XLcanRxEvent) -> XLstringType;
    }

    pub fn xl_open_driver() -> XLstatus {
        return unsafe { xlOpenDriver() };
    }

    pub fn xl_close_driver() -> XLstatus {
        return unsafe { xlCloseDriver() };
    }

    pub fn xl_get_driver_config(pDriverConfig: &mut XLdriverConfig) -> XLstatus {
        return unsafe { xlGetDriverConfig(pDriverConfig) };
    }

    pub fn xl_open_port(
        portHandle: &mut XLportHandle,
        userName: &str,
        accessMask: XLaccess,
        permissionMask: &XLaccess,
        rxQueueSize: c_uint,
        xlInterfaceVersion: c_uint,
        busType: c_uint,
    ) -> XLstatus {
        return unsafe {
            xlOpenPort(
                portHandle,
                #[allow(temporary_cstring_as_ptr)]
                CString::new(userName).unwrap().as_ptr(),
                accessMask,
                permissionMask,
                rxQueueSize,
                xlInterfaceVersion,
                busType,
            )
        };
    }

    pub fn xl_close_port(portHandle: XLportHandle) -> XLstatus {
        return unsafe { xlClosePort(portHandle) };
    }

    pub fn xl_set_timer_rate(portHandle: XLportHandle, timerRate: c_ulong) -> XLstatus {
        return unsafe { xlSetTimerRate(portHandle, timerRate) };
    }

    pub fn xl_set_timer_rate_and_channel(
        portHandle: XLportHandle,
        timerChannelMask: &XLaccess,
        timerRate: &c_ulong,
    ) -> XLstatus {
        return unsafe { xlSetTimerRateAndChannel(portHandle, timerChannelMask, timerRate) };
    }

    pub fn xl_reset_clock(portHandle: XLportHandle) -> XLstatus {
        return unsafe { xlResetClock(portHandle) };
    }

    pub fn xl_set_notification(
        portHandle: XLportHandle,
        handle: &mut XLhandle,
        queueLevel: c_uint,
    ) -> XLstatus {
        return unsafe { xlSetNotification(portHandle, handle, queueLevel) };
    }

    pub fn xl_activate_channel(
        portHandle: XLportHandle,
        accessMask: XLaccess,
        busType: c_uint,
        flags: c_uint,
    ) -> XLstatus {
        return unsafe { xlActivateChannel(portHandle, accessMask, busType, flags) };
    }

    pub fn xl_receive(
        portHandle: XLportHandle,
        pEventCount: &mut c_int,
        pEventList: &mut XLevent,
    ) -> XLstatus {
        return unsafe { xlReceive(portHandle, pEventCount, pEventList) };
    }

    pub fn xl_get_event_string(ev: &XLevent) -> &'static str {
        return unsafe { CStr::from_ptr(xlGetEventString(ev)).to_str().unwrap() };
    }

    pub fn xl_get_error_string(err: XLstatus) -> &'static str {
        return unsafe { CStr::from_ptr(xlGetErrorString(err)).to_str().unwrap() };
    }

    pub fn xl_generate_sync_pulse(portHandle: XLportHandle, accessMask: XLaccess) -> XLstatus {
        return unsafe { xlGenerateSyncPulse(portHandle, accessMask) };
    }

    pub fn xl_deactivate_channel(portHandle: XLportHandle, accessMask: XLaccess) -> XLstatus {
        return unsafe { xlDeactivateChannel(portHandle, accessMask) };
    }

    pub fn xl_can_set_channel_output ( portHandle: XLportHandle, accessMask: XLaccess, mode: c_uchar) -> XLstatus {
        return unsafe { xlCanSetChannelOutput ( portHandle, accessMask, mode) };
    }

    pub fn xl_can_set_channel_bitrate(
        portHandle: XLportHandle,
        accessMask: XLaccess,
        bitrate: c_ulong,
    ) -> XLstatus {
        return unsafe { xlCanSetChannelBitrate(portHandle, accessMask, bitrate) };
    }

    pub fn xl_can_request_chip_state(portHandle: XLportHandle, accessMask: XLaccess) -> XLstatus {
        return unsafe { xlCanRequestChipState(portHandle, accessMask) };
    }

    pub fn xl_can_transmit(
        portHandle: XLportHandle,
        accessMask: XLaccess,
        messageCount: &c_uint,
        pMessages: *const c_void,
    ) -> XLstatus {
        return unsafe {
            xlCanTransmit(
                portHandle,
                accessMask,
                messageCount,
                pMessages as *const c_void,
            )
        };
    }

    pub fn xl_can_fd_set_configuration(
        portHandle: XLportHandle,
        accessMask: XLaccess,
        pCanFdConf: &XLcanFdConf,
    ) -> XLstatus {
        return unsafe { xlCanFdSetConfiguration(portHandle, accessMask, pCanFdConf) };
    }

    pub fn xl_can_transmit_ex(
        portHandle: XLportHandle,
        accessMask: XLaccess,
        msgCnt: c_uint,
        pMsgCntSent: &mut c_uint,
        pXlCanTxEvt: *const XLcanTxEvent,
    ) -> XLstatus {
        return unsafe {
            xlCanTransmitEx(portHandle, accessMask, msgCnt, pMsgCntSent, pXlCanTxEvt)
        };
    }

    pub fn xl_can_receive(portHandle: XLportHandle, pXlCanRxEvt: &mut XLcanRxEvent) -> XLstatus {
        return unsafe { xlCanReceive(portHandle, pXlCanRxEvt) }; // 5.3.3
    }

    pub fn xl_can_get_event_string(pEv: &XLcanRxEvent) -> &str {
        return unsafe { CStr::from_ptr(xlCanGetEventString(pEv)) }
            .to_str()
            .unwrap();
    }
}
