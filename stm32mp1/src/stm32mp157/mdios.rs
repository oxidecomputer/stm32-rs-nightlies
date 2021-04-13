#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MDIOS configuration register"]
    pub mdios_cr: MDIOS_CR,
    #[doc = "0x04 - MDIOS write flag register"]
    pub mdios_wrfr: MDIOS_WRFR,
    #[doc = "0x08 - MDIOS clear write flag register"]
    pub mdios_cwrfr: MDIOS_CWRFR,
    #[doc = "0x0c - MDIOS read flag register"]
    pub mdios_rdfr: MDIOS_RDFR,
    #[doc = "0x10 - MDIOS clear read flag register"]
    pub mdios_crdfr: MDIOS_CRDFR,
    #[doc = "0x14 - MDIOS status register"]
    pub mdios_sr: MDIOS_SR,
    #[doc = "0x18 - MDIOS clear flag register"]
    pub mdios_clrfr: MDIOS_CLRFR,
    _reserved7: [u8; 228usize],
    #[doc = "0x100 - MDIOS input data register"]
    pub mdios_dinr0: MDIOS_DINR0,
    #[doc = "0x104 - MDIOS input data register"]
    pub mdios_dinr1: MDIOS_DINR1,
    #[doc = "0x108 - MDIOS input data register"]
    pub mdios_dinr2: MDIOS_DINR2,
    #[doc = "0x10c - MDIOS input data register"]
    pub mdios_dinr3: MDIOS_DINR3,
    #[doc = "0x110 - MDIOS input data register"]
    pub mdios_dinr4: MDIOS_DINR4,
    #[doc = "0x114 - MDIOS input data register"]
    pub mdios_dinr5: MDIOS_DINR5,
    #[doc = "0x118 - MDIOS input data register"]
    pub mdios_dinr6: MDIOS_DINR6,
    #[doc = "0x11c - MDIOS input data register"]
    pub mdios_dinr7: MDIOS_DINR7,
    #[doc = "0x120 - MDIOS input data register"]
    pub mdios_dinr8: MDIOS_DINR8,
    #[doc = "0x124 - MDIOS input data register"]
    pub mdios_dinr9: MDIOS_DINR9,
    #[doc = "0x128 - MDIOS input data register"]
    pub mdios_dinr10: MDIOS_DINR10,
    #[doc = "0x12c - MDIOS input data register"]
    pub mdios_dinr11: MDIOS_DINR11,
    #[doc = "0x130 - MDIOS input data register"]
    pub mdios_dinr12: MDIOS_DINR12,
    #[doc = "0x134 - MDIOS input data register"]
    pub mdios_dinr13: MDIOS_DINR13,
    #[doc = "0x138 - MDIOS input data register"]
    pub mdios_dinr14: MDIOS_DINR14,
    #[doc = "0x13c - MDIOS input data register"]
    pub mdios_dinr15: MDIOS_DINR15,
    #[doc = "0x140 - MDIOS input data register"]
    pub mdios_dinr16: MDIOS_DINR16,
    #[doc = "0x144 - MDIOS input data register"]
    pub mdios_dinr17: MDIOS_DINR17,
    #[doc = "0x148 - MDIOS input data register"]
    pub mdios_dinr18: MDIOS_DINR18,
    #[doc = "0x14c - MDIOS input data register"]
    pub mdios_dinr19: MDIOS_DINR19,
    #[doc = "0x150 - MDIOS input data register"]
    pub mdios_dinr20: MDIOS_DINR20,
    #[doc = "0x154 - MDIOS input data register"]
    pub mdios_dinr21: MDIOS_DINR21,
    #[doc = "0x158 - MDIOS input data register"]
    pub mdios_dinr22: MDIOS_DINR22,
    #[doc = "0x15c - MDIOS input data register"]
    pub mdios_dinr23: MDIOS_DINR23,
    #[doc = "0x160 - MDIOS input data register"]
    pub mdios_dinr24: MDIOS_DINR24,
    #[doc = "0x164 - MDIOS input data register"]
    pub mdios_dinr25: MDIOS_DINR25,
    #[doc = "0x168 - MDIOS input data register"]
    pub mdios_dinr26: MDIOS_DINR26,
    #[doc = "0x16c - MDIOS input data register"]
    pub mdios_dinr27: MDIOS_DINR27,
    #[doc = "0x170 - MDIOS input data register"]
    pub mdios_dinr28: MDIOS_DINR28,
    #[doc = "0x174 - MDIOS input data register"]
    pub mdios_dinr29: MDIOS_DINR29,
    #[doc = "0x178 - MDIOS input data register"]
    pub mdios_dinr30: MDIOS_DINR30,
    #[doc = "0x17c - MDIOS input data register"]
    pub mdios_dinr31: MDIOS_DINR31,
    #[doc = "0x180 - MDIOS input data register"]
    pub mdios_doutr0: MDIOS_DOUTR0,
    #[doc = "0x184 - MDIOS input data register"]
    pub mdios_doutr1: MDIOS_DOUTR1,
    #[doc = "0x188 - MDIOS output data register"]
    pub mdios_doutr2: MDIOS_DOUTR2,
    #[doc = "0x18c - MDIOS output data register"]
    pub mdios_doutr3: MDIOS_DOUTR3,
    #[doc = "0x190 - MDIOS output data register"]
    pub mdios_doutr4: MDIOS_DOUTR4,
    #[doc = "0x194 - MDIOS output data register"]
    pub mdios_doutr5: MDIOS_DOUTR5,
    #[doc = "0x198 - MDIOS output data register"]
    pub mdios_doutr6: MDIOS_DOUTR6,
    #[doc = "0x19c - MDIOS output data register"]
    pub mdios_doutr7: MDIOS_DOUTR7,
    #[doc = "0x1a0 - MDIOS output data register"]
    pub mdios_doutr8: MDIOS_DOUTR8,
    #[doc = "0x1a4 - MDIOS output data register"]
    pub mdios_doutr9: MDIOS_DOUTR9,
    #[doc = "0x1a8 - MDIOS output data register"]
    pub mdios_doutr10: MDIOS_DOUTR10,
    #[doc = "0x1ac - MDIOS output data register"]
    pub mdios_doutr11: MDIOS_DOUTR11,
    #[doc = "0x1b0 - MDIOS output data register"]
    pub mdios_doutr12: MDIOS_DOUTR12,
    #[doc = "0x1b4 - MDIOS output data register"]
    pub mdios_doutr13: MDIOS_DOUTR13,
    #[doc = "0x1b8 - MDIOS output data register"]
    pub mdios_doutr14: MDIOS_DOUTR14,
    #[doc = "0x1bc - MDIOS output data register"]
    pub mdios_doutr15: MDIOS_DOUTR15,
    #[doc = "0x1c0 - MDIOS output data register"]
    pub mdios_doutr16: MDIOS_DOUTR16,
    #[doc = "0x1c4 - MDIOS output data register"]
    pub mdios_doutr17: MDIOS_DOUTR17,
    #[doc = "0x1c8 - MDIOS output data register"]
    pub mdios_doutr18: MDIOS_DOUTR18,
    #[doc = "0x1cc - MDIOS output data register"]
    pub mdios_doutr19: MDIOS_DOUTR19,
    #[doc = "0x1d0 - MDIOS output data register"]
    pub mdios_doutr20: MDIOS_DOUTR20,
    #[doc = "0x1d4 - MDIOS output data register"]
    pub mdios_doutr21: MDIOS_DOUTR21,
    #[doc = "0x1d8 - MDIOS output data register"]
    pub mdios_doutr22: MDIOS_DOUTR22,
    #[doc = "0x1dc - MDIOS output data register"]
    pub mdios_doutr23: MDIOS_DOUTR23,
    #[doc = "0x1e0 - MDIOS output data register"]
    pub mdios_doutr24: MDIOS_DOUTR24,
    #[doc = "0x1e4 - MDIOS output data register"]
    pub mdios_doutr25: MDIOS_DOUTR25,
    #[doc = "0x1e8 - MDIOS output data register"]
    pub mdios_doutr26: MDIOS_DOUTR26,
    #[doc = "0x1ec - MDIOS output data register"]
    pub mdios_doutr27: MDIOS_DOUTR27,
    #[doc = "0x1f0 - MDIOS output data register"]
    pub mdios_doutr28: MDIOS_DOUTR28,
    #[doc = "0x1f4 - MDIOS output data register"]
    pub mdios_doutr29: MDIOS_DOUTR29,
    #[doc = "0x1f8 - MDIOS output data register"]
    pub mdios_doutr30: MDIOS_DOUTR30,
    #[doc = "0x1fc - MDIOS output data register"]
    pub mdios_doutr31: MDIOS_DOUTR31,
    _reserved71: [u8; 496usize],
    #[doc = "0x3f0 - MDIOS HW configuration register"]
    pub mdios_hwcfgr: MDIOS_HWCFGR,
    #[doc = "0x3f4 - MDIOS version register"]
    pub mdios_verr: MDIOS_VERR,
    #[doc = "0x3f8 - MDIOS identification register"]
    pub mdios_ipidr: MDIOS_IPIDR,
    #[doc = "0x3fc - MDIOS size identification register"]
    pub mdios_sidr: MDIOS_SIDR,
}
#[doc = "MDIOS configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_cr](mdios_cr) module"]
pub type MDIOS_CR = crate::Reg<u32, _MDIOS_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_CR;
#[doc = "`read()` method returns [mdios_cr::R](mdios_cr::R) reader structure"]
impl crate::Readable for MDIOS_CR {}
#[doc = "`write(|w| ..)` method takes [mdios_cr::W](mdios_cr::W) writer structure"]
impl crate::Writable for MDIOS_CR {}
#[doc = "MDIOS configuration register"]
pub mod mdios_cr;
#[doc = "MDIOS write flag register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_wrfr](mdios_wrfr) module"]
pub type MDIOS_WRFR = crate::Reg<u32, _MDIOS_WRFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_WRFR;
#[doc = "`read()` method returns [mdios_wrfr::R](mdios_wrfr::R) reader structure"]
impl crate::Readable for MDIOS_WRFR {}
#[doc = "MDIOS write flag register"]
pub mod mdios_wrfr;
#[doc = "MDIOS clear write flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_cwrfr](mdios_cwrfr) module"]
pub type MDIOS_CWRFR = crate::Reg<u32, _MDIOS_CWRFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_CWRFR;
#[doc = "`read()` method returns [mdios_cwrfr::R](mdios_cwrfr::R) reader structure"]
impl crate::Readable for MDIOS_CWRFR {}
#[doc = "`write(|w| ..)` method takes [mdios_cwrfr::W](mdios_cwrfr::W) writer structure"]
impl crate::Writable for MDIOS_CWRFR {}
#[doc = "MDIOS clear write flag register"]
pub mod mdios_cwrfr;
#[doc = "MDIOS read flag register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_rdfr](mdios_rdfr) module"]
pub type MDIOS_RDFR = crate::Reg<u32, _MDIOS_RDFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_RDFR;
#[doc = "`read()` method returns [mdios_rdfr::R](mdios_rdfr::R) reader structure"]
impl crate::Readable for MDIOS_RDFR {}
#[doc = "MDIOS read flag register"]
pub mod mdios_rdfr;
#[doc = "MDIOS clear read flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_crdfr](mdios_crdfr) module"]
pub type MDIOS_CRDFR = crate::Reg<u32, _MDIOS_CRDFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_CRDFR;
#[doc = "`read()` method returns [mdios_crdfr::R](mdios_crdfr::R) reader structure"]
impl crate::Readable for MDIOS_CRDFR {}
#[doc = "`write(|w| ..)` method takes [mdios_crdfr::W](mdios_crdfr::W) writer structure"]
impl crate::Writable for MDIOS_CRDFR {}
#[doc = "MDIOS clear read flag register"]
pub mod mdios_crdfr;
#[doc = "MDIOS status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_sr](mdios_sr) module"]
pub type MDIOS_SR = crate::Reg<u32, _MDIOS_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_SR;
#[doc = "`read()` method returns [mdios_sr::R](mdios_sr::R) reader structure"]
impl crate::Readable for MDIOS_SR {}
#[doc = "MDIOS status register"]
pub mod mdios_sr;
#[doc = "MDIOS clear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_clrfr](mdios_clrfr) module"]
pub type MDIOS_CLRFR = crate::Reg<u32, _MDIOS_CLRFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_CLRFR;
#[doc = "`read()` method returns [mdios_clrfr::R](mdios_clrfr::R) reader structure"]
impl crate::Readable for MDIOS_CLRFR {}
#[doc = "`write(|w| ..)` method takes [mdios_clrfr::W](mdios_clrfr::W) writer structure"]
impl crate::Writable for MDIOS_CLRFR {}
#[doc = "MDIOS clear flag register"]
pub mod mdios_clrfr;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr0](mdios_dinr0) module"]
pub type MDIOS_DINR0 = crate::Reg<u32, _MDIOS_DINR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DINR0;
#[doc = "`read()` method returns [mdios_dinr0::R](mdios_dinr0::R) reader structure"]
impl crate::Readable for MDIOS_DINR0 {}
#[doc = "MDIOS input data register"]
pub mod mdios_dinr0;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr1](mdios_dinr1) module"]
pub type MDIOS_DINR1 = crate::Reg<u32, _MDIOS_DINR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DINR1;
#[doc = "`read()` method returns [mdios_dinr1::R](mdios_dinr1::R) reader structure"]
impl crate::Readable for MDIOS_DINR1 {}
#[doc = "MDIOS input data register"]
pub mod mdios_dinr1;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr2](mdios_dinr2) module"]
pub type MDIOS_DINR2 = crate::Reg<u32, _MDIOS_DINR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DINR2;
#[doc = "`read()` method returns [mdios_dinr2::R](mdios_dinr2::R) reader structure"]
impl crate::Readable for MDIOS_DINR2 {}
#[doc = "MDIOS input data register"]
pub mod mdios_dinr2;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr3](mdios_dinr3) module"]
pub type MDIOS_DINR3 = crate::Reg<u32, _MDIOS_DINR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DINR3;
#[doc = "`read()` method returns [mdios_dinr3::R](mdios_dinr3::R) reader structure"]
impl crate::Readable for MDIOS_DINR3 {}
#[doc = "MDIOS input data register"]
pub mod mdios_dinr3;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr4](mdios_dinr4) module"]
pub type MDIOS_DINR4 = crate::Reg<u32, _MDIOS_DINR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DINR4;
#[doc = "`read()` method returns [mdios_dinr4::R](mdios_dinr4::R) reader structure"]
impl crate::Readable for MDIOS_DINR4 {}
#[doc = "MDIOS input data register"]
pub mod mdios_dinr4;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr5](mdios_dinr5) module"]
pub type MDIOS_DINR5 = crate::Reg<u32, _MDIOS_DINR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DINR5;
#[doc = "`read()` method returns [mdios_dinr5::R](mdios_dinr5::R) reader structure"]
impl crate::Readable for MDIOS_DINR5 {}
#[doc = "MDIOS input data register"]
pub mod mdios_dinr5;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr6](mdios_dinr6) module"]
pub type MDIOS_DINR6 = crate::Reg<u32, _MDIOS_DINR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DINR6;
#[doc = "`read()` method returns [mdios_dinr6::R](mdios_dinr6::R) reader structure"]
impl crate::Readable for MDIOS_DINR6 {}
#[doc = "MDIOS input data register"]
pub mod mdios_dinr6;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr7](mdios_dinr7) module"]
pub type MDIOS_DINR7 = crate::Reg<u32, _MDIOS_DINR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DINR7;
#[doc = "`read()` method returns [mdios_dinr7::R](mdios_dinr7::R) reader structure"]
impl crate::Readable for MDIOS_DINR7 {}
#[doc = "MDIOS input data register"]
pub mod mdios_dinr7;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr8](mdios_dinr8) module"]
pub type MDIOS_DINR8 = crate::Reg<u32, _MDIOS_DINR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DINR8;
#[doc = "`read()` method returns [mdios_dinr8::R](mdios_dinr8::R) reader structure"]
impl crate::Readable for MDIOS_DINR8 {}
#[doc = "MDIOS input data register"]
pub mod mdios_dinr8;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr9](mdios_dinr9) module"]
pub type MDIOS_DINR9 = crate::Reg<u32, _MDIOS_DINR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DINR9;
#[doc = "`read()` method returns [mdios_dinr9::R](mdios_dinr9::R) reader structure"]
impl crate::Readable for MDIOS_DINR9 {}
#[doc = "MDIOS input data register"]
pub mod mdios_dinr9;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr10](mdios_dinr10) module"]
pub type MDIOS_DINR10 = crate::Reg<u32, _MDIOS_DINR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DINR10;
#[doc = "`read()` method returns [mdios_dinr10::R](mdios_dinr10::R) reader structure"]
impl crate::Readable for MDIOS_DINR10 {}
#[doc = "MDIOS input data register"]
pub mod mdios_dinr10;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr11](mdios_dinr11) module"]
pub type MDIOS_DINR11 = crate::Reg<u32, _MDIOS_DINR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DINR11;
#[doc = "`read()` method returns [mdios_dinr11::R](mdios_dinr11::R) reader structure"]
impl crate::Readable for MDIOS_DINR11 {}
#[doc = "MDIOS input data register"]
pub mod mdios_dinr11;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr12](mdios_dinr12) module"]
pub type MDIOS_DINR12 = crate::Reg<u32, _MDIOS_DINR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DINR12;
#[doc = "`read()` method returns [mdios_dinr12::R](mdios_dinr12::R) reader structure"]
impl crate::Readable for MDIOS_DINR12 {}
#[doc = "MDIOS input data register"]
pub mod mdios_dinr12;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr13](mdios_dinr13) module"]
pub type MDIOS_DINR13 = crate::Reg<u32, _MDIOS_DINR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DINR13;
#[doc = "`read()` method returns [mdios_dinr13::R](mdios_dinr13::R) reader structure"]
impl crate::Readable for MDIOS_DINR13 {}
#[doc = "MDIOS input data register"]
pub mod mdios_dinr13;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr14](mdios_dinr14) module"]
pub type MDIOS_DINR14 = crate::Reg<u32, _MDIOS_DINR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DINR14;
#[doc = "`read()` method returns [mdios_dinr14::R](mdios_dinr14::R) reader structure"]
impl crate::Readable for MDIOS_DINR14 {}
#[doc = "MDIOS input data register"]
pub mod mdios_dinr14;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr15](mdios_dinr15) module"]
pub type MDIOS_DINR15 = crate::Reg<u32, _MDIOS_DINR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DINR15;
#[doc = "`read()` method returns [mdios_dinr15::R](mdios_dinr15::R) reader structure"]
impl crate::Readable for MDIOS_DINR15 {}
#[doc = "MDIOS input data register"]
pub mod mdios_dinr15;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr16](mdios_dinr16) module"]
pub type MDIOS_DINR16 = crate::Reg<u32, _MDIOS_DINR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DINR16;
#[doc = "`read()` method returns [mdios_dinr16::R](mdios_dinr16::R) reader structure"]
impl crate::Readable for MDIOS_DINR16 {}
#[doc = "MDIOS input data register"]
pub mod mdios_dinr16;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr17](mdios_dinr17) module"]
pub type MDIOS_DINR17 = crate::Reg<u32, _MDIOS_DINR17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DINR17;
#[doc = "`read()` method returns [mdios_dinr17::R](mdios_dinr17::R) reader structure"]
impl crate::Readable for MDIOS_DINR17 {}
#[doc = "MDIOS input data register"]
pub mod mdios_dinr17;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr18](mdios_dinr18) module"]
pub type MDIOS_DINR18 = crate::Reg<u32, _MDIOS_DINR18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DINR18;
#[doc = "`read()` method returns [mdios_dinr18::R](mdios_dinr18::R) reader structure"]
impl crate::Readable for MDIOS_DINR18 {}
#[doc = "MDIOS input data register"]
pub mod mdios_dinr18;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr19](mdios_dinr19) module"]
pub type MDIOS_DINR19 = crate::Reg<u32, _MDIOS_DINR19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DINR19;
#[doc = "`read()` method returns [mdios_dinr19::R](mdios_dinr19::R) reader structure"]
impl crate::Readable for MDIOS_DINR19 {}
#[doc = "MDIOS input data register"]
pub mod mdios_dinr19;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr20](mdios_dinr20) module"]
pub type MDIOS_DINR20 = crate::Reg<u32, _MDIOS_DINR20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DINR20;
#[doc = "`read()` method returns [mdios_dinr20::R](mdios_dinr20::R) reader structure"]
impl crate::Readable for MDIOS_DINR20 {}
#[doc = "MDIOS input data register"]
pub mod mdios_dinr20;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr21](mdios_dinr21) module"]
pub type MDIOS_DINR21 = crate::Reg<u32, _MDIOS_DINR21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DINR21;
#[doc = "`read()` method returns [mdios_dinr21::R](mdios_dinr21::R) reader structure"]
impl crate::Readable for MDIOS_DINR21 {}
#[doc = "MDIOS input data register"]
pub mod mdios_dinr21;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr22](mdios_dinr22) module"]
pub type MDIOS_DINR22 = crate::Reg<u32, _MDIOS_DINR22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DINR22;
#[doc = "`read()` method returns [mdios_dinr22::R](mdios_dinr22::R) reader structure"]
impl crate::Readable for MDIOS_DINR22 {}
#[doc = "MDIOS input data register"]
pub mod mdios_dinr22;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr23](mdios_dinr23) module"]
pub type MDIOS_DINR23 = crate::Reg<u32, _MDIOS_DINR23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DINR23;
#[doc = "`read()` method returns [mdios_dinr23::R](mdios_dinr23::R) reader structure"]
impl crate::Readable for MDIOS_DINR23 {}
#[doc = "MDIOS input data register"]
pub mod mdios_dinr23;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr24](mdios_dinr24) module"]
pub type MDIOS_DINR24 = crate::Reg<u32, _MDIOS_DINR24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DINR24;
#[doc = "`read()` method returns [mdios_dinr24::R](mdios_dinr24::R) reader structure"]
impl crate::Readable for MDIOS_DINR24 {}
#[doc = "MDIOS input data register"]
pub mod mdios_dinr24;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr25](mdios_dinr25) module"]
pub type MDIOS_DINR25 = crate::Reg<u32, _MDIOS_DINR25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DINR25;
#[doc = "`read()` method returns [mdios_dinr25::R](mdios_dinr25::R) reader structure"]
impl crate::Readable for MDIOS_DINR25 {}
#[doc = "MDIOS input data register"]
pub mod mdios_dinr25;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr26](mdios_dinr26) module"]
pub type MDIOS_DINR26 = crate::Reg<u32, _MDIOS_DINR26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DINR26;
#[doc = "`read()` method returns [mdios_dinr26::R](mdios_dinr26::R) reader structure"]
impl crate::Readable for MDIOS_DINR26 {}
#[doc = "MDIOS input data register"]
pub mod mdios_dinr26;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr27](mdios_dinr27) module"]
pub type MDIOS_DINR27 = crate::Reg<u32, _MDIOS_DINR27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DINR27;
#[doc = "`read()` method returns [mdios_dinr27::R](mdios_dinr27::R) reader structure"]
impl crate::Readable for MDIOS_DINR27 {}
#[doc = "MDIOS input data register"]
pub mod mdios_dinr27;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr28](mdios_dinr28) module"]
pub type MDIOS_DINR28 = crate::Reg<u32, _MDIOS_DINR28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DINR28;
#[doc = "`read()` method returns [mdios_dinr28::R](mdios_dinr28::R) reader structure"]
impl crate::Readable for MDIOS_DINR28 {}
#[doc = "MDIOS input data register"]
pub mod mdios_dinr28;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr29](mdios_dinr29) module"]
pub type MDIOS_DINR29 = crate::Reg<u32, _MDIOS_DINR29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DINR29;
#[doc = "`read()` method returns [mdios_dinr29::R](mdios_dinr29::R) reader structure"]
impl crate::Readable for MDIOS_DINR29 {}
#[doc = "MDIOS input data register"]
pub mod mdios_dinr29;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr30](mdios_dinr30) module"]
pub type MDIOS_DINR30 = crate::Reg<u32, _MDIOS_DINR30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DINR30;
#[doc = "`read()` method returns [mdios_dinr30::R](mdios_dinr30::R) reader structure"]
impl crate::Readable for MDIOS_DINR30 {}
#[doc = "MDIOS input data register"]
pub mod mdios_dinr30;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr31](mdios_dinr31) module"]
pub type MDIOS_DINR31 = crate::Reg<u32, _MDIOS_DINR31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DINR31;
#[doc = "`read()` method returns [mdios_dinr31::R](mdios_dinr31::R) reader structure"]
impl crate::Readable for MDIOS_DINR31 {}
#[doc = "MDIOS input data register"]
pub mod mdios_dinr31;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr0](mdios_doutr0) module"]
pub type MDIOS_DOUTR0 = crate::Reg<u32, _MDIOS_DOUTR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DOUTR0;
#[doc = "`read()` method returns [mdios_doutr0::R](mdios_doutr0::R) reader structure"]
impl crate::Readable for MDIOS_DOUTR0 {}
#[doc = "MDIOS input data register"]
pub mod mdios_doutr0;
#[doc = "MDIOS input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr1](mdios_doutr1) module"]
pub type MDIOS_DOUTR1 = crate::Reg<u32, _MDIOS_DOUTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DOUTR1;
#[doc = "`read()` method returns [mdios_doutr1::R](mdios_doutr1::R) reader structure"]
impl crate::Readable for MDIOS_DOUTR1 {}
#[doc = "MDIOS input data register"]
pub mod mdios_doutr1;
#[doc = "MDIOS output data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr2](mdios_doutr2) module"]
pub type MDIOS_DOUTR2 = crate::Reg<u32, _MDIOS_DOUTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DOUTR2;
#[doc = "`read()` method returns [mdios_doutr2::R](mdios_doutr2::R) reader structure"]
impl crate::Readable for MDIOS_DOUTR2 {}
#[doc = "MDIOS output data register"]
pub mod mdios_doutr2;
#[doc = "MDIOS output data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr3](mdios_doutr3) module"]
pub type MDIOS_DOUTR3 = crate::Reg<u32, _MDIOS_DOUTR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DOUTR3;
#[doc = "`read()` method returns [mdios_doutr3::R](mdios_doutr3::R) reader structure"]
impl crate::Readable for MDIOS_DOUTR3 {}
#[doc = "MDIOS output data register"]
pub mod mdios_doutr3;
#[doc = "MDIOS output data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr4](mdios_doutr4) module"]
pub type MDIOS_DOUTR4 = crate::Reg<u32, _MDIOS_DOUTR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DOUTR4;
#[doc = "`read()` method returns [mdios_doutr4::R](mdios_doutr4::R) reader structure"]
impl crate::Readable for MDIOS_DOUTR4 {}
#[doc = "MDIOS output data register"]
pub mod mdios_doutr4;
#[doc = "MDIOS output data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr5](mdios_doutr5) module"]
pub type MDIOS_DOUTR5 = crate::Reg<u32, _MDIOS_DOUTR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DOUTR5;
#[doc = "`read()` method returns [mdios_doutr5::R](mdios_doutr5::R) reader structure"]
impl crate::Readable for MDIOS_DOUTR5 {}
#[doc = "MDIOS output data register"]
pub mod mdios_doutr5;
#[doc = "MDIOS output data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr6](mdios_doutr6) module"]
pub type MDIOS_DOUTR6 = crate::Reg<u32, _MDIOS_DOUTR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DOUTR6;
#[doc = "`read()` method returns [mdios_doutr6::R](mdios_doutr6::R) reader structure"]
impl crate::Readable for MDIOS_DOUTR6 {}
#[doc = "MDIOS output data register"]
pub mod mdios_doutr6;
#[doc = "MDIOS output data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr7](mdios_doutr7) module"]
pub type MDIOS_DOUTR7 = crate::Reg<u32, _MDIOS_DOUTR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DOUTR7;
#[doc = "`read()` method returns [mdios_doutr7::R](mdios_doutr7::R) reader structure"]
impl crate::Readable for MDIOS_DOUTR7 {}
#[doc = "MDIOS output data register"]
pub mod mdios_doutr7;
#[doc = "MDIOS output data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr8](mdios_doutr8) module"]
pub type MDIOS_DOUTR8 = crate::Reg<u32, _MDIOS_DOUTR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DOUTR8;
#[doc = "`read()` method returns [mdios_doutr8::R](mdios_doutr8::R) reader structure"]
impl crate::Readable for MDIOS_DOUTR8 {}
#[doc = "MDIOS output data register"]
pub mod mdios_doutr8;
#[doc = "MDIOS output data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr9](mdios_doutr9) module"]
pub type MDIOS_DOUTR9 = crate::Reg<u32, _MDIOS_DOUTR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DOUTR9;
#[doc = "`read()` method returns [mdios_doutr9::R](mdios_doutr9::R) reader structure"]
impl crate::Readable for MDIOS_DOUTR9 {}
#[doc = "MDIOS output data register"]
pub mod mdios_doutr9;
#[doc = "MDIOS output data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr10](mdios_doutr10) module"]
pub type MDIOS_DOUTR10 = crate::Reg<u32, _MDIOS_DOUTR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DOUTR10;
#[doc = "`read()` method returns [mdios_doutr10::R](mdios_doutr10::R) reader structure"]
impl crate::Readable for MDIOS_DOUTR10 {}
#[doc = "MDIOS output data register"]
pub mod mdios_doutr10;
#[doc = "MDIOS output data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr11](mdios_doutr11) module"]
pub type MDIOS_DOUTR11 = crate::Reg<u32, _MDIOS_DOUTR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DOUTR11;
#[doc = "`read()` method returns [mdios_doutr11::R](mdios_doutr11::R) reader structure"]
impl crate::Readable for MDIOS_DOUTR11 {}
#[doc = "MDIOS output data register"]
pub mod mdios_doutr11;
#[doc = "MDIOS output data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr12](mdios_doutr12) module"]
pub type MDIOS_DOUTR12 = crate::Reg<u32, _MDIOS_DOUTR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DOUTR12;
#[doc = "`read()` method returns [mdios_doutr12::R](mdios_doutr12::R) reader structure"]
impl crate::Readable for MDIOS_DOUTR12 {}
#[doc = "MDIOS output data register"]
pub mod mdios_doutr12;
#[doc = "MDIOS output data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr13](mdios_doutr13) module"]
pub type MDIOS_DOUTR13 = crate::Reg<u32, _MDIOS_DOUTR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DOUTR13;
#[doc = "`read()` method returns [mdios_doutr13::R](mdios_doutr13::R) reader structure"]
impl crate::Readable for MDIOS_DOUTR13 {}
#[doc = "MDIOS output data register"]
pub mod mdios_doutr13;
#[doc = "MDIOS output data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr14](mdios_doutr14) module"]
pub type MDIOS_DOUTR14 = crate::Reg<u32, _MDIOS_DOUTR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DOUTR14;
#[doc = "`read()` method returns [mdios_doutr14::R](mdios_doutr14::R) reader structure"]
impl crate::Readable for MDIOS_DOUTR14 {}
#[doc = "MDIOS output data register"]
pub mod mdios_doutr14;
#[doc = "MDIOS output data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr15](mdios_doutr15) module"]
pub type MDIOS_DOUTR15 = crate::Reg<u32, _MDIOS_DOUTR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DOUTR15;
#[doc = "`read()` method returns [mdios_doutr15::R](mdios_doutr15::R) reader structure"]
impl crate::Readable for MDIOS_DOUTR15 {}
#[doc = "MDIOS output data register"]
pub mod mdios_doutr15;
#[doc = "MDIOS output data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr16](mdios_doutr16) module"]
pub type MDIOS_DOUTR16 = crate::Reg<u32, _MDIOS_DOUTR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DOUTR16;
#[doc = "`read()` method returns [mdios_doutr16::R](mdios_doutr16::R) reader structure"]
impl crate::Readable for MDIOS_DOUTR16 {}
#[doc = "MDIOS output data register"]
pub mod mdios_doutr16;
#[doc = "MDIOS output data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr17](mdios_doutr17) module"]
pub type MDIOS_DOUTR17 = crate::Reg<u32, _MDIOS_DOUTR17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DOUTR17;
#[doc = "`read()` method returns [mdios_doutr17::R](mdios_doutr17::R) reader structure"]
impl crate::Readable for MDIOS_DOUTR17 {}
#[doc = "MDIOS output data register"]
pub mod mdios_doutr17;
#[doc = "MDIOS output data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr18](mdios_doutr18) module"]
pub type MDIOS_DOUTR18 = crate::Reg<u32, _MDIOS_DOUTR18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DOUTR18;
#[doc = "`read()` method returns [mdios_doutr18::R](mdios_doutr18::R) reader structure"]
impl crate::Readable for MDIOS_DOUTR18 {}
#[doc = "MDIOS output data register"]
pub mod mdios_doutr18;
#[doc = "MDIOS output data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr19](mdios_doutr19) module"]
pub type MDIOS_DOUTR19 = crate::Reg<u32, _MDIOS_DOUTR19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DOUTR19;
#[doc = "`read()` method returns [mdios_doutr19::R](mdios_doutr19::R) reader structure"]
impl crate::Readable for MDIOS_DOUTR19 {}
#[doc = "MDIOS output data register"]
pub mod mdios_doutr19;
#[doc = "MDIOS output data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr20](mdios_doutr20) module"]
pub type MDIOS_DOUTR20 = crate::Reg<u32, _MDIOS_DOUTR20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DOUTR20;
#[doc = "`read()` method returns [mdios_doutr20::R](mdios_doutr20::R) reader structure"]
impl crate::Readable for MDIOS_DOUTR20 {}
#[doc = "MDIOS output data register"]
pub mod mdios_doutr20;
#[doc = "MDIOS output data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr21](mdios_doutr21) module"]
pub type MDIOS_DOUTR21 = crate::Reg<u32, _MDIOS_DOUTR21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DOUTR21;
#[doc = "`read()` method returns [mdios_doutr21::R](mdios_doutr21::R) reader structure"]
impl crate::Readable for MDIOS_DOUTR21 {}
#[doc = "MDIOS output data register"]
pub mod mdios_doutr21;
#[doc = "MDIOS output data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr22](mdios_doutr22) module"]
pub type MDIOS_DOUTR22 = crate::Reg<u32, _MDIOS_DOUTR22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DOUTR22;
#[doc = "`read()` method returns [mdios_doutr22::R](mdios_doutr22::R) reader structure"]
impl crate::Readable for MDIOS_DOUTR22 {}
#[doc = "MDIOS output data register"]
pub mod mdios_doutr22;
#[doc = "MDIOS output data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr23](mdios_doutr23) module"]
pub type MDIOS_DOUTR23 = crate::Reg<u32, _MDIOS_DOUTR23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DOUTR23;
#[doc = "`read()` method returns [mdios_doutr23::R](mdios_doutr23::R) reader structure"]
impl crate::Readable for MDIOS_DOUTR23 {}
#[doc = "MDIOS output data register"]
pub mod mdios_doutr23;
#[doc = "MDIOS output data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr24](mdios_doutr24) module"]
pub type MDIOS_DOUTR24 = crate::Reg<u32, _MDIOS_DOUTR24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DOUTR24;
#[doc = "`read()` method returns [mdios_doutr24::R](mdios_doutr24::R) reader structure"]
impl crate::Readable for MDIOS_DOUTR24 {}
#[doc = "MDIOS output data register"]
pub mod mdios_doutr24;
#[doc = "MDIOS output data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr25](mdios_doutr25) module"]
pub type MDIOS_DOUTR25 = crate::Reg<u32, _MDIOS_DOUTR25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DOUTR25;
#[doc = "`read()` method returns [mdios_doutr25::R](mdios_doutr25::R) reader structure"]
impl crate::Readable for MDIOS_DOUTR25 {}
#[doc = "MDIOS output data register"]
pub mod mdios_doutr25;
#[doc = "MDIOS output data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr26](mdios_doutr26) module"]
pub type MDIOS_DOUTR26 = crate::Reg<u32, _MDIOS_DOUTR26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DOUTR26;
#[doc = "`read()` method returns [mdios_doutr26::R](mdios_doutr26::R) reader structure"]
impl crate::Readable for MDIOS_DOUTR26 {}
#[doc = "MDIOS output data register"]
pub mod mdios_doutr26;
#[doc = "MDIOS output data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr27](mdios_doutr27) module"]
pub type MDIOS_DOUTR27 = crate::Reg<u32, _MDIOS_DOUTR27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DOUTR27;
#[doc = "`read()` method returns [mdios_doutr27::R](mdios_doutr27::R) reader structure"]
impl crate::Readable for MDIOS_DOUTR27 {}
#[doc = "MDIOS output data register"]
pub mod mdios_doutr27;
#[doc = "MDIOS output data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr28](mdios_doutr28) module"]
pub type MDIOS_DOUTR28 = crate::Reg<u32, _MDIOS_DOUTR28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DOUTR28;
#[doc = "`read()` method returns [mdios_doutr28::R](mdios_doutr28::R) reader structure"]
impl crate::Readable for MDIOS_DOUTR28 {}
#[doc = "MDIOS output data register"]
pub mod mdios_doutr28;
#[doc = "MDIOS output data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr29](mdios_doutr29) module"]
pub type MDIOS_DOUTR29 = crate::Reg<u32, _MDIOS_DOUTR29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DOUTR29;
#[doc = "`read()` method returns [mdios_doutr29::R](mdios_doutr29::R) reader structure"]
impl crate::Readable for MDIOS_DOUTR29 {}
#[doc = "MDIOS output data register"]
pub mod mdios_doutr29;
#[doc = "MDIOS output data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr30](mdios_doutr30) module"]
pub type MDIOS_DOUTR30 = crate::Reg<u32, _MDIOS_DOUTR30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DOUTR30;
#[doc = "`read()` method returns [mdios_doutr30::R](mdios_doutr30::R) reader structure"]
impl crate::Readable for MDIOS_DOUTR30 {}
#[doc = "MDIOS output data register"]
pub mod mdios_doutr30;
#[doc = "MDIOS output data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr31](mdios_doutr31) module"]
pub type MDIOS_DOUTR31 = crate::Reg<u32, _MDIOS_DOUTR31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_DOUTR31;
#[doc = "`read()` method returns [mdios_doutr31::R](mdios_doutr31::R) reader structure"]
impl crate::Readable for MDIOS_DOUTR31 {}
#[doc = "MDIOS output data register"]
pub mod mdios_doutr31;
#[doc = "MDIOS HW configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_hwcfgr](mdios_hwcfgr) module"]
pub type MDIOS_HWCFGR = crate::Reg<u32, _MDIOS_HWCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_HWCFGR;
#[doc = "`read()` method returns [mdios_hwcfgr::R](mdios_hwcfgr::R) reader structure"]
impl crate::Readable for MDIOS_HWCFGR {}
#[doc = "MDIOS HW configuration register"]
pub mod mdios_hwcfgr;
#[doc = "MDIOS version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_verr](mdios_verr) module"]
pub type MDIOS_VERR = crate::Reg<u32, _MDIOS_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_VERR;
#[doc = "`read()` method returns [mdios_verr::R](mdios_verr::R) reader structure"]
impl crate::Readable for MDIOS_VERR {}
#[doc = "MDIOS version register"]
pub mod mdios_verr;
#[doc = "MDIOS identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_ipidr](mdios_ipidr) module"]
pub type MDIOS_IPIDR = crate::Reg<u32, _MDIOS_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_IPIDR;
#[doc = "`read()` method returns [mdios_ipidr::R](mdios_ipidr::R) reader structure"]
impl crate::Readable for MDIOS_IPIDR {}
#[doc = "MDIOS identification register"]
pub mod mdios_ipidr;
#[doc = "MDIOS size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_sidr](mdios_sidr) module"]
pub type MDIOS_SIDR = crate::Reg<u32, _MDIOS_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIOS_SIDR;
#[doc = "`read()` method returns [mdios_sidr::R](mdios_sidr::R) reader structure"]
impl crate::Readable for MDIOS_SIDR {}
#[doc = "MDIOS size identification register"]
pub mod mdios_sidr;
