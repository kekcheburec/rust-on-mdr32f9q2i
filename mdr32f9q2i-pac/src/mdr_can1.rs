#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CAN Control Register"]
    pub control: crate::Reg<control::CONTROL_SPEC>,
    #[doc = "0x04 - CAN Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x08 - CAN Bittiming Register"]
    pub bittmng: crate::Reg<bittmng::BITTMNG_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - CAN Interrupt enable Register"]
    pub int_en: crate::Reg<int_en::INT_EN_SPEC>,
    _reserved4: [u8; 0x08],
    #[doc = "0x1c - "]
    pub over: crate::Reg<over::OVER_SPEC>,
    #[doc = "0x20 - CAN Receive ID Register"]
    pub rxid: crate::Reg<rxid::RXID_SPEC>,
    #[doc = "0x24 - CAN Receive DLC Register"]
    pub rxdlc: crate::Reg<rxdlc::RXDLC_SPEC>,
    #[doc = "0x28 - CAN Receive Data low Register"]
    pub rxdatal: crate::Reg<rxdatal::RXDATAL_SPEC>,
    #[doc = "0x2c - CAN Receive Data high Register"]
    pub rxdatah: crate::Reg<rxdatah::RXDATAH_SPEC>,
    #[doc = "0x30 - CAN Transmit ID Register"]
    pub txid: crate::Reg<txid::TXID_SPEC>,
    #[doc = "0x34 - CAN Transmit DLC Register"]
    pub txdlc: crate::Reg<txdlc::TXDLC_SPEC>,
    #[doc = "0x38 - CAN Transmit Data low Register"]
    pub datal: crate::Reg<datal::DATAL_SPEC>,
    #[doc = "0x3c - CAN Transmit Data high Register"]
    pub datah: crate::Reg<datah::DATAH_SPEC>,
    #[doc = "0x40 - CAN Buffer Connection Register"]
    pub buf_con00: crate::Reg<buf_con00::BUF_CON00_SPEC>,
    #[doc = "0x44 - CAN Buffer Connection Register"]
    pub buf_con01: crate::Reg<buf_con01::BUF_CON01_SPEC>,
    #[doc = "0x48 - CAN Buffer Connection Register"]
    pub buf_con02: crate::Reg<buf_con02::BUF_CON02_SPEC>,
    #[doc = "0x4c - CAN Buffer Connection Register"]
    pub buf_con03: crate::Reg<buf_con03::BUF_CON03_SPEC>,
    #[doc = "0x50 - CAN Buffer Connection Register"]
    pub buf_con04: crate::Reg<buf_con04::BUF_CON04_SPEC>,
    #[doc = "0x54 - CAN Buffer Connection Register"]
    pub buf_con05: crate::Reg<buf_con05::BUF_CON05_SPEC>,
    #[doc = "0x58 - CAN Buffer Connection Register"]
    pub buf_con06: crate::Reg<buf_con06::BUF_CON06_SPEC>,
    #[doc = "0x5c - CAN Buffer Connection Register"]
    pub buf_con07: crate::Reg<buf_con07::BUF_CON07_SPEC>,
    #[doc = "0x60 - CAN Buffer Connection Register"]
    pub buf_con08: crate::Reg<buf_con08::BUF_CON08_SPEC>,
    #[doc = "0x64 - CAN Buffer Connection Register"]
    pub buf_con09: crate::Reg<buf_con09::BUF_CON09_SPEC>,
    #[doc = "0x68 - CAN Buffer Connection Register"]
    pub buf_con10: crate::Reg<buf_con10::BUF_CON10_SPEC>,
    #[doc = "0x6c - CAN Buffer Connection Register"]
    pub buf_con11: crate::Reg<buf_con11::BUF_CON11_SPEC>,
    #[doc = "0x70 - CAN Buffer Connection Register"]
    pub buf_con12: crate::Reg<buf_con12::BUF_CON12_SPEC>,
    #[doc = "0x74 - CAN Buffer Connection Register"]
    pub buf_con13: crate::Reg<buf_con13::BUF_CON13_SPEC>,
    #[doc = "0x78 - CAN Buffer Connection Register"]
    pub buf_con14: crate::Reg<buf_con14::BUF_CON14_SPEC>,
    #[doc = "0x7c - CAN Buffer Connection Register"]
    pub buf_con15: crate::Reg<buf_con15::BUF_CON15_SPEC>,
    #[doc = "0x80 - CAN Buffer Connection Register"]
    pub buf_con16: crate::Reg<buf_con16::BUF_CON16_SPEC>,
    #[doc = "0x84 - CAN Buffer Connection Register"]
    pub buf_con17: crate::Reg<buf_con17::BUF_CON17_SPEC>,
    #[doc = "0x88 - CAN Buffer Connection Register"]
    pub buf_con18: crate::Reg<buf_con18::BUF_CON18_SPEC>,
    #[doc = "0x8c - CAN Buffer Connection Register"]
    pub buf_con19: crate::Reg<buf_con19::BUF_CON19_SPEC>,
    #[doc = "0x90 - CAN Buffer Connection Register"]
    pub buf_con20: crate::Reg<buf_con20::BUF_CON20_SPEC>,
    #[doc = "0x94 - CAN Buffer Connection Register"]
    pub buf_con21: crate::Reg<buf_con21::BUF_CON21_SPEC>,
    #[doc = "0x98 - CAN Buffer Connection Register"]
    pub buf_con22: crate::Reg<buf_con22::BUF_CON22_SPEC>,
    #[doc = "0x9c - CAN Buffer Connection Register"]
    pub buf_con23: crate::Reg<buf_con23::BUF_CON23_SPEC>,
    #[doc = "0xa0 - CAN Buffer Connection Register"]
    pub buf_con24: crate::Reg<buf_con24::BUF_CON24_SPEC>,
    #[doc = "0xa4 - CAN Buffer Connection Register"]
    pub buf_con25: crate::Reg<buf_con25::BUF_CON25_SPEC>,
    #[doc = "0xa8 - CAN Buffer Connection Register"]
    pub buf_con26: crate::Reg<buf_con26::BUF_CON26_SPEC>,
    #[doc = "0xac - CAN Buffer Connection Register"]
    pub buf_con27: crate::Reg<buf_con27::BUF_CON27_SPEC>,
    #[doc = "0xb0 - CAN Buffer Connection Register"]
    pub buf_con28: crate::Reg<buf_con28::BUF_CON28_SPEC>,
    #[doc = "0xb4 - CAN Buffer Connection Register"]
    pub buf_con29: crate::Reg<buf_con29::BUF_CON29_SPEC>,
    #[doc = "0xb8 - CAN Buffer Connection Register"]
    pub buf_con30: crate::Reg<buf_con30::BUF_CON30_SPEC>,
    #[doc = "0xbc - CAN Buffer Connection Register"]
    pub buf_con31: crate::Reg<buf_con31::BUF_CON31_SPEC>,
    #[doc = "0xc0 - "]
    pub int_rx: crate::Reg<int_rx::INT_RX_SPEC>,
    #[doc = "0xc4 - "]
    pub rx: crate::Reg<rx::RX_SPEC>,
    #[doc = "0xc8 - "]
    pub int_tx: crate::Reg<int_tx::INT_TX_SPEC>,
    #[doc = "0xcc - "]
    pub tx: crate::Reg<tx::TX_SPEC>,
    _reserved49: [u8; 0x0130],
    #[doc = "0x200 - CAN Buffer ID Register"]
    pub buf_00_id: crate::Reg<buf_00_id::BUF_00_ID_SPEC>,
    #[doc = "0x204 - CAN Buffer DLC Register"]
    pub buf_00_dlc: crate::Reg<buf_00_dlc::BUF_00_DLC_SPEC>,
    #[doc = "0x208 - CAN Buffer Data low Register"]
    pub buf_00_datal: crate::Reg<buf_00_datal::BUF_00_DATAL_SPEC>,
    #[doc = "0x20c - CAN Buffer Data high Register"]
    pub buf_00_datah: crate::Reg<buf_00_datah::BUF_00_DATAH_SPEC>,
    #[doc = "0x210 - CAN Buffer ID Register"]
    pub buf_01_id: crate::Reg<buf_01_id::BUF_01_ID_SPEC>,
    #[doc = "0x214 - CAN Buffer DLC Register"]
    pub buf_01_dlc: crate::Reg<buf_01_dlc::BUF_01_DLC_SPEC>,
    #[doc = "0x218 - CAN Buffer Data low Register"]
    pub buf_01_datal: crate::Reg<buf_01_datal::BUF_01_DATAL_SPEC>,
    #[doc = "0x21c - CAN Buffer Data high Register"]
    pub buf_01_datah: crate::Reg<buf_01_datah::BUF_01_DATAH_SPEC>,
    #[doc = "0x220 - CAN Buffer ID Register"]
    pub buf_02_id: crate::Reg<buf_02_id::BUF_02_ID_SPEC>,
    #[doc = "0x224 - CAN Buffer DLC Register"]
    pub buf_02_dlc: crate::Reg<buf_02_dlc::BUF_02_DLC_SPEC>,
    #[doc = "0x228 - CAN Buffer Data low Register"]
    pub buf_02_datal: crate::Reg<buf_02_datal::BUF_02_DATAL_SPEC>,
    #[doc = "0x22c - CAN Buffer Data high Register"]
    pub buf_02_datah: crate::Reg<buf_02_datah::BUF_02_DATAH_SPEC>,
    #[doc = "0x230 - CAN Buffer ID Register"]
    pub buf_03_id: crate::Reg<buf_03_id::BUF_03_ID_SPEC>,
    #[doc = "0x234 - CAN Buffer DLC Register"]
    pub buf_03_dlc: crate::Reg<buf_03_dlc::BUF_03_DLC_SPEC>,
    #[doc = "0x238 - CAN Buffer Data low Register"]
    pub buf_03_datal: crate::Reg<buf_03_datal::BUF_03_DATAL_SPEC>,
    #[doc = "0x23c - CAN Buffer Data high Register"]
    pub buf_03_datah: crate::Reg<buf_03_datah::BUF_03_DATAH_SPEC>,
    #[doc = "0x240 - CAN Buffer ID Register"]
    pub buf_04_id: crate::Reg<buf_04_id::BUF_04_ID_SPEC>,
    #[doc = "0x244 - CAN Buffer DLC Register"]
    pub buf_04_dlc: crate::Reg<buf_04_dlc::BUF_04_DLC_SPEC>,
    #[doc = "0x248 - CAN Buffer Data low Register"]
    pub buf_04_datal: crate::Reg<buf_04_datal::BUF_04_DATAL_SPEC>,
    #[doc = "0x24c - CAN Buffer Data high Register"]
    pub buf_04_datah: crate::Reg<buf_04_datah::BUF_04_DATAH_SPEC>,
    #[doc = "0x250 - CAN Buffer ID Register"]
    pub buf_05_id: crate::Reg<buf_05_id::BUF_05_ID_SPEC>,
    #[doc = "0x254 - CAN Buffer DLC Register"]
    pub buf_05_dlc: crate::Reg<buf_05_dlc::BUF_05_DLC_SPEC>,
    #[doc = "0x258 - CAN Buffer Data low Register"]
    pub buf_05_datal: crate::Reg<buf_05_datal::BUF_05_DATAL_SPEC>,
    #[doc = "0x25c - CAN Buffer Data high Register"]
    pub buf_05_datah: crate::Reg<buf_05_datah::BUF_05_DATAH_SPEC>,
    #[doc = "0x260 - CAN Buffer ID Register"]
    pub buf_06_id: crate::Reg<buf_06_id::BUF_06_ID_SPEC>,
    #[doc = "0x264 - CAN Buffer DLC Register"]
    pub buf_06_dlc: crate::Reg<buf_06_dlc::BUF_06_DLC_SPEC>,
    #[doc = "0x268 - CAN Buffer Data low Register"]
    pub buf_06_datal: crate::Reg<buf_06_datal::BUF_06_DATAL_SPEC>,
    #[doc = "0x26c - CAN Buffer Data high Register"]
    pub buf_06_datah: crate::Reg<buf_06_datah::BUF_06_DATAH_SPEC>,
    #[doc = "0x270 - CAN Buffer ID Register"]
    pub buf_07_id: crate::Reg<buf_07_id::BUF_07_ID_SPEC>,
    #[doc = "0x274 - CAN Buffer DLC Register"]
    pub buf_07_dlc: crate::Reg<buf_07_dlc::BUF_07_DLC_SPEC>,
    #[doc = "0x278 - CAN Buffer Data low Register"]
    pub buf_07_datal: crate::Reg<buf_07_datal::BUF_07_DATAL_SPEC>,
    #[doc = "0x27c - CAN Buffer Data high Register"]
    pub buf_07_datah: crate::Reg<buf_07_datah::BUF_07_DATAH_SPEC>,
    #[doc = "0x280 - CAN Buffer ID Register"]
    pub buf_08_id: crate::Reg<buf_08_id::BUF_08_ID_SPEC>,
    #[doc = "0x284 - CAN Buffer DLC Register"]
    pub buf_08_dlc: crate::Reg<buf_08_dlc::BUF_08_DLC_SPEC>,
    #[doc = "0x288 - CAN Buffer Data low Register"]
    pub buf_08_datal: crate::Reg<buf_08_datal::BUF_08_DATAL_SPEC>,
    #[doc = "0x28c - CAN Buffer Data high Register"]
    pub buf_08_datah: crate::Reg<buf_08_datah::BUF_08_DATAH_SPEC>,
    #[doc = "0x290 - CAN Buffer ID Register"]
    pub buf_09_id: crate::Reg<buf_09_id::BUF_09_ID_SPEC>,
    #[doc = "0x294 - CAN Buffer DLC Register"]
    pub buf_09_dlc: crate::Reg<buf_09_dlc::BUF_09_DLC_SPEC>,
    #[doc = "0x298 - CAN Buffer Data low Register"]
    pub buf_09_datal: crate::Reg<buf_09_datal::BUF_09_DATAL_SPEC>,
    #[doc = "0x29c - CAN Buffer Data high Register"]
    pub buf_09_datah: crate::Reg<buf_09_datah::BUF_09_DATAH_SPEC>,
    #[doc = "0x2a0 - CAN Buffer ID Register"]
    pub buf_10_id: crate::Reg<buf_10_id::BUF_10_ID_SPEC>,
    #[doc = "0x2a4 - CAN Buffer DLC Register"]
    pub buf_10_dlc: crate::Reg<buf_10_dlc::BUF_10_DLC_SPEC>,
    #[doc = "0x2a8 - CAN Buffer Data low Register"]
    pub buf_10_datal: crate::Reg<buf_10_datal::BUF_10_DATAL_SPEC>,
    #[doc = "0x2ac - CAN Buffer Data high Register"]
    pub buf_10_datah: crate::Reg<buf_10_datah::BUF_10_DATAH_SPEC>,
    #[doc = "0x2b0 - CAN Buffer ID Register"]
    pub buf_11_id: crate::Reg<buf_11_id::BUF_11_ID_SPEC>,
    #[doc = "0x2b4 - CAN Buffer DLC Register"]
    pub buf_11_dlc: crate::Reg<buf_11_dlc::BUF_11_DLC_SPEC>,
    #[doc = "0x2b8 - CAN Buffer Data low Register"]
    pub buf_11_datal: crate::Reg<buf_11_datal::BUF_11_DATAL_SPEC>,
    #[doc = "0x2bc - CAN Buffer Data high Register"]
    pub buf_11_datah: crate::Reg<buf_11_datah::BUF_11_DATAH_SPEC>,
    #[doc = "0x2c0 - CAN Buffer ID Register"]
    pub buf_12_id: crate::Reg<buf_12_id::BUF_12_ID_SPEC>,
    #[doc = "0x2c4 - CAN Buffer DLC Register"]
    pub buf_12_dlc: crate::Reg<buf_12_dlc::BUF_12_DLC_SPEC>,
    #[doc = "0x2c8 - CAN Buffer Data low Register"]
    pub buf_12_datal: crate::Reg<buf_12_datal::BUF_12_DATAL_SPEC>,
    #[doc = "0x2cc - CAN Buffer Data high Register"]
    pub buf_12_datah: crate::Reg<buf_12_datah::BUF_12_DATAH_SPEC>,
    #[doc = "0x2d0 - CAN Buffer ID Register"]
    pub buf_13_id: crate::Reg<buf_13_id::BUF_13_ID_SPEC>,
    #[doc = "0x2d4 - CAN Buffer DLC Register"]
    pub buf_13_dlc: crate::Reg<buf_13_dlc::BUF_13_DLC_SPEC>,
    #[doc = "0x2d8 - CAN Buffer Data low Register"]
    pub buf_13_datal: crate::Reg<buf_13_datal::BUF_13_DATAL_SPEC>,
    #[doc = "0x2dc - CAN Buffer Data high Register"]
    pub buf_13_datah: crate::Reg<buf_13_datah::BUF_13_DATAH_SPEC>,
    #[doc = "0x2e0 - CAN Buffer ID Register"]
    pub buf_14_id: crate::Reg<buf_14_id::BUF_14_ID_SPEC>,
    #[doc = "0x2e4 - CAN Buffer DLC Register"]
    pub buf_14_dlc: crate::Reg<buf_14_dlc::BUF_14_DLC_SPEC>,
    #[doc = "0x2e8 - CAN Buffer Data low Register"]
    pub buf_14_datal: crate::Reg<buf_14_datal::BUF_14_DATAL_SPEC>,
    #[doc = "0x2ec - CAN Buffer Data high Register"]
    pub buf_14_datah: crate::Reg<buf_14_datah::BUF_14_DATAH_SPEC>,
    #[doc = "0x2f0 - CAN Buffer ID Register"]
    pub buf_15_id: crate::Reg<buf_15_id::BUF_15_ID_SPEC>,
    #[doc = "0x2f4 - CAN Buffer DLC Register"]
    pub buf_15_dlc: crate::Reg<buf_15_dlc::BUF_15_DLC_SPEC>,
    #[doc = "0x2f8 - CAN Buffer Data low Register"]
    pub buf_15_datal: crate::Reg<buf_15_datal::BUF_15_DATAL_SPEC>,
    #[doc = "0x2fc - CAN Buffer Data high Register"]
    pub buf_15_datah: crate::Reg<buf_15_datah::BUF_15_DATAH_SPEC>,
    #[doc = "0x300 - CAN Buffer ID Register"]
    pub buf_16_id: crate::Reg<buf_16_id::BUF_16_ID_SPEC>,
    #[doc = "0x304 - CAN Buffer DLC Register"]
    pub buf_16_dlc: crate::Reg<buf_16_dlc::BUF_16_DLC_SPEC>,
    #[doc = "0x308 - CAN Buffer Data low Register"]
    pub buf_16_datal: crate::Reg<buf_16_datal::BUF_16_DATAL_SPEC>,
    #[doc = "0x30c - CAN Buffer Data high Register"]
    pub buf_16_datah: crate::Reg<buf_16_datah::BUF_16_DATAH_SPEC>,
    #[doc = "0x310 - CAN Buffer ID Register"]
    pub buf_17_id: crate::Reg<buf_17_id::BUF_17_ID_SPEC>,
    #[doc = "0x314 - CAN Buffer DLC Register"]
    pub buf_17_dlc: crate::Reg<buf_17_dlc::BUF_17_DLC_SPEC>,
    #[doc = "0x318 - CAN Buffer Data low Register"]
    pub buf_17_datal: crate::Reg<buf_17_datal::BUF_17_DATAL_SPEC>,
    #[doc = "0x31c - CAN Buffer Data high Register"]
    pub buf_17_datah: crate::Reg<buf_17_datah::BUF_17_DATAH_SPEC>,
    #[doc = "0x320 - CAN Buffer ID Register"]
    pub buf_18_id: crate::Reg<buf_18_id::BUF_18_ID_SPEC>,
    #[doc = "0x324 - CAN Buffer DLC Register"]
    pub buf_18_dlc: crate::Reg<buf_18_dlc::BUF_18_DLC_SPEC>,
    #[doc = "0x328 - CAN Buffer Data low Register"]
    pub buf_18_datal: crate::Reg<buf_18_datal::BUF_18_DATAL_SPEC>,
    #[doc = "0x32c - CAN Buffer Data high Register"]
    pub buf_18_datah: crate::Reg<buf_18_datah::BUF_18_DATAH_SPEC>,
    #[doc = "0x330 - CAN Buffer ID Register"]
    pub buf_19_id: crate::Reg<buf_19_id::BUF_19_ID_SPEC>,
    #[doc = "0x334 - CAN Buffer DLC Register"]
    pub buf_19_dlc: crate::Reg<buf_19_dlc::BUF_19_DLC_SPEC>,
    #[doc = "0x338 - CAN Buffer Data low Register"]
    pub buf_19_datal: crate::Reg<buf_19_datal::BUF_19_DATAL_SPEC>,
    #[doc = "0x33c - CAN Buffer Data high Register"]
    pub buf_19_datah: crate::Reg<buf_19_datah::BUF_19_DATAH_SPEC>,
    #[doc = "0x340 - CAN Buffer ID Register"]
    pub buf_20_id: crate::Reg<buf_20_id::BUF_20_ID_SPEC>,
    #[doc = "0x344 - CAN Buffer DLC Register"]
    pub buf_20_dlc: crate::Reg<buf_20_dlc::BUF_20_DLC_SPEC>,
    #[doc = "0x348 - CAN Buffer Data low Register"]
    pub buf_20_datal: crate::Reg<buf_20_datal::BUF_20_DATAL_SPEC>,
    #[doc = "0x34c - CAN Buffer Data high Register"]
    pub buf_20_datah: crate::Reg<buf_20_datah::BUF_20_DATAH_SPEC>,
    #[doc = "0x350 - CAN Buffer ID Register"]
    pub buf_21_id: crate::Reg<buf_21_id::BUF_21_ID_SPEC>,
    #[doc = "0x354 - CAN Buffer DLC Register"]
    pub buf_21_dlc: crate::Reg<buf_21_dlc::BUF_21_DLC_SPEC>,
    #[doc = "0x358 - CAN Buffer Data low Register"]
    pub buf_21_datal: crate::Reg<buf_21_datal::BUF_21_DATAL_SPEC>,
    #[doc = "0x35c - CAN Buffer Data high Register"]
    pub buf_21_datah: crate::Reg<buf_21_datah::BUF_21_DATAH_SPEC>,
    #[doc = "0x360 - CAN Buffer ID Register"]
    pub buf_22_id: crate::Reg<buf_22_id::BUF_22_ID_SPEC>,
    #[doc = "0x364 - CAN Buffer DLC Register"]
    pub buf_22_dlc: crate::Reg<buf_22_dlc::BUF_22_DLC_SPEC>,
    #[doc = "0x368 - CAN Buffer Data low Register"]
    pub buf_22_datal: crate::Reg<buf_22_datal::BUF_22_DATAL_SPEC>,
    #[doc = "0x36c - CAN Buffer Data high Register"]
    pub buf_22_datah: crate::Reg<buf_22_datah::BUF_22_DATAH_SPEC>,
    #[doc = "0x370 - CAN Buffer ID Register"]
    pub buf_23_id: crate::Reg<buf_23_id::BUF_23_ID_SPEC>,
    #[doc = "0x374 - CAN Buffer DLC Register"]
    pub buf_23_dlc: crate::Reg<buf_23_dlc::BUF_23_DLC_SPEC>,
    #[doc = "0x378 - CAN Buffer Data low Register"]
    pub buf_23_datal: crate::Reg<buf_23_datal::BUF_23_DATAL_SPEC>,
    #[doc = "0x37c - CAN Buffer Data high Register"]
    pub buf_23_datah: crate::Reg<buf_23_datah::BUF_23_DATAH_SPEC>,
    #[doc = "0x380 - CAN Buffer ID Register"]
    pub buf_24_id: crate::Reg<buf_24_id::BUF_24_ID_SPEC>,
    #[doc = "0x384 - CAN Buffer DLC Register"]
    pub buf_24_dlc: crate::Reg<buf_24_dlc::BUF_24_DLC_SPEC>,
    #[doc = "0x388 - CAN Buffer Data low Register"]
    pub buf_24_datal: crate::Reg<buf_24_datal::BUF_24_DATAL_SPEC>,
    #[doc = "0x38c - CAN Buffer Data high Register"]
    pub buf_24_datah: crate::Reg<buf_24_datah::BUF_24_DATAH_SPEC>,
    #[doc = "0x390 - CAN Buffer ID Register"]
    pub buf_25_id: crate::Reg<buf_25_id::BUF_25_ID_SPEC>,
    #[doc = "0x394 - CAN Buffer DLC Register"]
    pub buf_25_dlc: crate::Reg<buf_25_dlc::BUF_25_DLC_SPEC>,
    #[doc = "0x398 - CAN Buffer Data low Register"]
    pub buf_25_datal: crate::Reg<buf_25_datal::BUF_25_DATAL_SPEC>,
    #[doc = "0x39c - CAN Buffer Data high Register"]
    pub buf_25_datah: crate::Reg<buf_25_datah::BUF_25_DATAH_SPEC>,
    #[doc = "0x3a0 - CAN Buffer ID Register"]
    pub buf_26_id: crate::Reg<buf_26_id::BUF_26_ID_SPEC>,
    #[doc = "0x3a4 - CAN Buffer DLC Register"]
    pub buf_26_dlc: crate::Reg<buf_26_dlc::BUF_26_DLC_SPEC>,
    #[doc = "0x3a8 - CAN Buffer Data low Register"]
    pub buf_26_datal: crate::Reg<buf_26_datal::BUF_26_DATAL_SPEC>,
    #[doc = "0x3ac - CAN Buffer Data high Register"]
    pub buf_26_datah: crate::Reg<buf_26_datah::BUF_26_DATAH_SPEC>,
    #[doc = "0x3b0 - CAN Buffer ID Register"]
    pub buf_27_id: crate::Reg<buf_27_id::BUF_27_ID_SPEC>,
    #[doc = "0x3b4 - CAN Buffer DLC Register"]
    pub buf_27_dlc: crate::Reg<buf_27_dlc::BUF_27_DLC_SPEC>,
    #[doc = "0x3b8 - CAN Buffer Data low Register"]
    pub buf_27_datal: crate::Reg<buf_27_datal::BUF_27_DATAL_SPEC>,
    #[doc = "0x3bc - CAN Buffer Data high Register"]
    pub buf_27_datah: crate::Reg<buf_27_datah::BUF_27_DATAH_SPEC>,
    #[doc = "0x3c0 - CAN Buffer ID Register"]
    pub buf_28_id: crate::Reg<buf_28_id::BUF_28_ID_SPEC>,
    #[doc = "0x3c4 - CAN Buffer DLC Register"]
    pub buf_28_dlc: crate::Reg<buf_28_dlc::BUF_28_DLC_SPEC>,
    #[doc = "0x3c8 - CAN Buffer Data low Register"]
    pub buf_28_datal: crate::Reg<buf_28_datal::BUF_28_DATAL_SPEC>,
    #[doc = "0x3cc - CAN Buffer Data high Register"]
    pub buf_28_datah: crate::Reg<buf_28_datah::BUF_28_DATAH_SPEC>,
    #[doc = "0x3d0 - CAN Buffer ID Register"]
    pub buf_29_id: crate::Reg<buf_29_id::BUF_29_ID_SPEC>,
    #[doc = "0x3d4 - CAN Buffer DLC Register"]
    pub buf_29_dlc: crate::Reg<buf_29_dlc::BUF_29_DLC_SPEC>,
    #[doc = "0x3d8 - CAN Buffer Data low Register"]
    pub buf_29_datal: crate::Reg<buf_29_datal::BUF_29_DATAL_SPEC>,
    #[doc = "0x3dc - CAN Buffer Data high Register"]
    pub buf_29_datah: crate::Reg<buf_29_datah::BUF_29_DATAH_SPEC>,
    #[doc = "0x3e0 - CAN Buffer ID Register"]
    pub buf_30_id: crate::Reg<buf_30_id::BUF_30_ID_SPEC>,
    #[doc = "0x3e4 - CAN Buffer DLC Register"]
    pub buf_30_dlc: crate::Reg<buf_30_dlc::BUF_30_DLC_SPEC>,
    #[doc = "0x3e8 - CAN Buffer Data low Register"]
    pub buf_30_datal: crate::Reg<buf_30_datal::BUF_30_DATAL_SPEC>,
    #[doc = "0x3ec - CAN Buffer Data high Register"]
    pub buf_30_datah: crate::Reg<buf_30_datah::BUF_30_DATAH_SPEC>,
    #[doc = "0x3f0 - CAN Buffer ID Register"]
    pub buf_31_id: crate::Reg<buf_31_id::BUF_31_ID_SPEC>,
    #[doc = "0x3f4 - CAN Buffer DLC Register"]
    pub buf_31_dlc: crate::Reg<buf_31_dlc::BUF_31_DLC_SPEC>,
    #[doc = "0x3f8 - CAN Buffer Data low Register"]
    pub buf_31_datal: crate::Reg<buf_31_datal::BUF_31_DATAL_SPEC>,
    #[doc = "0x3fc - CAN Buffer Data high Register"]
    pub buf_31_datah: crate::Reg<buf_31_datah::BUF_31_DATAH_SPEC>,
    _reserved177: [u8; 0x0100],
    #[doc = "0x500 - "]
    pub buf_filter00_mask: crate::Reg<buf_filter00_mask::BUF_FILTER00_MASK_SPEC>,
    #[doc = "0x504 - "]
    pub buf_filter00_filter: crate::Reg<buf_filter00_filter::BUF_FILTER00_FILTER_SPEC>,
    #[doc = "0x508 - "]
    pub buf_filter01_mask: crate::Reg<buf_filter01_mask::BUF_FILTER01_MASK_SPEC>,
    #[doc = "0x50c - "]
    pub buf_filter01_filter: crate::Reg<buf_filter01_filter::BUF_FILTER01_FILTER_SPEC>,
    #[doc = "0x510 - "]
    pub buf_filter02_mask: crate::Reg<buf_filter02_mask::BUF_FILTER02_MASK_SPEC>,
    #[doc = "0x514 - "]
    pub buf_filter02_filter: crate::Reg<buf_filter02_filter::BUF_FILTER02_FILTER_SPEC>,
    #[doc = "0x518 - "]
    pub buf_filter03_mask: crate::Reg<buf_filter03_mask::BUF_FILTER03_MASK_SPEC>,
    #[doc = "0x51c - "]
    pub buf_filter03_filter: crate::Reg<buf_filter03_filter::BUF_FILTER03_FILTER_SPEC>,
    #[doc = "0x520 - "]
    pub buf_filter04_mask: crate::Reg<buf_filter04_mask::BUF_FILTER04_MASK_SPEC>,
    #[doc = "0x524 - "]
    pub buf_filter04_filter: crate::Reg<buf_filter04_filter::BUF_FILTER04_FILTER_SPEC>,
    #[doc = "0x528 - "]
    pub buf_filter05_mask: crate::Reg<buf_filter05_mask::BUF_FILTER05_MASK_SPEC>,
    #[doc = "0x52c - "]
    pub buf_filter05_filter: crate::Reg<buf_filter05_filter::BUF_FILTER05_FILTER_SPEC>,
    #[doc = "0x530 - "]
    pub buf_filter06_mask: crate::Reg<buf_filter06_mask::BUF_FILTER06_MASK_SPEC>,
    #[doc = "0x534 - "]
    pub buf_filter06_filter: crate::Reg<buf_filter06_filter::BUF_FILTER06_FILTER_SPEC>,
    #[doc = "0x538 - "]
    pub buf_filter07_mask: crate::Reg<buf_filter07_mask::BUF_FILTER07_MASK_SPEC>,
    #[doc = "0x53c - "]
    pub buf_filter07_filter: crate::Reg<buf_filter07_filter::BUF_FILTER07_FILTER_SPEC>,
    #[doc = "0x540 - "]
    pub buf_filter08_mask: crate::Reg<buf_filter08_mask::BUF_FILTER08_MASK_SPEC>,
    #[doc = "0x544 - "]
    pub buf_filter08_filter: crate::Reg<buf_filter08_filter::BUF_FILTER08_FILTER_SPEC>,
    #[doc = "0x548 - "]
    pub buf_filter09_mask: crate::Reg<buf_filter09_mask::BUF_FILTER09_MASK_SPEC>,
    #[doc = "0x54c - "]
    pub buf_filter09_filter: crate::Reg<buf_filter09_filter::BUF_FILTER09_FILTER_SPEC>,
    #[doc = "0x550 - "]
    pub buf_filter10_mask: crate::Reg<buf_filter10_mask::BUF_FILTER10_MASK_SPEC>,
    #[doc = "0x554 - "]
    pub buf_filter10_filter: crate::Reg<buf_filter10_filter::BUF_FILTER10_FILTER_SPEC>,
    #[doc = "0x558 - "]
    pub buf_filter11_mask: crate::Reg<buf_filter11_mask::BUF_FILTER11_MASK_SPEC>,
    #[doc = "0x55c - "]
    pub buf_filter11_filter: crate::Reg<buf_filter11_filter::BUF_FILTER11_FILTER_SPEC>,
    #[doc = "0x560 - "]
    pub buf_filter12_mask: crate::Reg<buf_filter12_mask::BUF_FILTER12_MASK_SPEC>,
    #[doc = "0x564 - "]
    pub buf_filter12_filter: crate::Reg<buf_filter12_filter::BUF_FILTER12_FILTER_SPEC>,
    #[doc = "0x568 - "]
    pub buf_filter13_mask: crate::Reg<buf_filter13_mask::BUF_FILTER13_MASK_SPEC>,
    #[doc = "0x56c - "]
    pub buf_filter13_filter: crate::Reg<buf_filter13_filter::BUF_FILTER13_FILTER_SPEC>,
    #[doc = "0x570 - "]
    pub buf_filter14_mask: crate::Reg<buf_filter14_mask::BUF_FILTER14_MASK_SPEC>,
    #[doc = "0x574 - "]
    pub buf_filter14_filter: crate::Reg<buf_filter14_filter::BUF_FILTER14_FILTER_SPEC>,
    #[doc = "0x578 - "]
    pub buf_filter15_mask: crate::Reg<buf_filter15_mask::BUF_FILTER15_MASK_SPEC>,
    #[doc = "0x57c - "]
    pub buf_filter15_filter: crate::Reg<buf_filter15_filter::BUF_FILTER15_FILTER_SPEC>,
    #[doc = "0x580 - "]
    pub buf_filter16_mask: crate::Reg<buf_filter16_mask::BUF_FILTER16_MASK_SPEC>,
    #[doc = "0x584 - "]
    pub buf_filter16_filter: crate::Reg<buf_filter16_filter::BUF_FILTER16_FILTER_SPEC>,
    #[doc = "0x588 - "]
    pub buf_filter17_mask: crate::Reg<buf_filter17_mask::BUF_FILTER17_MASK_SPEC>,
    #[doc = "0x58c - "]
    pub buf_filter17_filter: crate::Reg<buf_filter17_filter::BUF_FILTER17_FILTER_SPEC>,
    #[doc = "0x590 - "]
    pub buf_filter18_mask: crate::Reg<buf_filter18_mask::BUF_FILTER18_MASK_SPEC>,
    #[doc = "0x594 - "]
    pub buf_filter18_filter: crate::Reg<buf_filter18_filter::BUF_FILTER18_FILTER_SPEC>,
    #[doc = "0x598 - "]
    pub buf_filter19_mask: crate::Reg<buf_filter19_mask::BUF_FILTER19_MASK_SPEC>,
    #[doc = "0x59c - "]
    pub buf_filter19_filter: crate::Reg<buf_filter19_filter::BUF_FILTER19_FILTER_SPEC>,
    #[doc = "0x5a0 - "]
    pub buf_filter20_mask: crate::Reg<buf_filter20_mask::BUF_FILTER20_MASK_SPEC>,
    #[doc = "0x5a4 - "]
    pub buf_filter20_filter: crate::Reg<buf_filter20_filter::BUF_FILTER20_FILTER_SPEC>,
    #[doc = "0x5a8 - "]
    pub buf_filter21_mask: crate::Reg<buf_filter21_mask::BUF_FILTER21_MASK_SPEC>,
    #[doc = "0x5ac - "]
    pub buf_filter21_filter: crate::Reg<buf_filter21_filter::BUF_FILTER21_FILTER_SPEC>,
    #[doc = "0x5b0 - "]
    pub buf_filter22_mask: crate::Reg<buf_filter22_mask::BUF_FILTER22_MASK_SPEC>,
    #[doc = "0x5b4 - "]
    pub buf_filter22_filter: crate::Reg<buf_filter22_filter::BUF_FILTER22_FILTER_SPEC>,
    #[doc = "0x5b8 - "]
    pub buf_filter23_mask: crate::Reg<buf_filter23_mask::BUF_FILTER23_MASK_SPEC>,
    #[doc = "0x5bc - "]
    pub buf_filter23_filter: crate::Reg<buf_filter23_filter::BUF_FILTER23_FILTER_SPEC>,
    #[doc = "0x5c0 - "]
    pub buf_filter24_mask: crate::Reg<buf_filter24_mask::BUF_FILTER24_MASK_SPEC>,
    #[doc = "0x5c4 - "]
    pub buf_filter24_filter: crate::Reg<buf_filter24_filter::BUF_FILTER24_FILTER_SPEC>,
    #[doc = "0x5c8 - "]
    pub buf_filter25_mask: crate::Reg<buf_filter25_mask::BUF_FILTER25_MASK_SPEC>,
    #[doc = "0x5cc - "]
    pub buf_filter25_filter: crate::Reg<buf_filter25_filter::BUF_FILTER25_FILTER_SPEC>,
    #[doc = "0x5d0 - "]
    pub buf_filter26_mask: crate::Reg<buf_filter26_mask::BUF_FILTER26_MASK_SPEC>,
    #[doc = "0x5d4 - "]
    pub buf_filter26_filter: crate::Reg<buf_filter26_filter::BUF_FILTER26_FILTER_SPEC>,
    #[doc = "0x5d8 - "]
    pub buf_filter27_mask: crate::Reg<buf_filter27_mask::BUF_FILTER27_MASK_SPEC>,
    #[doc = "0x5dc - "]
    pub buf_filter27_filter: crate::Reg<buf_filter27_filter::BUF_FILTER27_FILTER_SPEC>,
    #[doc = "0x5e0 - "]
    pub buf_filter28_mask: crate::Reg<buf_filter28_mask::BUF_FILTER28_MASK_SPEC>,
    #[doc = "0x5e4 - "]
    pub buf_filter28_filter: crate::Reg<buf_filter28_filter::BUF_FILTER28_FILTER_SPEC>,
    #[doc = "0x5e8 - "]
    pub buf_filter29_mask: crate::Reg<buf_filter29_mask::BUF_FILTER29_MASK_SPEC>,
    #[doc = "0x5ec - "]
    pub buf_filter29_filter: crate::Reg<buf_filter29_filter::BUF_FILTER29_FILTER_SPEC>,
    #[doc = "0x5f0 - "]
    pub buf_filter30_mask: crate::Reg<buf_filter30_mask::BUF_FILTER30_MASK_SPEC>,
    #[doc = "0x5f4 - "]
    pub buf_filter30_filter: crate::Reg<buf_filter30_filter::BUF_FILTER30_FILTER_SPEC>,
    #[doc = "0x5f8 - "]
    pub buf_filter31_mask: crate::Reg<buf_filter31_mask::BUF_FILTER31_MASK_SPEC>,
    #[doc = "0x5fc - "]
    pub buf_filter31_filter: crate::Reg<buf_filter31_filter::BUF_FILTER31_FILTER_SPEC>,
}
#[doc = "CONTROL register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "CAN Control Register"]
pub mod control;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "CAN Status Register"]
pub mod status;
#[doc = "BITTMNG register accessor: an alias for `Reg<BITTMNG_SPEC>`"]
pub type BITTMNG = crate::Reg<bittmng::BITTMNG_SPEC>;
#[doc = "CAN Bittiming Register"]
pub mod bittmng;
#[doc = "INT_EN register accessor: an alias for `Reg<INT_EN_SPEC>`"]
pub type INT_EN = crate::Reg<int_en::INT_EN_SPEC>;
#[doc = "CAN Interrupt enable Register"]
pub mod int_en;
#[doc = "OVER register accessor: an alias for `Reg<OVER_SPEC>`"]
pub type OVER = crate::Reg<over::OVER_SPEC>;
#[doc = ""]
pub mod over;
#[doc = "RXID register accessor: an alias for `Reg<RXID_SPEC>`"]
pub type RXID = crate::Reg<rxid::RXID_SPEC>;
#[doc = "CAN Receive ID Register"]
pub mod rxid;
#[doc = "RXDLC register accessor: an alias for `Reg<RXDLC_SPEC>`"]
pub type RXDLC = crate::Reg<rxdlc::RXDLC_SPEC>;
#[doc = "CAN Receive DLC Register"]
pub mod rxdlc;
#[doc = "RXDATAL register accessor: an alias for `Reg<RXDATAL_SPEC>`"]
pub type RXDATAL = crate::Reg<rxdatal::RXDATAL_SPEC>;
#[doc = "CAN Receive Data low Register"]
pub mod rxdatal;
#[doc = "RXDATAH register accessor: an alias for `Reg<RXDATAH_SPEC>`"]
pub type RXDATAH = crate::Reg<rxdatah::RXDATAH_SPEC>;
#[doc = "CAN Receive Data high Register"]
pub mod rxdatah;
#[doc = "TXID register accessor: an alias for `Reg<TXID_SPEC>`"]
pub type TXID = crate::Reg<txid::TXID_SPEC>;
#[doc = "CAN Transmit ID Register"]
pub mod txid;
#[doc = "TXDLC register accessor: an alias for `Reg<TXDLC_SPEC>`"]
pub type TXDLC = crate::Reg<txdlc::TXDLC_SPEC>;
#[doc = "CAN Transmit DLC Register"]
pub mod txdlc;
#[doc = "DATAL register accessor: an alias for `Reg<DATAL_SPEC>`"]
pub type DATAL = crate::Reg<datal::DATAL_SPEC>;
#[doc = "CAN Transmit Data low Register"]
pub mod datal;
#[doc = "DATAH register accessor: an alias for `Reg<DATAH_SPEC>`"]
pub type DATAH = crate::Reg<datah::DATAH_SPEC>;
#[doc = "CAN Transmit Data high Register"]
pub mod datah;
#[doc = "BUF_CON00 register accessor: an alias for `Reg<BUF_CON00_SPEC>`"]
pub type BUF_CON00 = crate::Reg<buf_con00::BUF_CON00_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con00;
#[doc = "BUF_CON01 register accessor: an alias for `Reg<BUF_CON01_SPEC>`"]
pub type BUF_CON01 = crate::Reg<buf_con01::BUF_CON01_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con01;
#[doc = "BUF_CON02 register accessor: an alias for `Reg<BUF_CON02_SPEC>`"]
pub type BUF_CON02 = crate::Reg<buf_con02::BUF_CON02_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con02;
#[doc = "BUF_CON03 register accessor: an alias for `Reg<BUF_CON03_SPEC>`"]
pub type BUF_CON03 = crate::Reg<buf_con03::BUF_CON03_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con03;
#[doc = "BUF_CON04 register accessor: an alias for `Reg<BUF_CON04_SPEC>`"]
pub type BUF_CON04 = crate::Reg<buf_con04::BUF_CON04_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con04;
#[doc = "BUF_CON05 register accessor: an alias for `Reg<BUF_CON05_SPEC>`"]
pub type BUF_CON05 = crate::Reg<buf_con05::BUF_CON05_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con05;
#[doc = "BUF_CON06 register accessor: an alias for `Reg<BUF_CON06_SPEC>`"]
pub type BUF_CON06 = crate::Reg<buf_con06::BUF_CON06_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con06;
#[doc = "BUF_CON07 register accessor: an alias for `Reg<BUF_CON07_SPEC>`"]
pub type BUF_CON07 = crate::Reg<buf_con07::BUF_CON07_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con07;
#[doc = "BUF_CON08 register accessor: an alias for `Reg<BUF_CON08_SPEC>`"]
pub type BUF_CON08 = crate::Reg<buf_con08::BUF_CON08_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con08;
#[doc = "BUF_CON09 register accessor: an alias for `Reg<BUF_CON09_SPEC>`"]
pub type BUF_CON09 = crate::Reg<buf_con09::BUF_CON09_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con09;
#[doc = "BUF_CON10 register accessor: an alias for `Reg<BUF_CON10_SPEC>`"]
pub type BUF_CON10 = crate::Reg<buf_con10::BUF_CON10_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con10;
#[doc = "BUF_CON11 register accessor: an alias for `Reg<BUF_CON11_SPEC>`"]
pub type BUF_CON11 = crate::Reg<buf_con11::BUF_CON11_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con11;
#[doc = "BUF_CON12 register accessor: an alias for `Reg<BUF_CON12_SPEC>`"]
pub type BUF_CON12 = crate::Reg<buf_con12::BUF_CON12_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con12;
#[doc = "BUF_CON13 register accessor: an alias for `Reg<BUF_CON13_SPEC>`"]
pub type BUF_CON13 = crate::Reg<buf_con13::BUF_CON13_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con13;
#[doc = "BUF_CON14 register accessor: an alias for `Reg<BUF_CON14_SPEC>`"]
pub type BUF_CON14 = crate::Reg<buf_con14::BUF_CON14_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con14;
#[doc = "BUF_CON15 register accessor: an alias for `Reg<BUF_CON15_SPEC>`"]
pub type BUF_CON15 = crate::Reg<buf_con15::BUF_CON15_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con15;
#[doc = "BUF_CON16 register accessor: an alias for `Reg<BUF_CON16_SPEC>`"]
pub type BUF_CON16 = crate::Reg<buf_con16::BUF_CON16_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con16;
#[doc = "BUF_CON17 register accessor: an alias for `Reg<BUF_CON17_SPEC>`"]
pub type BUF_CON17 = crate::Reg<buf_con17::BUF_CON17_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con17;
#[doc = "BUF_CON18 register accessor: an alias for `Reg<BUF_CON18_SPEC>`"]
pub type BUF_CON18 = crate::Reg<buf_con18::BUF_CON18_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con18;
#[doc = "BUF_CON19 register accessor: an alias for `Reg<BUF_CON19_SPEC>`"]
pub type BUF_CON19 = crate::Reg<buf_con19::BUF_CON19_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con19;
#[doc = "BUF_CON20 register accessor: an alias for `Reg<BUF_CON20_SPEC>`"]
pub type BUF_CON20 = crate::Reg<buf_con20::BUF_CON20_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con20;
#[doc = "BUF_CON21 register accessor: an alias for `Reg<BUF_CON21_SPEC>`"]
pub type BUF_CON21 = crate::Reg<buf_con21::BUF_CON21_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con21;
#[doc = "BUF_CON22 register accessor: an alias for `Reg<BUF_CON22_SPEC>`"]
pub type BUF_CON22 = crate::Reg<buf_con22::BUF_CON22_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con22;
#[doc = "BUF_CON23 register accessor: an alias for `Reg<BUF_CON23_SPEC>`"]
pub type BUF_CON23 = crate::Reg<buf_con23::BUF_CON23_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con23;
#[doc = "BUF_CON24 register accessor: an alias for `Reg<BUF_CON24_SPEC>`"]
pub type BUF_CON24 = crate::Reg<buf_con24::BUF_CON24_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con24;
#[doc = "BUF_CON25 register accessor: an alias for `Reg<BUF_CON25_SPEC>`"]
pub type BUF_CON25 = crate::Reg<buf_con25::BUF_CON25_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con25;
#[doc = "BUF_CON26 register accessor: an alias for `Reg<BUF_CON26_SPEC>`"]
pub type BUF_CON26 = crate::Reg<buf_con26::BUF_CON26_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con26;
#[doc = "BUF_CON27 register accessor: an alias for `Reg<BUF_CON27_SPEC>`"]
pub type BUF_CON27 = crate::Reg<buf_con27::BUF_CON27_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con27;
#[doc = "BUF_CON28 register accessor: an alias for `Reg<BUF_CON28_SPEC>`"]
pub type BUF_CON28 = crate::Reg<buf_con28::BUF_CON28_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con28;
#[doc = "BUF_CON29 register accessor: an alias for `Reg<BUF_CON29_SPEC>`"]
pub type BUF_CON29 = crate::Reg<buf_con29::BUF_CON29_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con29;
#[doc = "BUF_CON30 register accessor: an alias for `Reg<BUF_CON30_SPEC>`"]
pub type BUF_CON30 = crate::Reg<buf_con30::BUF_CON30_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con30;
#[doc = "BUF_CON31 register accessor: an alias for `Reg<BUF_CON31_SPEC>`"]
pub type BUF_CON31 = crate::Reg<buf_con31::BUF_CON31_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con31;
#[doc = "INT_RX register accessor: an alias for `Reg<INT_RX_SPEC>`"]
pub type INT_RX = crate::Reg<int_rx::INT_RX_SPEC>;
#[doc = ""]
pub mod int_rx;
#[doc = "RX register accessor: an alias for `Reg<RX_SPEC>`"]
pub type RX = crate::Reg<rx::RX_SPEC>;
#[doc = ""]
pub mod rx;
#[doc = "INT_TX register accessor: an alias for `Reg<INT_TX_SPEC>`"]
pub type INT_TX = crate::Reg<int_tx::INT_TX_SPEC>;
#[doc = ""]
pub mod int_tx;
#[doc = "TX register accessor: an alias for `Reg<TX_SPEC>`"]
pub type TX = crate::Reg<tx::TX_SPEC>;
#[doc = ""]
pub mod tx;
#[doc = "BUF_00_ID register accessor: an alias for `Reg<BUF_00_ID_SPEC>`"]
pub type BUF_00_ID = crate::Reg<buf_00_id::BUF_00_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_00_id;
#[doc = "BUF_00_DLC register accessor: an alias for `Reg<BUF_00_DLC_SPEC>`"]
pub type BUF_00_DLC = crate::Reg<buf_00_dlc::BUF_00_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_00_dlc;
#[doc = "BUF_00_DATAL register accessor: an alias for `Reg<BUF_00_DATAL_SPEC>`"]
pub type BUF_00_DATAL = crate::Reg<buf_00_datal::BUF_00_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_00_datal;
#[doc = "BUF_00_DATAH register accessor: an alias for `Reg<BUF_00_DATAH_SPEC>`"]
pub type BUF_00_DATAH = crate::Reg<buf_00_datah::BUF_00_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_00_datah;
#[doc = "BUF_01_ID register accessor: an alias for `Reg<BUF_01_ID_SPEC>`"]
pub type BUF_01_ID = crate::Reg<buf_01_id::BUF_01_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_01_id;
#[doc = "BUF_01_DLC register accessor: an alias for `Reg<BUF_01_DLC_SPEC>`"]
pub type BUF_01_DLC = crate::Reg<buf_01_dlc::BUF_01_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_01_dlc;
#[doc = "BUF_01_DATAL register accessor: an alias for `Reg<BUF_01_DATAL_SPEC>`"]
pub type BUF_01_DATAL = crate::Reg<buf_01_datal::BUF_01_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_01_datal;
#[doc = "BUF_01_DATAH register accessor: an alias for `Reg<BUF_01_DATAH_SPEC>`"]
pub type BUF_01_DATAH = crate::Reg<buf_01_datah::BUF_01_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_01_datah;
#[doc = "BUF_02_ID register accessor: an alias for `Reg<BUF_02_ID_SPEC>`"]
pub type BUF_02_ID = crate::Reg<buf_02_id::BUF_02_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_02_id;
#[doc = "BUF_02_DLC register accessor: an alias for `Reg<BUF_02_DLC_SPEC>`"]
pub type BUF_02_DLC = crate::Reg<buf_02_dlc::BUF_02_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_02_dlc;
#[doc = "BUF_02_DATAL register accessor: an alias for `Reg<BUF_02_DATAL_SPEC>`"]
pub type BUF_02_DATAL = crate::Reg<buf_02_datal::BUF_02_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_02_datal;
#[doc = "BUF_02_DATAH register accessor: an alias for `Reg<BUF_02_DATAH_SPEC>`"]
pub type BUF_02_DATAH = crate::Reg<buf_02_datah::BUF_02_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_02_datah;
#[doc = "BUF_03_ID register accessor: an alias for `Reg<BUF_03_ID_SPEC>`"]
pub type BUF_03_ID = crate::Reg<buf_03_id::BUF_03_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_03_id;
#[doc = "BUF_03_DLC register accessor: an alias for `Reg<BUF_03_DLC_SPEC>`"]
pub type BUF_03_DLC = crate::Reg<buf_03_dlc::BUF_03_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_03_dlc;
#[doc = "BUF_03_DATAL register accessor: an alias for `Reg<BUF_03_DATAL_SPEC>`"]
pub type BUF_03_DATAL = crate::Reg<buf_03_datal::BUF_03_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_03_datal;
#[doc = "BUF_03_DATAH register accessor: an alias for `Reg<BUF_03_DATAH_SPEC>`"]
pub type BUF_03_DATAH = crate::Reg<buf_03_datah::BUF_03_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_03_datah;
#[doc = "BUF_04_ID register accessor: an alias for `Reg<BUF_04_ID_SPEC>`"]
pub type BUF_04_ID = crate::Reg<buf_04_id::BUF_04_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_04_id;
#[doc = "BUF_04_DLC register accessor: an alias for `Reg<BUF_04_DLC_SPEC>`"]
pub type BUF_04_DLC = crate::Reg<buf_04_dlc::BUF_04_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_04_dlc;
#[doc = "BUF_04_DATAL register accessor: an alias for `Reg<BUF_04_DATAL_SPEC>`"]
pub type BUF_04_DATAL = crate::Reg<buf_04_datal::BUF_04_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_04_datal;
#[doc = "BUF_04_DATAH register accessor: an alias for `Reg<BUF_04_DATAH_SPEC>`"]
pub type BUF_04_DATAH = crate::Reg<buf_04_datah::BUF_04_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_04_datah;
#[doc = "BUF_05_ID register accessor: an alias for `Reg<BUF_05_ID_SPEC>`"]
pub type BUF_05_ID = crate::Reg<buf_05_id::BUF_05_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_05_id;
#[doc = "BUF_05_DLC register accessor: an alias for `Reg<BUF_05_DLC_SPEC>`"]
pub type BUF_05_DLC = crate::Reg<buf_05_dlc::BUF_05_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_05_dlc;
#[doc = "BUF_05_DATAL register accessor: an alias for `Reg<BUF_05_DATAL_SPEC>`"]
pub type BUF_05_DATAL = crate::Reg<buf_05_datal::BUF_05_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_05_datal;
#[doc = "BUF_05_DATAH register accessor: an alias for `Reg<BUF_05_DATAH_SPEC>`"]
pub type BUF_05_DATAH = crate::Reg<buf_05_datah::BUF_05_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_05_datah;
#[doc = "BUF_06_ID register accessor: an alias for `Reg<BUF_06_ID_SPEC>`"]
pub type BUF_06_ID = crate::Reg<buf_06_id::BUF_06_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_06_id;
#[doc = "BUF_06_DLC register accessor: an alias for `Reg<BUF_06_DLC_SPEC>`"]
pub type BUF_06_DLC = crate::Reg<buf_06_dlc::BUF_06_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_06_dlc;
#[doc = "BUF_06_DATAL register accessor: an alias for `Reg<BUF_06_DATAL_SPEC>`"]
pub type BUF_06_DATAL = crate::Reg<buf_06_datal::BUF_06_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_06_datal;
#[doc = "BUF_06_DATAH register accessor: an alias for `Reg<BUF_06_DATAH_SPEC>`"]
pub type BUF_06_DATAH = crate::Reg<buf_06_datah::BUF_06_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_06_datah;
#[doc = "BUF_07_ID register accessor: an alias for `Reg<BUF_07_ID_SPEC>`"]
pub type BUF_07_ID = crate::Reg<buf_07_id::BUF_07_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_07_id;
#[doc = "BUF_07_DLC register accessor: an alias for `Reg<BUF_07_DLC_SPEC>`"]
pub type BUF_07_DLC = crate::Reg<buf_07_dlc::BUF_07_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_07_dlc;
#[doc = "BUF_07_DATAL register accessor: an alias for `Reg<BUF_07_DATAL_SPEC>`"]
pub type BUF_07_DATAL = crate::Reg<buf_07_datal::BUF_07_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_07_datal;
#[doc = "BUF_07_DATAH register accessor: an alias for `Reg<BUF_07_DATAH_SPEC>`"]
pub type BUF_07_DATAH = crate::Reg<buf_07_datah::BUF_07_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_07_datah;
#[doc = "BUF_08_ID register accessor: an alias for `Reg<BUF_08_ID_SPEC>`"]
pub type BUF_08_ID = crate::Reg<buf_08_id::BUF_08_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_08_id;
#[doc = "BUF_08_DLC register accessor: an alias for `Reg<BUF_08_DLC_SPEC>`"]
pub type BUF_08_DLC = crate::Reg<buf_08_dlc::BUF_08_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_08_dlc;
#[doc = "BUF_08_DATAL register accessor: an alias for `Reg<BUF_08_DATAL_SPEC>`"]
pub type BUF_08_DATAL = crate::Reg<buf_08_datal::BUF_08_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_08_datal;
#[doc = "BUF_08_DATAH register accessor: an alias for `Reg<BUF_08_DATAH_SPEC>`"]
pub type BUF_08_DATAH = crate::Reg<buf_08_datah::BUF_08_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_08_datah;
#[doc = "BUF_09_ID register accessor: an alias for `Reg<BUF_09_ID_SPEC>`"]
pub type BUF_09_ID = crate::Reg<buf_09_id::BUF_09_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_09_id;
#[doc = "BUF_09_DLC register accessor: an alias for `Reg<BUF_09_DLC_SPEC>`"]
pub type BUF_09_DLC = crate::Reg<buf_09_dlc::BUF_09_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_09_dlc;
#[doc = "BUF_09_DATAL register accessor: an alias for `Reg<BUF_09_DATAL_SPEC>`"]
pub type BUF_09_DATAL = crate::Reg<buf_09_datal::BUF_09_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_09_datal;
#[doc = "BUF_09_DATAH register accessor: an alias for `Reg<BUF_09_DATAH_SPEC>`"]
pub type BUF_09_DATAH = crate::Reg<buf_09_datah::BUF_09_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_09_datah;
#[doc = "BUF_10_ID register accessor: an alias for `Reg<BUF_10_ID_SPEC>`"]
pub type BUF_10_ID = crate::Reg<buf_10_id::BUF_10_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_10_id;
#[doc = "BUF_10_DLC register accessor: an alias for `Reg<BUF_10_DLC_SPEC>`"]
pub type BUF_10_DLC = crate::Reg<buf_10_dlc::BUF_10_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_10_dlc;
#[doc = "BUF_10_DATAL register accessor: an alias for `Reg<BUF_10_DATAL_SPEC>`"]
pub type BUF_10_DATAL = crate::Reg<buf_10_datal::BUF_10_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_10_datal;
#[doc = "BUF_10_DATAH register accessor: an alias for `Reg<BUF_10_DATAH_SPEC>`"]
pub type BUF_10_DATAH = crate::Reg<buf_10_datah::BUF_10_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_10_datah;
#[doc = "BUF_11_ID register accessor: an alias for `Reg<BUF_11_ID_SPEC>`"]
pub type BUF_11_ID = crate::Reg<buf_11_id::BUF_11_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_11_id;
#[doc = "BUF_11_DLC register accessor: an alias for `Reg<BUF_11_DLC_SPEC>`"]
pub type BUF_11_DLC = crate::Reg<buf_11_dlc::BUF_11_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_11_dlc;
#[doc = "BUF_11_DATAL register accessor: an alias for `Reg<BUF_11_DATAL_SPEC>`"]
pub type BUF_11_DATAL = crate::Reg<buf_11_datal::BUF_11_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_11_datal;
#[doc = "BUF_11_DATAH register accessor: an alias for `Reg<BUF_11_DATAH_SPEC>`"]
pub type BUF_11_DATAH = crate::Reg<buf_11_datah::BUF_11_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_11_datah;
#[doc = "BUF_12_ID register accessor: an alias for `Reg<BUF_12_ID_SPEC>`"]
pub type BUF_12_ID = crate::Reg<buf_12_id::BUF_12_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_12_id;
#[doc = "BUF_12_DLC register accessor: an alias for `Reg<BUF_12_DLC_SPEC>`"]
pub type BUF_12_DLC = crate::Reg<buf_12_dlc::BUF_12_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_12_dlc;
#[doc = "BUF_12_DATAL register accessor: an alias for `Reg<BUF_12_DATAL_SPEC>`"]
pub type BUF_12_DATAL = crate::Reg<buf_12_datal::BUF_12_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_12_datal;
#[doc = "BUF_12_DATAH register accessor: an alias for `Reg<BUF_12_DATAH_SPEC>`"]
pub type BUF_12_DATAH = crate::Reg<buf_12_datah::BUF_12_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_12_datah;
#[doc = "BUF_13_ID register accessor: an alias for `Reg<BUF_13_ID_SPEC>`"]
pub type BUF_13_ID = crate::Reg<buf_13_id::BUF_13_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_13_id;
#[doc = "BUF_13_DLC register accessor: an alias for `Reg<BUF_13_DLC_SPEC>`"]
pub type BUF_13_DLC = crate::Reg<buf_13_dlc::BUF_13_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_13_dlc;
#[doc = "BUF_13_DATAL register accessor: an alias for `Reg<BUF_13_DATAL_SPEC>`"]
pub type BUF_13_DATAL = crate::Reg<buf_13_datal::BUF_13_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_13_datal;
#[doc = "BUF_13_DATAH register accessor: an alias for `Reg<BUF_13_DATAH_SPEC>`"]
pub type BUF_13_DATAH = crate::Reg<buf_13_datah::BUF_13_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_13_datah;
#[doc = "BUF_14_ID register accessor: an alias for `Reg<BUF_14_ID_SPEC>`"]
pub type BUF_14_ID = crate::Reg<buf_14_id::BUF_14_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_14_id;
#[doc = "BUF_14_DLC register accessor: an alias for `Reg<BUF_14_DLC_SPEC>`"]
pub type BUF_14_DLC = crate::Reg<buf_14_dlc::BUF_14_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_14_dlc;
#[doc = "BUF_14_DATAL register accessor: an alias for `Reg<BUF_14_DATAL_SPEC>`"]
pub type BUF_14_DATAL = crate::Reg<buf_14_datal::BUF_14_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_14_datal;
#[doc = "BUF_14_DATAH register accessor: an alias for `Reg<BUF_14_DATAH_SPEC>`"]
pub type BUF_14_DATAH = crate::Reg<buf_14_datah::BUF_14_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_14_datah;
#[doc = "BUF_15_ID register accessor: an alias for `Reg<BUF_15_ID_SPEC>`"]
pub type BUF_15_ID = crate::Reg<buf_15_id::BUF_15_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_15_id;
#[doc = "BUF_15_DLC register accessor: an alias for `Reg<BUF_15_DLC_SPEC>`"]
pub type BUF_15_DLC = crate::Reg<buf_15_dlc::BUF_15_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_15_dlc;
#[doc = "BUF_15_DATAL register accessor: an alias for `Reg<BUF_15_DATAL_SPEC>`"]
pub type BUF_15_DATAL = crate::Reg<buf_15_datal::BUF_15_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_15_datal;
#[doc = "BUF_15_DATAH register accessor: an alias for `Reg<BUF_15_DATAH_SPEC>`"]
pub type BUF_15_DATAH = crate::Reg<buf_15_datah::BUF_15_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_15_datah;
#[doc = "BUF_16_ID register accessor: an alias for `Reg<BUF_16_ID_SPEC>`"]
pub type BUF_16_ID = crate::Reg<buf_16_id::BUF_16_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_16_id;
#[doc = "BUF_16_DLC register accessor: an alias for `Reg<BUF_16_DLC_SPEC>`"]
pub type BUF_16_DLC = crate::Reg<buf_16_dlc::BUF_16_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_16_dlc;
#[doc = "BUF_16_DATAL register accessor: an alias for `Reg<BUF_16_DATAL_SPEC>`"]
pub type BUF_16_DATAL = crate::Reg<buf_16_datal::BUF_16_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_16_datal;
#[doc = "BUF_16_DATAH register accessor: an alias for `Reg<BUF_16_DATAH_SPEC>`"]
pub type BUF_16_DATAH = crate::Reg<buf_16_datah::BUF_16_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_16_datah;
#[doc = "BUF_17_ID register accessor: an alias for `Reg<BUF_17_ID_SPEC>`"]
pub type BUF_17_ID = crate::Reg<buf_17_id::BUF_17_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_17_id;
#[doc = "BUF_17_DLC register accessor: an alias for `Reg<BUF_17_DLC_SPEC>`"]
pub type BUF_17_DLC = crate::Reg<buf_17_dlc::BUF_17_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_17_dlc;
#[doc = "BUF_17_DATAL register accessor: an alias for `Reg<BUF_17_DATAL_SPEC>`"]
pub type BUF_17_DATAL = crate::Reg<buf_17_datal::BUF_17_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_17_datal;
#[doc = "BUF_17_DATAH register accessor: an alias for `Reg<BUF_17_DATAH_SPEC>`"]
pub type BUF_17_DATAH = crate::Reg<buf_17_datah::BUF_17_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_17_datah;
#[doc = "BUF_18_ID register accessor: an alias for `Reg<BUF_18_ID_SPEC>`"]
pub type BUF_18_ID = crate::Reg<buf_18_id::BUF_18_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_18_id;
#[doc = "BUF_18_DLC register accessor: an alias for `Reg<BUF_18_DLC_SPEC>`"]
pub type BUF_18_DLC = crate::Reg<buf_18_dlc::BUF_18_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_18_dlc;
#[doc = "BUF_18_DATAL register accessor: an alias for `Reg<BUF_18_DATAL_SPEC>`"]
pub type BUF_18_DATAL = crate::Reg<buf_18_datal::BUF_18_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_18_datal;
#[doc = "BUF_18_DATAH register accessor: an alias for `Reg<BUF_18_DATAH_SPEC>`"]
pub type BUF_18_DATAH = crate::Reg<buf_18_datah::BUF_18_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_18_datah;
#[doc = "BUF_19_ID register accessor: an alias for `Reg<BUF_19_ID_SPEC>`"]
pub type BUF_19_ID = crate::Reg<buf_19_id::BUF_19_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_19_id;
#[doc = "BUF_19_DLC register accessor: an alias for `Reg<BUF_19_DLC_SPEC>`"]
pub type BUF_19_DLC = crate::Reg<buf_19_dlc::BUF_19_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_19_dlc;
#[doc = "BUF_19_DATAL register accessor: an alias for `Reg<BUF_19_DATAL_SPEC>`"]
pub type BUF_19_DATAL = crate::Reg<buf_19_datal::BUF_19_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_19_datal;
#[doc = "BUF_19_DATAH register accessor: an alias for `Reg<BUF_19_DATAH_SPEC>`"]
pub type BUF_19_DATAH = crate::Reg<buf_19_datah::BUF_19_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_19_datah;
#[doc = "BUF_20_ID register accessor: an alias for `Reg<BUF_20_ID_SPEC>`"]
pub type BUF_20_ID = crate::Reg<buf_20_id::BUF_20_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_20_id;
#[doc = "BUF_20_DLC register accessor: an alias for `Reg<BUF_20_DLC_SPEC>`"]
pub type BUF_20_DLC = crate::Reg<buf_20_dlc::BUF_20_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_20_dlc;
#[doc = "BUF_20_DATAL register accessor: an alias for `Reg<BUF_20_DATAL_SPEC>`"]
pub type BUF_20_DATAL = crate::Reg<buf_20_datal::BUF_20_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_20_datal;
#[doc = "BUF_20_DATAH register accessor: an alias for `Reg<BUF_20_DATAH_SPEC>`"]
pub type BUF_20_DATAH = crate::Reg<buf_20_datah::BUF_20_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_20_datah;
#[doc = "BUF_21_ID register accessor: an alias for `Reg<BUF_21_ID_SPEC>`"]
pub type BUF_21_ID = crate::Reg<buf_21_id::BUF_21_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_21_id;
#[doc = "BUF_21_DLC register accessor: an alias for `Reg<BUF_21_DLC_SPEC>`"]
pub type BUF_21_DLC = crate::Reg<buf_21_dlc::BUF_21_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_21_dlc;
#[doc = "BUF_21_DATAL register accessor: an alias for `Reg<BUF_21_DATAL_SPEC>`"]
pub type BUF_21_DATAL = crate::Reg<buf_21_datal::BUF_21_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_21_datal;
#[doc = "BUF_21_DATAH register accessor: an alias for `Reg<BUF_21_DATAH_SPEC>`"]
pub type BUF_21_DATAH = crate::Reg<buf_21_datah::BUF_21_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_21_datah;
#[doc = "BUF_22_ID register accessor: an alias for `Reg<BUF_22_ID_SPEC>`"]
pub type BUF_22_ID = crate::Reg<buf_22_id::BUF_22_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_22_id;
#[doc = "BUF_22_DLC register accessor: an alias for `Reg<BUF_22_DLC_SPEC>`"]
pub type BUF_22_DLC = crate::Reg<buf_22_dlc::BUF_22_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_22_dlc;
#[doc = "BUF_22_DATAL register accessor: an alias for `Reg<BUF_22_DATAL_SPEC>`"]
pub type BUF_22_DATAL = crate::Reg<buf_22_datal::BUF_22_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_22_datal;
#[doc = "BUF_22_DATAH register accessor: an alias for `Reg<BUF_22_DATAH_SPEC>`"]
pub type BUF_22_DATAH = crate::Reg<buf_22_datah::BUF_22_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_22_datah;
#[doc = "BUF_23_ID register accessor: an alias for `Reg<BUF_23_ID_SPEC>`"]
pub type BUF_23_ID = crate::Reg<buf_23_id::BUF_23_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_23_id;
#[doc = "BUF_23_DLC register accessor: an alias for `Reg<BUF_23_DLC_SPEC>`"]
pub type BUF_23_DLC = crate::Reg<buf_23_dlc::BUF_23_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_23_dlc;
#[doc = "BUF_23_DATAL register accessor: an alias for `Reg<BUF_23_DATAL_SPEC>`"]
pub type BUF_23_DATAL = crate::Reg<buf_23_datal::BUF_23_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_23_datal;
#[doc = "BUF_23_DATAH register accessor: an alias for `Reg<BUF_23_DATAH_SPEC>`"]
pub type BUF_23_DATAH = crate::Reg<buf_23_datah::BUF_23_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_23_datah;
#[doc = "BUF_24_ID register accessor: an alias for `Reg<BUF_24_ID_SPEC>`"]
pub type BUF_24_ID = crate::Reg<buf_24_id::BUF_24_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_24_id;
#[doc = "BUF_24_DLC register accessor: an alias for `Reg<BUF_24_DLC_SPEC>`"]
pub type BUF_24_DLC = crate::Reg<buf_24_dlc::BUF_24_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_24_dlc;
#[doc = "BUF_24_DATAL register accessor: an alias for `Reg<BUF_24_DATAL_SPEC>`"]
pub type BUF_24_DATAL = crate::Reg<buf_24_datal::BUF_24_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_24_datal;
#[doc = "BUF_24_DATAH register accessor: an alias for `Reg<BUF_24_DATAH_SPEC>`"]
pub type BUF_24_DATAH = crate::Reg<buf_24_datah::BUF_24_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_24_datah;
#[doc = "BUF_25_ID register accessor: an alias for `Reg<BUF_25_ID_SPEC>`"]
pub type BUF_25_ID = crate::Reg<buf_25_id::BUF_25_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_25_id;
#[doc = "BUF_25_DLC register accessor: an alias for `Reg<BUF_25_DLC_SPEC>`"]
pub type BUF_25_DLC = crate::Reg<buf_25_dlc::BUF_25_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_25_dlc;
#[doc = "BUF_25_DATAL register accessor: an alias for `Reg<BUF_25_DATAL_SPEC>`"]
pub type BUF_25_DATAL = crate::Reg<buf_25_datal::BUF_25_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_25_datal;
#[doc = "BUF_25_DATAH register accessor: an alias for `Reg<BUF_25_DATAH_SPEC>`"]
pub type BUF_25_DATAH = crate::Reg<buf_25_datah::BUF_25_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_25_datah;
#[doc = "BUF_26_ID register accessor: an alias for `Reg<BUF_26_ID_SPEC>`"]
pub type BUF_26_ID = crate::Reg<buf_26_id::BUF_26_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_26_id;
#[doc = "BUF_26_DLC register accessor: an alias for `Reg<BUF_26_DLC_SPEC>`"]
pub type BUF_26_DLC = crate::Reg<buf_26_dlc::BUF_26_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_26_dlc;
#[doc = "BUF_26_DATAL register accessor: an alias for `Reg<BUF_26_DATAL_SPEC>`"]
pub type BUF_26_DATAL = crate::Reg<buf_26_datal::BUF_26_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_26_datal;
#[doc = "BUF_26_DATAH register accessor: an alias for `Reg<BUF_26_DATAH_SPEC>`"]
pub type BUF_26_DATAH = crate::Reg<buf_26_datah::BUF_26_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_26_datah;
#[doc = "BUF_27_ID register accessor: an alias for `Reg<BUF_27_ID_SPEC>`"]
pub type BUF_27_ID = crate::Reg<buf_27_id::BUF_27_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_27_id;
#[doc = "BUF_27_DLC register accessor: an alias for `Reg<BUF_27_DLC_SPEC>`"]
pub type BUF_27_DLC = crate::Reg<buf_27_dlc::BUF_27_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_27_dlc;
#[doc = "BUF_27_DATAL register accessor: an alias for `Reg<BUF_27_DATAL_SPEC>`"]
pub type BUF_27_DATAL = crate::Reg<buf_27_datal::BUF_27_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_27_datal;
#[doc = "BUF_27_DATAH register accessor: an alias for `Reg<BUF_27_DATAH_SPEC>`"]
pub type BUF_27_DATAH = crate::Reg<buf_27_datah::BUF_27_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_27_datah;
#[doc = "BUF_28_ID register accessor: an alias for `Reg<BUF_28_ID_SPEC>`"]
pub type BUF_28_ID = crate::Reg<buf_28_id::BUF_28_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_28_id;
#[doc = "BUF_28_DLC register accessor: an alias for `Reg<BUF_28_DLC_SPEC>`"]
pub type BUF_28_DLC = crate::Reg<buf_28_dlc::BUF_28_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_28_dlc;
#[doc = "BUF_28_DATAL register accessor: an alias for `Reg<BUF_28_DATAL_SPEC>`"]
pub type BUF_28_DATAL = crate::Reg<buf_28_datal::BUF_28_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_28_datal;
#[doc = "BUF_28_DATAH register accessor: an alias for `Reg<BUF_28_DATAH_SPEC>`"]
pub type BUF_28_DATAH = crate::Reg<buf_28_datah::BUF_28_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_28_datah;
#[doc = "BUF_29_ID register accessor: an alias for `Reg<BUF_29_ID_SPEC>`"]
pub type BUF_29_ID = crate::Reg<buf_29_id::BUF_29_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_29_id;
#[doc = "BUF_29_DLC register accessor: an alias for `Reg<BUF_29_DLC_SPEC>`"]
pub type BUF_29_DLC = crate::Reg<buf_29_dlc::BUF_29_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_29_dlc;
#[doc = "BUF_29_DATAL register accessor: an alias for `Reg<BUF_29_DATAL_SPEC>`"]
pub type BUF_29_DATAL = crate::Reg<buf_29_datal::BUF_29_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_29_datal;
#[doc = "BUF_29_DATAH register accessor: an alias for `Reg<BUF_29_DATAH_SPEC>`"]
pub type BUF_29_DATAH = crate::Reg<buf_29_datah::BUF_29_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_29_datah;
#[doc = "BUF_30_ID register accessor: an alias for `Reg<BUF_30_ID_SPEC>`"]
pub type BUF_30_ID = crate::Reg<buf_30_id::BUF_30_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_30_id;
#[doc = "BUF_30_DLC register accessor: an alias for `Reg<BUF_30_DLC_SPEC>`"]
pub type BUF_30_DLC = crate::Reg<buf_30_dlc::BUF_30_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_30_dlc;
#[doc = "BUF_30_DATAL register accessor: an alias for `Reg<BUF_30_DATAL_SPEC>`"]
pub type BUF_30_DATAL = crate::Reg<buf_30_datal::BUF_30_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_30_datal;
#[doc = "BUF_30_DATAH register accessor: an alias for `Reg<BUF_30_DATAH_SPEC>`"]
pub type BUF_30_DATAH = crate::Reg<buf_30_datah::BUF_30_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_30_datah;
#[doc = "BUF_31_ID register accessor: an alias for `Reg<BUF_31_ID_SPEC>`"]
pub type BUF_31_ID = crate::Reg<buf_31_id::BUF_31_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_31_id;
#[doc = "BUF_31_DLC register accessor: an alias for `Reg<BUF_31_DLC_SPEC>`"]
pub type BUF_31_DLC = crate::Reg<buf_31_dlc::BUF_31_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_31_dlc;
#[doc = "BUF_31_DATAL register accessor: an alias for `Reg<BUF_31_DATAL_SPEC>`"]
pub type BUF_31_DATAL = crate::Reg<buf_31_datal::BUF_31_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_31_datal;
#[doc = "BUF_31_DATAH register accessor: an alias for `Reg<BUF_31_DATAH_SPEC>`"]
pub type BUF_31_DATAH = crate::Reg<buf_31_datah::BUF_31_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_31_datah;
#[doc = "BUF_FILTER00_MASK register accessor: an alias for `Reg<BUF_FILTER00_MASK_SPEC>`"]
pub type BUF_FILTER00_MASK = crate::Reg<buf_filter00_mask::BUF_FILTER00_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter00_mask;
#[doc = "BUF_FILTER00_FILTER register accessor: an alias for `Reg<BUF_FILTER00_FILTER_SPEC>`"]
pub type BUF_FILTER00_FILTER = crate::Reg<buf_filter00_filter::BUF_FILTER00_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter00_filter;
#[doc = "BUF_FILTER01_MASK register accessor: an alias for `Reg<BUF_FILTER01_MASK_SPEC>`"]
pub type BUF_FILTER01_MASK = crate::Reg<buf_filter01_mask::BUF_FILTER01_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter01_mask;
#[doc = "BUF_FILTER01_FILTER register accessor: an alias for `Reg<BUF_FILTER01_FILTER_SPEC>`"]
pub type BUF_FILTER01_FILTER = crate::Reg<buf_filter01_filter::BUF_FILTER01_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter01_filter;
#[doc = "BUF_FILTER02_MASK register accessor: an alias for `Reg<BUF_FILTER02_MASK_SPEC>`"]
pub type BUF_FILTER02_MASK = crate::Reg<buf_filter02_mask::BUF_FILTER02_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter02_mask;
#[doc = "BUF_FILTER02_FILTER register accessor: an alias for `Reg<BUF_FILTER02_FILTER_SPEC>`"]
pub type BUF_FILTER02_FILTER = crate::Reg<buf_filter02_filter::BUF_FILTER02_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter02_filter;
#[doc = "BUF_FILTER03_MASK register accessor: an alias for `Reg<BUF_FILTER03_MASK_SPEC>`"]
pub type BUF_FILTER03_MASK = crate::Reg<buf_filter03_mask::BUF_FILTER03_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter03_mask;
#[doc = "BUF_FILTER03_FILTER register accessor: an alias for `Reg<BUF_FILTER03_FILTER_SPEC>`"]
pub type BUF_FILTER03_FILTER = crate::Reg<buf_filter03_filter::BUF_FILTER03_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter03_filter;
#[doc = "BUF_FILTER04_MASK register accessor: an alias for `Reg<BUF_FILTER04_MASK_SPEC>`"]
pub type BUF_FILTER04_MASK = crate::Reg<buf_filter04_mask::BUF_FILTER04_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter04_mask;
#[doc = "BUF_FILTER04_FILTER register accessor: an alias for `Reg<BUF_FILTER04_FILTER_SPEC>`"]
pub type BUF_FILTER04_FILTER = crate::Reg<buf_filter04_filter::BUF_FILTER04_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter04_filter;
#[doc = "BUF_FILTER05_MASK register accessor: an alias for `Reg<BUF_FILTER05_MASK_SPEC>`"]
pub type BUF_FILTER05_MASK = crate::Reg<buf_filter05_mask::BUF_FILTER05_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter05_mask;
#[doc = "BUF_FILTER05_FILTER register accessor: an alias for `Reg<BUF_FILTER05_FILTER_SPEC>`"]
pub type BUF_FILTER05_FILTER = crate::Reg<buf_filter05_filter::BUF_FILTER05_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter05_filter;
#[doc = "BUF_FILTER06_MASK register accessor: an alias for `Reg<BUF_FILTER06_MASK_SPEC>`"]
pub type BUF_FILTER06_MASK = crate::Reg<buf_filter06_mask::BUF_FILTER06_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter06_mask;
#[doc = "BUF_FILTER06_FILTER register accessor: an alias for `Reg<BUF_FILTER06_FILTER_SPEC>`"]
pub type BUF_FILTER06_FILTER = crate::Reg<buf_filter06_filter::BUF_FILTER06_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter06_filter;
#[doc = "BUF_FILTER07_MASK register accessor: an alias for `Reg<BUF_FILTER07_MASK_SPEC>`"]
pub type BUF_FILTER07_MASK = crate::Reg<buf_filter07_mask::BUF_FILTER07_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter07_mask;
#[doc = "BUF_FILTER07_FILTER register accessor: an alias for `Reg<BUF_FILTER07_FILTER_SPEC>`"]
pub type BUF_FILTER07_FILTER = crate::Reg<buf_filter07_filter::BUF_FILTER07_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter07_filter;
#[doc = "BUF_FILTER08_MASK register accessor: an alias for `Reg<BUF_FILTER08_MASK_SPEC>`"]
pub type BUF_FILTER08_MASK = crate::Reg<buf_filter08_mask::BUF_FILTER08_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter08_mask;
#[doc = "BUF_FILTER08_FILTER register accessor: an alias for `Reg<BUF_FILTER08_FILTER_SPEC>`"]
pub type BUF_FILTER08_FILTER = crate::Reg<buf_filter08_filter::BUF_FILTER08_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter08_filter;
#[doc = "BUF_FILTER09_MASK register accessor: an alias for `Reg<BUF_FILTER09_MASK_SPEC>`"]
pub type BUF_FILTER09_MASK = crate::Reg<buf_filter09_mask::BUF_FILTER09_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter09_mask;
#[doc = "BUF_FILTER09_FILTER register accessor: an alias for `Reg<BUF_FILTER09_FILTER_SPEC>`"]
pub type BUF_FILTER09_FILTER = crate::Reg<buf_filter09_filter::BUF_FILTER09_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter09_filter;
#[doc = "BUF_FILTER10_MASK register accessor: an alias for `Reg<BUF_FILTER10_MASK_SPEC>`"]
pub type BUF_FILTER10_MASK = crate::Reg<buf_filter10_mask::BUF_FILTER10_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter10_mask;
#[doc = "BUF_FILTER10_FILTER register accessor: an alias for `Reg<BUF_FILTER10_FILTER_SPEC>`"]
pub type BUF_FILTER10_FILTER = crate::Reg<buf_filter10_filter::BUF_FILTER10_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter10_filter;
#[doc = "BUF_FILTER11_MASK register accessor: an alias for `Reg<BUF_FILTER11_MASK_SPEC>`"]
pub type BUF_FILTER11_MASK = crate::Reg<buf_filter11_mask::BUF_FILTER11_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter11_mask;
#[doc = "BUF_FILTER11_FILTER register accessor: an alias for `Reg<BUF_FILTER11_FILTER_SPEC>`"]
pub type BUF_FILTER11_FILTER = crate::Reg<buf_filter11_filter::BUF_FILTER11_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter11_filter;
#[doc = "BUF_FILTER12_MASK register accessor: an alias for `Reg<BUF_FILTER12_MASK_SPEC>`"]
pub type BUF_FILTER12_MASK = crate::Reg<buf_filter12_mask::BUF_FILTER12_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter12_mask;
#[doc = "BUF_FILTER12_FILTER register accessor: an alias for `Reg<BUF_FILTER12_FILTER_SPEC>`"]
pub type BUF_FILTER12_FILTER = crate::Reg<buf_filter12_filter::BUF_FILTER12_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter12_filter;
#[doc = "BUF_FILTER13_MASK register accessor: an alias for `Reg<BUF_FILTER13_MASK_SPEC>`"]
pub type BUF_FILTER13_MASK = crate::Reg<buf_filter13_mask::BUF_FILTER13_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter13_mask;
#[doc = "BUF_FILTER13_FILTER register accessor: an alias for `Reg<BUF_FILTER13_FILTER_SPEC>`"]
pub type BUF_FILTER13_FILTER = crate::Reg<buf_filter13_filter::BUF_FILTER13_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter13_filter;
#[doc = "BUF_FILTER14_MASK register accessor: an alias for `Reg<BUF_FILTER14_MASK_SPEC>`"]
pub type BUF_FILTER14_MASK = crate::Reg<buf_filter14_mask::BUF_FILTER14_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter14_mask;
#[doc = "BUF_FILTER14_FILTER register accessor: an alias for `Reg<BUF_FILTER14_FILTER_SPEC>`"]
pub type BUF_FILTER14_FILTER = crate::Reg<buf_filter14_filter::BUF_FILTER14_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter14_filter;
#[doc = "BUF_FILTER15_MASK register accessor: an alias for `Reg<BUF_FILTER15_MASK_SPEC>`"]
pub type BUF_FILTER15_MASK = crate::Reg<buf_filter15_mask::BUF_FILTER15_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter15_mask;
#[doc = "BUF_FILTER15_FILTER register accessor: an alias for `Reg<BUF_FILTER15_FILTER_SPEC>`"]
pub type BUF_FILTER15_FILTER = crate::Reg<buf_filter15_filter::BUF_FILTER15_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter15_filter;
#[doc = "BUF_FILTER16_MASK register accessor: an alias for `Reg<BUF_FILTER16_MASK_SPEC>`"]
pub type BUF_FILTER16_MASK = crate::Reg<buf_filter16_mask::BUF_FILTER16_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter16_mask;
#[doc = "BUF_FILTER16_FILTER register accessor: an alias for `Reg<BUF_FILTER16_FILTER_SPEC>`"]
pub type BUF_FILTER16_FILTER = crate::Reg<buf_filter16_filter::BUF_FILTER16_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter16_filter;
#[doc = "BUF_FILTER17_MASK register accessor: an alias for `Reg<BUF_FILTER17_MASK_SPEC>`"]
pub type BUF_FILTER17_MASK = crate::Reg<buf_filter17_mask::BUF_FILTER17_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter17_mask;
#[doc = "BUF_FILTER17_FILTER register accessor: an alias for `Reg<BUF_FILTER17_FILTER_SPEC>`"]
pub type BUF_FILTER17_FILTER = crate::Reg<buf_filter17_filter::BUF_FILTER17_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter17_filter;
#[doc = "BUF_FILTER18_MASK register accessor: an alias for `Reg<BUF_FILTER18_MASK_SPEC>`"]
pub type BUF_FILTER18_MASK = crate::Reg<buf_filter18_mask::BUF_FILTER18_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter18_mask;
#[doc = "BUF_FILTER18_FILTER register accessor: an alias for `Reg<BUF_FILTER18_FILTER_SPEC>`"]
pub type BUF_FILTER18_FILTER = crate::Reg<buf_filter18_filter::BUF_FILTER18_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter18_filter;
#[doc = "BUF_FILTER19_MASK register accessor: an alias for `Reg<BUF_FILTER19_MASK_SPEC>`"]
pub type BUF_FILTER19_MASK = crate::Reg<buf_filter19_mask::BUF_FILTER19_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter19_mask;
#[doc = "BUF_FILTER19_FILTER register accessor: an alias for `Reg<BUF_FILTER19_FILTER_SPEC>`"]
pub type BUF_FILTER19_FILTER = crate::Reg<buf_filter19_filter::BUF_FILTER19_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter19_filter;
#[doc = "BUF_FILTER20_MASK register accessor: an alias for `Reg<BUF_FILTER20_MASK_SPEC>`"]
pub type BUF_FILTER20_MASK = crate::Reg<buf_filter20_mask::BUF_FILTER20_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter20_mask;
#[doc = "BUF_FILTER20_FILTER register accessor: an alias for `Reg<BUF_FILTER20_FILTER_SPEC>`"]
pub type BUF_FILTER20_FILTER = crate::Reg<buf_filter20_filter::BUF_FILTER20_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter20_filter;
#[doc = "BUF_FILTER21_MASK register accessor: an alias for `Reg<BUF_FILTER21_MASK_SPEC>`"]
pub type BUF_FILTER21_MASK = crate::Reg<buf_filter21_mask::BUF_FILTER21_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter21_mask;
#[doc = "BUF_FILTER21_FILTER register accessor: an alias for `Reg<BUF_FILTER21_FILTER_SPEC>`"]
pub type BUF_FILTER21_FILTER = crate::Reg<buf_filter21_filter::BUF_FILTER21_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter21_filter;
#[doc = "BUF_FILTER22_MASK register accessor: an alias for `Reg<BUF_FILTER22_MASK_SPEC>`"]
pub type BUF_FILTER22_MASK = crate::Reg<buf_filter22_mask::BUF_FILTER22_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter22_mask;
#[doc = "BUF_FILTER22_FILTER register accessor: an alias for `Reg<BUF_FILTER22_FILTER_SPEC>`"]
pub type BUF_FILTER22_FILTER = crate::Reg<buf_filter22_filter::BUF_FILTER22_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter22_filter;
#[doc = "BUF_FILTER23_MASK register accessor: an alias for `Reg<BUF_FILTER23_MASK_SPEC>`"]
pub type BUF_FILTER23_MASK = crate::Reg<buf_filter23_mask::BUF_FILTER23_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter23_mask;
#[doc = "BUF_FILTER23_FILTER register accessor: an alias for `Reg<BUF_FILTER23_FILTER_SPEC>`"]
pub type BUF_FILTER23_FILTER = crate::Reg<buf_filter23_filter::BUF_FILTER23_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter23_filter;
#[doc = "BUF_FILTER24_MASK register accessor: an alias for `Reg<BUF_FILTER24_MASK_SPEC>`"]
pub type BUF_FILTER24_MASK = crate::Reg<buf_filter24_mask::BUF_FILTER24_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter24_mask;
#[doc = "BUF_FILTER24_FILTER register accessor: an alias for `Reg<BUF_FILTER24_FILTER_SPEC>`"]
pub type BUF_FILTER24_FILTER = crate::Reg<buf_filter24_filter::BUF_FILTER24_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter24_filter;
#[doc = "BUF_FILTER25_MASK register accessor: an alias for `Reg<BUF_FILTER25_MASK_SPEC>`"]
pub type BUF_FILTER25_MASK = crate::Reg<buf_filter25_mask::BUF_FILTER25_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter25_mask;
#[doc = "BUF_FILTER25_FILTER register accessor: an alias for `Reg<BUF_FILTER25_FILTER_SPEC>`"]
pub type BUF_FILTER25_FILTER = crate::Reg<buf_filter25_filter::BUF_FILTER25_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter25_filter;
#[doc = "BUF_FILTER26_MASK register accessor: an alias for `Reg<BUF_FILTER26_MASK_SPEC>`"]
pub type BUF_FILTER26_MASK = crate::Reg<buf_filter26_mask::BUF_FILTER26_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter26_mask;
#[doc = "BUF_FILTER26_FILTER register accessor: an alias for `Reg<BUF_FILTER26_FILTER_SPEC>`"]
pub type BUF_FILTER26_FILTER = crate::Reg<buf_filter26_filter::BUF_FILTER26_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter26_filter;
#[doc = "BUF_FILTER27_MASK register accessor: an alias for `Reg<BUF_FILTER27_MASK_SPEC>`"]
pub type BUF_FILTER27_MASK = crate::Reg<buf_filter27_mask::BUF_FILTER27_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter27_mask;
#[doc = "BUF_FILTER27_FILTER register accessor: an alias for `Reg<BUF_FILTER27_FILTER_SPEC>`"]
pub type BUF_FILTER27_FILTER = crate::Reg<buf_filter27_filter::BUF_FILTER27_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter27_filter;
#[doc = "BUF_FILTER28_MASK register accessor: an alias for `Reg<BUF_FILTER28_MASK_SPEC>`"]
pub type BUF_FILTER28_MASK = crate::Reg<buf_filter28_mask::BUF_FILTER28_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter28_mask;
#[doc = "BUF_FILTER28_FILTER register accessor: an alias for `Reg<BUF_FILTER28_FILTER_SPEC>`"]
pub type BUF_FILTER28_FILTER = crate::Reg<buf_filter28_filter::BUF_FILTER28_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter28_filter;
#[doc = "BUF_FILTER29_MASK register accessor: an alias for `Reg<BUF_FILTER29_MASK_SPEC>`"]
pub type BUF_FILTER29_MASK = crate::Reg<buf_filter29_mask::BUF_FILTER29_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter29_mask;
#[doc = "BUF_FILTER29_FILTER register accessor: an alias for `Reg<BUF_FILTER29_FILTER_SPEC>`"]
pub type BUF_FILTER29_FILTER = crate::Reg<buf_filter29_filter::BUF_FILTER29_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter29_filter;
#[doc = "BUF_FILTER30_MASK register accessor: an alias for `Reg<BUF_FILTER30_MASK_SPEC>`"]
pub type BUF_FILTER30_MASK = crate::Reg<buf_filter30_mask::BUF_FILTER30_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter30_mask;
#[doc = "BUF_FILTER30_FILTER register accessor: an alias for `Reg<BUF_FILTER30_FILTER_SPEC>`"]
pub type BUF_FILTER30_FILTER = crate::Reg<buf_filter30_filter::BUF_FILTER30_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter30_filter;
#[doc = "BUF_FILTER31_MASK register accessor: an alias for `Reg<BUF_FILTER31_MASK_SPEC>`"]
pub type BUF_FILTER31_MASK = crate::Reg<buf_filter31_mask::BUF_FILTER31_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter31_mask;
#[doc = "BUF_FILTER31_FILTER register accessor: an alias for `Reg<BUF_FILTER31_FILTER_SPEC>`"]
pub type BUF_FILTER31_FILTER = crate::Reg<buf_filter31_filter::BUF_FILTER31_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter31_filter;
