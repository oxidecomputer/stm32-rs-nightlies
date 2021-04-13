#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register 1"]
    pub i2c_cr1: I2C_CR1,
    #[doc = "0x04 - Control register 2"]
    pub i2c_cr2: I2C_CR2,
    #[doc = "0x08 - Own address register 1"]
    pub i2c_oar1: I2C_OAR1,
    #[doc = "0x0c - Own address register 2"]
    pub i2c_oar2: I2C_OAR2,
    #[doc = "0x10 - Timing register"]
    pub i2c_timingr: I2C_TIMINGR,
    #[doc = "0x14 - Status register 1"]
    pub i2c_timeoutr: I2C_TIMEOUTR,
    #[doc = "0x18 - Interrupt and Status register"]
    pub i2c_isr: I2C_ISR,
    #[doc = "0x1c - Interrupt clear register"]
    pub i2c_icr: I2C_ICR,
    #[doc = "0x20 - PEC register"]
    pub i2c_pecr: I2C_PECR,
    #[doc = "0x24 - Receive data register"]
    pub i2c_rxdr: I2C_RXDR,
    #[doc = "0x28 - Transmit data register"]
    pub i2c_txdr: I2C_TXDR,
}
#[doc = "Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_cr1](i2c_cr1) module"]
pub type I2C_CR1 = crate::Reg<u32, _I2C_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_CR1;
#[doc = "`read()` method returns [i2c_cr1::R](i2c_cr1::R) reader structure"]
impl crate::Readable for I2C_CR1 {}
#[doc = "`write(|w| ..)` method takes [i2c_cr1::W](i2c_cr1::W) writer structure"]
impl crate::Writable for I2C_CR1 {}
#[doc = "Control register 1"]
pub mod i2c_cr1;
#[doc = "Control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_cr2](i2c_cr2) module"]
pub type I2C_CR2 = crate::Reg<u32, _I2C_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_CR2;
#[doc = "`read()` method returns [i2c_cr2::R](i2c_cr2::R) reader structure"]
impl crate::Readable for I2C_CR2 {}
#[doc = "`write(|w| ..)` method takes [i2c_cr2::W](i2c_cr2::W) writer structure"]
impl crate::Writable for I2C_CR2 {}
#[doc = "Control register 2"]
pub mod i2c_cr2;
#[doc = "Own address register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_oar1](i2c_oar1) module"]
pub type I2C_OAR1 = crate::Reg<u32, _I2C_OAR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_OAR1;
#[doc = "`read()` method returns [i2c_oar1::R](i2c_oar1::R) reader structure"]
impl crate::Readable for I2C_OAR1 {}
#[doc = "`write(|w| ..)` method takes [i2c_oar1::W](i2c_oar1::W) writer structure"]
impl crate::Writable for I2C_OAR1 {}
#[doc = "Own address register 1"]
pub mod i2c_oar1;
#[doc = "Own address register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_oar2](i2c_oar2) module"]
pub type I2C_OAR2 = crate::Reg<u32, _I2C_OAR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_OAR2;
#[doc = "`read()` method returns [i2c_oar2::R](i2c_oar2::R) reader structure"]
impl crate::Readable for I2C_OAR2 {}
#[doc = "`write(|w| ..)` method takes [i2c_oar2::W](i2c_oar2::W) writer structure"]
impl crate::Writable for I2C_OAR2 {}
#[doc = "Own address register 2"]
pub mod i2c_oar2;
#[doc = "Timing register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_timingr](i2c_timingr) module"]
pub type I2C_TIMINGR = crate::Reg<u32, _I2C_TIMINGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_TIMINGR;
#[doc = "`read()` method returns [i2c_timingr::R](i2c_timingr::R) reader structure"]
impl crate::Readable for I2C_TIMINGR {}
#[doc = "`write(|w| ..)` method takes [i2c_timingr::W](i2c_timingr::W) writer structure"]
impl crate::Writable for I2C_TIMINGR {}
#[doc = "Timing register"]
pub mod i2c_timingr;
#[doc = "Status register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_timeoutr](i2c_timeoutr) module"]
pub type I2C_TIMEOUTR = crate::Reg<u32, _I2C_TIMEOUTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_TIMEOUTR;
#[doc = "`read()` method returns [i2c_timeoutr::R](i2c_timeoutr::R) reader structure"]
impl crate::Readable for I2C_TIMEOUTR {}
#[doc = "`write(|w| ..)` method takes [i2c_timeoutr::W](i2c_timeoutr::W) writer structure"]
impl crate::Writable for I2C_TIMEOUTR {}
#[doc = "Status register 1"]
pub mod i2c_timeoutr;
#[doc = "Interrupt and Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_isr](i2c_isr) module"]
pub type I2C_ISR = crate::Reg<u32, _I2C_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_ISR;
#[doc = "`read()` method returns [i2c_isr::R](i2c_isr::R) reader structure"]
impl crate::Readable for I2C_ISR {}
#[doc = "`write(|w| ..)` method takes [i2c_isr::W](i2c_isr::W) writer structure"]
impl crate::Writable for I2C_ISR {}
#[doc = "Interrupt and Status register"]
pub mod i2c_isr;
#[doc = "Interrupt clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_icr](i2c_icr) module"]
pub type I2C_ICR = crate::Reg<u32, _I2C_ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_ICR;
#[doc = "`write(|w| ..)` method takes [i2c_icr::W](i2c_icr::W) writer structure"]
impl crate::Writable for I2C_ICR {}
#[doc = "Interrupt clear register"]
pub mod i2c_icr;
#[doc = "PEC register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_pecr](i2c_pecr) module"]
pub type I2C_PECR = crate::Reg<u32, _I2C_PECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_PECR;
#[doc = "`read()` method returns [i2c_pecr::R](i2c_pecr::R) reader structure"]
impl crate::Readable for I2C_PECR {}
#[doc = "PEC register"]
pub mod i2c_pecr;
#[doc = "Receive data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_rxdr](i2c_rxdr) module"]
pub type I2C_RXDR = crate::Reg<u32, _I2C_RXDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_RXDR;
#[doc = "`read()` method returns [i2c_rxdr::R](i2c_rxdr::R) reader structure"]
impl crate::Readable for I2C_RXDR {}
#[doc = "Receive data register"]
pub mod i2c_rxdr;
#[doc = "Transmit data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_txdr](i2c_txdr) module"]
pub type I2C_TXDR = crate::Reg<u32, _I2C_TXDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_TXDR;
#[doc = "`read()` method returns [i2c_txdr::R](i2c_txdr::R) reader structure"]
impl crate::Readable for I2C_TXDR {}
#[doc = "`write(|w| ..)` method takes [i2c_txdr::W](i2c_txdr::W) writer structure"]
impl crate::Writable for I2C_TXDR {}
#[doc = "Transmit data register"]
pub mod i2c_txdr;
