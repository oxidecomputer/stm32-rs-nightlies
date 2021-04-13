#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub tim1_cr1: TIM1_CR1,
    #[doc = "0x04 - control register 2"]
    pub tim1_cr2: TIM1_CR2,
    #[doc = "0x08 - slave mode control register"]
    pub tim1_smcr: TIM1_SMCR,
    #[doc = "0x0c - DMA/Interrupt enable register"]
    pub tim1_dier: TIM1_DIER,
    #[doc = "0x10 - status register"]
    pub tim1_sr: TIM1_SR,
    #[doc = "0x14 - event generation register"]
    pub tim1_egr: TIM1_EGR,
    _reserved_6_ccmr1: [u8; 4usize],
    _reserved_7_ccmr2: [u8; 4usize],
    #[doc = "0x20 - capture/compare enable register"]
    pub tim1_ccer: TIM1_CCER,
    #[doc = "0x24 - counter"]
    pub tim1_cnt: TIM1_CNT,
    #[doc = "0x28 - prescaler"]
    pub tim1_psc: TIM1_PSC,
    #[doc = "0x2c - auto-reload register"]
    pub tim1_arr: TIM1_ARR,
    #[doc = "0x30 - repetition counter register"]
    pub tim1_rcr: TIM1_RCR,
    #[doc = "0x34 - capture/compare register 1"]
    pub tim1_ccr1: TIM1_CCR1,
    #[doc = "0x38 - capture/compare register 2"]
    pub tim1_ccr2: TIM1_CCR2,
    #[doc = "0x3c - capture/compare register 3"]
    pub tim1_ccr3: TIM1_CCR3,
    #[doc = "0x40 - capture/compare register 4"]
    pub tim1_ccr4: TIM1_CCR4,
    #[doc = "0x44 - break and dead-time register"]
    pub tim1_bdtr: TIM1_BDTR,
    #[doc = "0x48 - DMA control register"]
    pub tim1_dcr: TIM1_DCR,
    #[doc = "0x4c - DMA address for full transfer"]
    pub tim1_dmar: TIM1_DMAR,
    #[doc = "0x50 - option register 1"]
    pub tim1_or1: TIM1_OR1,
    #[doc = "0x54 - capture/compare mode register 2 (output mode)"]
    pub ccmr3_output: CCMR3_OUTPUT,
    #[doc = "0x58 - capture/compare register 4"]
    pub tim1_ccr5: TIM1_CCR5,
    #[doc = "0x5c - capture/compare register 4"]
    pub tim1_ccr6: TIM1_CCR6,
    #[doc = "0x60 - DMA address for full transfer"]
    pub tim1_af1: TIM1_AF1,
    #[doc = "0x64 - DMA address for full transfer"]
    pub tim1_af2: TIM1_AF2,
    #[doc = "0x68 - TIM1 timer input selection register"]
    pub tim1_tisel: TIM1_TISEL,
}
impl RegisterBlock {
    #[doc = "0x18 - capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub fn ccmr1_input(&self) -> &CCMR1_INPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const CCMR1_INPUT) }
    }
    #[doc = "0x18 - capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub fn ccmr1_input_mut(&self) -> &mut CCMR1_INPUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(24usize) as *mut CCMR1_INPUT) }
    }
    #[doc = "0x18 - capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub fn ccmr1_output(&self) -> &CCMR1_OUTPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const CCMR1_OUTPUT) }
    }
    #[doc = "0x18 - capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub fn ccmr1_output_mut(&self) -> &mut CCMR1_OUTPUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(24usize) as *mut CCMR1_OUTPUT) }
    }
    #[doc = "0x1c - capture/compare mode register 2 (output mode)"]
    #[inline(always)]
    pub fn ccmr2_input(&self) -> &CCMR2_INPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const CCMR2_INPUT) }
    }
    #[doc = "0x1c - capture/compare mode register 2 (output mode)"]
    #[inline(always)]
    pub fn ccmr2_input_mut(&self) -> &mut CCMR2_INPUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut CCMR2_INPUT) }
    }
    #[doc = "0x1c - capture/compare mode register 2 (output mode)"]
    #[inline(always)]
    pub fn ccmr2_output(&self) -> &CCMR2_OUTPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const CCMR2_OUTPUT) }
    }
    #[doc = "0x1c - capture/compare mode register 2 (output mode)"]
    #[inline(always)]
    pub fn ccmr2_output_mut(&self) -> &mut CCMR2_OUTPUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut CCMR2_OUTPUT) }
    }
}
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_cr1](tim1_cr1) module"]
pub type TIM1_CR1 = crate::Reg<u32, _TIM1_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_CR1;
#[doc = "`read()` method returns [tim1_cr1::R](tim1_cr1::R) reader structure"]
impl crate::Readable for TIM1_CR1 {}
#[doc = "`write(|w| ..)` method takes [tim1_cr1::W](tim1_cr1::W) writer structure"]
impl crate::Writable for TIM1_CR1 {}
#[doc = "control register 1"]
pub mod tim1_cr1;
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_cr2](tim1_cr2) module"]
pub type TIM1_CR2 = crate::Reg<u32, _TIM1_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_CR2;
#[doc = "`read()` method returns [tim1_cr2::R](tim1_cr2::R) reader structure"]
impl crate::Readable for TIM1_CR2 {}
#[doc = "`write(|w| ..)` method takes [tim1_cr2::W](tim1_cr2::W) writer structure"]
impl crate::Writable for TIM1_CR2 {}
#[doc = "control register 2"]
pub mod tim1_cr2;
#[doc = "slave mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_smcr](tim1_smcr) module"]
pub type TIM1_SMCR = crate::Reg<u32, _TIM1_SMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_SMCR;
#[doc = "`read()` method returns [tim1_smcr::R](tim1_smcr::R) reader structure"]
impl crate::Readable for TIM1_SMCR {}
#[doc = "`write(|w| ..)` method takes [tim1_smcr::W](tim1_smcr::W) writer structure"]
impl crate::Writable for TIM1_SMCR {}
#[doc = "slave mode control register"]
pub mod tim1_smcr;
#[doc = "DMA/Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_dier](tim1_dier) module"]
pub type TIM1_DIER = crate::Reg<u32, _TIM1_DIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_DIER;
#[doc = "`read()` method returns [tim1_dier::R](tim1_dier::R) reader structure"]
impl crate::Readable for TIM1_DIER {}
#[doc = "`write(|w| ..)` method takes [tim1_dier::W](tim1_dier::W) writer structure"]
impl crate::Writable for TIM1_DIER {}
#[doc = "DMA/Interrupt enable register"]
pub mod tim1_dier;
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_sr](tim1_sr) module"]
pub type TIM1_SR = crate::Reg<u32, _TIM1_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_SR;
#[doc = "`read()` method returns [tim1_sr::R](tim1_sr::R) reader structure"]
impl crate::Readable for TIM1_SR {}
#[doc = "`write(|w| ..)` method takes [tim1_sr::W](tim1_sr::W) writer structure"]
impl crate::Writable for TIM1_SR {}
#[doc = "status register"]
pub mod tim1_sr;
#[doc = "event generation register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_egr](tim1_egr) module"]
pub type TIM1_EGR = crate::Reg<u32, _TIM1_EGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_EGR;
#[doc = "`write(|w| ..)` method takes [tim1_egr::W](tim1_egr::W) writer structure"]
impl crate::Writable for TIM1_EGR {}
#[doc = "event generation register"]
pub mod tim1_egr;
#[doc = "capture/compare mode register 1 (output mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccmr1_output](ccmr1_output) module"]
pub type CCMR1_OUTPUT = crate::Reg<u32, _CCMR1_OUTPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCMR1_OUTPUT;
#[doc = "`read()` method returns [ccmr1_output::R](ccmr1_output::R) reader structure"]
impl crate::Readable for CCMR1_OUTPUT {}
#[doc = "`write(|w| ..)` method takes [ccmr1_output::W](ccmr1_output::W) writer structure"]
impl crate::Writable for CCMR1_OUTPUT {}
#[doc = "capture/compare mode register 1 (output mode)"]
pub mod ccmr1_output;
#[doc = "capture/compare mode register 1 (output mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccmr1_input](ccmr1_input) module"]
pub type CCMR1_INPUT = crate::Reg<u32, _CCMR1_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCMR1_INPUT;
#[doc = "`read()` method returns [ccmr1_input::R](ccmr1_input::R) reader structure"]
impl crate::Readable for CCMR1_INPUT {}
#[doc = "`write(|w| ..)` method takes [ccmr1_input::W](ccmr1_input::W) writer structure"]
impl crate::Writable for CCMR1_INPUT {}
#[doc = "capture/compare mode register 1 (output mode)"]
pub mod ccmr1_input;
#[doc = "capture/compare mode register 2 (output mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccmr2_output](ccmr2_output) module"]
pub type CCMR2_OUTPUT = crate::Reg<u32, _CCMR2_OUTPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCMR2_OUTPUT;
#[doc = "`read()` method returns [ccmr2_output::R](ccmr2_output::R) reader structure"]
impl crate::Readable for CCMR2_OUTPUT {}
#[doc = "`write(|w| ..)` method takes [ccmr2_output::W](ccmr2_output::W) writer structure"]
impl crate::Writable for CCMR2_OUTPUT {}
#[doc = "capture/compare mode register 2 (output mode)"]
pub mod ccmr2_output;
#[doc = "capture/compare mode register 2 (output mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccmr2_input](ccmr2_input) module"]
pub type CCMR2_INPUT = crate::Reg<u32, _CCMR2_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCMR2_INPUT;
#[doc = "`read()` method returns [ccmr2_input::R](ccmr2_input::R) reader structure"]
impl crate::Readable for CCMR2_INPUT {}
#[doc = "`write(|w| ..)` method takes [ccmr2_input::W](ccmr2_input::W) writer structure"]
impl crate::Writable for CCMR2_INPUT {}
#[doc = "capture/compare mode register 2 (output mode)"]
pub mod ccmr2_input;
#[doc = "capture/compare enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_ccer](tim1_ccer) module"]
pub type TIM1_CCER = crate::Reg<u32, _TIM1_CCER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_CCER;
#[doc = "`read()` method returns [tim1_ccer::R](tim1_ccer::R) reader structure"]
impl crate::Readable for TIM1_CCER {}
#[doc = "`write(|w| ..)` method takes [tim1_ccer::W](tim1_ccer::W) writer structure"]
impl crate::Writable for TIM1_CCER {}
#[doc = "capture/compare enable register"]
pub mod tim1_ccer;
#[doc = "counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_cnt](tim1_cnt) module"]
pub type TIM1_CNT = crate::Reg<u32, _TIM1_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_CNT;
#[doc = "`read()` method returns [tim1_cnt::R](tim1_cnt::R) reader structure"]
impl crate::Readable for TIM1_CNT {}
#[doc = "`write(|w| ..)` method takes [tim1_cnt::W](tim1_cnt::W) writer structure"]
impl crate::Writable for TIM1_CNT {}
#[doc = "counter"]
pub mod tim1_cnt;
#[doc = "prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_psc](tim1_psc) module"]
pub type TIM1_PSC = crate::Reg<u32, _TIM1_PSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_PSC;
#[doc = "`read()` method returns [tim1_psc::R](tim1_psc::R) reader structure"]
impl crate::Readable for TIM1_PSC {}
#[doc = "`write(|w| ..)` method takes [tim1_psc::W](tim1_psc::W) writer structure"]
impl crate::Writable for TIM1_PSC {}
#[doc = "prescaler"]
pub mod tim1_psc;
#[doc = "auto-reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_arr](tim1_arr) module"]
pub type TIM1_ARR = crate::Reg<u32, _TIM1_ARR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_ARR;
#[doc = "`read()` method returns [tim1_arr::R](tim1_arr::R) reader structure"]
impl crate::Readable for TIM1_ARR {}
#[doc = "`write(|w| ..)` method takes [tim1_arr::W](tim1_arr::W) writer structure"]
impl crate::Writable for TIM1_ARR {}
#[doc = "auto-reload register"]
pub mod tim1_arr;
#[doc = "repetition counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_rcr](tim1_rcr) module"]
pub type TIM1_RCR = crate::Reg<u32, _TIM1_RCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_RCR;
#[doc = "`read()` method returns [tim1_rcr::R](tim1_rcr::R) reader structure"]
impl crate::Readable for TIM1_RCR {}
#[doc = "`write(|w| ..)` method takes [tim1_rcr::W](tim1_rcr::W) writer structure"]
impl crate::Writable for TIM1_RCR {}
#[doc = "repetition counter register"]
pub mod tim1_rcr;
#[doc = "capture/compare register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_ccr1](tim1_ccr1) module"]
pub type TIM1_CCR1 = crate::Reg<u32, _TIM1_CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_CCR1;
#[doc = "`read()` method returns [tim1_ccr1::R](tim1_ccr1::R) reader structure"]
impl crate::Readable for TIM1_CCR1 {}
#[doc = "`write(|w| ..)` method takes [tim1_ccr1::W](tim1_ccr1::W) writer structure"]
impl crate::Writable for TIM1_CCR1 {}
#[doc = "capture/compare register 1"]
pub mod tim1_ccr1;
#[doc = "capture/compare register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_ccr2](tim1_ccr2) module"]
pub type TIM1_CCR2 = crate::Reg<u32, _TIM1_CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_CCR2;
#[doc = "`read()` method returns [tim1_ccr2::R](tim1_ccr2::R) reader structure"]
impl crate::Readable for TIM1_CCR2 {}
#[doc = "`write(|w| ..)` method takes [tim1_ccr2::W](tim1_ccr2::W) writer structure"]
impl crate::Writable for TIM1_CCR2 {}
#[doc = "capture/compare register 2"]
pub mod tim1_ccr2;
#[doc = "capture/compare register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_ccr3](tim1_ccr3) module"]
pub type TIM1_CCR3 = crate::Reg<u32, _TIM1_CCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_CCR3;
#[doc = "`read()` method returns [tim1_ccr3::R](tim1_ccr3::R) reader structure"]
impl crate::Readable for TIM1_CCR3 {}
#[doc = "`write(|w| ..)` method takes [tim1_ccr3::W](tim1_ccr3::W) writer structure"]
impl crate::Writable for TIM1_CCR3 {}
#[doc = "capture/compare register 3"]
pub mod tim1_ccr3;
#[doc = "capture/compare register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_ccr4](tim1_ccr4) module"]
pub type TIM1_CCR4 = crate::Reg<u32, _TIM1_CCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_CCR4;
#[doc = "`read()` method returns [tim1_ccr4::R](tim1_ccr4::R) reader structure"]
impl crate::Readable for TIM1_CCR4 {}
#[doc = "`write(|w| ..)` method takes [tim1_ccr4::W](tim1_ccr4::W) writer structure"]
impl crate::Writable for TIM1_CCR4 {}
#[doc = "capture/compare register 4"]
pub mod tim1_ccr4;
#[doc = "break and dead-time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_bdtr](tim1_bdtr) module"]
pub type TIM1_BDTR = crate::Reg<u32, _TIM1_BDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_BDTR;
#[doc = "`read()` method returns [tim1_bdtr::R](tim1_bdtr::R) reader structure"]
impl crate::Readable for TIM1_BDTR {}
#[doc = "`write(|w| ..)` method takes [tim1_bdtr::W](tim1_bdtr::W) writer structure"]
impl crate::Writable for TIM1_BDTR {}
#[doc = "break and dead-time register"]
pub mod tim1_bdtr;
#[doc = "DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_dcr](tim1_dcr) module"]
pub type TIM1_DCR = crate::Reg<u32, _TIM1_DCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_DCR;
#[doc = "`read()` method returns [tim1_dcr::R](tim1_dcr::R) reader structure"]
impl crate::Readable for TIM1_DCR {}
#[doc = "`write(|w| ..)` method takes [tim1_dcr::W](tim1_dcr::W) writer structure"]
impl crate::Writable for TIM1_DCR {}
#[doc = "DMA control register"]
pub mod tim1_dcr;
#[doc = "DMA address for full transfer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_dmar](tim1_dmar) module"]
pub type TIM1_DMAR = crate::Reg<u32, _TIM1_DMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_DMAR;
#[doc = "`read()` method returns [tim1_dmar::R](tim1_dmar::R) reader structure"]
impl crate::Readable for TIM1_DMAR {}
#[doc = "`write(|w| ..)` method takes [tim1_dmar::W](tim1_dmar::W) writer structure"]
impl crate::Writable for TIM1_DMAR {}
#[doc = "DMA address for full transfer"]
pub mod tim1_dmar;
#[doc = "option register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_or1](tim1_or1) module"]
pub type TIM1_OR1 = crate::Reg<u32, _TIM1_OR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_OR1;
#[doc = "`read()` method returns [tim1_or1::R](tim1_or1::R) reader structure"]
impl crate::Readable for TIM1_OR1 {}
#[doc = "`write(|w| ..)` method takes [tim1_or1::W](tim1_or1::W) writer structure"]
impl crate::Writable for TIM1_OR1 {}
#[doc = "option register 1"]
pub mod tim1_or1;
#[doc = "capture/compare mode register 2 (output mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccmr3_output](ccmr3_output) module"]
pub type CCMR3_OUTPUT = crate::Reg<u32, _CCMR3_OUTPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCMR3_OUTPUT;
#[doc = "`read()` method returns [ccmr3_output::R](ccmr3_output::R) reader structure"]
impl crate::Readable for CCMR3_OUTPUT {}
#[doc = "`write(|w| ..)` method takes [ccmr3_output::W](ccmr3_output::W) writer structure"]
impl crate::Writable for CCMR3_OUTPUT {}
#[doc = "capture/compare mode register 2 (output mode)"]
pub mod ccmr3_output;
#[doc = "capture/compare register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_ccr5](tim1_ccr5) module"]
pub type TIM1_CCR5 = crate::Reg<u32, _TIM1_CCR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_CCR5;
#[doc = "`read()` method returns [tim1_ccr5::R](tim1_ccr5::R) reader structure"]
impl crate::Readable for TIM1_CCR5 {}
#[doc = "`write(|w| ..)` method takes [tim1_ccr5::W](tim1_ccr5::W) writer structure"]
impl crate::Writable for TIM1_CCR5 {}
#[doc = "capture/compare register 4"]
pub mod tim1_ccr5;
#[doc = "capture/compare register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_ccr6](tim1_ccr6) module"]
pub type TIM1_CCR6 = crate::Reg<u32, _TIM1_CCR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_CCR6;
#[doc = "`read()` method returns [tim1_ccr6::R](tim1_ccr6::R) reader structure"]
impl crate::Readable for TIM1_CCR6 {}
#[doc = "`write(|w| ..)` method takes [tim1_ccr6::W](tim1_ccr6::W) writer structure"]
impl crate::Writable for TIM1_CCR6 {}
#[doc = "capture/compare register 4"]
pub mod tim1_ccr6;
#[doc = "DMA address for full transfer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_af1](tim1_af1) module"]
pub type TIM1_AF1 = crate::Reg<u32, _TIM1_AF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_AF1;
#[doc = "`read()` method returns [tim1_af1::R](tim1_af1::R) reader structure"]
impl crate::Readable for TIM1_AF1 {}
#[doc = "`write(|w| ..)` method takes [tim1_af1::W](tim1_af1::W) writer structure"]
impl crate::Writable for TIM1_AF1 {}
#[doc = "DMA address for full transfer"]
pub mod tim1_af1;
#[doc = "DMA address for full transfer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_af2](tim1_af2) module"]
pub type TIM1_AF2 = crate::Reg<u32, _TIM1_AF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_AF2;
#[doc = "`read()` method returns [tim1_af2::R](tim1_af2::R) reader structure"]
impl crate::Readable for TIM1_AF2 {}
#[doc = "`write(|w| ..)` method takes [tim1_af2::W](tim1_af2::W) writer structure"]
impl crate::Writable for TIM1_AF2 {}
#[doc = "DMA address for full transfer"]
pub mod tim1_af2;
#[doc = "TIM1 timer input selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_tisel](tim1_tisel) module"]
pub type TIM1_TISEL = crate::Reg<u32, _TIM1_TISEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_TISEL;
#[doc = "`read()` method returns [tim1_tisel::R](tim1_tisel::R) reader structure"]
impl crate::Readable for TIM1_TISEL {}
#[doc = "`write(|w| ..)` method takes [tim1_tisel::W](tim1_tisel::W) writer structure"]
impl crate::Writable for TIM1_TISEL {}
#[doc = "TIM1 timer input selection register"]
pub mod tim1_tisel;
