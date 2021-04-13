#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub rng_cr: RNG_CR,
    #[doc = "0x04 - status register"]
    pub rng_sr: RNG_SR,
    #[doc = "0x08 - data register"]
    pub rng_dr: RNG_DR,
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rng_cr](rng_cr) module"]
pub type RNG_CR = crate::Reg<u32, _RNG_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RNG_CR;
#[doc = "`read()` method returns [rng_cr::R](rng_cr::R) reader structure"]
impl crate::Readable for RNG_CR {}
#[doc = "`write(|w| ..)` method takes [rng_cr::W](rng_cr::W) writer structure"]
impl crate::Writable for RNG_CR {}
#[doc = "control register"]
pub mod rng_cr;
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rng_sr](rng_sr) module"]
pub type RNG_SR = crate::Reg<u32, _RNG_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RNG_SR;
#[doc = "`read()` method returns [rng_sr::R](rng_sr::R) reader structure"]
impl crate::Readable for RNG_SR {}
#[doc = "`write(|w| ..)` method takes [rng_sr::W](rng_sr::W) writer structure"]
impl crate::Writable for RNG_SR {}
#[doc = "status register"]
pub mod rng_sr;
#[doc = "data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rng_dr](rng_dr) module"]
pub type RNG_DR = crate::Reg<u32, _RNG_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RNG_DR;
#[doc = "`read()` method returns [rng_dr::R](rng_dr::R) reader structure"]
impl crate::Readable for RNG_DR {}
#[doc = "data register"]
pub mod rng_dr;
