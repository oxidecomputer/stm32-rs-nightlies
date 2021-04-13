#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub wwdg_cr: WWDG_CR,
    #[doc = "0x04 - Configuration register"]
    pub wwdg_cfr: WWDG_CFR,
    #[doc = "0x08 - Status register"]
    pub wwdg_sr: WWDG_SR,
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wwdg_cr](wwdg_cr) module"]
pub type WWDG_CR = crate::Reg<u32, _WWDG_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WWDG_CR;
#[doc = "`read()` method returns [wwdg_cr::R](wwdg_cr::R) reader structure"]
impl crate::Readable for WWDG_CR {}
#[doc = "`write(|w| ..)` method takes [wwdg_cr::W](wwdg_cr::W) writer structure"]
impl crate::Writable for WWDG_CR {}
#[doc = "Control register"]
pub mod wwdg_cr;
#[doc = "Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wwdg_cfr](wwdg_cfr) module"]
pub type WWDG_CFR = crate::Reg<u32, _WWDG_CFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WWDG_CFR;
#[doc = "`read()` method returns [wwdg_cfr::R](wwdg_cfr::R) reader structure"]
impl crate::Readable for WWDG_CFR {}
#[doc = "`write(|w| ..)` method takes [wwdg_cfr::W](wwdg_cfr::W) writer structure"]
impl crate::Writable for WWDG_CFR {}
#[doc = "Configuration register"]
pub mod wwdg_cfr;
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wwdg_sr](wwdg_sr) module"]
pub type WWDG_SR = crate::Reg<u32, _WWDG_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WWDG_SR;
#[doc = "`read()` method returns [wwdg_sr::R](wwdg_sr::R) reader structure"]
impl crate::Readable for WWDG_SR {}
#[doc = "`write(|w| ..)` method takes [wwdg_sr::W](wwdg_sr::W) writer structure"]
impl crate::Writable for WWDG_SR {}
#[doc = "Status register"]
pub mod wwdg_sr;
