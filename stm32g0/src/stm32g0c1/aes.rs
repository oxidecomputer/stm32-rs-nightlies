#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AES control register"]
    pub aes_cr: AES_CR,
    #[doc = "0x04 - AES status register"]
    pub aes_sr: AES_SR,
    #[doc = "0x08 - AES data input register"]
    pub aes_dinr: AES_DINR,
    #[doc = "0x0c - AES data output register"]
    pub aes_doutr: AES_DOUTR,
    #[doc = "0x10 - AES key register 0"]
    pub aes_keyr0: AES_KEYR0,
    #[doc = "0x14 - AES key register 1"]
    pub aes_keyr1: AES_KEYR1,
    #[doc = "0x18 - AES key register 2"]
    pub aes_keyr2: AES_KEYR2,
    #[doc = "0x1c - AES key register 3"]
    pub aes_keyr3: AES_KEYR3,
    #[doc = "0x20 - AES initialization vector register 0"]
    pub aes_ivr0: AES_IVR0,
    #[doc = "0x24 - AES initialization vector register 1"]
    pub aes_ivr1: AES_IVR1,
    #[doc = "0x28 - AES initialization vector register 2"]
    pub aes_ivr2: AES_IVR2,
    #[doc = "0x2c - AES initialization vector register 3"]
    pub aes_ivr3: AES_IVR3,
    #[doc = "0x30 - AES key register 4"]
    pub aes_keyr4: AES_KEYR4,
    #[doc = "0x34 - AES key register 5"]
    pub aes_keyr5: AES_KEYR5,
    #[doc = "0x38 - AES key register 6"]
    pub aes_keyr6: AES_KEYR6,
    #[doc = "0x3c - AES key register 7"]
    pub aes_keyr7: AES_KEYR7,
    #[doc = "0x40 - AES suspend registers"]
    pub aes_susp0r: AES_SUSP0R,
    #[doc = "0x44 - AES suspend registers"]
    pub aes_susp1r: AES_SUSP1R,
    #[doc = "0x48 - AES suspend registers"]
    pub aes_susp2r: AES_SUSP2R,
    #[doc = "0x4c - AES suspend registers"]
    pub aes_susp3r: AES_SUSP3R,
    #[doc = "0x50 - AES suspend registers"]
    pub aes_susp4r: AES_SUSP4R,
    #[doc = "0x54 - AES suspend registers"]
    pub aes_susp5r: AES_SUSP5R,
    #[doc = "0x58 - AES suspend registers"]
    pub aes_susp6r: AES_SUSP6R,
    #[doc = "0x5c - AES suspend registers"]
    pub aes_susp7r: AES_SUSP7R,
}
#[doc = "AES control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_cr](aes_cr) module"]
pub type AES_CR = crate::Reg<u32, _AES_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_CR;
#[doc = "`read()` method returns [aes_cr::R](aes_cr::R) reader structure"]
impl crate::Readable for AES_CR {}
#[doc = "`write(|w| ..)` method takes [aes_cr::W](aes_cr::W) writer structure"]
impl crate::Writable for AES_CR {}
#[doc = "AES control register"]
pub mod aes_cr;
#[doc = "AES status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_sr](aes_sr) module"]
pub type AES_SR = crate::Reg<u32, _AES_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_SR;
#[doc = "`read()` method returns [aes_sr::R](aes_sr::R) reader structure"]
impl crate::Readable for AES_SR {}
#[doc = "AES status register"]
pub mod aes_sr;
#[doc = "AES data input register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_dinr](aes_dinr) module"]
pub type AES_DINR = crate::Reg<u32, _AES_DINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_DINR;
#[doc = "`read()` method returns [aes_dinr::R](aes_dinr::R) reader structure"]
impl crate::Readable for AES_DINR {}
#[doc = "`write(|w| ..)` method takes [aes_dinr::W](aes_dinr::W) writer structure"]
impl crate::Writable for AES_DINR {}
#[doc = "AES data input register"]
pub mod aes_dinr;
#[doc = "AES data output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_doutr](aes_doutr) module"]
pub type AES_DOUTR = crate::Reg<u32, _AES_DOUTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_DOUTR;
#[doc = "`read()` method returns [aes_doutr::R](aes_doutr::R) reader structure"]
impl crate::Readable for AES_DOUTR {}
#[doc = "AES data output register"]
pub mod aes_doutr;
#[doc = "AES key register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_keyr0](aes_keyr0) module"]
pub type AES_KEYR0 = crate::Reg<u32, _AES_KEYR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_KEYR0;
#[doc = "`read()` method returns [aes_keyr0::R](aes_keyr0::R) reader structure"]
impl crate::Readable for AES_KEYR0 {}
#[doc = "`write(|w| ..)` method takes [aes_keyr0::W](aes_keyr0::W) writer structure"]
impl crate::Writable for AES_KEYR0 {}
#[doc = "AES key register 0"]
pub mod aes_keyr0;
#[doc = "AES key register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_keyr1](aes_keyr1) module"]
pub type AES_KEYR1 = crate::Reg<u32, _AES_KEYR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_KEYR1;
#[doc = "`read()` method returns [aes_keyr1::R](aes_keyr1::R) reader structure"]
impl crate::Readable for AES_KEYR1 {}
#[doc = "`write(|w| ..)` method takes [aes_keyr1::W](aes_keyr1::W) writer structure"]
impl crate::Writable for AES_KEYR1 {}
#[doc = "AES key register 1"]
pub mod aes_keyr1;
#[doc = "AES key register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_keyr2](aes_keyr2) module"]
pub type AES_KEYR2 = crate::Reg<u32, _AES_KEYR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_KEYR2;
#[doc = "`read()` method returns [aes_keyr2::R](aes_keyr2::R) reader structure"]
impl crate::Readable for AES_KEYR2 {}
#[doc = "`write(|w| ..)` method takes [aes_keyr2::W](aes_keyr2::W) writer structure"]
impl crate::Writable for AES_KEYR2 {}
#[doc = "AES key register 2"]
pub mod aes_keyr2;
#[doc = "AES key register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_keyr3](aes_keyr3) module"]
pub type AES_KEYR3 = crate::Reg<u32, _AES_KEYR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_KEYR3;
#[doc = "`read()` method returns [aes_keyr3::R](aes_keyr3::R) reader structure"]
impl crate::Readable for AES_KEYR3 {}
#[doc = "`write(|w| ..)` method takes [aes_keyr3::W](aes_keyr3::W) writer structure"]
impl crate::Writable for AES_KEYR3 {}
#[doc = "AES key register 3"]
pub mod aes_keyr3;
#[doc = "AES initialization vector register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_ivr0](aes_ivr0) module"]
pub type AES_IVR0 = crate::Reg<u32, _AES_IVR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_IVR0;
#[doc = "`read()` method returns [aes_ivr0::R](aes_ivr0::R) reader structure"]
impl crate::Readable for AES_IVR0 {}
#[doc = "`write(|w| ..)` method takes [aes_ivr0::W](aes_ivr0::W) writer structure"]
impl crate::Writable for AES_IVR0 {}
#[doc = "AES initialization vector register 0"]
pub mod aes_ivr0;
#[doc = "AES initialization vector register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_ivr1](aes_ivr1) module"]
pub type AES_IVR1 = crate::Reg<u32, _AES_IVR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_IVR1;
#[doc = "`read()` method returns [aes_ivr1::R](aes_ivr1::R) reader structure"]
impl crate::Readable for AES_IVR1 {}
#[doc = "`write(|w| ..)` method takes [aes_ivr1::W](aes_ivr1::W) writer structure"]
impl crate::Writable for AES_IVR1 {}
#[doc = "AES initialization vector register 1"]
pub mod aes_ivr1;
#[doc = "AES initialization vector register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_ivr2](aes_ivr2) module"]
pub type AES_IVR2 = crate::Reg<u32, _AES_IVR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_IVR2;
#[doc = "`read()` method returns [aes_ivr2::R](aes_ivr2::R) reader structure"]
impl crate::Readable for AES_IVR2 {}
#[doc = "`write(|w| ..)` method takes [aes_ivr2::W](aes_ivr2::W) writer structure"]
impl crate::Writable for AES_IVR2 {}
#[doc = "AES initialization vector register 2"]
pub mod aes_ivr2;
#[doc = "AES initialization vector register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_ivr3](aes_ivr3) module"]
pub type AES_IVR3 = crate::Reg<u32, _AES_IVR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_IVR3;
#[doc = "`read()` method returns [aes_ivr3::R](aes_ivr3::R) reader structure"]
impl crate::Readable for AES_IVR3 {}
#[doc = "`write(|w| ..)` method takes [aes_ivr3::W](aes_ivr3::W) writer structure"]
impl crate::Writable for AES_IVR3 {}
#[doc = "AES initialization vector register 3"]
pub mod aes_ivr3;
#[doc = "AES key register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_keyr4](aes_keyr4) module"]
pub type AES_KEYR4 = crate::Reg<u32, _AES_KEYR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_KEYR4;
#[doc = "`read()` method returns [aes_keyr4::R](aes_keyr4::R) reader structure"]
impl crate::Readable for AES_KEYR4 {}
#[doc = "`write(|w| ..)` method takes [aes_keyr4::W](aes_keyr4::W) writer structure"]
impl crate::Writable for AES_KEYR4 {}
#[doc = "AES key register 4"]
pub mod aes_keyr4;
#[doc = "AES key register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_keyr5](aes_keyr5) module"]
pub type AES_KEYR5 = crate::Reg<u32, _AES_KEYR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_KEYR5;
#[doc = "`read()` method returns [aes_keyr5::R](aes_keyr5::R) reader structure"]
impl crate::Readable for AES_KEYR5 {}
#[doc = "`write(|w| ..)` method takes [aes_keyr5::W](aes_keyr5::W) writer structure"]
impl crate::Writable for AES_KEYR5 {}
#[doc = "AES key register 5"]
pub mod aes_keyr5;
#[doc = "AES key register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_keyr6](aes_keyr6) module"]
pub type AES_KEYR6 = crate::Reg<u32, _AES_KEYR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_KEYR6;
#[doc = "`read()` method returns [aes_keyr6::R](aes_keyr6::R) reader structure"]
impl crate::Readable for AES_KEYR6 {}
#[doc = "`write(|w| ..)` method takes [aes_keyr6::W](aes_keyr6::W) writer structure"]
impl crate::Writable for AES_KEYR6 {}
#[doc = "AES key register 6"]
pub mod aes_keyr6;
#[doc = "AES key register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_keyr7](aes_keyr7) module"]
pub type AES_KEYR7 = crate::Reg<u32, _AES_KEYR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_KEYR7;
#[doc = "`read()` method returns [aes_keyr7::R](aes_keyr7::R) reader structure"]
impl crate::Readable for AES_KEYR7 {}
#[doc = "`write(|w| ..)` method takes [aes_keyr7::W](aes_keyr7::W) writer structure"]
impl crate::Writable for AES_KEYR7 {}
#[doc = "AES key register 7"]
pub mod aes_keyr7;
#[doc = "AES suspend registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_susp0r](aes_susp0r) module"]
pub type AES_SUSP0R = crate::Reg<u32, _AES_SUSP0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_SUSP0R;
#[doc = "`read()` method returns [aes_susp0r::R](aes_susp0r::R) reader structure"]
impl crate::Readable for AES_SUSP0R {}
#[doc = "`write(|w| ..)` method takes [aes_susp0r::W](aes_susp0r::W) writer structure"]
impl crate::Writable for AES_SUSP0R {}
#[doc = "AES suspend registers"]
pub mod aes_susp0r;
#[doc = "AES suspend registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_susp1r](aes_susp1r) module"]
pub type AES_SUSP1R = crate::Reg<u32, _AES_SUSP1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_SUSP1R;
#[doc = "`read()` method returns [aes_susp1r::R](aes_susp1r::R) reader structure"]
impl crate::Readable for AES_SUSP1R {}
#[doc = "`write(|w| ..)` method takes [aes_susp1r::W](aes_susp1r::W) writer structure"]
impl crate::Writable for AES_SUSP1R {}
#[doc = "AES suspend registers"]
pub mod aes_susp1r;
#[doc = "AES suspend registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_susp2r](aes_susp2r) module"]
pub type AES_SUSP2R = crate::Reg<u32, _AES_SUSP2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_SUSP2R;
#[doc = "`read()` method returns [aes_susp2r::R](aes_susp2r::R) reader structure"]
impl crate::Readable for AES_SUSP2R {}
#[doc = "`write(|w| ..)` method takes [aes_susp2r::W](aes_susp2r::W) writer structure"]
impl crate::Writable for AES_SUSP2R {}
#[doc = "AES suspend registers"]
pub mod aes_susp2r;
#[doc = "AES suspend registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_susp3r](aes_susp3r) module"]
pub type AES_SUSP3R = crate::Reg<u32, _AES_SUSP3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_SUSP3R;
#[doc = "`read()` method returns [aes_susp3r::R](aes_susp3r::R) reader structure"]
impl crate::Readable for AES_SUSP3R {}
#[doc = "`write(|w| ..)` method takes [aes_susp3r::W](aes_susp3r::W) writer structure"]
impl crate::Writable for AES_SUSP3R {}
#[doc = "AES suspend registers"]
pub mod aes_susp3r;
#[doc = "AES suspend registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_susp4r](aes_susp4r) module"]
pub type AES_SUSP4R = crate::Reg<u32, _AES_SUSP4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_SUSP4R;
#[doc = "`read()` method returns [aes_susp4r::R](aes_susp4r::R) reader structure"]
impl crate::Readable for AES_SUSP4R {}
#[doc = "`write(|w| ..)` method takes [aes_susp4r::W](aes_susp4r::W) writer structure"]
impl crate::Writable for AES_SUSP4R {}
#[doc = "AES suspend registers"]
pub mod aes_susp4r;
#[doc = "AES suspend registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_susp5r](aes_susp5r) module"]
pub type AES_SUSP5R = crate::Reg<u32, _AES_SUSP5R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_SUSP5R;
#[doc = "`read()` method returns [aes_susp5r::R](aes_susp5r::R) reader structure"]
impl crate::Readable for AES_SUSP5R {}
#[doc = "`write(|w| ..)` method takes [aes_susp5r::W](aes_susp5r::W) writer structure"]
impl crate::Writable for AES_SUSP5R {}
#[doc = "AES suspend registers"]
pub mod aes_susp5r;
#[doc = "AES suspend registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_susp6r](aes_susp6r) module"]
pub type AES_SUSP6R = crate::Reg<u32, _AES_SUSP6R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_SUSP6R;
#[doc = "`read()` method returns [aes_susp6r::R](aes_susp6r::R) reader structure"]
impl crate::Readable for AES_SUSP6R {}
#[doc = "`write(|w| ..)` method takes [aes_susp6r::W](aes_susp6r::W) writer structure"]
impl crate::Writable for AES_SUSP6R {}
#[doc = "AES suspend registers"]
pub mod aes_susp6r;
#[doc = "AES suspend registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_susp7r](aes_susp7r) module"]
pub type AES_SUSP7R = crate::Reg<u32, _AES_SUSP7R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_SUSP7R;
#[doc = "`read()` method returns [aes_susp7r::R](aes_susp7r::R) reader structure"]
impl crate::Readable for AES_SUSP7R {}
#[doc = "`write(|w| ..)` method takes [aes_susp7r::W](aes_susp7r::W) writer structure"]
impl crate::Writable for AES_SUSP7R {}
#[doc = "AES suspend registers"]
pub mod aes_susp7r;
