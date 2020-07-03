#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA interrupt status register"]
    pub bdma_isr: BDMA_ISR,
    #[doc = "0x04 - DMA interrupt flag clear register"]
    pub bdma_ifcr: BDMA_IFCR,
    #[doc = "0x08 - DMA channel x configuration register"]
    pub bdma_ccr0: BDMA_CCR0,
    #[doc = "0x0c - DMA channel x number of data register"]
    pub bdma_cndtr0: BDMA_CNDTR0,
    #[doc = "0x10 - This register must not be written when the channel is enabled."]
    pub bdma_cpar0: BDMA_CPAR0,
    #[doc = "0x14 - This register must not be written when the channel is enabled."]
    pub bdma_cm0ar0: BDMA_CM0AR0,
    #[doc = "0x18 - This register must not be written when the channel is enabled"]
    pub bdma_cm1ar0: BDMA_CM1AR0,
    #[doc = "0x1c - DMA channel x configuration register"]
    pub bdma_ccr1: BDMA_CCR1,
    #[doc = "0x20 - DMA channel x number of data register"]
    pub bdma_cndtr1: BDMA_CNDTR1,
    #[doc = "0x24 - This register must not be written when the channel is enabled."]
    pub bdma_cpar1: BDMA_CPAR1,
    #[doc = "0x28 - This register must not be written when the channel is enabled."]
    pub bdma_cm0ar1: BDMA_CM0AR1,
    #[doc = "0x2c - BDMA_CMAR1"]
    pub bdma_cm1ar1: BDMA_CM1AR1,
    #[doc = "0x30 - DMA channel x configuration register"]
    pub bdma_ccr2: BDMA_CCR2,
    #[doc = "0x34 - DMA channel x number of data register"]
    pub bdma_cndtr2: BDMA_CNDTR2,
    #[doc = "0x38 - This register must not be written when the channel is enabled."]
    pub bdma_cpar2: BDMA_CPAR2,
    #[doc = "0x3c - This register must not be written when the channel is enabled."]
    pub bdma_cm0ar2: BDMA_CM0AR2,
    #[doc = "0x40 - BDMA_CM1AR2"]
    pub bdma_cm1ar2: BDMA_CM1AR2,
    #[doc = "0x44 - DMA channel x configuration register"]
    pub bdma_ccr3: BDMA_CCR3,
    #[doc = "0x48 - DMA channel x number of data register"]
    pub bdma_cndtr3: BDMA_CNDTR3,
    #[doc = "0x4c - This register must not be written when the channel is enabled."]
    pub bdma_cpar3: BDMA_CPAR3,
    #[doc = "0x50 - This register must not be written when the channel is enabled."]
    pub bdma_cm0ar3: BDMA_CM0AR3,
    #[doc = "0x54 - BDMA_CMAR3"]
    pub bdma_cm1ar3: BDMA_CM1AR3,
    #[doc = "0x58 - DMA channel x configuration register"]
    pub bdma_ccr4: BDMA_CCR4,
    #[doc = "0x5c - DMA channel x number of data register"]
    pub bdma_cndtr4: BDMA_CNDTR4,
    #[doc = "0x60 - This register must not be written when the channel is enabled."]
    pub bdma_cpar4: BDMA_CPAR4,
    #[doc = "0x64 - This register must not be written when the channel is enabled."]
    pub bdma_cm0ar4: BDMA_CM0AR4,
    #[doc = "0x68 - BDMA_CM1AR4"]
    pub bdma_cm1ar4: BDMA_CM1AR4,
    #[doc = "0x6c - DMA channel x configuration register"]
    pub bdma_ccr5: BDMA_CCR5,
    #[doc = "0x70 - DMA channel x number of data register"]
    pub bdma_cndtr5: BDMA_CNDTR5,
    #[doc = "0x74 - This register must not be written when the channel is enabled."]
    pub bdma_cpar5: BDMA_CPAR5,
    #[doc = "0x78 - This register must not be written when the channel is enabled."]
    pub bdma_cm0ar5: BDMA_CM0AR5,
    #[doc = "0x7c - This register must not be written when the channel is enabled."]
    pub bdma_cm1ar5: BDMA_CM1AR5,
    #[doc = "0x80 - DMA channel x configuration register"]
    pub bdma_ccr6: BDMA_CCR6,
    #[doc = "0x84 - DMA channel x number of data register"]
    pub bdma_cndtr6: BDMA_CNDTR6,
    #[doc = "0x88 - This register must not be written when the channel is enabled."]
    pub bdma_cpar6: BDMA_CPAR6,
    #[doc = "0x8c - This register must not be written when the channel is enabled."]
    pub bdma_cm0ar6: BDMA_CM0AR6,
    #[doc = "0x90 - This register must not be written when the channel is enabled."]
    pub bdma_cm1ar6: BDMA_CM1AR6,
    #[doc = "0x94 - DMA channel x configuration register"]
    pub bdma_ccr7: BDMA_CCR7,
    #[doc = "0x98 - DMA channel x number of data register"]
    pub bdma_cndtr7: BDMA_CNDTR7,
    #[doc = "0x9c - This register must not be written when the channel is enabled."]
    pub bdma_cpar7: BDMA_CPAR7,
    #[doc = "0xa0 - This register must not be written when the channel is enabled."]
    pub bdma_cm0ar7: BDMA_CM0AR7,
    #[doc = "0xa4 - This register must not be written when the channel is enabled."]
    pub bdma_cm1ar7: BDMA_CM1AR7,
}
#[doc = "DMA interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_isr](bdma_isr) module"]
pub type BDMA_ISR = crate::Reg<u32, _BDMA_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_ISR;
#[doc = "`read()` method returns [bdma_isr::R](bdma_isr::R) reader structure"]
impl crate::Readable for BDMA_ISR {}
#[doc = "DMA interrupt status register"]
pub mod bdma_isr;
#[doc = "DMA interrupt flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_ifcr](bdma_ifcr) module"]
pub type BDMA_IFCR = crate::Reg<u32, _BDMA_IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_IFCR;
#[doc = "`write(|w| ..)` method takes [bdma_ifcr::W](bdma_ifcr::W) writer structure"]
impl crate::Writable for BDMA_IFCR {}
#[doc = "DMA interrupt flag clear register"]
pub mod bdma_ifcr;
#[doc = "DMA channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_ccr0](bdma_ccr0) module"]
pub type BDMA_CCR0 = crate::Reg<u32, _BDMA_CCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CCR0;
#[doc = "`read()` method returns [bdma_ccr0::R](bdma_ccr0::R) reader structure"]
impl crate::Readable for BDMA_CCR0 {}
#[doc = "`write(|w| ..)` method takes [bdma_ccr0::W](bdma_ccr0::W) writer structure"]
impl crate::Writable for BDMA_CCR0 {}
#[doc = "DMA channel x configuration register"]
pub mod bdma_ccr0;
#[doc = "DMA channel x number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_cndtr0](bdma_cndtr0) module"]
pub type BDMA_CNDTR0 = crate::Reg<u32, _BDMA_CNDTR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CNDTR0;
#[doc = "`read()` method returns [bdma_cndtr0::R](bdma_cndtr0::R) reader structure"]
impl crate::Readable for BDMA_CNDTR0 {}
#[doc = "`write(|w| ..)` method takes [bdma_cndtr0::W](bdma_cndtr0::W) writer structure"]
impl crate::Writable for BDMA_CNDTR0 {}
#[doc = "DMA channel x number of data register"]
pub mod bdma_cndtr0;
#[doc = "This register must not be written when the channel is enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_cpar0](bdma_cpar0) module"]
pub type BDMA_CPAR0 = crate::Reg<u32, _BDMA_CPAR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CPAR0;
#[doc = "`read()` method returns [bdma_cpar0::R](bdma_cpar0::R) reader structure"]
impl crate::Readable for BDMA_CPAR0 {}
#[doc = "`write(|w| ..)` method takes [bdma_cpar0::W](bdma_cpar0::W) writer structure"]
impl crate::Writable for BDMA_CPAR0 {}
#[doc = "This register must not be written when the channel is enabled."]
pub mod bdma_cpar0;
#[doc = "This register must not be written when the channel is enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_cm0ar0](bdma_cm0ar0) module"]
pub type BDMA_CM0AR0 = crate::Reg<u32, _BDMA_CM0AR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CM0AR0;
#[doc = "`read()` method returns [bdma_cm0ar0::R](bdma_cm0ar0::R) reader structure"]
impl crate::Readable for BDMA_CM0AR0 {}
#[doc = "`write(|w| ..)` method takes [bdma_cm0ar0::W](bdma_cm0ar0::W) writer structure"]
impl crate::Writable for BDMA_CM0AR0 {}
#[doc = "This register must not be written when the channel is enabled."]
pub mod bdma_cm0ar0;
#[doc = "DMA channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_ccr1](bdma_ccr1) module"]
pub type BDMA_CCR1 = crate::Reg<u32, _BDMA_CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CCR1;
#[doc = "`read()` method returns [bdma_ccr1::R](bdma_ccr1::R) reader structure"]
impl crate::Readable for BDMA_CCR1 {}
#[doc = "`write(|w| ..)` method takes [bdma_ccr1::W](bdma_ccr1::W) writer structure"]
impl crate::Writable for BDMA_CCR1 {}
#[doc = "DMA channel x configuration register"]
pub mod bdma_ccr1;
#[doc = "DMA channel x number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_cndtr1](bdma_cndtr1) module"]
pub type BDMA_CNDTR1 = crate::Reg<u32, _BDMA_CNDTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CNDTR1;
#[doc = "`read()` method returns [bdma_cndtr1::R](bdma_cndtr1::R) reader structure"]
impl crate::Readable for BDMA_CNDTR1 {}
#[doc = "`write(|w| ..)` method takes [bdma_cndtr1::W](bdma_cndtr1::W) writer structure"]
impl crate::Writable for BDMA_CNDTR1 {}
#[doc = "DMA channel x number of data register"]
pub mod bdma_cndtr1;
#[doc = "This register must not be written when the channel is enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_cpar1](bdma_cpar1) module"]
pub type BDMA_CPAR1 = crate::Reg<u32, _BDMA_CPAR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CPAR1;
#[doc = "`read()` method returns [bdma_cpar1::R](bdma_cpar1::R) reader structure"]
impl crate::Readable for BDMA_CPAR1 {}
#[doc = "`write(|w| ..)` method takes [bdma_cpar1::W](bdma_cpar1::W) writer structure"]
impl crate::Writable for BDMA_CPAR1 {}
#[doc = "This register must not be written when the channel is enabled."]
pub mod bdma_cpar1;
#[doc = "This register must not be written when the channel is enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_cm0ar1](bdma_cm0ar1) module"]
pub type BDMA_CM0AR1 = crate::Reg<u32, _BDMA_CM0AR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CM0AR1;
#[doc = "`read()` method returns [bdma_cm0ar1::R](bdma_cm0ar1::R) reader structure"]
impl crate::Readable for BDMA_CM0AR1 {}
#[doc = "`write(|w| ..)` method takes [bdma_cm0ar1::W](bdma_cm0ar1::W) writer structure"]
impl crate::Writable for BDMA_CM0AR1 {}
#[doc = "This register must not be written when the channel is enabled."]
pub mod bdma_cm0ar1;
#[doc = "DMA channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_ccr2](bdma_ccr2) module"]
pub type BDMA_CCR2 = crate::Reg<u32, _BDMA_CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CCR2;
#[doc = "`read()` method returns [bdma_ccr2::R](bdma_ccr2::R) reader structure"]
impl crate::Readable for BDMA_CCR2 {}
#[doc = "`write(|w| ..)` method takes [bdma_ccr2::W](bdma_ccr2::W) writer structure"]
impl crate::Writable for BDMA_CCR2 {}
#[doc = "DMA channel x configuration register"]
pub mod bdma_ccr2;
#[doc = "DMA channel x number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_cndtr2](bdma_cndtr2) module"]
pub type BDMA_CNDTR2 = crate::Reg<u32, _BDMA_CNDTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CNDTR2;
#[doc = "`read()` method returns [bdma_cndtr2::R](bdma_cndtr2::R) reader structure"]
impl crate::Readable for BDMA_CNDTR2 {}
#[doc = "`write(|w| ..)` method takes [bdma_cndtr2::W](bdma_cndtr2::W) writer structure"]
impl crate::Writable for BDMA_CNDTR2 {}
#[doc = "DMA channel x number of data register"]
pub mod bdma_cndtr2;
#[doc = "This register must not be written when the channel is enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_cpar2](bdma_cpar2) module"]
pub type BDMA_CPAR2 = crate::Reg<u32, _BDMA_CPAR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CPAR2;
#[doc = "`read()` method returns [bdma_cpar2::R](bdma_cpar2::R) reader structure"]
impl crate::Readable for BDMA_CPAR2 {}
#[doc = "`write(|w| ..)` method takes [bdma_cpar2::W](bdma_cpar2::W) writer structure"]
impl crate::Writable for BDMA_CPAR2 {}
#[doc = "This register must not be written when the channel is enabled."]
pub mod bdma_cpar2;
#[doc = "This register must not be written when the channel is enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_cm0ar2](bdma_cm0ar2) module"]
pub type BDMA_CM0AR2 = crate::Reg<u32, _BDMA_CM0AR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CM0AR2;
#[doc = "`read()` method returns [bdma_cm0ar2::R](bdma_cm0ar2::R) reader structure"]
impl crate::Readable for BDMA_CM0AR2 {}
#[doc = "`write(|w| ..)` method takes [bdma_cm0ar2::W](bdma_cm0ar2::W) writer structure"]
impl crate::Writable for BDMA_CM0AR2 {}
#[doc = "This register must not be written when the channel is enabled."]
pub mod bdma_cm0ar2;
#[doc = "DMA channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_ccr3](bdma_ccr3) module"]
pub type BDMA_CCR3 = crate::Reg<u32, _BDMA_CCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CCR3;
#[doc = "`read()` method returns [bdma_ccr3::R](bdma_ccr3::R) reader structure"]
impl crate::Readable for BDMA_CCR3 {}
#[doc = "`write(|w| ..)` method takes [bdma_ccr3::W](bdma_ccr3::W) writer structure"]
impl crate::Writable for BDMA_CCR3 {}
#[doc = "DMA channel x configuration register"]
pub mod bdma_ccr3;
#[doc = "DMA channel x number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_cndtr3](bdma_cndtr3) module"]
pub type BDMA_CNDTR3 = crate::Reg<u32, _BDMA_CNDTR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CNDTR3;
#[doc = "`read()` method returns [bdma_cndtr3::R](bdma_cndtr3::R) reader structure"]
impl crate::Readable for BDMA_CNDTR3 {}
#[doc = "`write(|w| ..)` method takes [bdma_cndtr3::W](bdma_cndtr3::W) writer structure"]
impl crate::Writable for BDMA_CNDTR3 {}
#[doc = "DMA channel x number of data register"]
pub mod bdma_cndtr3;
#[doc = "This register must not be written when the channel is enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_cpar3](bdma_cpar3) module"]
pub type BDMA_CPAR3 = crate::Reg<u32, _BDMA_CPAR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CPAR3;
#[doc = "`read()` method returns [bdma_cpar3::R](bdma_cpar3::R) reader structure"]
impl crate::Readable for BDMA_CPAR3 {}
#[doc = "`write(|w| ..)` method takes [bdma_cpar3::W](bdma_cpar3::W) writer structure"]
impl crate::Writable for BDMA_CPAR3 {}
#[doc = "This register must not be written when the channel is enabled."]
pub mod bdma_cpar3;
#[doc = "This register must not be written when the channel is enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_cm0ar3](bdma_cm0ar3) module"]
pub type BDMA_CM0AR3 = crate::Reg<u32, _BDMA_CM0AR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CM0AR3;
#[doc = "`read()` method returns [bdma_cm0ar3::R](bdma_cm0ar3::R) reader structure"]
impl crate::Readable for BDMA_CM0AR3 {}
#[doc = "`write(|w| ..)` method takes [bdma_cm0ar3::W](bdma_cm0ar3::W) writer structure"]
impl crate::Writable for BDMA_CM0AR3 {}
#[doc = "This register must not be written when the channel is enabled."]
pub mod bdma_cm0ar3;
#[doc = "DMA channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_ccr4](bdma_ccr4) module"]
pub type BDMA_CCR4 = crate::Reg<u32, _BDMA_CCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CCR4;
#[doc = "`read()` method returns [bdma_ccr4::R](bdma_ccr4::R) reader structure"]
impl crate::Readable for BDMA_CCR4 {}
#[doc = "`write(|w| ..)` method takes [bdma_ccr4::W](bdma_ccr4::W) writer structure"]
impl crate::Writable for BDMA_CCR4 {}
#[doc = "DMA channel x configuration register"]
pub mod bdma_ccr4;
#[doc = "DMA channel x number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_cndtr4](bdma_cndtr4) module"]
pub type BDMA_CNDTR4 = crate::Reg<u32, _BDMA_CNDTR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CNDTR4;
#[doc = "`read()` method returns [bdma_cndtr4::R](bdma_cndtr4::R) reader structure"]
impl crate::Readable for BDMA_CNDTR4 {}
#[doc = "`write(|w| ..)` method takes [bdma_cndtr4::W](bdma_cndtr4::W) writer structure"]
impl crate::Writable for BDMA_CNDTR4 {}
#[doc = "DMA channel x number of data register"]
pub mod bdma_cndtr4;
#[doc = "This register must not be written when the channel is enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_cpar4](bdma_cpar4) module"]
pub type BDMA_CPAR4 = crate::Reg<u32, _BDMA_CPAR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CPAR4;
#[doc = "`read()` method returns [bdma_cpar4::R](bdma_cpar4::R) reader structure"]
impl crate::Readable for BDMA_CPAR4 {}
#[doc = "`write(|w| ..)` method takes [bdma_cpar4::W](bdma_cpar4::W) writer structure"]
impl crate::Writable for BDMA_CPAR4 {}
#[doc = "This register must not be written when the channel is enabled."]
pub mod bdma_cpar4;
#[doc = "This register must not be written when the channel is enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_cm0ar4](bdma_cm0ar4) module"]
pub type BDMA_CM0AR4 = crate::Reg<u32, _BDMA_CM0AR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CM0AR4;
#[doc = "`read()` method returns [bdma_cm0ar4::R](bdma_cm0ar4::R) reader structure"]
impl crate::Readable for BDMA_CM0AR4 {}
#[doc = "`write(|w| ..)` method takes [bdma_cm0ar4::W](bdma_cm0ar4::W) writer structure"]
impl crate::Writable for BDMA_CM0AR4 {}
#[doc = "This register must not be written when the channel is enabled."]
pub mod bdma_cm0ar4;
#[doc = "DMA channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_ccr5](bdma_ccr5) module"]
pub type BDMA_CCR5 = crate::Reg<u32, _BDMA_CCR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CCR5;
#[doc = "`read()` method returns [bdma_ccr5::R](bdma_ccr5::R) reader structure"]
impl crate::Readable for BDMA_CCR5 {}
#[doc = "`write(|w| ..)` method takes [bdma_ccr5::W](bdma_ccr5::W) writer structure"]
impl crate::Writable for BDMA_CCR5 {}
#[doc = "DMA channel x configuration register"]
pub mod bdma_ccr5;
#[doc = "DMA channel x number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_cndtr5](bdma_cndtr5) module"]
pub type BDMA_CNDTR5 = crate::Reg<u32, _BDMA_CNDTR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CNDTR5;
#[doc = "`read()` method returns [bdma_cndtr5::R](bdma_cndtr5::R) reader structure"]
impl crate::Readable for BDMA_CNDTR5 {}
#[doc = "`write(|w| ..)` method takes [bdma_cndtr5::W](bdma_cndtr5::W) writer structure"]
impl crate::Writable for BDMA_CNDTR5 {}
#[doc = "DMA channel x number of data register"]
pub mod bdma_cndtr5;
#[doc = "This register must not be written when the channel is enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_cpar5](bdma_cpar5) module"]
pub type BDMA_CPAR5 = crate::Reg<u32, _BDMA_CPAR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CPAR5;
#[doc = "`read()` method returns [bdma_cpar5::R](bdma_cpar5::R) reader structure"]
impl crate::Readable for BDMA_CPAR5 {}
#[doc = "`write(|w| ..)` method takes [bdma_cpar5::W](bdma_cpar5::W) writer structure"]
impl crate::Writable for BDMA_CPAR5 {}
#[doc = "This register must not be written when the channel is enabled."]
pub mod bdma_cpar5;
#[doc = "This register must not be written when the channel is enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_cm0ar5](bdma_cm0ar5) module"]
pub type BDMA_CM0AR5 = crate::Reg<u32, _BDMA_CM0AR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CM0AR5;
#[doc = "`read()` method returns [bdma_cm0ar5::R](bdma_cm0ar5::R) reader structure"]
impl crate::Readable for BDMA_CM0AR5 {}
#[doc = "`write(|w| ..)` method takes [bdma_cm0ar5::W](bdma_cm0ar5::W) writer structure"]
impl crate::Writable for BDMA_CM0AR5 {}
#[doc = "This register must not be written when the channel is enabled."]
pub mod bdma_cm0ar5;
#[doc = "DMA channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_ccr6](bdma_ccr6) module"]
pub type BDMA_CCR6 = crate::Reg<u32, _BDMA_CCR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CCR6;
#[doc = "`read()` method returns [bdma_ccr6::R](bdma_ccr6::R) reader structure"]
impl crate::Readable for BDMA_CCR6 {}
#[doc = "`write(|w| ..)` method takes [bdma_ccr6::W](bdma_ccr6::W) writer structure"]
impl crate::Writable for BDMA_CCR6 {}
#[doc = "DMA channel x configuration register"]
pub mod bdma_ccr6;
#[doc = "DMA channel x number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_cndtr6](bdma_cndtr6) module"]
pub type BDMA_CNDTR6 = crate::Reg<u32, _BDMA_CNDTR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CNDTR6;
#[doc = "`read()` method returns [bdma_cndtr6::R](bdma_cndtr6::R) reader structure"]
impl crate::Readable for BDMA_CNDTR6 {}
#[doc = "`write(|w| ..)` method takes [bdma_cndtr6::W](bdma_cndtr6::W) writer structure"]
impl crate::Writable for BDMA_CNDTR6 {}
#[doc = "DMA channel x number of data register"]
pub mod bdma_cndtr6;
#[doc = "This register must not be written when the channel is enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_cpar6](bdma_cpar6) module"]
pub type BDMA_CPAR6 = crate::Reg<u32, _BDMA_CPAR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CPAR6;
#[doc = "`read()` method returns [bdma_cpar6::R](bdma_cpar6::R) reader structure"]
impl crate::Readable for BDMA_CPAR6 {}
#[doc = "`write(|w| ..)` method takes [bdma_cpar6::W](bdma_cpar6::W) writer structure"]
impl crate::Writable for BDMA_CPAR6 {}
#[doc = "This register must not be written when the channel is enabled."]
pub mod bdma_cpar6;
#[doc = "This register must not be written when the channel is enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_cm0ar6](bdma_cm0ar6) module"]
pub type BDMA_CM0AR6 = crate::Reg<u32, _BDMA_CM0AR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CM0AR6;
#[doc = "`read()` method returns [bdma_cm0ar6::R](bdma_cm0ar6::R) reader structure"]
impl crate::Readable for BDMA_CM0AR6 {}
#[doc = "`write(|w| ..)` method takes [bdma_cm0ar6::W](bdma_cm0ar6::W) writer structure"]
impl crate::Writable for BDMA_CM0AR6 {}
#[doc = "This register must not be written when the channel is enabled."]
pub mod bdma_cm0ar6;
#[doc = "DMA channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_ccr7](bdma_ccr7) module"]
pub type BDMA_CCR7 = crate::Reg<u32, _BDMA_CCR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CCR7;
#[doc = "`read()` method returns [bdma_ccr7::R](bdma_ccr7::R) reader structure"]
impl crate::Readable for BDMA_CCR7 {}
#[doc = "`write(|w| ..)` method takes [bdma_ccr7::W](bdma_ccr7::W) writer structure"]
impl crate::Writable for BDMA_CCR7 {}
#[doc = "DMA channel x configuration register"]
pub mod bdma_ccr7;
#[doc = "DMA channel x number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_cndtr7](bdma_cndtr7) module"]
pub type BDMA_CNDTR7 = crate::Reg<u32, _BDMA_CNDTR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CNDTR7;
#[doc = "`read()` method returns [bdma_cndtr7::R](bdma_cndtr7::R) reader structure"]
impl crate::Readable for BDMA_CNDTR7 {}
#[doc = "`write(|w| ..)` method takes [bdma_cndtr7::W](bdma_cndtr7::W) writer structure"]
impl crate::Writable for BDMA_CNDTR7 {}
#[doc = "DMA channel x number of data register"]
pub mod bdma_cndtr7;
#[doc = "This register must not be written when the channel is enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_cpar7](bdma_cpar7) module"]
pub type BDMA_CPAR7 = crate::Reg<u32, _BDMA_CPAR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CPAR7;
#[doc = "`read()` method returns [bdma_cpar7::R](bdma_cpar7::R) reader structure"]
impl crate::Readable for BDMA_CPAR7 {}
#[doc = "`write(|w| ..)` method takes [bdma_cpar7::W](bdma_cpar7::W) writer structure"]
impl crate::Writable for BDMA_CPAR7 {}
#[doc = "This register must not be written when the channel is enabled."]
pub mod bdma_cpar7;
#[doc = "This register must not be written when the channel is enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_cm0ar7](bdma_cm0ar7) module"]
pub type BDMA_CM0AR7 = crate::Reg<u32, _BDMA_CM0AR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CM0AR7;
#[doc = "`read()` method returns [bdma_cm0ar7::R](bdma_cm0ar7::R) reader structure"]
impl crate::Readable for BDMA_CM0AR7 {}
#[doc = "`write(|w| ..)` method takes [bdma_cm0ar7::W](bdma_cm0ar7::W) writer structure"]
impl crate::Writable for BDMA_CM0AR7 {}
#[doc = "This register must not be written when the channel is enabled."]
pub mod bdma_cm0ar7;
#[doc = "This register must not be written when the channel is enabled\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_cm1ar0](bdma_cm1ar0) module"]
pub type BDMA_CM1AR0 = crate::Reg<u32, _BDMA_CM1AR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CM1AR0;
#[doc = "`read()` method returns [bdma_cm1ar0::R](bdma_cm1ar0::R) reader structure"]
impl crate::Readable for BDMA_CM1AR0 {}
#[doc = "`write(|w| ..)` method takes [bdma_cm1ar0::W](bdma_cm1ar0::W) writer structure"]
impl crate::Writable for BDMA_CM1AR0 {}
#[doc = "This register must not be written when the channel is enabled"]
pub mod bdma_cm1ar0;
#[doc = "BDMA_CMAR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_cm1ar1](bdma_cm1ar1) module"]
pub type BDMA_CM1AR1 = crate::Reg<u32, _BDMA_CM1AR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CM1AR1;
#[doc = "`read()` method returns [bdma_cm1ar1::R](bdma_cm1ar1::R) reader structure"]
impl crate::Readable for BDMA_CM1AR1 {}
#[doc = "`write(|w| ..)` method takes [bdma_cm1ar1::W](bdma_cm1ar1::W) writer structure"]
impl crate::Writable for BDMA_CM1AR1 {}
#[doc = "BDMA_CMAR1"]
pub mod bdma_cm1ar1;
#[doc = "BDMA_CM1AR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_cm1ar2](bdma_cm1ar2) module"]
pub type BDMA_CM1AR2 = crate::Reg<u32, _BDMA_CM1AR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CM1AR2;
#[doc = "`read()` method returns [bdma_cm1ar2::R](bdma_cm1ar2::R) reader structure"]
impl crate::Readable for BDMA_CM1AR2 {}
#[doc = "`write(|w| ..)` method takes [bdma_cm1ar2::W](bdma_cm1ar2::W) writer structure"]
impl crate::Writable for BDMA_CM1AR2 {}
#[doc = "BDMA_CM1AR2"]
pub mod bdma_cm1ar2;
#[doc = "BDMA_CMAR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_cm1ar3](bdma_cm1ar3) module"]
pub type BDMA_CM1AR3 = crate::Reg<u32, _BDMA_CM1AR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CM1AR3;
#[doc = "`read()` method returns [bdma_cm1ar3::R](bdma_cm1ar3::R) reader structure"]
impl crate::Readable for BDMA_CM1AR3 {}
#[doc = "`write(|w| ..)` method takes [bdma_cm1ar3::W](bdma_cm1ar3::W) writer structure"]
impl crate::Writable for BDMA_CM1AR3 {}
#[doc = "BDMA_CMAR3"]
pub mod bdma_cm1ar3;
#[doc = "BDMA_CM1AR4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_cm1ar4](bdma_cm1ar4) module"]
pub type BDMA_CM1AR4 = crate::Reg<u32, _BDMA_CM1AR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CM1AR4;
#[doc = "`read()` method returns [bdma_cm1ar4::R](bdma_cm1ar4::R) reader structure"]
impl crate::Readable for BDMA_CM1AR4 {}
#[doc = "`write(|w| ..)` method takes [bdma_cm1ar4::W](bdma_cm1ar4::W) writer structure"]
impl crate::Writable for BDMA_CM1AR4 {}
#[doc = "BDMA_CM1AR4"]
pub mod bdma_cm1ar4;
#[doc = "This register must not be written when the channel is enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_cm1ar5](bdma_cm1ar5) module"]
pub type BDMA_CM1AR5 = crate::Reg<u32, _BDMA_CM1AR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CM1AR5;
#[doc = "`read()` method returns [bdma_cm1ar5::R](bdma_cm1ar5::R) reader structure"]
impl crate::Readable for BDMA_CM1AR5 {}
#[doc = "`write(|w| ..)` method takes [bdma_cm1ar5::W](bdma_cm1ar5::W) writer structure"]
impl crate::Writable for BDMA_CM1AR5 {}
#[doc = "This register must not be written when the channel is enabled."]
pub mod bdma_cm1ar5;
#[doc = "This register must not be written when the channel is enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_cm1ar6](bdma_cm1ar6) module"]
pub type BDMA_CM1AR6 = crate::Reg<u32, _BDMA_CM1AR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CM1AR6;
#[doc = "`read()` method returns [bdma_cm1ar6::R](bdma_cm1ar6::R) reader structure"]
impl crate::Readable for BDMA_CM1AR6 {}
#[doc = "`write(|w| ..)` method takes [bdma_cm1ar6::W](bdma_cm1ar6::W) writer structure"]
impl crate::Writable for BDMA_CM1AR6 {}
#[doc = "This register must not be written when the channel is enabled."]
pub mod bdma_cm1ar6;
#[doc = "This register must not be written when the channel is enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdma_cm1ar7](bdma_cm1ar7) module"]
pub type BDMA_CM1AR7 = crate::Reg<u32, _BDMA_CM1AR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMA_CM1AR7;
#[doc = "`read()` method returns [bdma_cm1ar7::R](bdma_cm1ar7::R) reader structure"]
impl crate::Readable for BDMA_CM1AR7 {}
#[doc = "`write(|w| ..)` method takes [bdma_cm1ar7::W](bdma_cm1ar7::W) writer structure"]
impl crate::Writable for BDMA_CM1AR7 {}
#[doc = "This register must not be written when the channel is enabled."]
pub mod bdma_cm1ar7;
