#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR"]
    pub cha: CH,
    #[doc = "0x24 - Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR"]
    pub chb: CH,
    #[doc = "0x44 - PDM control register"]
    pub pdmcr: PDMCR,
    #[doc = "0x48 - PDM delay register"]
    pub pdmdly: PDMDLY,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - AConfiguration register 1"]
    pub cr1: self::ch::CR1,
    #[doc = "0x04 - AConfiguration register 2"]
    pub cr2: self::ch::CR2,
    #[doc = "0x08 - AFRCR"]
    pub frcr: self::ch::FRCR,
    #[doc = "0x0c - ASlot register"]
    pub slotr: self::ch::SLOTR,
    #[doc = "0x10 - AInterrupt mask register2"]
    pub im: self::ch::IM,
    #[doc = "0x14 - AStatus register"]
    pub sr: self::ch::SR,
    #[doc = "0x18 - AClear flag register"]
    pub clrfr: self::ch::CLRFR,
    #[doc = "0x1c - AData register"]
    pub dr: self::ch::DR,
}
#[doc = r"Register block"]
#[doc = "Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR"]
pub mod ch;
#[doc = "PDM control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdmcr](pdmcr) module"]
pub type PDMCR = crate::Reg<u32, _PDMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMCR;
#[doc = "`read()` method returns [pdmcr::R](pdmcr::R) reader structure"]
impl crate::Readable for PDMCR {}
#[doc = "`write(|w| ..)` method takes [pdmcr::W](pdmcr::W) writer structure"]
impl crate::Writable for PDMCR {}
#[doc = "PDM control register"]
pub mod pdmcr;
#[doc = "PDM delay register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdmdly](pdmdly) module"]
pub type PDMDLY = crate::Reg<u32, _PDMDLY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMDLY;
#[doc = "`read()` method returns [pdmdly::R](pdmdly::R) reader structure"]
impl crate::Readable for PDMDLY {}
#[doc = "`write(|w| ..)` method takes [pdmdly::W](pdmdly::W) writer structure"]
impl crate::Writable for PDMDLY {}
#[doc = "PDM delay register"]
pub mod pdmdly;
