#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FDCAN Core Release Register"]
    pub fdcan_crel: FDCAN_CREL,
    #[doc = "0x04 - FDCAN Core Release Register"]
    pub fdcan_endn: FDCAN_ENDN,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - FDCAN Data Bit Timing and Prescaler Register"]
    pub fdcan_dbtp: FDCAN_DBTP,
    #[doc = "0x10 - FDCAN Test Register"]
    pub fdcan_test: FDCAN_TEST,
    #[doc = "0x14 - FDCAN RAM Watchdog Register"]
    pub fdcan_rwd: FDCAN_RWD,
    #[doc = "0x18 - FDCAN CC Control Register"]
    pub fdcan_cccr: FDCAN_CCCR,
    #[doc = "0x1c - FDCAN Nominal Bit Timing and Prescaler Register"]
    pub fdcan_nbtp: FDCAN_NBTP,
    #[doc = "0x20 - FDCAN Timestamp Counter Configuration Register"]
    pub fdcan_tscc: FDCAN_TSCC,
    #[doc = "0x24 - FDCAN Timestamp Counter Value Register"]
    pub fdcan_tscv: FDCAN_TSCV,
    #[doc = "0x28 - FDCAN Timeout Counter Configuration Register"]
    pub fdcan_tocc: FDCAN_TOCC,
    #[doc = "0x2c - FDCAN Timeout Counter Value Register"]
    pub fdcan_tocv: FDCAN_TOCV,
    _reserved11: [u8; 16usize],
    #[doc = "0x40 - FDCAN Error Counter Register"]
    pub fdcan_ecr: FDCAN_ECR,
    #[doc = "0x44 - FDCAN Protocol Status Register"]
    pub fdcan_psr: FDCAN_PSR,
    #[doc = "0x48 - FDCAN Transmitter Delay Compensation Register"]
    pub fdcan_tdcr: FDCAN_TDCR,
    _reserved14: [u8; 4usize],
    #[doc = "0x50 - FDCAN Interrupt Register"]
    pub fdcan_ir: FDCAN_IR,
    #[doc = "0x54 - FDCAN Interrupt Enable Register"]
    pub fdcan_ie: FDCAN_IE,
    #[doc = "0x58 - FDCAN Interrupt Line Select Register"]
    pub fdcan_ils: FDCAN_ILS,
    #[doc = "0x5c - FDCAN Interrupt Line Enable Register"]
    pub fdcan_ile: FDCAN_ILE,
    _reserved18: [u8; 32usize],
    #[doc = "0x80 - FDCAN Global Filter Configuration Register"]
    pub fdcan_gfc: FDCAN_GFC,
    #[doc = "0x84 - FDCAN Standard ID Filter Configuration Register"]
    pub fdcan_sidfc: FDCAN_SIDFC,
    #[doc = "0x88 - FDCAN Extended ID Filter Configuration Register"]
    pub fdcan_xidfc: FDCAN_XIDFC,
    _reserved21: [u8; 4usize],
    #[doc = "0x90 - FDCAN Extended ID and Mask Register"]
    pub fdcan_xidam: FDCAN_XIDAM,
    #[doc = "0x94 - FDCAN High Priority Message Status Register"]
    pub fdcan_hpms: FDCAN_HPMS,
    #[doc = "0x98 - FDCAN New Data 1 Register"]
    pub fdcan_ndat1: FDCAN_NDAT1,
    #[doc = "0x9c - FDCAN New Data 2 Register"]
    pub fdcan_ndat2: FDCAN_NDAT2,
    #[doc = "0xa0 - FDCAN Rx FIFO 0 Configuration Register"]
    pub fdcan_rxf0c: FDCAN_RXF0C,
    #[doc = "0xa4 - FDCAN Rx FIFO 0 Status Register"]
    pub fdcan_rxf0s: FDCAN_RXF0S,
    #[doc = "0xa8 - CAN Rx FIFO 0 Acknowledge Register"]
    pub fdcan_rxf0a: FDCAN_RXF0A,
    #[doc = "0xac - FDCAN Rx Buffer Configuration Register"]
    pub fdcan_rxbc: FDCAN_RXBC,
    #[doc = "0xb0 - FDCAN Rx FIFO 1 Configuration Register"]
    pub fdcan_rxf1c: FDCAN_RXF1C,
    #[doc = "0xb4 - FDCAN Rx FIFO 1 Status Register"]
    pub fdcan_rxf1s: FDCAN_RXF1S,
    #[doc = "0xb8 - FDCAN Rx FIFO 1 Acknowledge Register"]
    pub fdcan_rxf1a: FDCAN_RXF1A,
    #[doc = "0xbc - FDCAN Rx Buffer Element Size Configuration Register"]
    pub fdcan_rxesc: FDCAN_RXESC,
    #[doc = "0xc0 - FDCAN Tx Buffer Configuration Register"]
    pub fdcan_txbc: FDCAN_TXBC,
    #[doc = "0xc4 - FDCAN Tx FIFO/Queue Status Register"]
    pub fdcan_txfqs: FDCAN_TXFQS,
    #[doc = "0xc8 - FDCAN Tx Buffer Element Size Configuration Register"]
    pub fdcan_txesc: FDCAN_TXESC,
    #[doc = "0xcc - FDCAN Tx Buffer Request Pending Register"]
    pub fdcan_txbrp: FDCAN_TXBRP,
    #[doc = "0xd0 - FDCAN Tx Buffer Add Request Register"]
    pub fdcan_txbar: FDCAN_TXBAR,
    #[doc = "0xd4 - FDCAN Tx Buffer Cancellation Request Register"]
    pub fdcan_txbcr: FDCAN_TXBCR,
    #[doc = "0xd8 - FDCAN Tx Buffer Transmission Occurred Register"]
    pub fdcan_txbto: FDCAN_TXBTO,
    #[doc = "0xdc - FDCAN Tx Buffer Cancellation Finished Register"]
    pub fdcan_txbcf: FDCAN_TXBCF,
    #[doc = "0xe0 - FDCAN Tx Buffer Transmission Interrupt Enable Register"]
    pub fdcan_txbtie: FDCAN_TXBTIE,
    #[doc = "0xe4 - FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register"]
    pub fdcan_txbcie: FDCAN_TXBCIE,
    _reserved43: [u8; 8usize],
    #[doc = "0xf0 - FDCAN Tx Event FIFO Configuration Register"]
    pub fdcan_txefc: FDCAN_TXEFC,
    #[doc = "0xf4 - FDCAN Tx Event FIFO Status Register"]
    pub fdcan_txefs: FDCAN_TXEFS,
    #[doc = "0xf8 - FDCAN Tx Event FIFO Acknowledge Register"]
    pub fdcan_txefa: FDCAN_TXEFA,
    _reserved46: [u8; 4usize],
    #[doc = "0x100 - FDCAN TT Trigger Memory Configuration Register"]
    pub fdcan_tttmc: FDCAN_TTTMC,
    #[doc = "0x104 - FDCAN TT Reference Message Configuration Register"]
    pub fdcan_ttrmc: FDCAN_TTRMC,
    #[doc = "0x108 - FDCAN TT Operation Configuration Register"]
    pub fdcan_ttocf: FDCAN_TTOCF,
    #[doc = "0x10c - FDCAN TT Matrix Limits Register"]
    pub fdcan_ttmlm: FDCAN_TTMLM,
    #[doc = "0x110 - FDCAN TUR Configuration Register"]
    pub fdcan_turcf: FDCAN_TURCF,
    #[doc = "0x114 - FDCAN TT Operation Control Register"]
    pub fdcan_ttocn: FDCAN_TTOCN,
    #[doc = "0x118 - FDCAN TT Global Time Preset Register"]
    pub can_ttgtp: CAN_TTGTP,
    #[doc = "0x11c - FDCAN TT Time Mark Register"]
    pub fdcan_tttmk: FDCAN_TTTMK,
    #[doc = "0x120 - FDCAN TT Interrupt Register"]
    pub fdcan_ttir: FDCAN_TTIR,
    #[doc = "0x124 - FDCAN TT Interrupt Enable Register"]
    pub fdcan_ttie: FDCAN_TTIE,
    #[doc = "0x128 - FDCAN TT Interrupt Line Select Register"]
    pub fdcan_ttils: FDCAN_TTILS,
    #[doc = "0x12c - FDCAN TT Operation Status Register"]
    pub fdcan_ttost: FDCAN_TTOST,
    #[doc = "0x130 - FDCAN TUR Numerator Actual Register"]
    pub fdcan_turna: FDCAN_TURNA,
    #[doc = "0x134 - FDCAN TT Local and Global Time Register"]
    pub fdcan_ttlgt: FDCAN_TTLGT,
    #[doc = "0x138 - FDCAN TT Cycle Time and Count Register"]
    pub fdcan_ttctc: FDCAN_TTCTC,
    #[doc = "0x13c - FDCAN TT Capture Time Register"]
    pub fdcan_ttcpt: FDCAN_TTCPT,
    #[doc = "0x140 - FDCAN TT Cycle Sync Mark Register"]
    pub fdcan_ttcsm: FDCAN_TTCSM,
    _reserved63: [u8; 444usize],
    #[doc = "0x300 - FDCAN TT Trigger Select Register"]
    pub fdcan_ttts: FDCAN_TTTS,
}
#[doc = "FDCAN Core Release Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_crel](fdcan_crel) module"]
pub type FDCAN_CREL = crate::Reg<u32, _FDCAN_CREL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_CREL;
#[doc = "`read()` method returns [fdcan_crel::R](fdcan_crel::R) reader structure"]
impl crate::Readable for FDCAN_CREL {}
#[doc = "FDCAN Core Release Register"]
pub mod fdcan_crel;
#[doc = "FDCAN Core Release Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_endn](fdcan_endn) module"]
pub type FDCAN_ENDN = crate::Reg<u32, _FDCAN_ENDN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_ENDN;
#[doc = "`read()` method returns [fdcan_endn::R](fdcan_endn::R) reader structure"]
impl crate::Readable for FDCAN_ENDN {}
#[doc = "FDCAN Core Release Register"]
pub mod fdcan_endn;
#[doc = "FDCAN Data Bit Timing and Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_dbtp](fdcan_dbtp) module"]
pub type FDCAN_DBTP = crate::Reg<u32, _FDCAN_DBTP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_DBTP;
#[doc = "`read()` method returns [fdcan_dbtp::R](fdcan_dbtp::R) reader structure"]
impl crate::Readable for FDCAN_DBTP {}
#[doc = "FDCAN Data Bit Timing and Prescaler Register"]
pub mod fdcan_dbtp;
#[doc = "FDCAN Test Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_test](fdcan_test) module"]
pub type FDCAN_TEST = crate::Reg<u32, _FDCAN_TEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TEST;
#[doc = "`read()` method returns [fdcan_test::R](fdcan_test::R) reader structure"]
impl crate::Readable for FDCAN_TEST {}
#[doc = "FDCAN Test Register"]
pub mod fdcan_test;
#[doc = "FDCAN RAM Watchdog Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_rwd](fdcan_rwd) module"]
pub type FDCAN_RWD = crate::Reg<u32, _FDCAN_RWD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_RWD;
#[doc = "`read()` method returns [fdcan_rwd::R](fdcan_rwd::R) reader structure"]
impl crate::Readable for FDCAN_RWD {}
#[doc = "FDCAN RAM Watchdog Register"]
pub mod fdcan_rwd;
#[doc = "FDCAN CC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_cccr](fdcan_cccr) module"]
pub type FDCAN_CCCR = crate::Reg<u32, _FDCAN_CCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_CCCR;
#[doc = "`read()` method returns [fdcan_cccr::R](fdcan_cccr::R) reader structure"]
impl crate::Readable for FDCAN_CCCR {}
#[doc = "`write(|w| ..)` method takes [fdcan_cccr::W](fdcan_cccr::W) writer structure"]
impl crate::Writable for FDCAN_CCCR {}
#[doc = "FDCAN CC Control Register"]
pub mod fdcan_cccr;
#[doc = "FDCAN Nominal Bit Timing and Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_nbtp](fdcan_nbtp) module"]
pub type FDCAN_NBTP = crate::Reg<u32, _FDCAN_NBTP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_NBTP;
#[doc = "`read()` method returns [fdcan_nbtp::R](fdcan_nbtp::R) reader structure"]
impl crate::Readable for FDCAN_NBTP {}
#[doc = "`write(|w| ..)` method takes [fdcan_nbtp::W](fdcan_nbtp::W) writer structure"]
impl crate::Writable for FDCAN_NBTP {}
#[doc = "FDCAN Nominal Bit Timing and Prescaler Register"]
pub mod fdcan_nbtp;
#[doc = "FDCAN Timestamp Counter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_tscc](fdcan_tscc) module"]
pub type FDCAN_TSCC = crate::Reg<u32, _FDCAN_TSCC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TSCC;
#[doc = "`read()` method returns [fdcan_tscc::R](fdcan_tscc::R) reader structure"]
impl crate::Readable for FDCAN_TSCC {}
#[doc = "`write(|w| ..)` method takes [fdcan_tscc::W](fdcan_tscc::W) writer structure"]
impl crate::Writable for FDCAN_TSCC {}
#[doc = "FDCAN Timestamp Counter Configuration Register"]
pub mod fdcan_tscc;
#[doc = "FDCAN Timestamp Counter Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_tscv](fdcan_tscv) module"]
pub type FDCAN_TSCV = crate::Reg<u32, _FDCAN_TSCV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TSCV;
#[doc = "`read()` method returns [fdcan_tscv::R](fdcan_tscv::R) reader structure"]
impl crate::Readable for FDCAN_TSCV {}
#[doc = "`write(|w| ..)` method takes [fdcan_tscv::W](fdcan_tscv::W) writer structure"]
impl crate::Writable for FDCAN_TSCV {}
#[doc = "FDCAN Timestamp Counter Value Register"]
pub mod fdcan_tscv;
#[doc = "FDCAN Timeout Counter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_tocc](fdcan_tocc) module"]
pub type FDCAN_TOCC = crate::Reg<u32, _FDCAN_TOCC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TOCC;
#[doc = "`read()` method returns [fdcan_tocc::R](fdcan_tocc::R) reader structure"]
impl crate::Readable for FDCAN_TOCC {}
#[doc = "`write(|w| ..)` method takes [fdcan_tocc::W](fdcan_tocc::W) writer structure"]
impl crate::Writable for FDCAN_TOCC {}
#[doc = "FDCAN Timeout Counter Configuration Register"]
pub mod fdcan_tocc;
#[doc = "FDCAN Timeout Counter Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_tocv](fdcan_tocv) module"]
pub type FDCAN_TOCV = crate::Reg<u32, _FDCAN_TOCV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TOCV;
#[doc = "`read()` method returns [fdcan_tocv::R](fdcan_tocv::R) reader structure"]
impl crate::Readable for FDCAN_TOCV {}
#[doc = "`write(|w| ..)` method takes [fdcan_tocv::W](fdcan_tocv::W) writer structure"]
impl crate::Writable for FDCAN_TOCV {}
#[doc = "FDCAN Timeout Counter Value Register"]
pub mod fdcan_tocv;
#[doc = "FDCAN Error Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ecr](fdcan_ecr) module"]
pub type FDCAN_ECR = crate::Reg<u32, _FDCAN_ECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_ECR;
#[doc = "`read()` method returns [fdcan_ecr::R](fdcan_ecr::R) reader structure"]
impl crate::Readable for FDCAN_ECR {}
#[doc = "`write(|w| ..)` method takes [fdcan_ecr::W](fdcan_ecr::W) writer structure"]
impl crate::Writable for FDCAN_ECR {}
#[doc = "FDCAN Error Counter Register"]
pub mod fdcan_ecr;
#[doc = "FDCAN Protocol Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_psr](fdcan_psr) module"]
pub type FDCAN_PSR = crate::Reg<u32, _FDCAN_PSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_PSR;
#[doc = "`read()` method returns [fdcan_psr::R](fdcan_psr::R) reader structure"]
impl crate::Readable for FDCAN_PSR {}
#[doc = "`write(|w| ..)` method takes [fdcan_psr::W](fdcan_psr::W) writer structure"]
impl crate::Writable for FDCAN_PSR {}
#[doc = "FDCAN Protocol Status Register"]
pub mod fdcan_psr;
#[doc = "FDCAN Transmitter Delay Compensation Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_tdcr](fdcan_tdcr) module"]
pub type FDCAN_TDCR = crate::Reg<u32, _FDCAN_TDCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TDCR;
#[doc = "`read()` method returns [fdcan_tdcr::R](fdcan_tdcr::R) reader structure"]
impl crate::Readable for FDCAN_TDCR {}
#[doc = "FDCAN Transmitter Delay Compensation Register"]
pub mod fdcan_tdcr;
#[doc = "FDCAN Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ir](fdcan_ir) module"]
pub type FDCAN_IR = crate::Reg<u32, _FDCAN_IR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_IR;
#[doc = "`read()` method returns [fdcan_ir::R](fdcan_ir::R) reader structure"]
impl crate::Readable for FDCAN_IR {}
#[doc = "FDCAN Interrupt Register"]
pub mod fdcan_ir;
#[doc = "FDCAN Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ie](fdcan_ie) module"]
pub type FDCAN_IE = crate::Reg<u32, _FDCAN_IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_IE;
#[doc = "`read()` method returns [fdcan_ie::R](fdcan_ie::R) reader structure"]
impl crate::Readable for FDCAN_IE {}
#[doc = "FDCAN Interrupt Enable Register"]
pub mod fdcan_ie;
#[doc = "FDCAN Interrupt Line Select Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ils](fdcan_ils) module"]
pub type FDCAN_ILS = crate::Reg<u32, _FDCAN_ILS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_ILS;
#[doc = "`read()` method returns [fdcan_ils::R](fdcan_ils::R) reader structure"]
impl crate::Readable for FDCAN_ILS {}
#[doc = "FDCAN Interrupt Line Select Register"]
pub mod fdcan_ils;
#[doc = "FDCAN Interrupt Line Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ile](fdcan_ile) module"]
pub type FDCAN_ILE = crate::Reg<u32, _FDCAN_ILE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_ILE;
#[doc = "`read()` method returns [fdcan_ile::R](fdcan_ile::R) reader structure"]
impl crate::Readable for FDCAN_ILE {}
#[doc = "`write(|w| ..)` method takes [fdcan_ile::W](fdcan_ile::W) writer structure"]
impl crate::Writable for FDCAN_ILE {}
#[doc = "FDCAN Interrupt Line Enable Register"]
pub mod fdcan_ile;
#[doc = "FDCAN Global Filter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_gfc](fdcan_gfc) module"]
pub type FDCAN_GFC = crate::Reg<u32, _FDCAN_GFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_GFC;
#[doc = "`read()` method returns [fdcan_gfc::R](fdcan_gfc::R) reader structure"]
impl crate::Readable for FDCAN_GFC {}
#[doc = "`write(|w| ..)` method takes [fdcan_gfc::W](fdcan_gfc::W) writer structure"]
impl crate::Writable for FDCAN_GFC {}
#[doc = "FDCAN Global Filter Configuration Register"]
pub mod fdcan_gfc;
#[doc = "FDCAN Standard ID Filter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_sidfc](fdcan_sidfc) module"]
pub type FDCAN_SIDFC = crate::Reg<u32, _FDCAN_SIDFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_SIDFC;
#[doc = "`read()` method returns [fdcan_sidfc::R](fdcan_sidfc::R) reader structure"]
impl crate::Readable for FDCAN_SIDFC {}
#[doc = "`write(|w| ..)` method takes [fdcan_sidfc::W](fdcan_sidfc::W) writer structure"]
impl crate::Writable for FDCAN_SIDFC {}
#[doc = "FDCAN Standard ID Filter Configuration Register"]
pub mod fdcan_sidfc;
#[doc = "FDCAN Extended ID Filter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_xidfc](fdcan_xidfc) module"]
pub type FDCAN_XIDFC = crate::Reg<u32, _FDCAN_XIDFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_XIDFC;
#[doc = "`read()` method returns [fdcan_xidfc::R](fdcan_xidfc::R) reader structure"]
impl crate::Readable for FDCAN_XIDFC {}
#[doc = "`write(|w| ..)` method takes [fdcan_xidfc::W](fdcan_xidfc::W) writer structure"]
impl crate::Writable for FDCAN_XIDFC {}
#[doc = "FDCAN Extended ID Filter Configuration Register"]
pub mod fdcan_xidfc;
#[doc = "FDCAN Extended ID and Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_xidam](fdcan_xidam) module"]
pub type FDCAN_XIDAM = crate::Reg<u32, _FDCAN_XIDAM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_XIDAM;
#[doc = "`read()` method returns [fdcan_xidam::R](fdcan_xidam::R) reader structure"]
impl crate::Readable for FDCAN_XIDAM {}
#[doc = "`write(|w| ..)` method takes [fdcan_xidam::W](fdcan_xidam::W) writer structure"]
impl crate::Writable for FDCAN_XIDAM {}
#[doc = "FDCAN Extended ID and Mask Register"]
pub mod fdcan_xidam;
#[doc = "FDCAN High Priority Message Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_hpms](fdcan_hpms) module"]
pub type FDCAN_HPMS = crate::Reg<u32, _FDCAN_HPMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_HPMS;
#[doc = "`read()` method returns [fdcan_hpms::R](fdcan_hpms::R) reader structure"]
impl crate::Readable for FDCAN_HPMS {}
#[doc = "FDCAN High Priority Message Status Register"]
pub mod fdcan_hpms;
#[doc = "FDCAN New Data 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ndat1](fdcan_ndat1) module"]
pub type FDCAN_NDAT1 = crate::Reg<u32, _FDCAN_NDAT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_NDAT1;
#[doc = "`read()` method returns [fdcan_ndat1::R](fdcan_ndat1::R) reader structure"]
impl crate::Readable for FDCAN_NDAT1 {}
#[doc = "FDCAN New Data 1 Register"]
pub mod fdcan_ndat1;
#[doc = "FDCAN New Data 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ndat2](fdcan_ndat2) module"]
pub type FDCAN_NDAT2 = crate::Reg<u32, _FDCAN_NDAT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_NDAT2;
#[doc = "`read()` method returns [fdcan_ndat2::R](fdcan_ndat2::R) reader structure"]
impl crate::Readable for FDCAN_NDAT2 {}
#[doc = "FDCAN New Data 2 Register"]
pub mod fdcan_ndat2;
#[doc = "FDCAN Rx FIFO 0 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_rxf0c](fdcan_rxf0c) module"]
pub type FDCAN_RXF0C = crate::Reg<u32, _FDCAN_RXF0C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_RXF0C;
#[doc = "`read()` method returns [fdcan_rxf0c::R](fdcan_rxf0c::R) reader structure"]
impl crate::Readable for FDCAN_RXF0C {}
#[doc = "`write(|w| ..)` method takes [fdcan_rxf0c::W](fdcan_rxf0c::W) writer structure"]
impl crate::Writable for FDCAN_RXF0C {}
#[doc = "FDCAN Rx FIFO 0 Configuration Register"]
pub mod fdcan_rxf0c;
#[doc = "FDCAN Rx FIFO 0 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_rxf0s](fdcan_rxf0s) module"]
pub type FDCAN_RXF0S = crate::Reg<u32, _FDCAN_RXF0S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_RXF0S;
#[doc = "`read()` method returns [fdcan_rxf0s::R](fdcan_rxf0s::R) reader structure"]
impl crate::Readable for FDCAN_RXF0S {}
#[doc = "`write(|w| ..)` method takes [fdcan_rxf0s::W](fdcan_rxf0s::W) writer structure"]
impl crate::Writable for FDCAN_RXF0S {}
#[doc = "FDCAN Rx FIFO 0 Status Register"]
pub mod fdcan_rxf0s;
#[doc = "CAN Rx FIFO 0 Acknowledge Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_rxf0a](fdcan_rxf0a) module"]
pub type FDCAN_RXF0A = crate::Reg<u32, _FDCAN_RXF0A>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_RXF0A;
#[doc = "`read()` method returns [fdcan_rxf0a::R](fdcan_rxf0a::R) reader structure"]
impl crate::Readable for FDCAN_RXF0A {}
#[doc = "`write(|w| ..)` method takes [fdcan_rxf0a::W](fdcan_rxf0a::W) writer structure"]
impl crate::Writable for FDCAN_RXF0A {}
#[doc = "CAN Rx FIFO 0 Acknowledge Register"]
pub mod fdcan_rxf0a;
#[doc = "FDCAN Rx Buffer Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_rxbc](fdcan_rxbc) module"]
pub type FDCAN_RXBC = crate::Reg<u32, _FDCAN_RXBC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_RXBC;
#[doc = "`read()` method returns [fdcan_rxbc::R](fdcan_rxbc::R) reader structure"]
impl crate::Readable for FDCAN_RXBC {}
#[doc = "`write(|w| ..)` method takes [fdcan_rxbc::W](fdcan_rxbc::W) writer structure"]
impl crate::Writable for FDCAN_RXBC {}
#[doc = "FDCAN Rx Buffer Configuration Register"]
pub mod fdcan_rxbc;
#[doc = "FDCAN Rx FIFO 1 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_rxf1c](fdcan_rxf1c) module"]
pub type FDCAN_RXF1C = crate::Reg<u32, _FDCAN_RXF1C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_RXF1C;
#[doc = "`read()` method returns [fdcan_rxf1c::R](fdcan_rxf1c::R) reader structure"]
impl crate::Readable for FDCAN_RXF1C {}
#[doc = "`write(|w| ..)` method takes [fdcan_rxf1c::W](fdcan_rxf1c::W) writer structure"]
impl crate::Writable for FDCAN_RXF1C {}
#[doc = "FDCAN Rx FIFO 1 Configuration Register"]
pub mod fdcan_rxf1c;
#[doc = "FDCAN Rx FIFO 1 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_rxf1s](fdcan_rxf1s) module"]
pub type FDCAN_RXF1S = crate::Reg<u32, _FDCAN_RXF1S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_RXF1S;
#[doc = "`read()` method returns [fdcan_rxf1s::R](fdcan_rxf1s::R) reader structure"]
impl crate::Readable for FDCAN_RXF1S {}
#[doc = "`write(|w| ..)` method takes [fdcan_rxf1s::W](fdcan_rxf1s::W) writer structure"]
impl crate::Writable for FDCAN_RXF1S {}
#[doc = "FDCAN Rx FIFO 1 Status Register"]
pub mod fdcan_rxf1s;
#[doc = "FDCAN Rx FIFO 1 Acknowledge Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_rxf1a](fdcan_rxf1a) module"]
pub type FDCAN_RXF1A = crate::Reg<u32, _FDCAN_RXF1A>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_RXF1A;
#[doc = "`read()` method returns [fdcan_rxf1a::R](fdcan_rxf1a::R) reader structure"]
impl crate::Readable for FDCAN_RXF1A {}
#[doc = "`write(|w| ..)` method takes [fdcan_rxf1a::W](fdcan_rxf1a::W) writer structure"]
impl crate::Writable for FDCAN_RXF1A {}
#[doc = "FDCAN Rx FIFO 1 Acknowledge Register"]
pub mod fdcan_rxf1a;
#[doc = "FDCAN Rx Buffer Element Size Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_rxesc](fdcan_rxesc) module"]
pub type FDCAN_RXESC = crate::Reg<u32, _FDCAN_RXESC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_RXESC;
#[doc = "`read()` method returns [fdcan_rxesc::R](fdcan_rxesc::R) reader structure"]
impl crate::Readable for FDCAN_RXESC {}
#[doc = "`write(|w| ..)` method takes [fdcan_rxesc::W](fdcan_rxesc::W) writer structure"]
impl crate::Writable for FDCAN_RXESC {}
#[doc = "FDCAN Rx Buffer Element Size Configuration Register"]
pub mod fdcan_rxesc;
#[doc = "FDCAN Tx Buffer Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_txbc](fdcan_txbc) module"]
pub type FDCAN_TXBC = crate::Reg<u32, _FDCAN_TXBC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TXBC;
#[doc = "`read()` method returns [fdcan_txbc::R](fdcan_txbc::R) reader structure"]
impl crate::Readable for FDCAN_TXBC {}
#[doc = "`write(|w| ..)` method takes [fdcan_txbc::W](fdcan_txbc::W) writer structure"]
impl crate::Writable for FDCAN_TXBC {}
#[doc = "FDCAN Tx Buffer Configuration Register"]
pub mod fdcan_txbc;
#[doc = "FDCAN Tx FIFO/Queue Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_txfqs](fdcan_txfqs) module"]
pub type FDCAN_TXFQS = crate::Reg<u32, _FDCAN_TXFQS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TXFQS;
#[doc = "`read()` method returns [fdcan_txfqs::R](fdcan_txfqs::R) reader structure"]
impl crate::Readable for FDCAN_TXFQS {}
#[doc = "FDCAN Tx FIFO/Queue Status Register"]
pub mod fdcan_txfqs;
#[doc = "FDCAN Tx Buffer Element Size Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_txesc](fdcan_txesc) module"]
pub type FDCAN_TXESC = crate::Reg<u32, _FDCAN_TXESC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TXESC;
#[doc = "`read()` method returns [fdcan_txesc::R](fdcan_txesc::R) reader structure"]
impl crate::Readable for FDCAN_TXESC {}
#[doc = "`write(|w| ..)` method takes [fdcan_txesc::W](fdcan_txesc::W) writer structure"]
impl crate::Writable for FDCAN_TXESC {}
#[doc = "FDCAN Tx Buffer Element Size Configuration Register"]
pub mod fdcan_txesc;
#[doc = "FDCAN Tx Buffer Request Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_txbrp](fdcan_txbrp) module"]
pub type FDCAN_TXBRP = crate::Reg<u32, _FDCAN_TXBRP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TXBRP;
#[doc = "`read()` method returns [fdcan_txbrp::R](fdcan_txbrp::R) reader structure"]
impl crate::Readable for FDCAN_TXBRP {}
#[doc = "FDCAN Tx Buffer Request Pending Register"]
pub mod fdcan_txbrp;
#[doc = "FDCAN Tx Buffer Add Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_txbar](fdcan_txbar) module"]
pub type FDCAN_TXBAR = crate::Reg<u32, _FDCAN_TXBAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TXBAR;
#[doc = "`read()` method returns [fdcan_txbar::R](fdcan_txbar::R) reader structure"]
impl crate::Readable for FDCAN_TXBAR {}
#[doc = "`write(|w| ..)` method takes [fdcan_txbar::W](fdcan_txbar::W) writer structure"]
impl crate::Writable for FDCAN_TXBAR {}
#[doc = "FDCAN Tx Buffer Add Request Register"]
pub mod fdcan_txbar;
#[doc = "FDCAN Tx Buffer Cancellation Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_txbcr](fdcan_txbcr) module"]
pub type FDCAN_TXBCR = crate::Reg<u32, _FDCAN_TXBCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TXBCR;
#[doc = "`read()` method returns [fdcan_txbcr::R](fdcan_txbcr::R) reader structure"]
impl crate::Readable for FDCAN_TXBCR {}
#[doc = "`write(|w| ..)` method takes [fdcan_txbcr::W](fdcan_txbcr::W) writer structure"]
impl crate::Writable for FDCAN_TXBCR {}
#[doc = "FDCAN Tx Buffer Cancellation Request Register"]
pub mod fdcan_txbcr;
#[doc = "FDCAN Tx Buffer Transmission Occurred Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_txbto](fdcan_txbto) module"]
pub type FDCAN_TXBTO = crate::Reg<u32, _FDCAN_TXBTO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TXBTO;
#[doc = "`read()` method returns [fdcan_txbto::R](fdcan_txbto::R) reader structure"]
impl crate::Readable for FDCAN_TXBTO {}
#[doc = "`write(|w| ..)` method takes [fdcan_txbto::W](fdcan_txbto::W) writer structure"]
impl crate::Writable for FDCAN_TXBTO {}
#[doc = "FDCAN Tx Buffer Transmission Occurred Register"]
pub mod fdcan_txbto;
#[doc = "FDCAN Tx Buffer Cancellation Finished Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_txbcf](fdcan_txbcf) module"]
pub type FDCAN_TXBCF = crate::Reg<u32, _FDCAN_TXBCF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TXBCF;
#[doc = "`read()` method returns [fdcan_txbcf::R](fdcan_txbcf::R) reader structure"]
impl crate::Readable for FDCAN_TXBCF {}
#[doc = "FDCAN Tx Buffer Cancellation Finished Register"]
pub mod fdcan_txbcf;
#[doc = "FDCAN Tx Buffer Transmission Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_txbtie](fdcan_txbtie) module"]
pub type FDCAN_TXBTIE = crate::Reg<u32, _FDCAN_TXBTIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TXBTIE;
#[doc = "`read()` method returns [fdcan_txbtie::R](fdcan_txbtie::R) reader structure"]
impl crate::Readable for FDCAN_TXBTIE {}
#[doc = "`write(|w| ..)` method takes [fdcan_txbtie::W](fdcan_txbtie::W) writer structure"]
impl crate::Writable for FDCAN_TXBTIE {}
#[doc = "FDCAN Tx Buffer Transmission Interrupt Enable Register"]
pub mod fdcan_txbtie;
#[doc = "FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_txbcie](fdcan_txbcie) module"]
pub type FDCAN_TXBCIE = crate::Reg<u32, _FDCAN_TXBCIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TXBCIE;
#[doc = "`read()` method returns [fdcan_txbcie::R](fdcan_txbcie::R) reader structure"]
impl crate::Readable for FDCAN_TXBCIE {}
#[doc = "`write(|w| ..)` method takes [fdcan_txbcie::W](fdcan_txbcie::W) writer structure"]
impl crate::Writable for FDCAN_TXBCIE {}
#[doc = "FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register"]
pub mod fdcan_txbcie;
#[doc = "FDCAN Tx Event FIFO Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_txefc](fdcan_txefc) module"]
pub type FDCAN_TXEFC = crate::Reg<u32, _FDCAN_TXEFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TXEFC;
#[doc = "`read()` method returns [fdcan_txefc::R](fdcan_txefc::R) reader structure"]
impl crate::Readable for FDCAN_TXEFC {}
#[doc = "`write(|w| ..)` method takes [fdcan_txefc::W](fdcan_txefc::W) writer structure"]
impl crate::Writable for FDCAN_TXEFC {}
#[doc = "FDCAN Tx Event FIFO Configuration Register"]
pub mod fdcan_txefc;
#[doc = "FDCAN Tx Event FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_txefs](fdcan_txefs) module"]
pub type FDCAN_TXEFS = crate::Reg<u32, _FDCAN_TXEFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TXEFS;
#[doc = "`read()` method returns [fdcan_txefs::R](fdcan_txefs::R) reader structure"]
impl crate::Readable for FDCAN_TXEFS {}
#[doc = "`write(|w| ..)` method takes [fdcan_txefs::W](fdcan_txefs::W) writer structure"]
impl crate::Writable for FDCAN_TXEFS {}
#[doc = "FDCAN Tx Event FIFO Status Register"]
pub mod fdcan_txefs;
#[doc = "FDCAN Tx Event FIFO Acknowledge Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_txefa](fdcan_txefa) module"]
pub type FDCAN_TXEFA = crate::Reg<u32, _FDCAN_TXEFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TXEFA;
#[doc = "`read()` method returns [fdcan_txefa::R](fdcan_txefa::R) reader structure"]
impl crate::Readable for FDCAN_TXEFA {}
#[doc = "`write(|w| ..)` method takes [fdcan_txefa::W](fdcan_txefa::W) writer structure"]
impl crate::Writable for FDCAN_TXEFA {}
#[doc = "FDCAN Tx Event FIFO Acknowledge Register"]
pub mod fdcan_txefa;
#[doc = "FDCAN TT Trigger Memory Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_tttmc](fdcan_tttmc) module"]
pub type FDCAN_TTTMC = crate::Reg<u32, _FDCAN_TTTMC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TTTMC;
#[doc = "`read()` method returns [fdcan_tttmc::R](fdcan_tttmc::R) reader structure"]
impl crate::Readable for FDCAN_TTTMC {}
#[doc = "`write(|w| ..)` method takes [fdcan_tttmc::W](fdcan_tttmc::W) writer structure"]
impl crate::Writable for FDCAN_TTTMC {}
#[doc = "FDCAN TT Trigger Memory Configuration Register"]
pub mod fdcan_tttmc;
#[doc = "FDCAN TT Reference Message Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ttrmc](fdcan_ttrmc) module"]
pub type FDCAN_TTRMC = crate::Reg<u32, _FDCAN_TTRMC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TTRMC;
#[doc = "`read()` method returns [fdcan_ttrmc::R](fdcan_ttrmc::R) reader structure"]
impl crate::Readable for FDCAN_TTRMC {}
#[doc = "`write(|w| ..)` method takes [fdcan_ttrmc::W](fdcan_ttrmc::W) writer structure"]
impl crate::Writable for FDCAN_TTRMC {}
#[doc = "FDCAN TT Reference Message Configuration Register"]
pub mod fdcan_ttrmc;
#[doc = "FDCAN TT Operation Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ttocf](fdcan_ttocf) module"]
pub type FDCAN_TTOCF = crate::Reg<u32, _FDCAN_TTOCF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TTOCF;
#[doc = "`read()` method returns [fdcan_ttocf::R](fdcan_ttocf::R) reader structure"]
impl crate::Readable for FDCAN_TTOCF {}
#[doc = "`write(|w| ..)` method takes [fdcan_ttocf::W](fdcan_ttocf::W) writer structure"]
impl crate::Writable for FDCAN_TTOCF {}
#[doc = "FDCAN TT Operation Configuration Register"]
pub mod fdcan_ttocf;
#[doc = "FDCAN TT Matrix Limits Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ttmlm](fdcan_ttmlm) module"]
pub type FDCAN_TTMLM = crate::Reg<u32, _FDCAN_TTMLM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TTMLM;
#[doc = "`read()` method returns [fdcan_ttmlm::R](fdcan_ttmlm::R) reader structure"]
impl crate::Readable for FDCAN_TTMLM {}
#[doc = "`write(|w| ..)` method takes [fdcan_ttmlm::W](fdcan_ttmlm::W) writer structure"]
impl crate::Writable for FDCAN_TTMLM {}
#[doc = "FDCAN TT Matrix Limits Register"]
pub mod fdcan_ttmlm;
#[doc = "FDCAN TUR Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_turcf](fdcan_turcf) module"]
pub type FDCAN_TURCF = crate::Reg<u32, _FDCAN_TURCF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TURCF;
#[doc = "`read()` method returns [fdcan_turcf::R](fdcan_turcf::R) reader structure"]
impl crate::Readable for FDCAN_TURCF {}
#[doc = "`write(|w| ..)` method takes [fdcan_turcf::W](fdcan_turcf::W) writer structure"]
impl crate::Writable for FDCAN_TURCF {}
#[doc = "FDCAN TUR Configuration Register"]
pub mod fdcan_turcf;
#[doc = "FDCAN TT Operation Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ttocn](fdcan_ttocn) module"]
pub type FDCAN_TTOCN = crate::Reg<u32, _FDCAN_TTOCN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TTOCN;
#[doc = "`read()` method returns [fdcan_ttocn::R](fdcan_ttocn::R) reader structure"]
impl crate::Readable for FDCAN_TTOCN {}
#[doc = "`write(|w| ..)` method takes [fdcan_ttocn::W](fdcan_ttocn::W) writer structure"]
impl crate::Writable for FDCAN_TTOCN {}
#[doc = "FDCAN TT Operation Control Register"]
pub mod fdcan_ttocn;
#[doc = "FDCAN TT Global Time Preset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_ttgtp](can_ttgtp) module"]
pub type CAN_TTGTP = crate::Reg<u32, _CAN_TTGTP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_TTGTP;
#[doc = "`read()` method returns [can_ttgtp::R](can_ttgtp::R) reader structure"]
impl crate::Readable for CAN_TTGTP {}
#[doc = "`write(|w| ..)` method takes [can_ttgtp::W](can_ttgtp::W) writer structure"]
impl crate::Writable for CAN_TTGTP {}
#[doc = "FDCAN TT Global Time Preset Register"]
pub mod can_ttgtp;
#[doc = "FDCAN TT Time Mark Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_tttmk](fdcan_tttmk) module"]
pub type FDCAN_TTTMK = crate::Reg<u32, _FDCAN_TTTMK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TTTMK;
#[doc = "`read()` method returns [fdcan_tttmk::R](fdcan_tttmk::R) reader structure"]
impl crate::Readable for FDCAN_TTTMK {}
#[doc = "`write(|w| ..)` method takes [fdcan_tttmk::W](fdcan_tttmk::W) writer structure"]
impl crate::Writable for FDCAN_TTTMK {}
#[doc = "FDCAN TT Time Mark Register"]
pub mod fdcan_tttmk;
#[doc = "FDCAN TT Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ttir](fdcan_ttir) module"]
pub type FDCAN_TTIR = crate::Reg<u32, _FDCAN_TTIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TTIR;
#[doc = "`read()` method returns [fdcan_ttir::R](fdcan_ttir::R) reader structure"]
impl crate::Readable for FDCAN_TTIR {}
#[doc = "`write(|w| ..)` method takes [fdcan_ttir::W](fdcan_ttir::W) writer structure"]
impl crate::Writable for FDCAN_TTIR {}
#[doc = "FDCAN TT Interrupt Register"]
pub mod fdcan_ttir;
#[doc = "FDCAN TT Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ttie](fdcan_ttie) module"]
pub type FDCAN_TTIE = crate::Reg<u32, _FDCAN_TTIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TTIE;
#[doc = "`read()` method returns [fdcan_ttie::R](fdcan_ttie::R) reader structure"]
impl crate::Readable for FDCAN_TTIE {}
#[doc = "`write(|w| ..)` method takes [fdcan_ttie::W](fdcan_ttie::W) writer structure"]
impl crate::Writable for FDCAN_TTIE {}
#[doc = "FDCAN TT Interrupt Enable Register"]
pub mod fdcan_ttie;
#[doc = "FDCAN TT Interrupt Line Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ttils](fdcan_ttils) module"]
pub type FDCAN_TTILS = crate::Reg<u32, _FDCAN_TTILS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TTILS;
#[doc = "`read()` method returns [fdcan_ttils::R](fdcan_ttils::R) reader structure"]
impl crate::Readable for FDCAN_TTILS {}
#[doc = "`write(|w| ..)` method takes [fdcan_ttils::W](fdcan_ttils::W) writer structure"]
impl crate::Writable for FDCAN_TTILS {}
#[doc = "FDCAN TT Interrupt Line Select Register"]
pub mod fdcan_ttils;
#[doc = "FDCAN TT Operation Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ttost](fdcan_ttost) module"]
pub type FDCAN_TTOST = crate::Reg<u32, _FDCAN_TTOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TTOST;
#[doc = "`read()` method returns [fdcan_ttost::R](fdcan_ttost::R) reader structure"]
impl crate::Readable for FDCAN_TTOST {}
#[doc = "FDCAN TT Operation Status Register"]
pub mod fdcan_ttost;
#[doc = "FDCAN TUR Numerator Actual Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_turna](fdcan_turna) module"]
pub type FDCAN_TURNA = crate::Reg<u32, _FDCAN_TURNA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TURNA;
#[doc = "`read()` method returns [fdcan_turna::R](fdcan_turna::R) reader structure"]
impl crate::Readable for FDCAN_TURNA {}
#[doc = "FDCAN TUR Numerator Actual Register"]
pub mod fdcan_turna;
#[doc = "FDCAN TT Local and Global Time Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ttlgt](fdcan_ttlgt) module"]
pub type FDCAN_TTLGT = crate::Reg<u32, _FDCAN_TTLGT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TTLGT;
#[doc = "`read()` method returns [fdcan_ttlgt::R](fdcan_ttlgt::R) reader structure"]
impl crate::Readable for FDCAN_TTLGT {}
#[doc = "FDCAN TT Local and Global Time Register"]
pub mod fdcan_ttlgt;
#[doc = "FDCAN TT Cycle Time and Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ttctc](fdcan_ttctc) module"]
pub type FDCAN_TTCTC = crate::Reg<u32, _FDCAN_TTCTC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TTCTC;
#[doc = "`read()` method returns [fdcan_ttctc::R](fdcan_ttctc::R) reader structure"]
impl crate::Readable for FDCAN_TTCTC {}
#[doc = "FDCAN TT Cycle Time and Count Register"]
pub mod fdcan_ttctc;
#[doc = "FDCAN TT Capture Time Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ttcpt](fdcan_ttcpt) module"]
pub type FDCAN_TTCPT = crate::Reg<u32, _FDCAN_TTCPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TTCPT;
#[doc = "`read()` method returns [fdcan_ttcpt::R](fdcan_ttcpt::R) reader structure"]
impl crate::Readable for FDCAN_TTCPT {}
#[doc = "FDCAN TT Capture Time Register"]
pub mod fdcan_ttcpt;
#[doc = "FDCAN TT Cycle Sync Mark Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ttcsm](fdcan_ttcsm) module"]
pub type FDCAN_TTCSM = crate::Reg<u32, _FDCAN_TTCSM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TTCSM;
#[doc = "`read()` method returns [fdcan_ttcsm::R](fdcan_ttcsm::R) reader structure"]
impl crate::Readable for FDCAN_TTCSM {}
#[doc = "FDCAN TT Cycle Sync Mark Register"]
pub mod fdcan_ttcsm;
#[doc = "FDCAN TT Trigger Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ttts](fdcan_ttts) module"]
pub type FDCAN_TTTS = crate::Reg<u32, _FDCAN_TTTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCAN_TTTS;
#[doc = "`read()` method returns [fdcan_ttts::R](fdcan_ttts::R) reader structure"]
impl crate::Readable for FDCAN_TTTS {}
#[doc = "`write(|w| ..)` method takes [fdcan_ttts::W](fdcan_ttts::W) writer structure"]
impl crate::Writable for FDCAN_TTTS {}
#[doc = "FDCAN TT Trigger Select Register"]
pub mod fdcan_ttts;
