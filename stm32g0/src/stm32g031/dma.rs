#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - low interrupt status register"]
    pub isr: ISR,
    #[doc = "0x04 - high interrupt status register"]
    pub ifcr: IFCR,
    #[doc = "0x08 - DMA channel x configuration register"]
    pub ccr1: CCR1,
    #[doc = "0x0c - DMA channel x number of data register"]
    pub cndtr1: CNDTR1,
    #[doc = "0x10 - DMA channel x peripheral address register"]
    pub cpar1: CPAR1,
    #[doc = "0x14 - DMA channel x memory address register"]
    pub cmar1: CMAR1,
    _reserved6: [u8; 4usize],
    #[doc = "0x1c - DMA channel x configuration register"]
    pub ccr2: CCR2,
    #[doc = "0x20 - DMA channel x number of data register"]
    pub cndtr2: CNDTR2,
    #[doc = "0x24 - DMA channel x peripheral address register"]
    pub cpar2: CPAR2,
    #[doc = "0x28 - DMA channel x memory address register"]
    pub cmar2: CMAR2,
    _reserved10: [u8; 4usize],
    #[doc = "0x30 - DMA channel x configuration register"]
    pub ccr3: CCR3,
    #[doc = "0x34 - DMA channel x configuration register"]
    pub cndtr3: CNDTR3,
    #[doc = "0x38 - DMA channel x peripheral address register"]
    pub cpar3: CPAR3,
    #[doc = "0x3c - DMA channel x memory address register"]
    pub cmar3: CMAR3,
    _reserved14: [u8; 4usize],
    #[doc = "0x44 - DMA channel x configuration register"]
    pub ccr4: CCR4,
    #[doc = "0x48 - DMA channel x configuration register"]
    pub cndtr4: CNDTR4,
    #[doc = "0x4c - DMA channel x peripheral address register"]
    pub cpar4: CPAR4,
    #[doc = "0x50 - DMA channel x memory address register"]
    pub cmar4: CMAR4,
    _reserved18: [u8; 4usize],
    #[doc = "0x58 - DMA channel x configuration register"]
    pub ccr5: CCR5,
    #[doc = "0x5c - DMA channel x configuration register"]
    pub cndtr5: CNDTR5,
    #[doc = "0x60 - DMA channel x peripheral address register"]
    pub cpar5: CPAR5,
    #[doc = "0x64 - DMA channel x memory address register"]
    pub cmar5: CMAR5,
}
#[doc = "low interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "low interrupt status register"]
pub mod isr;
#[doc = "high interrupt status register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifcr](ifcr) module"]
pub type IFCR = crate::Reg<u32, _IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFCR;
#[doc = "`write(|w| ..)` method takes [ifcr::W](ifcr::W) writer structure"]
impl crate::Writable for IFCR {}
#[doc = "high interrupt status register"]
pub mod ifcr;
#[doc = "DMA channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr1](ccr1) module"]
pub type CCR1 = crate::Reg<u32, _CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR1;
#[doc = "`read()` method returns [ccr1::R](ccr1::R) reader structure"]
impl crate::Readable for CCR1 {}
#[doc = "`write(|w| ..)` method takes [ccr1::W](ccr1::W) writer structure"]
impl crate::Writable for CCR1 {}
#[doc = "DMA channel x configuration register"]
pub mod ccr1;
#[doc = "DMA channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr2](ccr2) module"]
pub type CCR2 = crate::Reg<u32, _CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR2;
#[doc = "`read()` method returns [ccr2::R](ccr2::R) reader structure"]
impl crate::Readable for CCR2 {}
#[doc = "`write(|w| ..)` method takes [ccr2::W](ccr2::W) writer structure"]
impl crate::Writable for CCR2 {}
#[doc = "DMA channel x configuration register"]
pub mod ccr2;
#[doc = "DMA channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr3](ccr3) module"]
pub type CCR3 = crate::Reg<u32, _CCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR3;
#[doc = "`read()` method returns [ccr3::R](ccr3::R) reader structure"]
impl crate::Readable for CCR3 {}
#[doc = "`write(|w| ..)` method takes [ccr3::W](ccr3::W) writer structure"]
impl crate::Writable for CCR3 {}
#[doc = "DMA channel x configuration register"]
pub mod ccr3;
#[doc = "DMA channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr4](ccr4) module"]
pub type CCR4 = crate::Reg<u32, _CCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR4;
#[doc = "`read()` method returns [ccr4::R](ccr4::R) reader structure"]
impl crate::Readable for CCR4 {}
#[doc = "`write(|w| ..)` method takes [ccr4::W](ccr4::W) writer structure"]
impl crate::Writable for CCR4 {}
#[doc = "DMA channel x configuration register"]
pub mod ccr4;
#[doc = "DMA channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr5](ccr5) module"]
pub type CCR5 = crate::Reg<u32, _CCR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR5;
#[doc = "`read()` method returns [ccr5::R](ccr5::R) reader structure"]
impl crate::Readable for CCR5 {}
#[doc = "`write(|w| ..)` method takes [ccr5::W](ccr5::W) writer structure"]
impl crate::Writable for CCR5 {}
#[doc = "DMA channel x configuration register"]
pub mod ccr5;
#[doc = "DMA channel x number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndtr1](cndtr1) module"]
pub type CNDTR1 = crate::Reg<u32, _CNDTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDTR1;
#[doc = "`read()` method returns [cndtr1::R](cndtr1::R) reader structure"]
impl crate::Readable for CNDTR1 {}
#[doc = "`write(|w| ..)` method takes [cndtr1::W](cndtr1::W) writer structure"]
impl crate::Writable for CNDTR1 {}
#[doc = "DMA channel x number of data register"]
pub mod cndtr1;
#[doc = "DMA channel x number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndtr2](cndtr2) module"]
pub type CNDTR2 = crate::Reg<u32, _CNDTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDTR2;
#[doc = "`read()` method returns [cndtr2::R](cndtr2::R) reader structure"]
impl crate::Readable for CNDTR2 {}
#[doc = "`write(|w| ..)` method takes [cndtr2::W](cndtr2::W) writer structure"]
impl crate::Writable for CNDTR2 {}
#[doc = "DMA channel x number of data register"]
pub mod cndtr2;
#[doc = "DMA channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndtr3](cndtr3) module"]
pub type CNDTR3 = crate::Reg<u32, _CNDTR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDTR3;
#[doc = "`read()` method returns [cndtr3::R](cndtr3::R) reader structure"]
impl crate::Readable for CNDTR3 {}
#[doc = "`write(|w| ..)` method takes [cndtr3::W](cndtr3::W) writer structure"]
impl crate::Writable for CNDTR3 {}
#[doc = "DMA channel x configuration register"]
pub mod cndtr3;
#[doc = "DMA channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndtr4](cndtr4) module"]
pub type CNDTR4 = crate::Reg<u32, _CNDTR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDTR4;
#[doc = "`read()` method returns [cndtr4::R](cndtr4::R) reader structure"]
impl crate::Readable for CNDTR4 {}
#[doc = "`write(|w| ..)` method takes [cndtr4::W](cndtr4::W) writer structure"]
impl crate::Writable for CNDTR4 {}
#[doc = "DMA channel x configuration register"]
pub mod cndtr4;
#[doc = "DMA channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndtr5](cndtr5) module"]
pub type CNDTR5 = crate::Reg<u32, _CNDTR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDTR5;
#[doc = "`read()` method returns [cndtr5::R](cndtr5::R) reader structure"]
impl crate::Readable for CNDTR5 {}
#[doc = "`write(|w| ..)` method takes [cndtr5::W](cndtr5::W) writer structure"]
impl crate::Writable for CNDTR5 {}
#[doc = "DMA channel x configuration register"]
pub mod cndtr5;
#[doc = "DMA channel x peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpar1](cpar1) module"]
pub type CPAR1 = crate::Reg<u32, _CPAR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPAR1;
#[doc = "`read()` method returns [cpar1::R](cpar1::R) reader structure"]
impl crate::Readable for CPAR1 {}
#[doc = "`write(|w| ..)` method takes [cpar1::W](cpar1::W) writer structure"]
impl crate::Writable for CPAR1 {}
#[doc = "DMA channel x peripheral address register"]
pub mod cpar1;
#[doc = "DMA channel x peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpar2](cpar2) module"]
pub type CPAR2 = crate::Reg<u32, _CPAR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPAR2;
#[doc = "`read()` method returns [cpar2::R](cpar2::R) reader structure"]
impl crate::Readable for CPAR2 {}
#[doc = "`write(|w| ..)` method takes [cpar2::W](cpar2::W) writer structure"]
impl crate::Writable for CPAR2 {}
#[doc = "DMA channel x peripheral address register"]
pub mod cpar2;
#[doc = "DMA channel x peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpar3](cpar3) module"]
pub type CPAR3 = crate::Reg<u32, _CPAR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPAR3;
#[doc = "`read()` method returns [cpar3::R](cpar3::R) reader structure"]
impl crate::Readable for CPAR3 {}
#[doc = "`write(|w| ..)` method takes [cpar3::W](cpar3::W) writer structure"]
impl crate::Writable for CPAR3 {}
#[doc = "DMA channel x peripheral address register"]
pub mod cpar3;
#[doc = "DMA channel x peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpar4](cpar4) module"]
pub type CPAR4 = crate::Reg<u32, _CPAR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPAR4;
#[doc = "`read()` method returns [cpar4::R](cpar4::R) reader structure"]
impl crate::Readable for CPAR4 {}
#[doc = "`write(|w| ..)` method takes [cpar4::W](cpar4::W) writer structure"]
impl crate::Writable for CPAR4 {}
#[doc = "DMA channel x peripheral address register"]
pub mod cpar4;
#[doc = "DMA channel x peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpar5](cpar5) module"]
pub type CPAR5 = crate::Reg<u32, _CPAR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPAR5;
#[doc = "`read()` method returns [cpar5::R](cpar5::R) reader structure"]
impl crate::Readable for CPAR5 {}
#[doc = "`write(|w| ..)` method takes [cpar5::W](cpar5::W) writer structure"]
impl crate::Writable for CPAR5 {}
#[doc = "DMA channel x peripheral address register"]
pub mod cpar5;
#[doc = "DMA channel x memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmar1](cmar1) module"]
pub type CMAR1 = crate::Reg<u32, _CMAR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMAR1;
#[doc = "`read()` method returns [cmar1::R](cmar1::R) reader structure"]
impl crate::Readable for CMAR1 {}
#[doc = "`write(|w| ..)` method takes [cmar1::W](cmar1::W) writer structure"]
impl crate::Writable for CMAR1 {}
#[doc = "DMA channel x memory address register"]
pub mod cmar1;
#[doc = "DMA channel x memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmar2](cmar2) module"]
pub type CMAR2 = crate::Reg<u32, _CMAR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMAR2;
#[doc = "`read()` method returns [cmar2::R](cmar2::R) reader structure"]
impl crate::Readable for CMAR2 {}
#[doc = "`write(|w| ..)` method takes [cmar2::W](cmar2::W) writer structure"]
impl crate::Writable for CMAR2 {}
#[doc = "DMA channel x memory address register"]
pub mod cmar2;
#[doc = "DMA channel x memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmar3](cmar3) module"]
pub type CMAR3 = crate::Reg<u32, _CMAR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMAR3;
#[doc = "`read()` method returns [cmar3::R](cmar3::R) reader structure"]
impl crate::Readable for CMAR3 {}
#[doc = "`write(|w| ..)` method takes [cmar3::W](cmar3::W) writer structure"]
impl crate::Writable for CMAR3 {}
#[doc = "DMA channel x memory address register"]
pub mod cmar3;
#[doc = "DMA channel x memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmar4](cmar4) module"]
pub type CMAR4 = crate::Reg<u32, _CMAR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMAR4;
#[doc = "`read()` method returns [cmar4::R](cmar4::R) reader structure"]
impl crate::Readable for CMAR4 {}
#[doc = "`write(|w| ..)` method takes [cmar4::W](cmar4::W) writer structure"]
impl crate::Writable for CMAR4 {}
#[doc = "DMA channel x memory address register"]
pub mod cmar4;
#[doc = "DMA channel x memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmar5](cmar5) module"]
pub type CMAR5 = crate::Reg<u32, _CMAR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMAR5;
#[doc = "`read()` method returns [cmar5::R](cmar5::R) reader structure"]
impl crate::Readable for CMAR5 {}
#[doc = "`write(|w| ..)` method takes [cmar5::W](cmar5::W) writer structure"]
impl crate::Writable for CMAR5 {}
#[doc = "DMA channel x memory address register"]
pub mod cmar5;
