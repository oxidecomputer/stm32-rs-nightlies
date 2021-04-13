#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCU Device ID Code Register"]
    pub idcode: IDCODE,
    #[doc = "0x04 - DBG configuration register"]
    pub dbg_cr: DBG_CR,
    #[doc = "0x08 - DBG APB freeze register 1"]
    pub dbg_apb_fz1: DBG_APB_FZ1,
    #[doc = "0x0c - DBG APB freeze register 2"]
    pub dbg_apb_fz2: DBG_APB_FZ2,
}
#[doc = "MCU Device ID Code Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idcode](idcode) module"]
pub type IDCODE = crate::Reg<u32, _IDCODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDCODE;
#[doc = "`read()` method returns [idcode::R](idcode::R) reader structure"]
impl crate::Readable for IDCODE {}
#[doc = "MCU Device ID Code Register"]
pub mod idcode;
#[doc = "DBG configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_cr](dbg_cr) module"]
pub type DBG_CR = crate::Reg<u32, _DBG_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBG_CR;
#[doc = "`read()` method returns [dbg_cr::R](dbg_cr::R) reader structure"]
impl crate::Readable for DBG_CR {}
#[doc = "`write(|w| ..)` method takes [dbg_cr::W](dbg_cr::W) writer structure"]
impl crate::Writable for DBG_CR {}
#[doc = "DBG configuration register"]
pub mod dbg_cr;
#[doc = "DBG APB freeze register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_apb_fz1](dbg_apb_fz1) module"]
pub type DBG_APB_FZ1 = crate::Reg<u32, _DBG_APB_FZ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBG_APB_FZ1;
#[doc = "`read()` method returns [dbg_apb_fz1::R](dbg_apb_fz1::R) reader structure"]
impl crate::Readable for DBG_APB_FZ1 {}
#[doc = "`write(|w| ..)` method takes [dbg_apb_fz1::W](dbg_apb_fz1::W) writer structure"]
impl crate::Writable for DBG_APB_FZ1 {}
#[doc = "DBG APB freeze register 1"]
pub mod dbg_apb_fz1;
#[doc = "DBG APB freeze register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_apb_fz2](dbg_apb_fz2) module"]
pub type DBG_APB_FZ2 = crate::Reg<u32, _DBG_APB_FZ2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBG_APB_FZ2;
#[doc = "`read()` method returns [dbg_apb_fz2::R](dbg_apb_fz2::R) reader structure"]
impl crate::Readable for DBG_APB_FZ2 {}
#[doc = "`write(|w| ..)` method takes [dbg_apb_fz2::W](dbg_apb_fz2::W) writer structure"]
impl crate::Writable for DBG_APB_FZ2 {}
#[doc = "DBG APB freeze register 2"]
pub mod dbg_apb_fz2;
