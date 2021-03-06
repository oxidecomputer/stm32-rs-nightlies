#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA2D control register"]
    pub cr: CR,
    #[doc = "0x04 - DMA2D Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x08 - DMA2D interrupt flag clear register"]
    pub ifcr: IFCR,
    #[doc = "0x0c - DMA2D foreground memory address register"]
    pub fgmar: FGMAR,
    #[doc = "0x10 - DMA2D foreground offset register"]
    pub fgor: FGOR,
    #[doc = "0x14 - DMA2D background memory address register"]
    pub bgmar: BGMAR,
    #[doc = "0x18 - DMA2D background offset register"]
    pub bgor: BGOR,
    #[doc = "0x1c - DMA2D foreground PFC control register"]
    pub fgpfccr: FGPFCCR,
    #[doc = "0x20 - DMA2D foreground color register"]
    pub fgcolr: FGCOLR,
    #[doc = "0x24 - DMA2D background PFC control register"]
    pub bgpfccr: BGPFCCR,
    #[doc = "0x28 - DMA2D background color register"]
    pub bgcolr: BGCOLR,
    #[doc = "0x2c - DMA2D foreground CLUT memory address register"]
    pub fgcmar: FGCMAR,
    #[doc = "0x30 - DMA2D background CLUT memory address register"]
    pub bgcmar: BGCMAR,
    #[doc = "0x34 - DMA2D output PFC control register"]
    pub opfccr: OPFCCR,
    #[doc = "0x38 - DMA2D output color register"]
    pub ocolr: OCOLR,
    #[doc = "0x3c - DMA2D output memory address register"]
    pub omar: OMAR,
    #[doc = "0x40 - DMA2D output offset register"]
    pub oor: OOR,
    #[doc = "0x44 - DMA2D number of line register"]
    pub nlr: NLR,
    #[doc = "0x48 - DMA2D line watermark register"]
    pub lwr: LWR,
    #[doc = "0x4c - DMA2D AXI master timer configuration register"]
    pub amtcr: AMTCR,
}
#[doc = "DMA2D control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "DMA2D control register"]
pub mod cr;
#[doc = "DMA2D Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "DMA2D Interrupt Status Register"]
pub mod isr;
#[doc = "DMA2D interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifcr](ifcr) module"]
pub type IFCR = crate::Reg<u32, _IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFCR;
#[doc = "`read()` method returns [ifcr::R](ifcr::R) reader structure"]
impl crate::Readable for IFCR {}
#[doc = "`write(|w| ..)` method takes [ifcr::W](ifcr::W) writer structure"]
impl crate::Writable for IFCR {}
#[doc = "DMA2D interrupt flag clear register"]
pub mod ifcr;
#[doc = "DMA2D foreground memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgmar](fgmar) module"]
pub type FGMAR = crate::Reg<u32, _FGMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FGMAR;
#[doc = "`read()` method returns [fgmar::R](fgmar::R) reader structure"]
impl crate::Readable for FGMAR {}
#[doc = "`write(|w| ..)` method takes [fgmar::W](fgmar::W) writer structure"]
impl crate::Writable for FGMAR {}
#[doc = "DMA2D foreground memory address register"]
pub mod fgmar;
#[doc = "DMA2D foreground offset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgor](fgor) module"]
pub type FGOR = crate::Reg<u32, _FGOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FGOR;
#[doc = "`read()` method returns [fgor::R](fgor::R) reader structure"]
impl crate::Readable for FGOR {}
#[doc = "`write(|w| ..)` method takes [fgor::W](fgor::W) writer structure"]
impl crate::Writable for FGOR {}
#[doc = "DMA2D foreground offset register"]
pub mod fgor;
#[doc = "DMA2D background memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgmar](bgmar) module"]
pub type BGMAR = crate::Reg<u32, _BGMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BGMAR;
#[doc = "`read()` method returns [bgmar::R](bgmar::R) reader structure"]
impl crate::Readable for BGMAR {}
#[doc = "`write(|w| ..)` method takes [bgmar::W](bgmar::W) writer structure"]
impl crate::Writable for BGMAR {}
#[doc = "DMA2D background memory address register"]
pub mod bgmar;
#[doc = "DMA2D background offset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgor](bgor) module"]
pub type BGOR = crate::Reg<u32, _BGOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BGOR;
#[doc = "`read()` method returns [bgor::R](bgor::R) reader structure"]
impl crate::Readable for BGOR {}
#[doc = "`write(|w| ..)` method takes [bgor::W](bgor::W) writer structure"]
impl crate::Writable for BGOR {}
#[doc = "DMA2D background offset register"]
pub mod bgor;
#[doc = "DMA2D foreground PFC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgpfccr](fgpfccr) module"]
pub type FGPFCCR = crate::Reg<u32, _FGPFCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FGPFCCR;
#[doc = "`read()` method returns [fgpfccr::R](fgpfccr::R) reader structure"]
impl crate::Readable for FGPFCCR {}
#[doc = "`write(|w| ..)` method takes [fgpfccr::W](fgpfccr::W) writer structure"]
impl crate::Writable for FGPFCCR {}
#[doc = "DMA2D foreground PFC control register"]
pub mod fgpfccr;
#[doc = "DMA2D foreground color register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgcolr](fgcolr) module"]
pub type FGCOLR = crate::Reg<u32, _FGCOLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FGCOLR;
#[doc = "`read()` method returns [fgcolr::R](fgcolr::R) reader structure"]
impl crate::Readable for FGCOLR {}
#[doc = "`write(|w| ..)` method takes [fgcolr::W](fgcolr::W) writer structure"]
impl crate::Writable for FGCOLR {}
#[doc = "DMA2D foreground color register"]
pub mod fgcolr;
#[doc = "DMA2D background PFC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgpfccr](bgpfccr) module"]
pub type BGPFCCR = crate::Reg<u32, _BGPFCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BGPFCCR;
#[doc = "`read()` method returns [bgpfccr::R](bgpfccr::R) reader structure"]
impl crate::Readable for BGPFCCR {}
#[doc = "`write(|w| ..)` method takes [bgpfccr::W](bgpfccr::W) writer structure"]
impl crate::Writable for BGPFCCR {}
#[doc = "DMA2D background PFC control register"]
pub mod bgpfccr;
#[doc = "DMA2D background color register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgcolr](bgcolr) module"]
pub type BGCOLR = crate::Reg<u32, _BGCOLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BGCOLR;
#[doc = "`read()` method returns [bgcolr::R](bgcolr::R) reader structure"]
impl crate::Readable for BGCOLR {}
#[doc = "`write(|w| ..)` method takes [bgcolr::W](bgcolr::W) writer structure"]
impl crate::Writable for BGCOLR {}
#[doc = "DMA2D background color register"]
pub mod bgcolr;
#[doc = "DMA2D foreground CLUT memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgcmar](fgcmar) module"]
pub type FGCMAR = crate::Reg<u32, _FGCMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FGCMAR;
#[doc = "`read()` method returns [fgcmar::R](fgcmar::R) reader structure"]
impl crate::Readable for FGCMAR {}
#[doc = "`write(|w| ..)` method takes [fgcmar::W](fgcmar::W) writer structure"]
impl crate::Writable for FGCMAR {}
#[doc = "DMA2D foreground CLUT memory address register"]
pub mod fgcmar;
#[doc = "DMA2D background CLUT memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgcmar](bgcmar) module"]
pub type BGCMAR = crate::Reg<u32, _BGCMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BGCMAR;
#[doc = "`read()` method returns [bgcmar::R](bgcmar::R) reader structure"]
impl crate::Readable for BGCMAR {}
#[doc = "`write(|w| ..)` method takes [bgcmar::W](bgcmar::W) writer structure"]
impl crate::Writable for BGCMAR {}
#[doc = "DMA2D background CLUT memory address register"]
pub mod bgcmar;
#[doc = "DMA2D output PFC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opfccr](opfccr) module"]
pub type OPFCCR = crate::Reg<u32, _OPFCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPFCCR;
#[doc = "`read()` method returns [opfccr::R](opfccr::R) reader structure"]
impl crate::Readable for OPFCCR {}
#[doc = "`write(|w| ..)` method takes [opfccr::W](opfccr::W) writer structure"]
impl crate::Writable for OPFCCR {}
#[doc = "DMA2D output PFC control register"]
pub mod opfccr;
#[doc = "DMA2D output color register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocolr](ocolr) module"]
pub type OCOLR = crate::Reg<u32, _OCOLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OCOLR;
#[doc = "`read()` method returns [ocolr::R](ocolr::R) reader structure"]
impl crate::Readable for OCOLR {}
#[doc = "`write(|w| ..)` method takes [ocolr::W](ocolr::W) writer structure"]
impl crate::Writable for OCOLR {}
#[doc = "DMA2D output color register"]
pub mod ocolr;
#[doc = "DMA2D output memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [omar](omar) module"]
pub type OMAR = crate::Reg<u32, _OMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OMAR;
#[doc = "`read()` method returns [omar::R](omar::R) reader structure"]
impl crate::Readable for OMAR {}
#[doc = "`write(|w| ..)` method takes [omar::W](omar::W) writer structure"]
impl crate::Writable for OMAR {}
#[doc = "DMA2D output memory address register"]
pub mod omar;
#[doc = "DMA2D output offset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oor](oor) module"]
pub type OOR = crate::Reg<u32, _OOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OOR;
#[doc = "`read()` method returns [oor::R](oor::R) reader structure"]
impl crate::Readable for OOR {}
#[doc = "`write(|w| ..)` method takes [oor::W](oor::W) writer structure"]
impl crate::Writable for OOR {}
#[doc = "DMA2D output offset register"]
pub mod oor;
#[doc = "DMA2D number of line register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nlr](nlr) module"]
pub type NLR = crate::Reg<u32, _NLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NLR;
#[doc = "`read()` method returns [nlr::R](nlr::R) reader structure"]
impl crate::Readable for NLR {}
#[doc = "`write(|w| ..)` method takes [nlr::W](nlr::W) writer structure"]
impl crate::Writable for NLR {}
#[doc = "DMA2D number of line register"]
pub mod nlr;
#[doc = "DMA2D line watermark register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lwr](lwr) module"]
pub type LWR = crate::Reg<u32, _LWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LWR;
#[doc = "`read()` method returns [lwr::R](lwr::R) reader structure"]
impl crate::Readable for LWR {}
#[doc = "`write(|w| ..)` method takes [lwr::W](lwr::W) writer structure"]
impl crate::Writable for LWR {}
#[doc = "DMA2D line watermark register"]
pub mod lwr;
#[doc = "DMA2D AXI master timer configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amtcr](amtcr) module"]
pub type AMTCR = crate::Reg<u32, _AMTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AMTCR;
#[doc = "`read()` method returns [amtcr::R](amtcr::R) reader structure"]
impl crate::Readable for AMTCR {}
#[doc = "`write(|w| ..)` method takes [amtcr::W](amtcr::W) writer structure"]
impl crate::Writable for AMTCR {}
#[doc = "DMA2D AXI master timer configuration register"]
pub mod amtcr;
