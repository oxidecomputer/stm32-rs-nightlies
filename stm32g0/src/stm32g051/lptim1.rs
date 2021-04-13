#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt and Status Register"]
    pub lptim_isr: LPTIM_ISR,
    #[doc = "0x04 - Interrupt Clear Register"]
    pub lptim_icr: LPTIM_ICR,
    #[doc = "0x08 - Interrupt Enable Register"]
    pub lptim_ier: LPTIM_IER,
    #[doc = "0x0c - Configuration Register"]
    pub lptim_cfgr: LPTIM_CFGR,
    #[doc = "0x10 - Control Register"]
    pub lptim_cr: LPTIM_CR,
    #[doc = "0x14 - Compare Register"]
    pub lptim_cmp: LPTIM_CMP,
    #[doc = "0x18 - Autoreload Register"]
    pub lptim_arr: LPTIM_ARR,
    #[doc = "0x1c - Counter Register"]
    pub lptim_cnt: LPTIM_CNT,
    _reserved8: [u8; 4usize],
    #[doc = "0x24 - LPTIM configuration register 2"]
    pub lptim_cfgr2: LPTIM_CFGR2,
}
#[doc = "Interrupt and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_isr](lptim_isr) module"]
pub type LPTIM_ISR = crate::Reg<u32, _LPTIM_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPTIM_ISR;
#[doc = "`read()` method returns [lptim_isr::R](lptim_isr::R) reader structure"]
impl crate::Readable for LPTIM_ISR {}
#[doc = "Interrupt and Status Register"]
pub mod lptim_isr;
#[doc = "Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_icr](lptim_icr) module"]
pub type LPTIM_ICR = crate::Reg<u32, _LPTIM_ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPTIM_ICR;
#[doc = "`write(|w| ..)` method takes [lptim_icr::W](lptim_icr::W) writer structure"]
impl crate::Writable for LPTIM_ICR {}
#[doc = "Interrupt Clear Register"]
pub mod lptim_icr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_ier](lptim_ier) module"]
pub type LPTIM_IER = crate::Reg<u32, _LPTIM_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPTIM_IER;
#[doc = "`read()` method returns [lptim_ier::R](lptim_ier::R) reader structure"]
impl crate::Readable for LPTIM_IER {}
#[doc = "`write(|w| ..)` method takes [lptim_ier::W](lptim_ier::W) writer structure"]
impl crate::Writable for LPTIM_IER {}
#[doc = "Interrupt Enable Register"]
pub mod lptim_ier;
#[doc = "Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_cfgr](lptim_cfgr) module"]
pub type LPTIM_CFGR = crate::Reg<u32, _LPTIM_CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPTIM_CFGR;
#[doc = "`read()` method returns [lptim_cfgr::R](lptim_cfgr::R) reader structure"]
impl crate::Readable for LPTIM_CFGR {}
#[doc = "`write(|w| ..)` method takes [lptim_cfgr::W](lptim_cfgr::W) writer structure"]
impl crate::Writable for LPTIM_CFGR {}
#[doc = "Configuration Register"]
pub mod lptim_cfgr;
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_cr](lptim_cr) module"]
pub type LPTIM_CR = crate::Reg<u32, _LPTIM_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPTIM_CR;
#[doc = "`read()` method returns [lptim_cr::R](lptim_cr::R) reader structure"]
impl crate::Readable for LPTIM_CR {}
#[doc = "`write(|w| ..)` method takes [lptim_cr::W](lptim_cr::W) writer structure"]
impl crate::Writable for LPTIM_CR {}
#[doc = "Control Register"]
pub mod lptim_cr;
#[doc = "Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_cmp](lptim_cmp) module"]
pub type LPTIM_CMP = crate::Reg<u32, _LPTIM_CMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPTIM_CMP;
#[doc = "`read()` method returns [lptim_cmp::R](lptim_cmp::R) reader structure"]
impl crate::Readable for LPTIM_CMP {}
#[doc = "`write(|w| ..)` method takes [lptim_cmp::W](lptim_cmp::W) writer structure"]
impl crate::Writable for LPTIM_CMP {}
#[doc = "Compare Register"]
pub mod lptim_cmp;
#[doc = "Autoreload Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_arr](lptim_arr) module"]
pub type LPTIM_ARR = crate::Reg<u32, _LPTIM_ARR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPTIM_ARR;
#[doc = "`read()` method returns [lptim_arr::R](lptim_arr::R) reader structure"]
impl crate::Readable for LPTIM_ARR {}
#[doc = "`write(|w| ..)` method takes [lptim_arr::W](lptim_arr::W) writer structure"]
impl crate::Writable for LPTIM_ARR {}
#[doc = "Autoreload Register"]
pub mod lptim_arr;
#[doc = "Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_cnt](lptim_cnt) module"]
pub type LPTIM_CNT = crate::Reg<u32, _LPTIM_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPTIM_CNT;
#[doc = "`read()` method returns [lptim_cnt::R](lptim_cnt::R) reader structure"]
impl crate::Readable for LPTIM_CNT {}
#[doc = "Counter Register"]
pub mod lptim_cnt;
#[doc = "LPTIM configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_cfgr2](lptim_cfgr2) module"]
pub type LPTIM_CFGR2 = crate::Reg<u32, _LPTIM_CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPTIM_CFGR2;
#[doc = "`read()` method returns [lptim_cfgr2::R](lptim_cfgr2::R) reader structure"]
impl crate::Readable for LPTIM_CFGR2 {}
#[doc = "`write(|w| ..)` method takes [lptim_cfgr2::W](lptim_cfgr2::W) writer structure"]
impl crate::Writable for LPTIM_CFGR2 {}
#[doc = "LPTIM configuration register 2"]
pub mod lptim_cfgr2;
