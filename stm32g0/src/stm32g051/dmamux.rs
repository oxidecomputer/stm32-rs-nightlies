#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMAMUX request line multiplexer channel x configuration register"]
    pub dmamux_c0cr: DMAMUX_C0CR,
    #[doc = "0x04 - DMAMUX request line multiplexer channel x configuration register"]
    pub dmamux_c1cr: DMAMUX_C1CR,
    #[doc = "0x08 - DMAMUX request line multiplexer channel x configuration register"]
    pub dmamux_c2cr: DMAMUX_C2CR,
    #[doc = "0x0c - DMAMUX request line multiplexer channel x configuration register"]
    pub dmamux_c3cr: DMAMUX_C3CR,
    #[doc = "0x10 - DMAMUX request line multiplexer channel x configuration register"]
    pub dmamux_c4cr: DMAMUX_C4CR,
    #[doc = "0x14 - DMAMUX request line multiplexer channel x configuration register"]
    pub dmamux_c5cr: DMAMUX_C5CR,
    #[doc = "0x18 - DMAMUX request line multiplexer channel x configuration register"]
    pub dmamux_c6cr: DMAMUX_C6CR,
    _reserved7: [u8; 100usize],
    #[doc = "0x80 - DMAMUX request line multiplexer interrupt channel status register"]
    pub dmamux_csr: DMAMUX_CSR,
    #[doc = "0x84 - DMAMUX request line multiplexer interrupt clear flag register"]
    pub dmamux_cfr: DMAMUX_CFR,
    _reserved9: [u8; 120usize],
    #[doc = "0x100 - DMAMUX request generator channel x configuration register"]
    pub dmamux_rg0cr: DMAMUX_RG0CR,
    #[doc = "0x104 - DMAMUX request generator channel x configuration register"]
    pub dmamux_rg1cr: DMAMUX_RG1CR,
    #[doc = "0x108 - DMAMUX request generator channel x configuration register"]
    pub dmamux_rg2cr: DMAMUX_RG2CR,
    #[doc = "0x10c - DMAMUX request generator channel x configuration register"]
    pub dmamux_rg3cr: DMAMUX_RG3CR,
    _reserved13: [u8; 48usize],
    #[doc = "0x140 - DMAMUX request generator interrupt status register"]
    pub dmamux_rgsr: DMAMUX_RGSR,
    #[doc = "0x144 - DMAMUX request generator interrupt clear flag register"]
    pub dmamux_rgcfr: DMAMUX_RGCFR,
}
#[doc = "DMAMUX request line multiplexer channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_c0cr](dmamux_c0cr) module"]
pub type DMAMUX_C0CR = crate::Reg<u32, _DMAMUX_C0CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_C0CR;
#[doc = "`read()` method returns [dmamux_c0cr::R](dmamux_c0cr::R) reader structure"]
impl crate::Readable for DMAMUX_C0CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_c0cr::W](dmamux_c0cr::W) writer structure"]
impl crate::Writable for DMAMUX_C0CR {}
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod dmamux_c0cr;
#[doc = "DMAMUX request line multiplexer channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_c1cr](dmamux_c1cr) module"]
pub type DMAMUX_C1CR = crate::Reg<u32, _DMAMUX_C1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_C1CR;
#[doc = "`read()` method returns [dmamux_c1cr::R](dmamux_c1cr::R) reader structure"]
impl crate::Readable for DMAMUX_C1CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_c1cr::W](dmamux_c1cr::W) writer structure"]
impl crate::Writable for DMAMUX_C1CR {}
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod dmamux_c1cr;
#[doc = "DMAMUX request line multiplexer channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_c2cr](dmamux_c2cr) module"]
pub type DMAMUX_C2CR = crate::Reg<u32, _DMAMUX_C2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_C2CR;
#[doc = "`read()` method returns [dmamux_c2cr::R](dmamux_c2cr::R) reader structure"]
impl crate::Readable for DMAMUX_C2CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_c2cr::W](dmamux_c2cr::W) writer structure"]
impl crate::Writable for DMAMUX_C2CR {}
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod dmamux_c2cr;
#[doc = "DMAMUX request line multiplexer channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_c3cr](dmamux_c3cr) module"]
pub type DMAMUX_C3CR = crate::Reg<u32, _DMAMUX_C3CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_C3CR;
#[doc = "`read()` method returns [dmamux_c3cr::R](dmamux_c3cr::R) reader structure"]
impl crate::Readable for DMAMUX_C3CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_c3cr::W](dmamux_c3cr::W) writer structure"]
impl crate::Writable for DMAMUX_C3CR {}
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod dmamux_c3cr;
#[doc = "DMAMUX request line multiplexer channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_c4cr](dmamux_c4cr) module"]
pub type DMAMUX_C4CR = crate::Reg<u32, _DMAMUX_C4CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_C4CR;
#[doc = "`read()` method returns [dmamux_c4cr::R](dmamux_c4cr::R) reader structure"]
impl crate::Readable for DMAMUX_C4CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_c4cr::W](dmamux_c4cr::W) writer structure"]
impl crate::Writable for DMAMUX_C4CR {}
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod dmamux_c4cr;
#[doc = "DMAMUX request line multiplexer channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_c5cr](dmamux_c5cr) module"]
pub type DMAMUX_C5CR = crate::Reg<u32, _DMAMUX_C5CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_C5CR;
#[doc = "`read()` method returns [dmamux_c5cr::R](dmamux_c5cr::R) reader structure"]
impl crate::Readable for DMAMUX_C5CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_c5cr::W](dmamux_c5cr::W) writer structure"]
impl crate::Writable for DMAMUX_C5CR {}
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod dmamux_c5cr;
#[doc = "DMAMUX request line multiplexer channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_c6cr](dmamux_c6cr) module"]
pub type DMAMUX_C6CR = crate::Reg<u32, _DMAMUX_C6CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_C6CR;
#[doc = "`read()` method returns [dmamux_c6cr::R](dmamux_c6cr::R) reader structure"]
impl crate::Readable for DMAMUX_C6CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_c6cr::W](dmamux_c6cr::W) writer structure"]
impl crate::Writable for DMAMUX_C6CR {}
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod dmamux_c6cr;
#[doc = "DMAMUX request line multiplexer interrupt channel status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_csr](dmamux_csr) module"]
pub type DMAMUX_CSR = crate::Reg<u32, _DMAMUX_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_CSR;
#[doc = "`read()` method returns [dmamux_csr::R](dmamux_csr::R) reader structure"]
impl crate::Readable for DMAMUX_CSR {}
#[doc = "DMAMUX request line multiplexer interrupt channel status register"]
pub mod dmamux_csr;
#[doc = "DMAMUX request line multiplexer interrupt clear flag register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_cfr](dmamux_cfr) module"]
pub type DMAMUX_CFR = crate::Reg<u32, _DMAMUX_CFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_CFR;
#[doc = "`write(|w| ..)` method takes [dmamux_cfr::W](dmamux_cfr::W) writer structure"]
impl crate::Writable for DMAMUX_CFR {}
#[doc = "DMAMUX request line multiplexer interrupt clear flag register"]
pub mod dmamux_cfr;
#[doc = "DMAMUX request generator channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_rg0cr](dmamux_rg0cr) module"]
pub type DMAMUX_RG0CR = crate::Reg<u32, _DMAMUX_RG0CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_RG0CR;
#[doc = "`read()` method returns [dmamux_rg0cr::R](dmamux_rg0cr::R) reader structure"]
impl crate::Readable for DMAMUX_RG0CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_rg0cr::W](dmamux_rg0cr::W) writer structure"]
impl crate::Writable for DMAMUX_RG0CR {}
#[doc = "DMAMUX request generator channel x configuration register"]
pub mod dmamux_rg0cr;
#[doc = "DMAMUX request generator channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_rg1cr](dmamux_rg1cr) module"]
pub type DMAMUX_RG1CR = crate::Reg<u32, _DMAMUX_RG1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_RG1CR;
#[doc = "`read()` method returns [dmamux_rg1cr::R](dmamux_rg1cr::R) reader structure"]
impl crate::Readable for DMAMUX_RG1CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_rg1cr::W](dmamux_rg1cr::W) writer structure"]
impl crate::Writable for DMAMUX_RG1CR {}
#[doc = "DMAMUX request generator channel x configuration register"]
pub mod dmamux_rg1cr;
#[doc = "DMAMUX request generator channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_rg2cr](dmamux_rg2cr) module"]
pub type DMAMUX_RG2CR = crate::Reg<u32, _DMAMUX_RG2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_RG2CR;
#[doc = "`read()` method returns [dmamux_rg2cr::R](dmamux_rg2cr::R) reader structure"]
impl crate::Readable for DMAMUX_RG2CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_rg2cr::W](dmamux_rg2cr::W) writer structure"]
impl crate::Writable for DMAMUX_RG2CR {}
#[doc = "DMAMUX request generator channel x configuration register"]
pub mod dmamux_rg2cr;
#[doc = "DMAMUX request generator channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_rg3cr](dmamux_rg3cr) module"]
pub type DMAMUX_RG3CR = crate::Reg<u32, _DMAMUX_RG3CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_RG3CR;
#[doc = "`read()` method returns [dmamux_rg3cr::R](dmamux_rg3cr::R) reader structure"]
impl crate::Readable for DMAMUX_RG3CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_rg3cr::W](dmamux_rg3cr::W) writer structure"]
impl crate::Writable for DMAMUX_RG3CR {}
#[doc = "DMAMUX request generator channel x configuration register"]
pub mod dmamux_rg3cr;
#[doc = "DMAMUX request generator interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_rgsr](dmamux_rgsr) module"]
pub type DMAMUX_RGSR = crate::Reg<u32, _DMAMUX_RGSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_RGSR;
#[doc = "`read()` method returns [dmamux_rgsr::R](dmamux_rgsr::R) reader structure"]
impl crate::Readable for DMAMUX_RGSR {}
#[doc = "DMAMUX request generator interrupt status register"]
pub mod dmamux_rgsr;
#[doc = "DMAMUX request generator interrupt clear flag register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_rgcfr](dmamux_rgcfr) module"]
pub type DMAMUX_RGCFR = crate::Reg<u32, _DMAMUX_RGCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_RGCFR;
#[doc = "`write(|w| ..)` method takes [dmamux_rgcfr::W](dmamux_rgcfr::W) writer structure"]
impl crate::Writable for DMAMUX_RGCFR {}
#[doc = "DMAMUX request generator interrupt clear flag register"]
pub mod dmamux_rgcfr;
