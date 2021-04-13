#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - memory remap register"]
    pub memrmp: MEMRMP,
    #[doc = "0x04 - configuration register 1"]
    pub cfgr1: CFGR1,
    #[doc = "0x08 - external interrupt configuration register 1"]
    pub exticr1: EXTICR1,
    #[doc = "0x0c - external interrupt configuration register 2"]
    pub exticr2: EXTICR2,
    #[doc = "0x10 - external interrupt configuration register 3"]
    pub exticr3: EXTICR3,
    #[doc = "0x14 - external interrupt configuration register 4"]
    pub exticr4: EXTICR4,
    #[doc = "0x18 - SCSR"]
    pub scsr: SCSR,
    #[doc = "0x1c - CFGR2"]
    pub cfgr2: CFGR2,
    #[doc = "0x20 - SRAM2 write protection register"]
    pub swpr: SWPR,
    #[doc = "0x24 - SKR"]
    pub skr: SKR,
    #[doc = "0x28 - SRAM2 write protection register 2"]
    pub swpr2: SWPR2,
    #[doc = "0x2c - CPU1 interrupt mask register 1"]
    pub imr1: IMR1,
    #[doc = "0x30 - CPU1 interrupt mask register 2"]
    pub imr2: IMR2,
    #[doc = "0x34 - CPU2 interrupt mask register 1"]
    pub c2imr1: C2IMR1,
    #[doc = "0x38 - CPU2 interrupt mask register 1"]
    pub c2imr2: C2IMR2,
    #[doc = "0x3c - secure IP control register"]
    pub sipcr: SIPCR,
}
#[doc = "memory remap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memrmp](memrmp) module"]
pub type MEMRMP = crate::Reg<u32, _MEMRMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEMRMP;
#[doc = "`read()` method returns [memrmp::R](memrmp::R) reader structure"]
impl crate::Readable for MEMRMP {}
#[doc = "`write(|w| ..)` method takes [memrmp::W](memrmp::W) writer structure"]
impl crate::Writable for MEMRMP {}
#[doc = "memory remap register"]
pub mod memrmp;
#[doc = "configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr1](cfgr1) module"]
pub type CFGR1 = crate::Reg<u32, _CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR1;
#[doc = "`read()` method returns [cfgr1::R](cfgr1::R) reader structure"]
impl crate::Readable for CFGR1 {}
#[doc = "`write(|w| ..)` method takes [cfgr1::W](cfgr1::W) writer structure"]
impl crate::Writable for CFGR1 {}
#[doc = "configuration register 1"]
pub mod cfgr1;
#[doc = "external interrupt configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr1](exticr1) module"]
pub type EXTICR1 = crate::Reg<u32, _EXTICR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR1;
#[doc = "`read()` method returns [exticr1::R](exticr1::R) reader structure"]
impl crate::Readable for EXTICR1 {}
#[doc = "`write(|w| ..)` method takes [exticr1::W](exticr1::W) writer structure"]
impl crate::Writable for EXTICR1 {}
#[doc = "external interrupt configuration register 1"]
pub mod exticr1;
#[doc = "external interrupt configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr2](exticr2) module"]
pub type EXTICR2 = crate::Reg<u32, _EXTICR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR2;
#[doc = "`read()` method returns [exticr2::R](exticr2::R) reader structure"]
impl crate::Readable for EXTICR2 {}
#[doc = "`write(|w| ..)` method takes [exticr2::W](exticr2::W) writer structure"]
impl crate::Writable for EXTICR2 {}
#[doc = "external interrupt configuration register 2"]
pub mod exticr2;
#[doc = "external interrupt configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr3](exticr3) module"]
pub type EXTICR3 = crate::Reg<u32, _EXTICR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR3;
#[doc = "`read()` method returns [exticr3::R](exticr3::R) reader structure"]
impl crate::Readable for EXTICR3 {}
#[doc = "`write(|w| ..)` method takes [exticr3::W](exticr3::W) writer structure"]
impl crate::Writable for EXTICR3 {}
#[doc = "external interrupt configuration register 3"]
pub mod exticr3;
#[doc = "external interrupt configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr4](exticr4) module"]
pub type EXTICR4 = crate::Reg<u32, _EXTICR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR4;
#[doc = "`read()` method returns [exticr4::R](exticr4::R) reader structure"]
impl crate::Readable for EXTICR4 {}
#[doc = "`write(|w| ..)` method takes [exticr4::W](exticr4::W) writer structure"]
impl crate::Writable for EXTICR4 {}
#[doc = "external interrupt configuration register 4"]
pub mod exticr4;
#[doc = "SCSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scsr](scsr) module"]
pub type SCSR = crate::Reg<u32, _SCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCSR;
#[doc = "`read()` method returns [scsr::R](scsr::R) reader structure"]
impl crate::Readable for SCSR {}
#[doc = "`write(|w| ..)` method takes [scsr::W](scsr::W) writer structure"]
impl crate::Writable for SCSR {}
#[doc = "SCSR"]
pub mod scsr;
#[doc = "CFGR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](cfgr2) module"]
pub type CFGR2 = crate::Reg<u32, _CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR2;
#[doc = "`read()` method returns [cfgr2::R](cfgr2::R) reader structure"]
impl crate::Readable for CFGR2 {}
#[doc = "`write(|w| ..)` method takes [cfgr2::W](cfgr2::W) writer structure"]
impl crate::Writable for CFGR2 {}
#[doc = "CFGR2"]
pub mod cfgr2;
#[doc = "SRAM2 write protection register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swpr](swpr) module"]
pub type SWPR = crate::Reg<u32, _SWPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWPR;
#[doc = "`write(|w| ..)` method takes [swpr::W](swpr::W) writer structure"]
impl crate::Writable for SWPR {}
#[doc = "SRAM2 write protection register"]
pub mod swpr;
#[doc = "SKR\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [skr](skr) module"]
pub type SKR = crate::Reg<u32, _SKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SKR;
#[doc = "`write(|w| ..)` method takes [skr::W](skr::W) writer structure"]
impl crate::Writable for SKR {}
#[doc = "SKR"]
pub mod skr;
#[doc = "SRAM2 write protection register 2\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swpr2](swpr2) module"]
pub type SWPR2 = crate::Reg<u32, _SWPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWPR2;
#[doc = "`write(|w| ..)` method takes [swpr2::W](swpr2::W) writer structure"]
impl crate::Writable for SWPR2 {}
#[doc = "SRAM2 write protection register 2"]
pub mod swpr2;
#[doc = "CPU1 interrupt mask register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr1](imr1) module"]
pub type IMR1 = crate::Reg<u32, _IMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR1;
#[doc = "`read()` method returns [imr1::R](imr1::R) reader structure"]
impl crate::Readable for IMR1 {}
#[doc = "`write(|w| ..)` method takes [imr1::W](imr1::W) writer structure"]
impl crate::Writable for IMR1 {}
#[doc = "CPU1 interrupt mask register 1"]
pub mod imr1;
#[doc = "CPU1 interrupt mask register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr2](imr2) module"]
pub type IMR2 = crate::Reg<u32, _IMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR2;
#[doc = "`read()` method returns [imr2::R](imr2::R) reader structure"]
impl crate::Readable for IMR2 {}
#[doc = "`write(|w| ..)` method takes [imr2::W](imr2::W) writer structure"]
impl crate::Writable for IMR2 {}
#[doc = "CPU1 interrupt mask register 2"]
pub mod imr2;
#[doc = "CPU2 interrupt mask register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2imr1](c2imr1) module"]
pub type C2IMR1 = crate::Reg<u32, _C2IMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2IMR1;
#[doc = "`read()` method returns [c2imr1::R](c2imr1::R) reader structure"]
impl crate::Readable for C2IMR1 {}
#[doc = "`write(|w| ..)` method takes [c2imr1::W](c2imr1::W) writer structure"]
impl crate::Writable for C2IMR1 {}
#[doc = "CPU2 interrupt mask register 1"]
pub mod c2imr1;
#[doc = "CPU2 interrupt mask register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2imr2](c2imr2) module"]
pub type C2IMR2 = crate::Reg<u32, _C2IMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2IMR2;
#[doc = "`read()` method returns [c2imr2::R](c2imr2::R) reader structure"]
impl crate::Readable for C2IMR2 {}
#[doc = "`write(|w| ..)` method takes [c2imr2::W](c2imr2::W) writer structure"]
impl crate::Writable for C2IMR2 {}
#[doc = "CPU2 interrupt mask register 1"]
pub mod c2imr2;
#[doc = "secure IP control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sipcr](sipcr) module"]
pub type SIPCR = crate::Reg<u32, _SIPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIPCR;
#[doc = "`read()` method returns [sipcr::R](sipcr::R) reader structure"]
impl crate::Readable for SIPCR {}
#[doc = "`write(|w| ..)` method takes [sipcr::W](sipcr::W) writer structure"]
impl crate::Writable for SIPCR {}
#[doc = "secure IP control register"]
pub mod sipcr;
