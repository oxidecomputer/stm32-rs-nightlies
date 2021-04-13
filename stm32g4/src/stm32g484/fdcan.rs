#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FDCAN Core Release Register"]
    pub crel: CREL,
    #[doc = "0x04 - FDCAN Core Release Register"]
    pub endn: ENDN,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - This register is only writable if bits CCCR.CCE and CCCR.INIT are set. The CAN bit time may be programed in the range of 4 to 25 time quanta. The CAN time quantum may be programmed in the range of 1 to 1024 FDCAN clock periods. tq = (DBRP + 1) FDCAN clock period. DTSEG1 is the sum of Prop_Seg and Phase_Seg1. DTSEG2 is Phase_Seg2. Therefore the length of the bit time is (programmed values) \\[DTSEG1 + DTSEG2 + 3\\]
tq or (functional values) \\[Sync_Seg + Prop_Seg + Phase_Seg1 + Phase_Seg2\\]
tq. The Information Processing Time (IPT) is zero, meaning the data for the next bit is available at the first clock edge after the sample point."]
    pub dbtp: DBTP,
    #[doc = "0x10 - Write access to the Test Register has to be enabled by setting bit CCCR\\[TEST\\]
to 1 . All Test Register functions are set to their reset values when bit CCCR\\[TEST\\]
is reset. Loop Back mode and software control of Tx pin FDCANx_TX are hardware test modes. Programming TX differently from 00 may disturb the message transfer on the CAN bus."]
    pub test: TEST,
    #[doc = "0x14 - The RAM Watchdog monitors the READY output of the Message RAM. A Message RAM access starts the Message RAM Watchdog Counter with the value configured by the RWD\\[WDC\\]
bits. The counter is reloaded with RWD\\[WDC\\]
bits when the Message RAM signals successful completion by activating its READY output. In case there is no response from the Message RAM until the counter has counted down to 0, the counter stops and interrupt flag IR\\[WDI\\]
bit is set. The RAM Watchdog Counter is clocked by the fdcan_pclk clock."]
    pub rwd: RWD,
    #[doc = "0x18 - For details about setting and resetting of single bits see Software initialization."]
    pub cccr: CCCR,
    #[doc = "0x1c - FDCAN_NBTP"]
    pub nbtp: NBTP,
    #[doc = "0x20 - FDCAN Timestamp Counter Configuration Register"]
    pub tscc: TSCC,
    #[doc = "0x24 - FDCAN Timestamp Counter Value Register"]
    pub tscv: TSCV,
    #[doc = "0x28 - FDCAN Timeout Counter Configuration Register"]
    pub tocc: TOCC,
    #[doc = "0x2c - FDCAN Timeout Counter Value Register"]
    pub tocv: TOCV,
    _reserved11: [u8; 16usize],
    #[doc = "0x40 - FDCAN Error Counter Register"]
    pub ecr: ECR,
    #[doc = "0x44 - FDCAN Protocol Status Register"]
    pub psr: PSR,
    #[doc = "0x48 - FDCAN Transmitter Delay Compensation Register"]
    pub tdcr: TDCR,
    _reserved14: [u8; 4usize],
    #[doc = "0x50 - The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of IE controls whether an interrupt is generated. The configuration of ILS controls on which interrupt line an interrupt is signaled."]
    pub ir: IR,
    #[doc = "0x54 - The settings in the Interrupt Enable register determine which status changes in the Interrupt Register will be signaled on an interrupt line."]
    pub ie: IE,
    #[doc = "0x58 - The Interrupt Line Select register assigns an interrupt generated by a specific interrupt flag from the Interrupt Register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via ILE\\[EINT0\\]
and ILE\\[EINT1\\]."]
    pub ils: ILS,
    #[doc = "0x5c - Each of the two interrupt lines to the CPU can be enabled/disabled separately by programming bits EINT0 and EINT1."]
    pub ile: ILE,
    _reserved18: [u8; 32usize],
    #[doc = "0x80 - Global settings for Message ID filtering. The Global Filter Configuration controls the filter path for standard and extended messages as described in Figure706: Standard Message ID filter path and Figure707: Extended Message ID filter path."]
    pub rxgfc: RXGFC,
    #[doc = "0x84 - FDCAN Extended ID and Mask Register"]
    pub xidam: XIDAM,
    #[doc = "0x88 - This register is updated every time a Message ID filter element configured to generate a priority event match. This can be used to monitor the status of incoming high priority messages and to enable fast access to these messages."]
    pub hpms: HPMS,
    _reserved21: [u8; 4usize],
    #[doc = "0x90 - FDCAN Rx FIFO 0 Status Register"]
    pub rxf0s: RXF0S,
    #[doc = "0x94 - CAN Rx FIFO 0 Acknowledge Register"]
    pub rxf0a: RXF0A,
    #[doc = "0x98 - FDCAN Rx FIFO 1 Status Register"]
    pub rxf1s: RXF1S,
    #[doc = "0x9c - FDCAN Rx FIFO 1 Acknowledge Register"]
    pub rxf1a: RXF1A,
    _reserved25: [u8; 32usize],
    #[doc = "0xc0 - FDCAN Tx Buffer Configuration Register"]
    pub txbc: TXBC,
    #[doc = "0xc4 - The Tx FIFO/Queue status is related to the pending Tx requests listed in register TXBRP. Therefore the effect of Add/Cancellation requests may be delayed due to a running Tx scan (TXBRP not yet updated)."]
    pub txfqs: TXFQS,
    #[doc = "0xc8 - FDCAN Tx Buffer Request Pending Register"]
    pub txbrp: TXBRP,
    #[doc = "0xcc - FDCAN Tx Buffer Add Request Register"]
    pub txbar: TXBAR,
    #[doc = "0xd0 - FDCAN Tx Buffer Cancellation Request Register"]
    pub txbcr: TXBCR,
    #[doc = "0xd4 - FDCAN Tx Buffer Transmission Occurred Register"]
    pub txbto: TXBTO,
    #[doc = "0xd8 - FDCAN Tx Buffer Cancellation Finished Register"]
    pub txbcf: TXBCF,
    #[doc = "0xdc - FDCAN Tx Buffer Transmission Interrupt Enable Register"]
    pub txbtie: TXBTIE,
    #[doc = "0xe0 - FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register"]
    pub txbcie: TXBCIE,
    #[doc = "0xe4 - FDCAN Tx Event FIFO Status Register"]
    pub txefs: TXEFS,
    #[doc = "0xe8 - FDCAN Tx Event FIFO Acknowledge Register"]
    pub txefa: TXEFA,
    _reserved36: [u8; 20usize],
    #[doc = "0x100 - FDCAN CFG clock divider register"]
    pub ckdiv: CKDIV,
}
#[doc = "FDCAN Core Release Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crel](crel) module"]
pub type CREL = crate::Reg<u32, _CREL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CREL;
#[doc = "`read()` method returns [crel::R](crel::R) reader structure"]
impl crate::Readable for CREL {}
#[doc = "FDCAN Core Release Register"]
pub mod crel;
#[doc = "FDCAN Core Release Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endn](endn) module"]
pub type ENDN = crate::Reg<u32, _ENDN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENDN;
#[doc = "`read()` method returns [endn::R](endn::R) reader structure"]
impl crate::Readable for ENDN {}
#[doc = "FDCAN Core Release Register"]
pub mod endn;
#[doc = "This register is only writable if bits CCCR.CCE and CCCR.INIT are set. The CAN bit time may be programed in the range of 4 to 25 time quanta. The CAN time quantum may be programmed in the range of 1 to 1024 FDCAN clock periods. tq = (DBRP + 1) FDCAN clock period. DTSEG1 is the sum of Prop_Seg and Phase_Seg1. DTSEG2 is Phase_Seg2. Therefore the length of the bit time is (programmed values) \\[DTSEG1 + DTSEG2 + 3\\]
tq or (functional values) \\[Sync_Seg + Prop_Seg + Phase_Seg1 + Phase_Seg2\\]
tq. The Information Processing Time (IPT) is zero, meaning the data for the next bit is available at the first clock edge after the sample point.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbtp](dbtp) module"]
pub type DBTP = crate::Reg<u32, _DBTP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBTP;
#[doc = "`read()` method returns [dbtp::R](dbtp::R) reader structure"]
impl crate::Readable for DBTP {}
#[doc = "`write(|w| ..)` method takes [dbtp::W](dbtp::W) writer structure"]
impl crate::Writable for DBTP {}
#[doc = "This register is only writable if bits CCCR.CCE and CCCR.INIT are set. The CAN bit time may be programed in the range of 4 to 25 time quanta. The CAN time quantum may be programmed in the range of 1 to 1024 FDCAN clock periods. tq = (DBRP + 1) FDCAN clock period. DTSEG1 is the sum of Prop_Seg and Phase_Seg1. DTSEG2 is Phase_Seg2. Therefore the length of the bit time is (programmed values) \\[DTSEG1 + DTSEG2 + 3\\]
tq or (functional values) \\[Sync_Seg + Prop_Seg + Phase_Seg1 + Phase_Seg2\\]
tq. The Information Processing Time (IPT) is zero, meaning the data for the next bit is available at the first clock edge after the sample point."]
pub mod dbtp;
#[doc = "Write access to the Test Register has to be enabled by setting bit CCCR\\[TEST\\]
to 1 . All Test Register functions are set to their reset values when bit CCCR\\[TEST\\]
is reset. Loop Back mode and software control of Tx pin FDCANx_TX are hardware test modes. Programming TX differently from 00 may disturb the message transfer on the CAN bus.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test](test) module"]
pub type TEST = crate::Reg<u32, _TEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEST;
#[doc = "`read()` method returns [test::R](test::R) reader structure"]
impl crate::Readable for TEST {}
#[doc = "`write(|w| ..)` method takes [test::W](test::W) writer structure"]
impl crate::Writable for TEST {}
#[doc = "Write access to the Test Register has to be enabled by setting bit CCCR\\[TEST\\]
to 1 . All Test Register functions are set to their reset values when bit CCCR\\[TEST\\]
is reset. Loop Back mode and software control of Tx pin FDCANx_TX are hardware test modes. Programming TX differently from 00 may disturb the message transfer on the CAN bus."]
pub mod test;
#[doc = "The RAM Watchdog monitors the READY output of the Message RAM. A Message RAM access starts the Message RAM Watchdog Counter with the value configured by the RWD\\[WDC\\]
bits. The counter is reloaded with RWD\\[WDC\\]
bits when the Message RAM signals successful completion by activating its READY output. In case there is no response from the Message RAM until the counter has counted down to 0, the counter stops and interrupt flag IR\\[WDI\\]
bit is set. The RAM Watchdog Counter is clocked by the fdcan_pclk clock.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rwd](rwd) module"]
pub type RWD = crate::Reg<u32, _RWD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RWD;
#[doc = "`read()` method returns [rwd::R](rwd::R) reader structure"]
impl crate::Readable for RWD {}
#[doc = "`write(|w| ..)` method takes [rwd::W](rwd::W) writer structure"]
impl crate::Writable for RWD {}
#[doc = "The RAM Watchdog monitors the READY output of the Message RAM. A Message RAM access starts the Message RAM Watchdog Counter with the value configured by the RWD\\[WDC\\]
bits. The counter is reloaded with RWD\\[WDC\\]
bits when the Message RAM signals successful completion by activating its READY output. In case there is no response from the Message RAM until the counter has counted down to 0, the counter stops and interrupt flag IR\\[WDI\\]
bit is set. The RAM Watchdog Counter is clocked by the fdcan_pclk clock."]
pub mod rwd;
#[doc = "For details about setting and resetting of single bits see Software initialization.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cccr](cccr) module"]
pub type CCCR = crate::Reg<u32, _CCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCCR;
#[doc = "`read()` method returns [cccr::R](cccr::R) reader structure"]
impl crate::Readable for CCCR {}
#[doc = "`write(|w| ..)` method takes [cccr::W](cccr::W) writer structure"]
impl crate::Writable for CCCR {}
#[doc = "For details about setting and resetting of single bits see Software initialization."]
pub mod cccr;
#[doc = "FDCAN_NBTP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nbtp](nbtp) module"]
pub type NBTP = crate::Reg<u32, _NBTP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NBTP;
#[doc = "`read()` method returns [nbtp::R](nbtp::R) reader structure"]
impl crate::Readable for NBTP {}
#[doc = "`write(|w| ..)` method takes [nbtp::W](nbtp::W) writer structure"]
impl crate::Writable for NBTP {}
#[doc = "FDCAN_NBTP"]
pub mod nbtp;
#[doc = "FDCAN Timestamp Counter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tscc](tscc) module"]
pub type TSCC = crate::Reg<u32, _TSCC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSCC;
#[doc = "`read()` method returns [tscc::R](tscc::R) reader structure"]
impl crate::Readable for TSCC {}
#[doc = "`write(|w| ..)` method takes [tscc::W](tscc::W) writer structure"]
impl crate::Writable for TSCC {}
#[doc = "FDCAN Timestamp Counter Configuration Register"]
pub mod tscc;
#[doc = "FDCAN Timestamp Counter Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tscv](tscv) module"]
pub type TSCV = crate::Reg<u32, _TSCV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSCV;
#[doc = "`read()` method returns [tscv::R](tscv::R) reader structure"]
impl crate::Readable for TSCV {}
#[doc = "`write(|w| ..)` method takes [tscv::W](tscv::W) writer structure"]
impl crate::Writable for TSCV {}
#[doc = "FDCAN Timestamp Counter Value Register"]
pub mod tscv;
#[doc = "FDCAN Timeout Counter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tocc](tocc) module"]
pub type TOCC = crate::Reg<u32, _TOCC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOCC;
#[doc = "`read()` method returns [tocc::R](tocc::R) reader structure"]
impl crate::Readable for TOCC {}
#[doc = "`write(|w| ..)` method takes [tocc::W](tocc::W) writer structure"]
impl crate::Writable for TOCC {}
#[doc = "FDCAN Timeout Counter Configuration Register"]
pub mod tocc;
#[doc = "FDCAN Timeout Counter Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tocv](tocv) module"]
pub type TOCV = crate::Reg<u32, _TOCV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOCV;
#[doc = "`read()` method returns [tocv::R](tocv::R) reader structure"]
impl crate::Readable for TOCV {}
#[doc = "`write(|w| ..)` method takes [tocv::W](tocv::W) writer structure"]
impl crate::Writable for TOCV {}
#[doc = "FDCAN Timeout Counter Value Register"]
pub mod tocv;
#[doc = "FDCAN Error Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecr](ecr) module"]
pub type ECR = crate::Reg<u32, _ECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECR;
#[doc = "`read()` method returns [ecr::R](ecr::R) reader structure"]
impl crate::Readable for ECR {}
#[doc = "FDCAN Error Counter Register"]
pub mod ecr;
#[doc = "FDCAN Protocol Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psr](psr) module"]
pub type PSR = crate::Reg<u32, _PSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSR;
#[doc = "`read()` method returns [psr::R](psr::R) reader structure"]
impl crate::Readable for PSR {}
#[doc = "`write(|w| ..)` method takes [psr::W](psr::W) writer structure"]
impl crate::Writable for PSR {}
#[doc = "FDCAN Protocol Status Register"]
pub mod psr;
#[doc = "FDCAN Transmitter Delay Compensation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdcr](tdcr) module"]
pub type TDCR = crate::Reg<u32, _TDCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TDCR;
#[doc = "`read()` method returns [tdcr::R](tdcr::R) reader structure"]
impl crate::Readable for TDCR {}
#[doc = "`write(|w| ..)` method takes [tdcr::W](tdcr::W) writer structure"]
impl crate::Writable for TDCR {}
#[doc = "FDCAN Transmitter Delay Compensation Register"]
pub mod tdcr;
#[doc = "The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of IE controls whether an interrupt is generated. The configuration of ILS controls on which interrupt line an interrupt is signaled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ir](ir) module"]
pub type IR = crate::Reg<u32, _IR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IR;
#[doc = "`read()` method returns [ir::R](ir::R) reader structure"]
impl crate::Readable for IR {}
#[doc = "`write(|w| ..)` method takes [ir::W](ir::W) writer structure"]
impl crate::Writable for IR {}
#[doc = "The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of IE controls whether an interrupt is generated. The configuration of ILS controls on which interrupt line an interrupt is signaled."]
pub mod ir;
#[doc = "The settings in the Interrupt Enable register determine which status changes in the Interrupt Register will be signaled on an interrupt line.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ie](ie) module"]
pub type IE = crate::Reg<u32, _IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IE;
#[doc = "`read()` method returns [ie::R](ie::R) reader structure"]
impl crate::Readable for IE {}
#[doc = "`write(|w| ..)` method takes [ie::W](ie::W) writer structure"]
impl crate::Writable for IE {}
#[doc = "The settings in the Interrupt Enable register determine which status changes in the Interrupt Register will be signaled on an interrupt line."]
pub mod ie;
#[doc = "The Interrupt Line Select register assigns an interrupt generated by a specific interrupt flag from the Interrupt Register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via ILE\\[EINT0\\]
and ILE\\[EINT1\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ils](ils) module"]
pub type ILS = crate::Reg<u32, _ILS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ILS;
#[doc = "`read()` method returns [ils::R](ils::R) reader structure"]
impl crate::Readable for ILS {}
#[doc = "`write(|w| ..)` method takes [ils::W](ils::W) writer structure"]
impl crate::Writable for ILS {}
#[doc = "The Interrupt Line Select register assigns an interrupt generated by a specific interrupt flag from the Interrupt Register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via ILE\\[EINT0\\]
and ILE\\[EINT1\\]."]
pub mod ils;
#[doc = "Each of the two interrupt lines to the CPU can be enabled/disabled separately by programming bits EINT0 and EINT1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ile](ile) module"]
pub type ILE = crate::Reg<u32, _ILE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ILE;
#[doc = "`read()` method returns [ile::R](ile::R) reader structure"]
impl crate::Readable for ILE {}
#[doc = "`write(|w| ..)` method takes [ile::W](ile::W) writer structure"]
impl crate::Writable for ILE {}
#[doc = "Each of the two interrupt lines to the CPU can be enabled/disabled separately by programming bits EINT0 and EINT1."]
pub mod ile;
#[doc = "Global settings for Message ID filtering. The Global Filter Configuration controls the filter path for standard and extended messages as described in Figure706: Standard Message ID filter path and Figure707: Extended Message ID filter path.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxgfc](rxgfc) module"]
pub type RXGFC = crate::Reg<u32, _RXGFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXGFC;
#[doc = "`read()` method returns [rxgfc::R](rxgfc::R) reader structure"]
impl crate::Readable for RXGFC {}
#[doc = "`write(|w| ..)` method takes [rxgfc::W](rxgfc::W) writer structure"]
impl crate::Writable for RXGFC {}
#[doc = "Global settings for Message ID filtering. The Global Filter Configuration controls the filter path for standard and extended messages as described in Figure706: Standard Message ID filter path and Figure707: Extended Message ID filter path."]
pub mod rxgfc;
#[doc = "FDCAN Extended ID and Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xidam](xidam) module"]
pub type XIDAM = crate::Reg<u32, _XIDAM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XIDAM;
#[doc = "`read()` method returns [xidam::R](xidam::R) reader structure"]
impl crate::Readable for XIDAM {}
#[doc = "`write(|w| ..)` method takes [xidam::W](xidam::W) writer structure"]
impl crate::Writable for XIDAM {}
#[doc = "FDCAN Extended ID and Mask Register"]
pub mod xidam;
#[doc = "This register is updated every time a Message ID filter element configured to generate a priority event match. This can be used to monitor the status of incoming high priority messages and to enable fast access to these messages.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hpms](hpms) module"]
pub type HPMS = crate::Reg<u32, _HPMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPMS;
#[doc = "`read()` method returns [hpms::R](hpms::R) reader structure"]
impl crate::Readable for HPMS {}
#[doc = "This register is updated every time a Message ID filter element configured to generate a priority event match. This can be used to monitor the status of incoming high priority messages and to enable fast access to these messages."]
pub mod hpms;
#[doc = "FDCAN Rx FIFO 0 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf0s](rxf0s) module"]
pub type RXF0S = crate::Reg<u32, _RXF0S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXF0S;
#[doc = "`read()` method returns [rxf0s::R](rxf0s::R) reader structure"]
impl crate::Readable for RXF0S {}
#[doc = "`write(|w| ..)` method takes [rxf0s::W](rxf0s::W) writer structure"]
impl crate::Writable for RXF0S {}
#[doc = "FDCAN Rx FIFO 0 Status Register"]
pub mod rxf0s;
#[doc = "CAN Rx FIFO 0 Acknowledge Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf0a](rxf0a) module"]
pub type RXF0A = crate::Reg<u32, _RXF0A>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXF0A;
#[doc = "`read()` method returns [rxf0a::R](rxf0a::R) reader structure"]
impl crate::Readable for RXF0A {}
#[doc = "`write(|w| ..)` method takes [rxf0a::W](rxf0a::W) writer structure"]
impl crate::Writable for RXF0A {}
#[doc = "CAN Rx FIFO 0 Acknowledge Register"]
pub mod rxf0a;
#[doc = "FDCAN Rx FIFO 1 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf1s](rxf1s) module"]
pub type RXF1S = crate::Reg<u32, _RXF1S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXF1S;
#[doc = "`read()` method returns [rxf1s::R](rxf1s::R) reader structure"]
impl crate::Readable for RXF1S {}
#[doc = "FDCAN Rx FIFO 1 Status Register"]
pub mod rxf1s;
#[doc = "FDCAN Rx FIFO 1 Acknowledge Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf1a](rxf1a) module"]
pub type RXF1A = crate::Reg<u32, _RXF1A>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXF1A;
#[doc = "`read()` method returns [rxf1a::R](rxf1a::R) reader structure"]
impl crate::Readable for RXF1A {}
#[doc = "`write(|w| ..)` method takes [rxf1a::W](rxf1a::W) writer structure"]
impl crate::Writable for RXF1A {}
#[doc = "FDCAN Rx FIFO 1 Acknowledge Register"]
pub mod rxf1a;
#[doc = "FDCAN Tx Buffer Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbc](txbc) module"]
pub type TXBC = crate::Reg<u32, _TXBC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBC;
#[doc = "`read()` method returns [txbc::R](txbc::R) reader structure"]
impl crate::Readable for TXBC {}
#[doc = "`write(|w| ..)` method takes [txbc::W](txbc::W) writer structure"]
impl crate::Writable for TXBC {}
#[doc = "FDCAN Tx Buffer Configuration Register"]
pub mod txbc;
#[doc = "The Tx FIFO/Queue status is related to the pending Tx requests listed in register TXBRP. Therefore the effect of Add/Cancellation requests may be delayed due to a running Tx scan (TXBRP not yet updated).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfqs](txfqs) module"]
pub type TXFQS = crate::Reg<u32, _TXFQS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXFQS;
#[doc = "`read()` method returns [txfqs::R](txfqs::R) reader structure"]
impl crate::Readable for TXFQS {}
#[doc = "The Tx FIFO/Queue status is related to the pending Tx requests listed in register TXBRP. Therefore the effect of Add/Cancellation requests may be delayed due to a running Tx scan (TXBRP not yet updated)."]
pub mod txfqs;
#[doc = "FDCAN Tx Buffer Request Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbrp](txbrp) module"]
pub type TXBRP = crate::Reg<u32, _TXBRP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBRP;
#[doc = "`read()` method returns [txbrp::R](txbrp::R) reader structure"]
impl crate::Readable for TXBRP {}
#[doc = "FDCAN Tx Buffer Request Pending Register"]
pub mod txbrp;
#[doc = "FDCAN Tx Buffer Add Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbar](txbar) module"]
pub type TXBAR = crate::Reg<u32, _TXBAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBAR;
#[doc = "`read()` method returns [txbar::R](txbar::R) reader structure"]
impl crate::Readable for TXBAR {}
#[doc = "`write(|w| ..)` method takes [txbar::W](txbar::W) writer structure"]
impl crate::Writable for TXBAR {}
#[doc = "FDCAN Tx Buffer Add Request Register"]
pub mod txbar;
#[doc = "FDCAN Tx Buffer Cancellation Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbcr](txbcr) module"]
pub type TXBCR = crate::Reg<u32, _TXBCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBCR;
#[doc = "`read()` method returns [txbcr::R](txbcr::R) reader structure"]
impl crate::Readable for TXBCR {}
#[doc = "`write(|w| ..)` method takes [txbcr::W](txbcr::W) writer structure"]
impl crate::Writable for TXBCR {}
#[doc = "FDCAN Tx Buffer Cancellation Request Register"]
pub mod txbcr;
#[doc = "FDCAN Tx Buffer Transmission Occurred Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbto](txbto) module"]
pub type TXBTO = crate::Reg<u32, _TXBTO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBTO;
#[doc = "`read()` method returns [txbto::R](txbto::R) reader structure"]
impl crate::Readable for TXBTO {}
#[doc = "FDCAN Tx Buffer Transmission Occurred Register"]
pub mod txbto;
#[doc = "FDCAN Tx Buffer Cancellation Finished Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbcf](txbcf) module"]
pub type TXBCF = crate::Reg<u32, _TXBCF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBCF;
#[doc = "`read()` method returns [txbcf::R](txbcf::R) reader structure"]
impl crate::Readable for TXBCF {}
#[doc = "FDCAN Tx Buffer Cancellation Finished Register"]
pub mod txbcf;
#[doc = "FDCAN Tx Buffer Transmission Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbtie](txbtie) module"]
pub type TXBTIE = crate::Reg<u32, _TXBTIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBTIE;
#[doc = "`read()` method returns [txbtie::R](txbtie::R) reader structure"]
impl crate::Readable for TXBTIE {}
#[doc = "`write(|w| ..)` method takes [txbtie::W](txbtie::W) writer structure"]
impl crate::Writable for TXBTIE {}
#[doc = "FDCAN Tx Buffer Transmission Interrupt Enable Register"]
pub mod txbtie;
#[doc = "FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbcie](txbcie) module"]
pub type TXBCIE = crate::Reg<u32, _TXBCIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBCIE;
#[doc = "`read()` method returns [txbcie::R](txbcie::R) reader structure"]
impl crate::Readable for TXBCIE {}
#[doc = "`write(|w| ..)` method takes [txbcie::W](txbcie::W) writer structure"]
impl crate::Writable for TXBCIE {}
#[doc = "FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register"]
pub mod txbcie;
#[doc = "FDCAN Tx Event FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txefs](txefs) module"]
pub type TXEFS = crate::Reg<u32, _TXEFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXEFS;
#[doc = "`read()` method returns [txefs::R](txefs::R) reader structure"]
impl crate::Readable for TXEFS {}
#[doc = "FDCAN Tx Event FIFO Status Register"]
pub mod txefs;
#[doc = "FDCAN Tx Event FIFO Acknowledge Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txefa](txefa) module"]
pub type TXEFA = crate::Reg<u32, _TXEFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXEFA;
#[doc = "`read()` method returns [txefa::R](txefa::R) reader structure"]
impl crate::Readable for TXEFA {}
#[doc = "`write(|w| ..)` method takes [txefa::W](txefa::W) writer structure"]
impl crate::Writable for TXEFA {}
#[doc = "FDCAN Tx Event FIFO Acknowledge Register"]
pub mod txefa;
#[doc = "FDCAN CFG clock divider register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckdiv](ckdiv) module"]
pub type CKDIV = crate::Reg<u32, _CKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKDIV;
#[doc = "`read()` method returns [ckdiv::R](ckdiv::R) reader structure"]
impl crate::Readable for CKDIV {}
#[doc = "`write(|w| ..)` method takes [ckdiv::W](ckdiv::W) writer structure"]
impl crate::Writable for CKDIV {}
#[doc = "FDCAN CFG clock divider register"]
pub mod ckdiv;
