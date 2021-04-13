#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - QUADSPI control register"]
    pub cr: CR,
    #[doc = "0x04 - QUADSPI device configuration register"]
    pub dcr: DCR,
    #[doc = "0x08 - QUADSPI status register"]
    pub sr: SR,
    #[doc = "0x0c - QUADSPI flag clear register"]
    pub fcr: FCR,
    #[doc = "0x10 - QUADSPI data length register"]
    pub dlr: DLR,
    #[doc = "0x14 - QUADSPI communication configuration register"]
    pub ccr: CCR,
    #[doc = "0x18 - QUADSPI address register"]
    pub ar: AR,
    #[doc = "0x1c - QUADSPI alternate bytes registers"]
    pub abr: ABR,
    #[doc = "0x20 - QUADSPI data register"]
    pub dr: DR,
    #[doc = "0x24 - QUADSPI polling status mask register"]
    pub psmkr: PSMKR,
    #[doc = "0x28 - QUADSPI polling status match register"]
    pub psmar: PSMAR,
    #[doc = "0x2c - QUADSPI polling interval register"]
    pub pir: PIR,
    #[doc = "0x30 - QUADSPI low-power timeout register"]
    pub lptr: LPTR,
}
#[doc = "QUADSPI control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "QUADSPI control register"]
pub mod cr;
#[doc = "QUADSPI device configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcr](dcr) module"]
pub type DCR = crate::Reg<u32, _DCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCR;
#[doc = "`read()` method returns [dcr::R](dcr::R) reader structure"]
impl crate::Readable for DCR {}
#[doc = "`write(|w| ..)` method takes [dcr::W](dcr::W) writer structure"]
impl crate::Writable for DCR {}
#[doc = "QUADSPI device configuration register"]
pub mod dcr;
#[doc = "QUADSPI status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "QUADSPI status register"]
pub mod sr;
#[doc = "QUADSPI flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr](fcr) module"]
pub type FCR = crate::Reg<u32, _FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCR;
#[doc = "`read()` method returns [fcr::R](fcr::R) reader structure"]
impl crate::Readable for FCR {}
#[doc = "`write(|w| ..)` method takes [fcr::W](fcr::W) writer structure"]
impl crate::Writable for FCR {}
#[doc = "QUADSPI flag clear register"]
pub mod fcr;
#[doc = "QUADSPI data length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlr](dlr) module"]
pub type DLR = crate::Reg<u32, _DLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLR;
#[doc = "`read()` method returns [dlr::R](dlr::R) reader structure"]
impl crate::Readable for DLR {}
#[doc = "`write(|w| ..)` method takes [dlr::W](dlr::W) writer structure"]
impl crate::Writable for DLR {}
#[doc = "QUADSPI data length register"]
pub mod dlr;
#[doc = "QUADSPI communication configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](ccr) module"]
pub type CCR = crate::Reg<u32, _CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR;
#[doc = "`read()` method returns [ccr::R](ccr::R) reader structure"]
impl crate::Readable for CCR {}
#[doc = "`write(|w| ..)` method takes [ccr::W](ccr::W) writer structure"]
impl crate::Writable for CCR {}
#[doc = "QUADSPI communication configuration register"]
pub mod ccr;
#[doc = "QUADSPI address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ar](ar) module"]
pub type AR = crate::Reg<u32, _AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AR;
#[doc = "`read()` method returns [ar::R](ar::R) reader structure"]
impl crate::Readable for AR {}
#[doc = "`write(|w| ..)` method takes [ar::W](ar::W) writer structure"]
impl crate::Writable for AR {}
#[doc = "QUADSPI address register"]
pub mod ar;
#[doc = "QUADSPI alternate bytes registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [abr](abr) module"]
pub type ABR = crate::Reg<u32, _ABR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ABR;
#[doc = "`read()` method returns [abr::R](abr::R) reader structure"]
impl crate::Readable for ABR {}
#[doc = "`write(|w| ..)` method takes [abr::W](abr::W) writer structure"]
impl crate::Writable for ABR {}
#[doc = "QUADSPI alternate bytes registers"]
pub mod abr;
#[doc = "QUADSPI data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](dr) module"]
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
#[doc = "`read()` method returns [dr::R](dr::R) reader structure"]
impl crate::Readable for DR {}
#[doc = "`write(|w| ..)` method takes [dr::W](dr::W) writer structure"]
impl crate::Writable for DR {}
#[doc = "QUADSPI data register"]
pub mod dr;
#[doc = "QUADSPI polling status mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psmkr](psmkr) module"]
pub type PSMKR = crate::Reg<u32, _PSMKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSMKR;
#[doc = "`read()` method returns [psmkr::R](psmkr::R) reader structure"]
impl crate::Readable for PSMKR {}
#[doc = "`write(|w| ..)` method takes [psmkr::W](psmkr::W) writer structure"]
impl crate::Writable for PSMKR {}
#[doc = "QUADSPI polling status mask register"]
pub mod psmkr;
#[doc = "QUADSPI polling status match register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psmar](psmar) module"]
pub type PSMAR = crate::Reg<u32, _PSMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSMAR;
#[doc = "`read()` method returns [psmar::R](psmar::R) reader structure"]
impl crate::Readable for PSMAR {}
#[doc = "`write(|w| ..)` method takes [psmar::W](psmar::W) writer structure"]
impl crate::Writable for PSMAR {}
#[doc = "QUADSPI polling status match register"]
pub mod psmar;
#[doc = "QUADSPI polling interval register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pir](pir) module"]
pub type PIR = crate::Reg<u32, _PIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIR;
#[doc = "`read()` method returns [pir::R](pir::R) reader structure"]
impl crate::Readable for PIR {}
#[doc = "`write(|w| ..)` method takes [pir::W](pir::W) writer structure"]
impl crate::Writable for PIR {}
#[doc = "QUADSPI polling interval register"]
pub mod pir;
#[doc = "QUADSPI low-power timeout register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptr](lptr) module"]
pub type LPTR = crate::Reg<u32, _LPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPTR;
#[doc = "`read()` method returns [lptr::R](lptr::R) reader structure"]
impl crate::Readable for LPTR {}
#[doc = "`write(|w| ..)` method takes [lptr::W](lptr::W) writer structure"]
impl crate::Writable for LPTR {}
#[doc = "QUADSPI low-power timeout register"]
pub mod lptr;
