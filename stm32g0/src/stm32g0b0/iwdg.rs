#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Key register"]
    pub iwdg_kr: IWDG_KR,
    #[doc = "0x04 - Prescaler register"]
    pub iwdg_pr: IWDG_PR,
    #[doc = "0x08 - Reload register"]
    pub iwdg_rlr: IWDG_RLR,
    #[doc = "0x0c - Status register"]
    pub iwdg_sr: IWDG_SR,
    #[doc = "0x10 - Window register"]
    pub iwdg_winr: IWDG_WINR,
}
#[doc = "Key register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iwdg_kr](iwdg_kr) module"]
pub type IWDG_KR = crate::Reg<u32, _IWDG_KR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IWDG_KR;
#[doc = "`write(|w| ..)` method takes [iwdg_kr::W](iwdg_kr::W) writer structure"]
impl crate::Writable for IWDG_KR {}
#[doc = "Key register"]
pub mod iwdg_kr;
#[doc = "Prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iwdg_pr](iwdg_pr) module"]
pub type IWDG_PR = crate::Reg<u32, _IWDG_PR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IWDG_PR;
#[doc = "`read()` method returns [iwdg_pr::R](iwdg_pr::R) reader structure"]
impl crate::Readable for IWDG_PR {}
#[doc = "`write(|w| ..)` method takes [iwdg_pr::W](iwdg_pr::W) writer structure"]
impl crate::Writable for IWDG_PR {}
#[doc = "Prescaler register"]
pub mod iwdg_pr;
#[doc = "Reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iwdg_rlr](iwdg_rlr) module"]
pub type IWDG_RLR = crate::Reg<u32, _IWDG_RLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IWDG_RLR;
#[doc = "`read()` method returns [iwdg_rlr::R](iwdg_rlr::R) reader structure"]
impl crate::Readable for IWDG_RLR {}
#[doc = "`write(|w| ..)` method takes [iwdg_rlr::W](iwdg_rlr::W) writer structure"]
impl crate::Writable for IWDG_RLR {}
#[doc = "Reload register"]
pub mod iwdg_rlr;
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iwdg_sr](iwdg_sr) module"]
pub type IWDG_SR = crate::Reg<u32, _IWDG_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IWDG_SR;
#[doc = "`read()` method returns [iwdg_sr::R](iwdg_sr::R) reader structure"]
impl crate::Readable for IWDG_SR {}
#[doc = "Status register"]
pub mod iwdg_sr;
#[doc = "Window register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iwdg_winr](iwdg_winr) module"]
pub type IWDG_WINR = crate::Reg<u32, _IWDG_WINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IWDG_WINR;
#[doc = "`read()` method returns [iwdg_winr::R](iwdg_winr::R) reader structure"]
impl crate::Readable for IWDG_WINR {}
#[doc = "`write(|w| ..)` method takes [iwdg_winr::W](iwdg_winr::W) writer structure"]
impl crate::Writable for IWDG_WINR {}
#[doc = "Window register"]
pub mod iwdg_winr;
