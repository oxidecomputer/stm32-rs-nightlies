#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    pub cr: CR,
    #[doc = "0x04 - Internal clock sources calibration register"]
    pub icscr: ICSCR,
    #[doc = "0x08 - Clock configuration register"]
    pub cfgr: CFGR,
    #[doc = "0x0c - PLL configuration register"]
    pub pllcfgr: PLLCFGR,
    _reserved4: [u8; 8usize],
    #[doc = "0x18 - Clock interrupt enable register"]
    pub cier: CIER,
    #[doc = "0x1c - Clock interrupt flag register"]
    pub cifr: CIFR,
    #[doc = "0x20 - Clock interrupt clear register"]
    pub cicr: CICR,
    _reserved7: [u8; 4usize],
    #[doc = "0x28 - AHB1 peripheral reset register"]
    pub ahb1rstr: AHB1RSTR,
    #[doc = "0x2c - AHB2 peripheral reset register"]
    pub ahb2rstr: AHB2RSTR,
    #[doc = "0x30 - AHB3 peripheral reset register"]
    pub ahb3rstr: AHB3RSTR,
    _reserved10: [u8; 4usize],
    #[doc = "0x38 - APB1 peripheral reset register 1"]
    pub apb1rstr1: APB1RSTR1,
    #[doc = "0x3c - APB1 peripheral reset register 2"]
    pub apb1rstr2: APB1RSTR2,
    #[doc = "0x40 - APB2 peripheral reset register"]
    pub apb2rstr: APB2RSTR,
    #[doc = "0x44 - APB3 peripheral reset register"]
    pub apb3rstr: APB3RSTR,
    #[doc = "0x48 - AHB1 peripheral clock enable register"]
    pub ahb1enr: AHB1ENR,
    #[doc = "0x4c - AHB2 peripheral clock enable register"]
    pub ahb2enr: AHB2ENR,
    #[doc = "0x50 - AHB3 peripheral clock enable register"]
    pub ahb3enr: AHB3ENR,
    _reserved17: [u8; 4usize],
    #[doc = "0x58 - APB1 peripheral clock enable register 1"]
    pub apb1enr1: APB1ENR1,
    #[doc = "0x5c - APB1 peripheral clock enable register 2"]
    pub apb1enr2: APB1ENR2,
    #[doc = "0x60 - APB2 peripheral clock enable register"]
    pub apb2enr: APB2ENR,
    #[doc = "0x64 - APB3 peripheral clock enable register"]
    pub apb3enr: APB3ENR,
    #[doc = "0x68 - AHB1 peripheral clocks enable in Sleep modes register"]
    pub ahb1smenr: AHB1SMENR,
    #[doc = "0x6c - AHB2 peripheral clocks enable in Sleep modes register"]
    pub ahb2smenr: AHB2SMENR,
    #[doc = "0x70 - AHB3 peripheral clocks enable in Sleep and Stop modes register"]
    pub ahb3smenr: AHB3SMENR,
    _reserved24: [u8; 4usize],
    #[doc = "0x78 - APB1 peripheral clocks enable in Sleep mode register 1"]
    pub apb1smenr1: APB1SMENR1,
    #[doc = "0x7c - APB1 peripheral clocks enable in Sleep mode register 2"]
    pub apb1smenr2: APB1SMENR2,
    #[doc = "0x80 - APB2 peripheral clocks enable in Sleep mode register"]
    pub apb2smenr: APB2SMENR,
    #[doc = "0x84 - APB3 peripheral clock enable in Sleep mode register"]
    pub apb3smenr: APB3SMENR,
    #[doc = "0x88 - Peripherals independent clock configuration register"]
    pub ccipr: CCIPR,
    _reserved29: [u8; 4usize],
    #[doc = "0x90 - Backup domain control register"]
    pub bdcr: BDCR,
    #[doc = "0x94 - Control/status register"]
    pub csr: CSR,
    _reserved31: [u8; 112usize],
    #[doc = "0x108 - Extended clock recovery register"]
    pub extcfgr: EXTCFGR,
    _reserved32: [u8; 60usize],
    #[doc = "0x148 - CPU2 AHB1 peripheral clock enable register"]
    pub c2ahb1enr: C2AHB1ENR,
    #[doc = "0x14c - CPU2 AHB2 peripheral clock enable register"]
    pub c2ahb2enr: C2AHB2ENR,
    #[doc = "0x150 - CPU2 AHB3 peripheral clock enable register \\[dual core device only\\]"]
    pub c2ahb3enr: C2AHB3ENR,
    _reserved35: [u8; 4usize],
    #[doc = "0x158 - CPU2 APB1 peripheral clock enable register 1 \\[dual core device only\\]"]
    pub c2apb1enr1: C2APB1ENR1,
    #[doc = "0x15c - CPU2 APB1 peripheral clock enable register 2 \\[dual core device only\\]"]
    pub c2apb1enr2: C2APB1ENR2,
    #[doc = "0x160 - CPU2 APB2 peripheral clock enable register \\[dual core device only\\]"]
    pub c2apb2enr: C2APB2ENR,
    #[doc = "0x164 - CPU2 APB3 peripheral clock enable register \\[dual core device only\\]"]
    pub c2apb3enr: C2APB3ENR,
    #[doc = "0x168 - CPU2 AHB1 peripheral clocks enable in Sleep modes register \\[dual core device only\\]"]
    pub c2ahb1smenr: C2AHB1SMENR,
    #[doc = "0x16c - CPU2 AHB2 peripheral clocks enable in Sleep modes register \\[dual core device only\\]"]
    pub c2ahb2smenr: C2AHB2SMENR,
    #[doc = "0x170 - CPU2 AHB3 peripheral clocks enable in Sleep mode register \\[dual core device only\\]"]
    pub c2ahb3smenr: C2AHB3SMENR,
    _reserved42: [u8; 4usize],
    #[doc = "0x178 - CPU2 APB1 peripheral clocks enable in Sleep mode register 1 \\[dual core device only\\]"]
    pub c2apb1smenr1: C2APB1SMENR1,
    #[doc = "0x17c - CPU2 APB1 peripheral clocks enable in Sleep mode register 2 \\[dual core device only\\]"]
    pub c2apb1smenr2: C2APB1SMENR2,
    #[doc = "0x180 - CPU2 APB2 peripheral clocks enable in Sleep mode register \\[dual core device only\\]"]
    pub c2apb2smenr: C2APB2SMENR,
    #[doc = "0x184 - CPU2 APB3 peripheral clock enable in Sleep mode register \\[dual core device only\\]"]
    pub c2apb3smenr: C2APB3SMENR,
}
#[doc = "Clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Clock control register"]
pub mod cr;
#[doc = "Internal clock sources calibration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icscr](icscr) module"]
pub type ICSCR = crate::Reg<u32, _ICSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICSCR;
#[doc = "`read()` method returns [icscr::R](icscr::R) reader structure"]
impl crate::Readable for ICSCR {}
#[doc = "`write(|w| ..)` method takes [icscr::W](icscr::W) writer structure"]
impl crate::Writable for ICSCR {}
#[doc = "Internal clock sources calibration register"]
pub mod icscr;
#[doc = "Clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](cfgr) module"]
pub type CFGR = crate::Reg<u32, _CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR;
#[doc = "`read()` method returns [cfgr::R](cfgr::R) reader structure"]
impl crate::Readable for CFGR {}
#[doc = "`write(|w| ..)` method takes [cfgr::W](cfgr::W) writer structure"]
impl crate::Writable for CFGR {}
#[doc = "Clock configuration register"]
pub mod cfgr;
#[doc = "PLL configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcfgr](pllcfgr) module"]
pub type PLLCFGR = crate::Reg<u32, _PLLCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLLCFGR;
#[doc = "`read()` method returns [pllcfgr::R](pllcfgr::R) reader structure"]
impl crate::Readable for PLLCFGR {}
#[doc = "`write(|w| ..)` method takes [pllcfgr::W](pllcfgr::W) writer structure"]
impl crate::Writable for PLLCFGR {}
#[doc = "PLL configuration register"]
pub mod pllcfgr;
#[doc = "Clock interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cier](cier) module"]
pub type CIER = crate::Reg<u32, _CIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIER;
#[doc = "`read()` method returns [cier::R](cier::R) reader structure"]
impl crate::Readable for CIER {}
#[doc = "`write(|w| ..)` method takes [cier::W](cier::W) writer structure"]
impl crate::Writable for CIER {}
#[doc = "Clock interrupt enable register"]
pub mod cier;
#[doc = "Clock interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cifr](cifr) module"]
pub type CIFR = crate::Reg<u32, _CIFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIFR;
#[doc = "`read()` method returns [cifr::R](cifr::R) reader structure"]
impl crate::Readable for CIFR {}
#[doc = "Clock interrupt flag register"]
pub mod cifr;
#[doc = "Clock interrupt clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cicr](cicr) module"]
pub type CICR = crate::Reg<u32, _CICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CICR;
#[doc = "`write(|w| ..)` method takes [cicr::W](cicr::W) writer structure"]
impl crate::Writable for CICR {}
#[doc = "Clock interrupt clear register"]
pub mod cicr;
#[doc = "AHB1 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb1rstr](ahb1rstr) module"]
pub type AHB1RSTR = crate::Reg<u32, _AHB1RSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB1RSTR;
#[doc = "`read()` method returns [ahb1rstr::R](ahb1rstr::R) reader structure"]
impl crate::Readable for AHB1RSTR {}
#[doc = "`write(|w| ..)` method takes [ahb1rstr::W](ahb1rstr::W) writer structure"]
impl crate::Writable for AHB1RSTR {}
#[doc = "AHB1 peripheral reset register"]
pub mod ahb1rstr;
#[doc = "AHB2 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2rstr](ahb2rstr) module"]
pub type AHB2RSTR = crate::Reg<u32, _AHB2RSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB2RSTR;
#[doc = "`read()` method returns [ahb2rstr::R](ahb2rstr::R) reader structure"]
impl crate::Readable for AHB2RSTR {}
#[doc = "`write(|w| ..)` method takes [ahb2rstr::W](ahb2rstr::W) writer structure"]
impl crate::Writable for AHB2RSTR {}
#[doc = "AHB2 peripheral reset register"]
pub mod ahb2rstr;
#[doc = "AHB3 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb3rstr](ahb3rstr) module"]
pub type AHB3RSTR = crate::Reg<u32, _AHB3RSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB3RSTR;
#[doc = "`read()` method returns [ahb3rstr::R](ahb3rstr::R) reader structure"]
impl crate::Readable for AHB3RSTR {}
#[doc = "`write(|w| ..)` method takes [ahb3rstr::W](ahb3rstr::W) writer structure"]
impl crate::Writable for AHB3RSTR {}
#[doc = "AHB3 peripheral reset register"]
pub mod ahb3rstr;
#[doc = "APB1 peripheral reset register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1rstr1](apb1rstr1) module"]
pub type APB1RSTR1 = crate::Reg<u32, _APB1RSTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1RSTR1;
#[doc = "`read()` method returns [apb1rstr1::R](apb1rstr1::R) reader structure"]
impl crate::Readable for APB1RSTR1 {}
#[doc = "`write(|w| ..)` method takes [apb1rstr1::W](apb1rstr1::W) writer structure"]
impl crate::Writable for APB1RSTR1 {}
#[doc = "APB1 peripheral reset register 1"]
pub mod apb1rstr1;
#[doc = "APB1 peripheral reset register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1rstr2](apb1rstr2) module"]
pub type APB1RSTR2 = crate::Reg<u32, _APB1RSTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1RSTR2;
#[doc = "`read()` method returns [apb1rstr2::R](apb1rstr2::R) reader structure"]
impl crate::Readable for APB1RSTR2 {}
#[doc = "`write(|w| ..)` method takes [apb1rstr2::W](apb1rstr2::W) writer structure"]
impl crate::Writable for APB1RSTR2 {}
#[doc = "APB1 peripheral reset register 2"]
pub mod apb1rstr2;
#[doc = "APB2 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2rstr](apb2rstr) module"]
pub type APB2RSTR = crate::Reg<u32, _APB2RSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2RSTR;
#[doc = "`read()` method returns [apb2rstr::R](apb2rstr::R) reader structure"]
impl crate::Readable for APB2RSTR {}
#[doc = "`write(|w| ..)` method takes [apb2rstr::W](apb2rstr::W) writer structure"]
impl crate::Writable for APB2RSTR {}
#[doc = "APB2 peripheral reset register"]
pub mod apb2rstr;
#[doc = "APB3 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb3rstr](apb3rstr) module"]
pub type APB3RSTR = crate::Reg<u32, _APB3RSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB3RSTR;
#[doc = "`read()` method returns [apb3rstr::R](apb3rstr::R) reader structure"]
impl crate::Readable for APB3RSTR {}
#[doc = "`write(|w| ..)` method takes [apb3rstr::W](apb3rstr::W) writer structure"]
impl crate::Writable for APB3RSTR {}
#[doc = "APB3 peripheral reset register"]
pub mod apb3rstr;
#[doc = "AHB1 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb1enr](ahb1enr) module"]
pub type AHB1ENR = crate::Reg<u32, _AHB1ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB1ENR;
#[doc = "`read()` method returns [ahb1enr::R](ahb1enr::R) reader structure"]
impl crate::Readable for AHB1ENR {}
#[doc = "`write(|w| ..)` method takes [ahb1enr::W](ahb1enr::W) writer structure"]
impl crate::Writable for AHB1ENR {}
#[doc = "AHB1 peripheral clock enable register"]
pub mod ahb1enr;
#[doc = "AHB2 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2enr](ahb2enr) module"]
pub type AHB2ENR = crate::Reg<u32, _AHB2ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB2ENR;
#[doc = "`read()` method returns [ahb2enr::R](ahb2enr::R) reader structure"]
impl crate::Readable for AHB2ENR {}
#[doc = "`write(|w| ..)` method takes [ahb2enr::W](ahb2enr::W) writer structure"]
impl crate::Writable for AHB2ENR {}
#[doc = "AHB2 peripheral clock enable register"]
pub mod ahb2enr;
#[doc = "AHB3 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb3enr](ahb3enr) module"]
pub type AHB3ENR = crate::Reg<u32, _AHB3ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB3ENR;
#[doc = "`read()` method returns [ahb3enr::R](ahb3enr::R) reader structure"]
impl crate::Readable for AHB3ENR {}
#[doc = "`write(|w| ..)` method takes [ahb3enr::W](ahb3enr::W) writer structure"]
impl crate::Writable for AHB3ENR {}
#[doc = "AHB3 peripheral clock enable register"]
pub mod ahb3enr;
#[doc = "APB1 peripheral clock enable register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1enr1](apb1enr1) module"]
pub type APB1ENR1 = crate::Reg<u32, _APB1ENR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1ENR1;
#[doc = "`read()` method returns [apb1enr1::R](apb1enr1::R) reader structure"]
impl crate::Readable for APB1ENR1 {}
#[doc = "`write(|w| ..)` method takes [apb1enr1::W](apb1enr1::W) writer structure"]
impl crate::Writable for APB1ENR1 {}
#[doc = "APB1 peripheral clock enable register 1"]
pub mod apb1enr1;
#[doc = "APB1 peripheral clock enable register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1enr2](apb1enr2) module"]
pub type APB1ENR2 = crate::Reg<u32, _APB1ENR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1ENR2;
#[doc = "`read()` method returns [apb1enr2::R](apb1enr2::R) reader structure"]
impl crate::Readable for APB1ENR2 {}
#[doc = "`write(|w| ..)` method takes [apb1enr2::W](apb1enr2::W) writer structure"]
impl crate::Writable for APB1ENR2 {}
#[doc = "APB1 peripheral clock enable register 2"]
pub mod apb1enr2;
#[doc = "APB2 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2enr](apb2enr) module"]
pub type APB2ENR = crate::Reg<u32, _APB2ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2ENR;
#[doc = "`read()` method returns [apb2enr::R](apb2enr::R) reader structure"]
impl crate::Readable for APB2ENR {}
#[doc = "`write(|w| ..)` method takes [apb2enr::W](apb2enr::W) writer structure"]
impl crate::Writable for APB2ENR {}
#[doc = "APB2 peripheral clock enable register"]
pub mod apb2enr;
#[doc = "APB3 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb3enr](apb3enr) module"]
pub type APB3ENR = crate::Reg<u32, _APB3ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB3ENR;
#[doc = "`read()` method returns [apb3enr::R](apb3enr::R) reader structure"]
impl crate::Readable for APB3ENR {}
#[doc = "`write(|w| ..)` method takes [apb3enr::W](apb3enr::W) writer structure"]
impl crate::Writable for APB3ENR {}
#[doc = "APB3 peripheral clock enable register"]
pub mod apb3enr;
#[doc = "AHB1 peripheral clocks enable in Sleep modes register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb1smenr](ahb1smenr) module"]
pub type AHB1SMENR = crate::Reg<u32, _AHB1SMENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB1SMENR;
#[doc = "`read()` method returns [ahb1smenr::R](ahb1smenr::R) reader structure"]
impl crate::Readable for AHB1SMENR {}
#[doc = "`write(|w| ..)` method takes [ahb1smenr::W](ahb1smenr::W) writer structure"]
impl crate::Writable for AHB1SMENR {}
#[doc = "AHB1 peripheral clocks enable in Sleep modes register"]
pub mod ahb1smenr;
#[doc = "AHB2 peripheral clocks enable in Sleep modes register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2smenr](ahb2smenr) module"]
pub type AHB2SMENR = crate::Reg<u32, _AHB2SMENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB2SMENR;
#[doc = "`read()` method returns [ahb2smenr::R](ahb2smenr::R) reader structure"]
impl crate::Readable for AHB2SMENR {}
#[doc = "`write(|w| ..)` method takes [ahb2smenr::W](ahb2smenr::W) writer structure"]
impl crate::Writable for AHB2SMENR {}
#[doc = "AHB2 peripheral clocks enable in Sleep modes register"]
pub mod ahb2smenr;
#[doc = "AHB3 peripheral clocks enable in Sleep and Stop modes register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb3smenr](ahb3smenr) module"]
pub type AHB3SMENR = crate::Reg<u32, _AHB3SMENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB3SMENR;
#[doc = "`read()` method returns [ahb3smenr::R](ahb3smenr::R) reader structure"]
impl crate::Readable for AHB3SMENR {}
#[doc = "`write(|w| ..)` method takes [ahb3smenr::W](ahb3smenr::W) writer structure"]
impl crate::Writable for AHB3SMENR {}
#[doc = "AHB3 peripheral clocks enable in Sleep and Stop modes register"]
pub mod ahb3smenr;
#[doc = "APB1 peripheral clocks enable in Sleep mode register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1smenr1](apb1smenr1) module"]
pub type APB1SMENR1 = crate::Reg<u32, _APB1SMENR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1SMENR1;
#[doc = "`read()` method returns [apb1smenr1::R](apb1smenr1::R) reader structure"]
impl crate::Readable for APB1SMENR1 {}
#[doc = "`write(|w| ..)` method takes [apb1smenr1::W](apb1smenr1::W) writer structure"]
impl crate::Writable for APB1SMENR1 {}
#[doc = "APB1 peripheral clocks enable in Sleep mode register 1"]
pub mod apb1smenr1;
#[doc = "APB1 peripheral clocks enable in Sleep mode register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1smenr2](apb1smenr2) module"]
pub type APB1SMENR2 = crate::Reg<u32, _APB1SMENR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1SMENR2;
#[doc = "`read()` method returns [apb1smenr2::R](apb1smenr2::R) reader structure"]
impl crate::Readable for APB1SMENR2 {}
#[doc = "`write(|w| ..)` method takes [apb1smenr2::W](apb1smenr2::W) writer structure"]
impl crate::Writable for APB1SMENR2 {}
#[doc = "APB1 peripheral clocks enable in Sleep mode register 2"]
pub mod apb1smenr2;
#[doc = "APB2 peripheral clocks enable in Sleep mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2smenr](apb2smenr) module"]
pub type APB2SMENR = crate::Reg<u32, _APB2SMENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2SMENR;
#[doc = "`read()` method returns [apb2smenr::R](apb2smenr::R) reader structure"]
impl crate::Readable for APB2SMENR {}
#[doc = "`write(|w| ..)` method takes [apb2smenr::W](apb2smenr::W) writer structure"]
impl crate::Writable for APB2SMENR {}
#[doc = "APB2 peripheral clocks enable in Sleep mode register"]
pub mod apb2smenr;
#[doc = "APB3 peripheral clock enable in Sleep mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb3smenr](apb3smenr) module"]
pub type APB3SMENR = crate::Reg<u32, _APB3SMENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB3SMENR;
#[doc = "`read()` method returns [apb3smenr::R](apb3smenr::R) reader structure"]
impl crate::Readable for APB3SMENR {}
#[doc = "`write(|w| ..)` method takes [apb3smenr::W](apb3smenr::W) writer structure"]
impl crate::Writable for APB3SMENR {}
#[doc = "APB3 peripheral clock enable in Sleep mode register"]
pub mod apb3smenr;
#[doc = "Peripherals independent clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccipr](ccipr) module"]
pub type CCIPR = crate::Reg<u32, _CCIPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCIPR;
#[doc = "`read()` method returns [ccipr::R](ccipr::R) reader structure"]
impl crate::Readable for CCIPR {}
#[doc = "`write(|w| ..)` method takes [ccipr::W](ccipr::W) writer structure"]
impl crate::Writable for CCIPR {}
#[doc = "Peripherals independent clock configuration register"]
pub mod ccipr;
#[doc = "Backup domain control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdcr](bdcr) module"]
pub type BDCR = crate::Reg<u32, _BDCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDCR;
#[doc = "`read()` method returns [bdcr::R](bdcr::R) reader structure"]
impl crate::Readable for BDCR {}
#[doc = "`write(|w| ..)` method takes [bdcr::W](bdcr::W) writer structure"]
impl crate::Writable for BDCR {}
#[doc = "Backup domain control register"]
pub mod bdcr;
#[doc = "Control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](csr) module"]
pub type CSR = crate::Reg<u32, _CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR;
#[doc = "`read()` method returns [csr::R](csr::R) reader structure"]
impl crate::Readable for CSR {}
#[doc = "`write(|w| ..)` method takes [csr::W](csr::W) writer structure"]
impl crate::Writable for CSR {}
#[doc = "Control/status register"]
pub mod csr;
#[doc = "Extended clock recovery register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extcfgr](extcfgr) module"]
pub type EXTCFGR = crate::Reg<u32, _EXTCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTCFGR;
#[doc = "`read()` method returns [extcfgr::R](extcfgr::R) reader structure"]
impl crate::Readable for EXTCFGR {}
#[doc = "`write(|w| ..)` method takes [extcfgr::W](extcfgr::W) writer structure"]
impl crate::Writable for EXTCFGR {}
#[doc = "Extended clock recovery register"]
pub mod extcfgr;
#[doc = "CPU2 AHB1 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2ahb1enr](c2ahb1enr) module"]
pub type C2AHB1ENR = crate::Reg<u32, _C2AHB1ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2AHB1ENR;
#[doc = "`read()` method returns [c2ahb1enr::R](c2ahb1enr::R) reader structure"]
impl crate::Readable for C2AHB1ENR {}
#[doc = "`write(|w| ..)` method takes [c2ahb1enr::W](c2ahb1enr::W) writer structure"]
impl crate::Writable for C2AHB1ENR {}
#[doc = "CPU2 AHB1 peripheral clock enable register"]
pub mod c2ahb1enr;
#[doc = "CPU2 AHB2 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2ahb2enr](c2ahb2enr) module"]
pub type C2AHB2ENR = crate::Reg<u32, _C2AHB2ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2AHB2ENR;
#[doc = "`read()` method returns [c2ahb2enr::R](c2ahb2enr::R) reader structure"]
impl crate::Readable for C2AHB2ENR {}
#[doc = "`write(|w| ..)` method takes [c2ahb2enr::W](c2ahb2enr::W) writer structure"]
impl crate::Writable for C2AHB2ENR {}
#[doc = "CPU2 AHB2 peripheral clock enable register"]
pub mod c2ahb2enr;
#[doc = "CPU2 AHB3 peripheral clock enable register \\[dual core device only\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2ahb3enr](c2ahb3enr) module"]
pub type C2AHB3ENR = crate::Reg<u32, _C2AHB3ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2AHB3ENR;
#[doc = "`read()` method returns [c2ahb3enr::R](c2ahb3enr::R) reader structure"]
impl crate::Readable for C2AHB3ENR {}
#[doc = "`write(|w| ..)` method takes [c2ahb3enr::W](c2ahb3enr::W) writer structure"]
impl crate::Writable for C2AHB3ENR {}
#[doc = "CPU2 AHB3 peripheral clock enable register \\[dual core device only\\]"]
pub mod c2ahb3enr;
#[doc = "CPU2 APB1 peripheral clock enable register 1 \\[dual core device only\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2apb1enr1](c2apb1enr1) module"]
pub type C2APB1ENR1 = crate::Reg<u32, _C2APB1ENR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2APB1ENR1;
#[doc = "`read()` method returns [c2apb1enr1::R](c2apb1enr1::R) reader structure"]
impl crate::Readable for C2APB1ENR1 {}
#[doc = "`write(|w| ..)` method takes [c2apb1enr1::W](c2apb1enr1::W) writer structure"]
impl crate::Writable for C2APB1ENR1 {}
#[doc = "CPU2 APB1 peripheral clock enable register 1 \\[dual core device only\\]"]
pub mod c2apb1enr1;
#[doc = "CPU2 APB1 peripheral clock enable register 2 \\[dual core device only\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2apb1enr2](c2apb1enr2) module"]
pub type C2APB1ENR2 = crate::Reg<u32, _C2APB1ENR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2APB1ENR2;
#[doc = "`read()` method returns [c2apb1enr2::R](c2apb1enr2::R) reader structure"]
impl crate::Readable for C2APB1ENR2 {}
#[doc = "`write(|w| ..)` method takes [c2apb1enr2::W](c2apb1enr2::W) writer structure"]
impl crate::Writable for C2APB1ENR2 {}
#[doc = "CPU2 APB1 peripheral clock enable register 2 \\[dual core device only\\]"]
pub mod c2apb1enr2;
#[doc = "CPU2 APB2 peripheral clock enable register \\[dual core device only\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2apb2enr](c2apb2enr) module"]
pub type C2APB2ENR = crate::Reg<u32, _C2APB2ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2APB2ENR;
#[doc = "`read()` method returns [c2apb2enr::R](c2apb2enr::R) reader structure"]
impl crate::Readable for C2APB2ENR {}
#[doc = "`write(|w| ..)` method takes [c2apb2enr::W](c2apb2enr::W) writer structure"]
impl crate::Writable for C2APB2ENR {}
#[doc = "CPU2 APB2 peripheral clock enable register \\[dual core device only\\]"]
pub mod c2apb2enr;
#[doc = "CPU2 APB3 peripheral clock enable register \\[dual core device only\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2apb3enr](c2apb3enr) module"]
pub type C2APB3ENR = crate::Reg<u32, _C2APB3ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2APB3ENR;
#[doc = "`read()` method returns [c2apb3enr::R](c2apb3enr::R) reader structure"]
impl crate::Readable for C2APB3ENR {}
#[doc = "`write(|w| ..)` method takes [c2apb3enr::W](c2apb3enr::W) writer structure"]
impl crate::Writable for C2APB3ENR {}
#[doc = "CPU2 APB3 peripheral clock enable register \\[dual core device only\\]"]
pub mod c2apb3enr;
#[doc = "CPU2 AHB1 peripheral clocks enable in Sleep modes register \\[dual core device only\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2ahb1smenr](c2ahb1smenr) module"]
pub type C2AHB1SMENR = crate::Reg<u32, _C2AHB1SMENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2AHB1SMENR;
#[doc = "`read()` method returns [c2ahb1smenr::R](c2ahb1smenr::R) reader structure"]
impl crate::Readable for C2AHB1SMENR {}
#[doc = "`write(|w| ..)` method takes [c2ahb1smenr::W](c2ahb1smenr::W) writer structure"]
impl crate::Writable for C2AHB1SMENR {}
#[doc = "CPU2 AHB1 peripheral clocks enable in Sleep modes register \\[dual core device only\\]"]
pub mod c2ahb1smenr;
#[doc = "CPU2 AHB2 peripheral clocks enable in Sleep modes register \\[dual core device only\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2ahb2smenr](c2ahb2smenr) module"]
pub type C2AHB2SMENR = crate::Reg<u32, _C2AHB2SMENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2AHB2SMENR;
#[doc = "`read()` method returns [c2ahb2smenr::R](c2ahb2smenr::R) reader structure"]
impl crate::Readable for C2AHB2SMENR {}
#[doc = "`write(|w| ..)` method takes [c2ahb2smenr::W](c2ahb2smenr::W) writer structure"]
impl crate::Writable for C2AHB2SMENR {}
#[doc = "CPU2 AHB2 peripheral clocks enable in Sleep modes register \\[dual core device only\\]"]
pub mod c2ahb2smenr;
#[doc = "CPU2 AHB3 peripheral clocks enable in Sleep mode register \\[dual core device only\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2ahb3smenr](c2ahb3smenr) module"]
pub type C2AHB3SMENR = crate::Reg<u32, _C2AHB3SMENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2AHB3SMENR;
#[doc = "`read()` method returns [c2ahb3smenr::R](c2ahb3smenr::R) reader structure"]
impl crate::Readable for C2AHB3SMENR {}
#[doc = "`write(|w| ..)` method takes [c2ahb3smenr::W](c2ahb3smenr::W) writer structure"]
impl crate::Writable for C2AHB3SMENR {}
#[doc = "CPU2 AHB3 peripheral clocks enable in Sleep mode register \\[dual core device only\\]"]
pub mod c2ahb3smenr;
#[doc = "CPU2 APB1 peripheral clocks enable in Sleep mode register 1 \\[dual core device only\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2apb1smenr1](c2apb1smenr1) module"]
pub type C2APB1SMENR1 = crate::Reg<u32, _C2APB1SMENR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2APB1SMENR1;
#[doc = "`read()` method returns [c2apb1smenr1::R](c2apb1smenr1::R) reader structure"]
impl crate::Readable for C2APB1SMENR1 {}
#[doc = "`write(|w| ..)` method takes [c2apb1smenr1::W](c2apb1smenr1::W) writer structure"]
impl crate::Writable for C2APB1SMENR1 {}
#[doc = "CPU2 APB1 peripheral clocks enable in Sleep mode register 1 \\[dual core device only\\]"]
pub mod c2apb1smenr1;
#[doc = "CPU2 APB1 peripheral clocks enable in Sleep mode register 2 \\[dual core device only\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2apb1smenr2](c2apb1smenr2) module"]
pub type C2APB1SMENR2 = crate::Reg<u32, _C2APB1SMENR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2APB1SMENR2;
#[doc = "`read()` method returns [c2apb1smenr2::R](c2apb1smenr2::R) reader structure"]
impl crate::Readable for C2APB1SMENR2 {}
#[doc = "`write(|w| ..)` method takes [c2apb1smenr2::W](c2apb1smenr2::W) writer structure"]
impl crate::Writable for C2APB1SMENR2 {}
#[doc = "CPU2 APB1 peripheral clocks enable in Sleep mode register 2 \\[dual core device only\\]"]
pub mod c2apb1smenr2;
#[doc = "CPU2 APB2 peripheral clocks enable in Sleep mode register \\[dual core device only\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2apb2smenr](c2apb2smenr) module"]
pub type C2APB2SMENR = crate::Reg<u32, _C2APB2SMENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2APB2SMENR;
#[doc = "`read()` method returns [c2apb2smenr::R](c2apb2smenr::R) reader structure"]
impl crate::Readable for C2APB2SMENR {}
#[doc = "`write(|w| ..)` method takes [c2apb2smenr::W](c2apb2smenr::W) writer structure"]
impl crate::Writable for C2APB2SMENR {}
#[doc = "CPU2 APB2 peripheral clocks enable in Sleep mode register \\[dual core device only\\]"]
pub mod c2apb2smenr;
#[doc = "CPU2 APB3 peripheral clock enable in Sleep mode register \\[dual core device only\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2apb3smenr](c2apb3smenr) module"]
pub type C2APB3SMENR = crate::Reg<u32, _C2APB3SMENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2APB3SMENR;
#[doc = "`read()` method returns [c2apb3smenr::R](c2apb3smenr::R) reader structure"]
impl crate::Readable for C2APB3SMENR {}
#[doc = "`write(|w| ..)` method takes [c2apb3smenr::W](c2apb3smenr::W) writer structure"]
impl crate::Writable for C2APB3SMENR {}
#[doc = "CPU2 APB3 peripheral clock enable in Sleep mode register \\[dual core device only\\]"]
pub mod c2apb3smenr;
