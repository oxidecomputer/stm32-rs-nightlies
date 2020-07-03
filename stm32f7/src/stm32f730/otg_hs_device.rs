#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_HS device configuration register"]
    pub otg_hs_dcfg: OTG_HS_DCFG,
    #[doc = "0x04 - OTG_HS device control register"]
    pub otg_hs_dctl: OTG_HS_DCTL,
    #[doc = "0x08 - OTG_HS device status register"]
    pub otg_hs_dsts: OTG_HS_DSTS,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - OTG_HS device IN endpoint common interrupt mask register"]
    pub otg_hs_diepmsk: OTG_HS_DIEPMSK,
    #[doc = "0x14 - OTG_HS device OUT endpoint common interrupt mask register"]
    pub otg_hs_doepmsk: OTG_HS_DOEPMSK,
    #[doc = "0x18 - OTG_HS device all endpoints interrupt register"]
    pub otg_hs_daint: OTG_HS_DAINT,
    #[doc = "0x1c - OTG_HS all endpoints interrupt mask register"]
    pub otg_hs_daintmsk: OTG_HS_DAINTMSK,
    _reserved7: [u8; 8usize],
    #[doc = "0x28 - OTG_HS device VBUS discharge time register"]
    pub otg_hs_dvbusdis: OTG_HS_DVBUSDIS,
    #[doc = "0x2c - OTG_HS device VBUS pulsing time register"]
    pub otg_hs_dvbuspulse: OTG_HS_DVBUSPULSE,
    #[doc = "0x30 - OTG_HS Device threshold control register"]
    pub otg_hs_dthrctl: OTG_HS_DTHRCTL,
    #[doc = "0x34 - OTG_HS device IN endpoint FIFO empty interrupt mask register"]
    pub otg_hs_diepempmsk: OTG_HS_DIEPEMPMSK,
    #[doc = "0x38 - OTG_HS device each endpoint interrupt register"]
    pub otg_hs_deachint: OTG_HS_DEACHINT,
    #[doc = "0x3c - OTG_HS device each endpoint interrupt register mask"]
    pub otg_hs_deachintmsk: OTG_HS_DEACHINTMSK,
    _reserved13: [u8; 192usize],
    #[doc = "0x100 - OTG device endpoint-0 control register"]
    pub otg_hs_diepctl0: OTG_HS_DIEPCTL0,
    _reserved14: [u8; 4usize],
    #[doc = "0x108 - OTG device endpoint-0 interrupt register"]
    pub otg_hs_diepint0: OTG_HS_DIEPINT0,
    _reserved15: [u8; 4usize],
    #[doc = "0x110 - OTG_HS device IN endpoint 0 transfer size register"]
    pub otg_hs_dieptsiz0: OTG_HS_DIEPTSIZ0,
    #[doc = "0x114 - OTG_HS device endpoint-1 DMA address register"]
    pub otg_hs_diepdma0: OTG_HS_DIEPDMA0,
    #[doc = "0x118 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub otg_hs_dtxfsts0: OTG_HS_DTXFSTS0,
    _reserved18: [u8; 4usize],
    #[doc = "0x120 - OTG device endpoint-1 control register"]
    pub otg_hs_diepctl1: OTG_HS_DIEPCTL1,
    _reserved19: [u8; 4usize],
    #[doc = "0x128 - OTG device endpoint-1 interrupt register"]
    pub otg_hs_diepint1: OTG_HS_DIEPINT1,
    _reserved20: [u8; 4usize],
    #[doc = "0x130 - OTG_HS device endpoint transfer size register"]
    pub otg_hs_dieptsiz1: OTG_HS_DIEPTSIZ1,
    #[doc = "0x134 - OTG_HS device endpoint-2 DMA address register"]
    pub otg_hs_diepdma1: OTG_HS_DIEPDMA1,
    #[doc = "0x138 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub otg_hs_dtxfsts1: OTG_HS_DTXFSTS1,
    _reserved23: [u8; 4usize],
    #[doc = "0x140 - OTG device endpoint-2 control register"]
    pub otg_hs_diepctl2: OTG_HS_DIEPCTL2,
    _reserved24: [u8; 4usize],
    #[doc = "0x148 - OTG device endpoint-2 interrupt register"]
    pub otg_hs_diepint2: OTG_HS_DIEPINT2,
    _reserved25: [u8; 4usize],
    #[doc = "0x150 - OTG_HS device endpoint transfer size register"]
    pub otg_hs_dieptsiz2: OTG_HS_DIEPTSIZ2,
    #[doc = "0x154 - OTG_HS device endpoint-3 DMA address register"]
    pub otg_hs_diepdma2: OTG_HS_DIEPDMA2,
    #[doc = "0x158 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub otg_hs_dtxfsts2: OTG_HS_DTXFSTS2,
    _reserved28: [u8; 4usize],
    #[doc = "0x160 - OTG device endpoint-3 control register"]
    pub otg_hs_diepctl3: OTG_HS_DIEPCTL3,
    _reserved29: [u8; 4usize],
    #[doc = "0x168 - OTG device endpoint-3 interrupt register"]
    pub otg_hs_diepint3: OTG_HS_DIEPINT3,
    _reserved30: [u8; 4usize],
    #[doc = "0x170 - OTG_HS device endpoint transfer size register"]
    pub otg_hs_dieptsiz3: OTG_HS_DIEPTSIZ3,
    #[doc = "0x174 - OTG_HS device endpoint-4 DMA address register"]
    pub otg_hs_diepdma3: OTG_HS_DIEPDMA3,
    #[doc = "0x178 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub otg_hs_dtxfsts3: OTG_HS_DTXFSTS3,
    _reserved33: [u8; 4usize],
    #[doc = "0x180 - OTG device endpoint-4 control register"]
    pub otg_hs_diepctl4: OTG_HS_DIEPCTL4,
    _reserved34: [u8; 4usize],
    #[doc = "0x188 - OTG device endpoint-4 interrupt register"]
    pub otg_hs_diepint4: OTG_HS_DIEPINT4,
    _reserved35: [u8; 4usize],
    #[doc = "0x190 - OTG_HS device endpoint transfer size register"]
    pub otg_hs_dieptsiz4: OTG_HS_DIEPTSIZ4,
    #[doc = "0x194 - OTG_HS device endpoint-5 DMA address register"]
    pub otg_hs_diepdma4: OTG_HS_DIEPDMA4,
    #[doc = "0x198 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub otg_hs_dtxfsts4: OTG_HS_DTXFSTS4,
    _reserved38: [u8; 4usize],
    #[doc = "0x1a0 - OTG device endpoint-5 control register"]
    pub otg_hs_diepctl5: OTG_HS_DIEPCTL5,
    _reserved39: [u8; 4usize],
    #[doc = "0x1a8 - OTG device endpoint-5 interrupt register"]
    pub otg_hs_diepint5: OTG_HS_DIEPINT5,
    _reserved40: [u8; 4usize],
    #[doc = "0x1b0 - OTG_HS device endpoint transfer size register"]
    pub otg_hs_dieptsiz5: OTG_HS_DIEPTSIZ5,
    #[doc = "0x1b4 - OTG Device channel-x DMA address register"]
    pub otg_hs_diepdma5: OTG_HS_DIEPDMA5,
    #[doc = "0x1b8 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub otg_hs_dtxfsts5: OTG_HS_DTXFSTS5,
    _reserved43: [u8; 4usize],
    #[doc = "0x1c0 - OTG device endpoint-6 control register"]
    pub otg_hs_diepctl6: OTG_HS_DIEPCTL6,
    _reserved44: [u8; 4usize],
    #[doc = "0x1c8 - OTG device endpoint-6 interrupt register"]
    pub otg_hs_diepint6: OTG_HS_DIEPINT6,
    _reserved45: [u8; 4usize],
    #[doc = "0x1d0 - OTG_HS device endpoint transfer size register"]
    pub otg_hs_dieptsiz6: OTG_HS_DIEPTSIZ6,
    #[doc = "0x1d4 - OTG Device channel-x DMA address register"]
    pub otg_hs_diepdma6: OTG_HS_DIEPDMA6,
    #[doc = "0x1d8 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub otg_hs_dtxfsts6: OTG_HS_DTXFSTS6,
    _reserved48: [u8; 4usize],
    #[doc = "0x1e0 - OTG device endpoint-7 control register"]
    pub otg_hs_diepctl7: OTG_HS_DIEPCTL7,
    _reserved49: [u8; 4usize],
    #[doc = "0x1e8 - OTG device endpoint-7 interrupt register"]
    pub otg_hs_diepint7: OTG_HS_DIEPINT7,
    _reserved50: [u8; 4usize],
    #[doc = "0x1f0 - OTG_HS device endpoint transfer size register"]
    pub otg_hs_dieptsiz7: OTG_HS_DIEPTSIZ7,
    #[doc = "0x1f4 - OTG Device channel-x DMA address register"]
    pub otg_hs_diepdma7: OTG_HS_DIEPDMA7,
    #[doc = "0x1f8 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub otg_hs_dtxfsts7: OTG_HS_DTXFSTS7,
    _reserved53: [u8; 24usize],
    #[doc = "0x214 - OTG Device channel-x DMA address register"]
    pub otg_hs_diepdma8: OTG_HS_DIEPDMA8,
    _reserved54: [u8; 28usize],
    #[doc = "0x234 - OTG Device channel-x DMA address register"]
    pub otg_hs_diepdma9: OTG_HS_DIEPDMA9,
    _reserved55: [u8; 28usize],
    #[doc = "0x254 - OTG Device channel-x DMA address register"]
    pub otg_hs_diepdma10: OTG_HS_DIEPDMA10,
    _reserved56: [u8; 28usize],
    #[doc = "0x274 - OTG Device channel-x DMA address register"]
    pub otg_hs_diepdma11: OTG_HS_DIEPDMA11,
    _reserved57: [u8; 28usize],
    #[doc = "0x294 - OTG Device channel-x DMA address register"]
    pub otg_hs_diepdma12: OTG_HS_DIEPDMA12,
    _reserved58: [u8; 28usize],
    #[doc = "0x2b4 - OTG Device channel-x DMA address register"]
    pub otg_hs_diepdma13: OTG_HS_DIEPDMA13,
    _reserved59: [u8; 28usize],
    #[doc = "0x2d4 - OTG Device channel-x DMA address register"]
    pub otg_hs_diepdma14: OTG_HS_DIEPDMA14,
    _reserved60: [u8; 28usize],
    #[doc = "0x2f4 - OTG Device channel-x DMA address register"]
    pub otg_hs_diepdma15: OTG_HS_DIEPDMA15,
    _reserved61: [u8; 8usize],
    #[doc = "0x300 - OTG_HS device control OUT endpoint 0 control register"]
    pub otg_hs_doepctl0: OTG_HS_DOEPCTL0,
    _reserved62: [u8; 4usize],
    #[doc = "0x308 - OTG_HS device endpoint-0 interrupt register"]
    pub otg_hs_doepint0: OTG_HS_DOEPINT0,
    _reserved63: [u8; 4usize],
    #[doc = "0x310 - OTG_HS device endpoint-0 transfer size register"]
    pub otg_hs_doeptsiz0: OTG_HS_DOEPTSIZ0,
    #[doc = "0x314 - OTG Device channel-x DMA address register"]
    pub otg_hs_doepdma0: OTG_HS_DOEPDMA0,
    _reserved65: [u8; 8usize],
    #[doc = "0x320 - OTG device endpoint-1 control register"]
    pub otg_hs_doepctl1: OTG_HS_DOEPCTL1,
    _reserved66: [u8; 4usize],
    #[doc = "0x328 - OTG_HS device endpoint-1 interrupt register"]
    pub otg_hs_doepint1: OTG_HS_DOEPINT1,
    _reserved67: [u8; 4usize],
    #[doc = "0x330 - OTG_HS device endpoint-1 transfer size register"]
    pub otg_hs_doeptsiz1: OTG_HS_DOEPTSIZ1,
    #[doc = "0x334 - OTG Device channel-x DMA address register"]
    pub otg_hs_doepdma1: OTG_HS_DOEPDMA1,
    _reserved69: [u8; 8usize],
    #[doc = "0x340 - OTG device endpoint-2 control register"]
    pub otg_hs_doepctl2: OTG_HS_DOEPCTL2,
    _reserved70: [u8; 4usize],
    #[doc = "0x348 - OTG_HS device endpoint-2 interrupt register"]
    pub otg_hs_doepint2: OTG_HS_DOEPINT2,
    _reserved71: [u8; 4usize],
    #[doc = "0x350 - OTG_HS device endpoint-2 transfer size register"]
    pub otg_hs_doeptsiz2: OTG_HS_DOEPTSIZ2,
    #[doc = "0x354 - OTG Device channel-x DMA address register"]
    pub otg_hs_doepdma2: OTG_HS_DOEPDMA2,
    _reserved73: [u8; 8usize],
    #[doc = "0x360 - OTG device endpoint-3 control register"]
    pub otg_hs_doepctl3: OTG_HS_DOEPCTL3,
    _reserved74: [u8; 4usize],
    #[doc = "0x368 - OTG_HS device endpoint-3 interrupt register"]
    pub otg_hs_doepint3: OTG_HS_DOEPINT3,
    _reserved75: [u8; 4usize],
    #[doc = "0x370 - OTG_HS device endpoint-3 transfer size register"]
    pub otg_hs_doeptsiz3: OTG_HS_DOEPTSIZ3,
    #[doc = "0x374 - OTG Device channel-x DMA address register"]
    pub otg_hs_doepdma3: OTG_HS_DOEPDMA3,
    _reserved77: [u8; 8usize],
    #[doc = "0x380 - OTG device endpoint-4 control register"]
    pub otg_hs_doepctl4: OTG_HS_DOEPCTL4,
    _reserved78: [u8; 4usize],
    #[doc = "0x388 - OTG_HS device endpoint-4 interrupt register"]
    pub otg_hs_doepint4: OTG_HS_DOEPINT4,
    _reserved79: [u8; 4usize],
    #[doc = "0x390 - OTG_HS device endpoint-4 transfer size register"]
    pub otg_hs_doeptsiz4: OTG_HS_DOEPTSIZ4,
    #[doc = "0x394 - OTG Device channel-x DMA address register"]
    pub otg_hs_doepdma4: OTG_HS_DOEPDMA4,
    _reserved81: [u8; 8usize],
    #[doc = "0x3a0 - OTG device endpoint-5 control register"]
    pub otg_hs_doepctl5: OTG_HS_DOEPCTL5,
    _reserved82: [u8; 4usize],
    #[doc = "0x3a8 - OTG_HS device endpoint-5 interrupt register"]
    pub otg_hs_doepint5: OTG_HS_DOEPINT5,
    _reserved83: [u8; 4usize],
    #[doc = "0x3b0 - OTG_HS device endpoint-5 transfer size register"]
    pub otg_hs_doeptsiz5: OTG_HS_DOEPTSIZ5,
    #[doc = "0x3b4 - OTG Device channel-x DMA address register"]
    pub otg_hs_doepdma5: OTG_HS_DOEPDMA5,
    _reserved85: [u8; 8usize],
    #[doc = "0x3c0 - OTG device endpoint-6 control register"]
    pub otg_hs_doepctl6: OTG_HS_DOEPCTL6,
    _reserved86: [u8; 4usize],
    #[doc = "0x3c8 - OTG_HS device endpoint-6 interrupt register"]
    pub otg_hs_doepint6: OTG_HS_DOEPINT6,
    _reserved87: [u8; 4usize],
    #[doc = "0x3d0 - OTG_HS device endpoint-6 transfer size register"]
    pub otg_hs_doeptsiz6: OTG_HS_DOEPTSIZ6,
    #[doc = "0x3d4 - OTG Device channel-x DMA address register"]
    pub otg_hs_doepdma6: OTG_HS_DOEPDMA6,
    _reserved89: [u8; 8usize],
    #[doc = "0x3e0 - OTG device endpoint-7 control register"]
    pub otg_hs_doepctl7: OTG_HS_DOEPCTL7,
    _reserved90: [u8; 4usize],
    #[doc = "0x3e8 - OTG_HS device endpoint-7 interrupt register"]
    pub otg_hs_doepint7: OTG_HS_DOEPINT7,
    _reserved91: [u8; 4usize],
    #[doc = "0x3f0 - OTG_HS device endpoint-7 transfer size register"]
    pub otg_hs_doeptsiz7: OTG_HS_DOEPTSIZ7,
    #[doc = "0x3f4 - OTG Device channel-x DMA address register"]
    pub otg_hs_doepdma7: OTG_HS_DOEPDMA7,
    _reserved93: [u8; 28usize],
    #[doc = "0x414 - OTG Device channel-x DMA address register"]
    pub otg_hs_doepdma8: OTG_HS_DOEPDMA8,
    _reserved94: [u8; 28usize],
    #[doc = "0x434 - OTG Device channel-x DMA address register"]
    pub otg_hs_doepdma9: OTG_HS_DOEPDMA9,
    _reserved95: [u8; 28usize],
    #[doc = "0x454 - OTG Device channel-x DMA address register"]
    pub otg_hs_doepdma10: OTG_HS_DOEPDMA10,
    _reserved96: [u8; 28usize],
    #[doc = "0x474 - OTG Device channel-x DMA address register"]
    pub otg_hs_doepdma11: OTG_HS_DOEPDMA11,
    _reserved97: [u8; 28usize],
    #[doc = "0x494 - OTG Device channel-x DMA address register"]
    pub otg_hs_doepdma12: OTG_HS_DOEPDMA12,
    _reserved98: [u8; 28usize],
    #[doc = "0x4b4 - OTG Device channel-x DMA address register"]
    pub otg_hs_doepdma13: OTG_HS_DOEPDMA13,
    _reserved99: [u8; 28usize],
    #[doc = "0x4d4 - OTG Device channel-x DMA address register"]
    pub otg_hs_doepdma14: OTG_HS_DOEPDMA14,
    _reserved100: [u8; 28usize],
    #[doc = "0x4f4 - OTG Device channel-x DMA address register"]
    pub otg_hs_doepdma15: OTG_HS_DOEPDMA15,
}
#[doc = "OTG_HS device configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_dcfg](otg_hs_dcfg) module"]
pub type OTG_HS_DCFG = crate::Reg<u32, _OTG_HS_DCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DCFG;
#[doc = "`read()` method returns [otg_hs_dcfg::R](otg_hs_dcfg::R) reader structure"]
impl crate::Readable for OTG_HS_DCFG {}
#[doc = "`write(|w| ..)` method takes [otg_hs_dcfg::W](otg_hs_dcfg::W) writer structure"]
impl crate::Writable for OTG_HS_DCFG {}
#[doc = "OTG_HS device configuration register"]
pub mod otg_hs_dcfg;
#[doc = "OTG_HS device control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_dctl](otg_hs_dctl) module"]
pub type OTG_HS_DCTL = crate::Reg<u32, _OTG_HS_DCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DCTL;
#[doc = "`read()` method returns [otg_hs_dctl::R](otg_hs_dctl::R) reader structure"]
impl crate::Readable for OTG_HS_DCTL {}
#[doc = "`write(|w| ..)` method takes [otg_hs_dctl::W](otg_hs_dctl::W) writer structure"]
impl crate::Writable for OTG_HS_DCTL {}
#[doc = "OTG_HS device control register"]
pub mod otg_hs_dctl;
#[doc = "OTG_HS device status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_dsts](otg_hs_dsts) module"]
pub type OTG_HS_DSTS = crate::Reg<u32, _OTG_HS_DSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DSTS;
#[doc = "`read()` method returns [otg_hs_dsts::R](otg_hs_dsts::R) reader structure"]
impl crate::Readable for OTG_HS_DSTS {}
#[doc = "OTG_HS device status register"]
pub mod otg_hs_dsts;
#[doc = "OTG_HS device IN endpoint common interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepmsk](otg_hs_diepmsk) module"]
pub type OTG_HS_DIEPMSK = crate::Reg<u32, _OTG_HS_DIEPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPMSK;
#[doc = "`read()` method returns [otg_hs_diepmsk::R](otg_hs_diepmsk::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPMSK {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepmsk::W](otg_hs_diepmsk::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPMSK {}
#[doc = "OTG_HS device IN endpoint common interrupt mask register"]
pub mod otg_hs_diepmsk;
#[doc = "OTG_HS device OUT endpoint common interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepmsk](otg_hs_doepmsk) module"]
pub type OTG_HS_DOEPMSK = crate::Reg<u32, _OTG_HS_DOEPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPMSK;
#[doc = "`read()` method returns [otg_hs_doepmsk::R](otg_hs_doepmsk::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPMSK {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepmsk::W](otg_hs_doepmsk::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPMSK {}
#[doc = "OTG_HS device OUT endpoint common interrupt mask register"]
pub mod otg_hs_doepmsk;
#[doc = "OTG_HS device all endpoints interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_daint](otg_hs_daint) module"]
pub type OTG_HS_DAINT = crate::Reg<u32, _OTG_HS_DAINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DAINT;
#[doc = "`read()` method returns [otg_hs_daint::R](otg_hs_daint::R) reader structure"]
impl crate::Readable for OTG_HS_DAINT {}
#[doc = "OTG_HS device all endpoints interrupt register"]
pub mod otg_hs_daint;
#[doc = "OTG_HS all endpoints interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_daintmsk](otg_hs_daintmsk) module"]
pub type OTG_HS_DAINTMSK = crate::Reg<u32, _OTG_HS_DAINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DAINTMSK;
#[doc = "`read()` method returns [otg_hs_daintmsk::R](otg_hs_daintmsk::R) reader structure"]
impl crate::Readable for OTG_HS_DAINTMSK {}
#[doc = "`write(|w| ..)` method takes [otg_hs_daintmsk::W](otg_hs_daintmsk::W) writer structure"]
impl crate::Writable for OTG_HS_DAINTMSK {}
#[doc = "OTG_HS all endpoints interrupt mask register"]
pub mod otg_hs_daintmsk;
#[doc = "OTG_HS device VBUS discharge time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_dvbusdis](otg_hs_dvbusdis) module"]
pub type OTG_HS_DVBUSDIS = crate::Reg<u32, _OTG_HS_DVBUSDIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DVBUSDIS;
#[doc = "`read()` method returns [otg_hs_dvbusdis::R](otg_hs_dvbusdis::R) reader structure"]
impl crate::Readable for OTG_HS_DVBUSDIS {}
#[doc = "`write(|w| ..)` method takes [otg_hs_dvbusdis::W](otg_hs_dvbusdis::W) writer structure"]
impl crate::Writable for OTG_HS_DVBUSDIS {}
#[doc = "OTG_HS device VBUS discharge time register"]
pub mod otg_hs_dvbusdis;
#[doc = "OTG_HS device VBUS pulsing time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_dvbuspulse](otg_hs_dvbuspulse) module"]
pub type OTG_HS_DVBUSPULSE = crate::Reg<u32, _OTG_HS_DVBUSPULSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DVBUSPULSE;
#[doc = "`read()` method returns [otg_hs_dvbuspulse::R](otg_hs_dvbuspulse::R) reader structure"]
impl crate::Readable for OTG_HS_DVBUSPULSE {}
#[doc = "`write(|w| ..)` method takes [otg_hs_dvbuspulse::W](otg_hs_dvbuspulse::W) writer structure"]
impl crate::Writable for OTG_HS_DVBUSPULSE {}
#[doc = "OTG_HS device VBUS pulsing time register"]
pub mod otg_hs_dvbuspulse;
#[doc = "OTG_HS Device threshold control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_dthrctl](otg_hs_dthrctl) module"]
pub type OTG_HS_DTHRCTL = crate::Reg<u32, _OTG_HS_DTHRCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DTHRCTL;
#[doc = "`read()` method returns [otg_hs_dthrctl::R](otg_hs_dthrctl::R) reader structure"]
impl crate::Readable for OTG_HS_DTHRCTL {}
#[doc = "`write(|w| ..)` method takes [otg_hs_dthrctl::W](otg_hs_dthrctl::W) writer structure"]
impl crate::Writable for OTG_HS_DTHRCTL {}
#[doc = "OTG_HS Device threshold control register"]
pub mod otg_hs_dthrctl;
#[doc = "OTG_HS device IN endpoint FIFO empty interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepempmsk](otg_hs_diepempmsk) module"]
pub type OTG_HS_DIEPEMPMSK = crate::Reg<u32, _OTG_HS_DIEPEMPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPEMPMSK;
#[doc = "`read()` method returns [otg_hs_diepempmsk::R](otg_hs_diepempmsk::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPEMPMSK {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepempmsk::W](otg_hs_diepempmsk::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPEMPMSK {}
#[doc = "OTG_HS device IN endpoint FIFO empty interrupt mask register"]
pub mod otg_hs_diepempmsk;
#[doc = "OTG_HS device each endpoint interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_deachint](otg_hs_deachint) module"]
pub type OTG_HS_DEACHINT = crate::Reg<u32, _OTG_HS_DEACHINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DEACHINT;
#[doc = "`read()` method returns [otg_hs_deachint::R](otg_hs_deachint::R) reader structure"]
impl crate::Readable for OTG_HS_DEACHINT {}
#[doc = "`write(|w| ..)` method takes [otg_hs_deachint::W](otg_hs_deachint::W) writer structure"]
impl crate::Writable for OTG_HS_DEACHINT {}
#[doc = "OTG_HS device each endpoint interrupt register"]
pub mod otg_hs_deachint;
#[doc = "OTG_HS device each endpoint interrupt register mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_deachintmsk](otg_hs_deachintmsk) module"]
pub type OTG_HS_DEACHINTMSK = crate::Reg<u32, _OTG_HS_DEACHINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DEACHINTMSK;
#[doc = "`read()` method returns [otg_hs_deachintmsk::R](otg_hs_deachintmsk::R) reader structure"]
impl crate::Readable for OTG_HS_DEACHINTMSK {}
#[doc = "`write(|w| ..)` method takes [otg_hs_deachintmsk::W](otg_hs_deachintmsk::W) writer structure"]
impl crate::Writable for OTG_HS_DEACHINTMSK {}
#[doc = "OTG_HS device each endpoint interrupt register mask"]
pub mod otg_hs_deachintmsk;
#[doc = "OTG device endpoint-0 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepctl0](otg_hs_diepctl0) module"]
pub type OTG_HS_DIEPCTL0 = crate::Reg<u32, _OTG_HS_DIEPCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPCTL0;
#[doc = "`read()` method returns [otg_hs_diepctl0::R](otg_hs_diepctl0::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPCTL0 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepctl0::W](otg_hs_diepctl0::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPCTL0 {}
#[doc = "OTG device endpoint-0 control register"]
pub mod otg_hs_diepctl0;
#[doc = "OTG device endpoint-1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepctl1](otg_hs_diepctl1) module"]
pub type OTG_HS_DIEPCTL1 = crate::Reg<u32, _OTG_HS_DIEPCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPCTL1;
#[doc = "`read()` method returns [otg_hs_diepctl1::R](otg_hs_diepctl1::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPCTL1 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepctl1::W](otg_hs_diepctl1::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPCTL1 {}
#[doc = "OTG device endpoint-1 control register"]
pub mod otg_hs_diepctl1;
#[doc = "OTG device endpoint-2 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepctl2](otg_hs_diepctl2) module"]
pub type OTG_HS_DIEPCTL2 = crate::Reg<u32, _OTG_HS_DIEPCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPCTL2;
#[doc = "`read()` method returns [otg_hs_diepctl2::R](otg_hs_diepctl2::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPCTL2 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepctl2::W](otg_hs_diepctl2::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPCTL2 {}
#[doc = "OTG device endpoint-2 control register"]
pub mod otg_hs_diepctl2;
#[doc = "OTG device endpoint-3 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepctl3](otg_hs_diepctl3) module"]
pub type OTG_HS_DIEPCTL3 = crate::Reg<u32, _OTG_HS_DIEPCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPCTL3;
#[doc = "`read()` method returns [otg_hs_diepctl3::R](otg_hs_diepctl3::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPCTL3 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepctl3::W](otg_hs_diepctl3::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPCTL3 {}
#[doc = "OTG device endpoint-3 control register"]
pub mod otg_hs_diepctl3;
#[doc = "OTG device endpoint-4 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepctl4](otg_hs_diepctl4) module"]
pub type OTG_HS_DIEPCTL4 = crate::Reg<u32, _OTG_HS_DIEPCTL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPCTL4;
#[doc = "`read()` method returns [otg_hs_diepctl4::R](otg_hs_diepctl4::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPCTL4 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepctl4::W](otg_hs_diepctl4::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPCTL4 {}
#[doc = "OTG device endpoint-4 control register"]
pub mod otg_hs_diepctl4;
#[doc = "OTG device endpoint-5 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepctl5](otg_hs_diepctl5) module"]
pub type OTG_HS_DIEPCTL5 = crate::Reg<u32, _OTG_HS_DIEPCTL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPCTL5;
#[doc = "`read()` method returns [otg_hs_diepctl5::R](otg_hs_diepctl5::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPCTL5 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepctl5::W](otg_hs_diepctl5::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPCTL5 {}
#[doc = "OTG device endpoint-5 control register"]
pub mod otg_hs_diepctl5;
#[doc = "OTG device endpoint-6 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepctl6](otg_hs_diepctl6) module"]
pub type OTG_HS_DIEPCTL6 = crate::Reg<u32, _OTG_HS_DIEPCTL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPCTL6;
#[doc = "`read()` method returns [otg_hs_diepctl6::R](otg_hs_diepctl6::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPCTL6 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepctl6::W](otg_hs_diepctl6::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPCTL6 {}
#[doc = "OTG device endpoint-6 control register"]
pub mod otg_hs_diepctl6;
#[doc = "OTG device endpoint-7 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepctl7](otg_hs_diepctl7) module"]
pub type OTG_HS_DIEPCTL7 = crate::Reg<u32, _OTG_HS_DIEPCTL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPCTL7;
#[doc = "`read()` method returns [otg_hs_diepctl7::R](otg_hs_diepctl7::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPCTL7 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepctl7::W](otg_hs_diepctl7::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPCTL7 {}
#[doc = "OTG device endpoint-7 control register"]
pub mod otg_hs_diepctl7;
#[doc = "OTG device endpoint-0 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepint0](otg_hs_diepint0) module"]
pub type OTG_HS_DIEPINT0 = crate::Reg<u32, _OTG_HS_DIEPINT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPINT0;
#[doc = "`read()` method returns [otg_hs_diepint0::R](otg_hs_diepint0::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPINT0 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepint0::W](otg_hs_diepint0::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPINT0 {}
#[doc = "OTG device endpoint-0 interrupt register"]
pub mod otg_hs_diepint0;
#[doc = "OTG device endpoint-1 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepint1](otg_hs_diepint1) module"]
pub type OTG_HS_DIEPINT1 = crate::Reg<u32, _OTG_HS_DIEPINT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPINT1;
#[doc = "`read()` method returns [otg_hs_diepint1::R](otg_hs_diepint1::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPINT1 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepint1::W](otg_hs_diepint1::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPINT1 {}
#[doc = "OTG device endpoint-1 interrupt register"]
pub mod otg_hs_diepint1;
#[doc = "OTG device endpoint-2 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepint2](otg_hs_diepint2) module"]
pub type OTG_HS_DIEPINT2 = crate::Reg<u32, _OTG_HS_DIEPINT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPINT2;
#[doc = "`read()` method returns [otg_hs_diepint2::R](otg_hs_diepint2::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPINT2 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepint2::W](otg_hs_diepint2::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPINT2 {}
#[doc = "OTG device endpoint-2 interrupt register"]
pub mod otg_hs_diepint2;
#[doc = "OTG device endpoint-3 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepint3](otg_hs_diepint3) module"]
pub type OTG_HS_DIEPINT3 = crate::Reg<u32, _OTG_HS_DIEPINT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPINT3;
#[doc = "`read()` method returns [otg_hs_diepint3::R](otg_hs_diepint3::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPINT3 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepint3::W](otg_hs_diepint3::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPINT3 {}
#[doc = "OTG device endpoint-3 interrupt register"]
pub mod otg_hs_diepint3;
#[doc = "OTG device endpoint-4 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepint4](otg_hs_diepint4) module"]
pub type OTG_HS_DIEPINT4 = crate::Reg<u32, _OTG_HS_DIEPINT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPINT4;
#[doc = "`read()` method returns [otg_hs_diepint4::R](otg_hs_diepint4::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPINT4 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepint4::W](otg_hs_diepint4::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPINT4 {}
#[doc = "OTG device endpoint-4 interrupt register"]
pub mod otg_hs_diepint4;
#[doc = "OTG device endpoint-5 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepint5](otg_hs_diepint5) module"]
pub type OTG_HS_DIEPINT5 = crate::Reg<u32, _OTG_HS_DIEPINT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPINT5;
#[doc = "`read()` method returns [otg_hs_diepint5::R](otg_hs_diepint5::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPINT5 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepint5::W](otg_hs_diepint5::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPINT5 {}
#[doc = "OTG device endpoint-5 interrupt register"]
pub mod otg_hs_diepint5;
#[doc = "OTG device endpoint-6 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepint6](otg_hs_diepint6) module"]
pub type OTG_HS_DIEPINT6 = crate::Reg<u32, _OTG_HS_DIEPINT6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPINT6;
#[doc = "`read()` method returns [otg_hs_diepint6::R](otg_hs_diepint6::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPINT6 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepint6::W](otg_hs_diepint6::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPINT6 {}
#[doc = "OTG device endpoint-6 interrupt register"]
pub mod otg_hs_diepint6;
#[doc = "OTG device endpoint-7 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepint7](otg_hs_diepint7) module"]
pub type OTG_HS_DIEPINT7 = crate::Reg<u32, _OTG_HS_DIEPINT7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPINT7;
#[doc = "`read()` method returns [otg_hs_diepint7::R](otg_hs_diepint7::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPINT7 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepint7::W](otg_hs_diepint7::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPINT7 {}
#[doc = "OTG device endpoint-7 interrupt register"]
pub mod otg_hs_diepint7;
#[doc = "OTG_HS device IN endpoint 0 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_dieptsiz0](otg_hs_dieptsiz0) module"]
pub type OTG_HS_DIEPTSIZ0 = crate::Reg<u32, _OTG_HS_DIEPTSIZ0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPTSIZ0;
#[doc = "`read()` method returns [otg_hs_dieptsiz0::R](otg_hs_dieptsiz0::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPTSIZ0 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_dieptsiz0::W](otg_hs_dieptsiz0::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPTSIZ0 {}
#[doc = "OTG_HS device IN endpoint 0 transfer size register"]
pub mod otg_hs_dieptsiz0;
#[doc = "OTG_HS device endpoint-1 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepdma0](otg_hs_diepdma0) module"]
pub type OTG_HS_DIEPDMA0 = crate::Reg<u32, _OTG_HS_DIEPDMA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPDMA0;
#[doc = "`read()` method returns [otg_hs_diepdma0::R](otg_hs_diepdma0::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPDMA0 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepdma0::W](otg_hs_diepdma0::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPDMA0 {}
#[doc = "OTG_HS device endpoint-1 DMA address register"]
pub mod otg_hs_diepdma0;
#[doc = "OTG_HS device endpoint-2 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepdma1](otg_hs_diepdma1) module"]
pub type OTG_HS_DIEPDMA1 = crate::Reg<u32, _OTG_HS_DIEPDMA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPDMA1;
#[doc = "`read()` method returns [otg_hs_diepdma1::R](otg_hs_diepdma1::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPDMA1 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepdma1::W](otg_hs_diepdma1::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPDMA1 {}
#[doc = "OTG_HS device endpoint-2 DMA address register"]
pub mod otg_hs_diepdma1;
#[doc = "OTG_HS device endpoint-3 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepdma2](otg_hs_diepdma2) module"]
pub type OTG_HS_DIEPDMA2 = crate::Reg<u32, _OTG_HS_DIEPDMA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPDMA2;
#[doc = "`read()` method returns [otg_hs_diepdma2::R](otg_hs_diepdma2::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPDMA2 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepdma2::W](otg_hs_diepdma2::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPDMA2 {}
#[doc = "OTG_HS device endpoint-3 DMA address register"]
pub mod otg_hs_diepdma2;
#[doc = "OTG_HS device endpoint-4 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepdma3](otg_hs_diepdma3) module"]
pub type OTG_HS_DIEPDMA3 = crate::Reg<u32, _OTG_HS_DIEPDMA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPDMA3;
#[doc = "`read()` method returns [otg_hs_diepdma3::R](otg_hs_diepdma3::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPDMA3 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepdma3::W](otg_hs_diepdma3::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPDMA3 {}
#[doc = "OTG_HS device endpoint-4 DMA address register"]
pub mod otg_hs_diepdma3;
#[doc = "OTG_HS device endpoint-5 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepdma4](otg_hs_diepdma4) module"]
pub type OTG_HS_DIEPDMA4 = crate::Reg<u32, _OTG_HS_DIEPDMA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPDMA4;
#[doc = "`read()` method returns [otg_hs_diepdma4::R](otg_hs_diepdma4::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPDMA4 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepdma4::W](otg_hs_diepdma4::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPDMA4 {}
#[doc = "OTG_HS device endpoint-5 DMA address register"]
pub mod otg_hs_diepdma4;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_dtxfsts0](otg_hs_dtxfsts0) module"]
pub type OTG_HS_DTXFSTS0 = crate::Reg<u32, _OTG_HS_DTXFSTS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DTXFSTS0;
#[doc = "`read()` method returns [otg_hs_dtxfsts0::R](otg_hs_dtxfsts0::R) reader structure"]
impl crate::Readable for OTG_HS_DTXFSTS0 {}
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod otg_hs_dtxfsts0;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_dtxfsts1](otg_hs_dtxfsts1) module"]
pub type OTG_HS_DTXFSTS1 = crate::Reg<u32, _OTG_HS_DTXFSTS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DTXFSTS1;
#[doc = "`read()` method returns [otg_hs_dtxfsts1::R](otg_hs_dtxfsts1::R) reader structure"]
impl crate::Readable for OTG_HS_DTXFSTS1 {}
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod otg_hs_dtxfsts1;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_dtxfsts2](otg_hs_dtxfsts2) module"]
pub type OTG_HS_DTXFSTS2 = crate::Reg<u32, _OTG_HS_DTXFSTS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DTXFSTS2;
#[doc = "`read()` method returns [otg_hs_dtxfsts2::R](otg_hs_dtxfsts2::R) reader structure"]
impl crate::Readable for OTG_HS_DTXFSTS2 {}
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod otg_hs_dtxfsts2;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_dtxfsts3](otg_hs_dtxfsts3) module"]
pub type OTG_HS_DTXFSTS3 = crate::Reg<u32, _OTG_HS_DTXFSTS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DTXFSTS3;
#[doc = "`read()` method returns [otg_hs_dtxfsts3::R](otg_hs_dtxfsts3::R) reader structure"]
impl crate::Readable for OTG_HS_DTXFSTS3 {}
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod otg_hs_dtxfsts3;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_dtxfsts4](otg_hs_dtxfsts4) module"]
pub type OTG_HS_DTXFSTS4 = crate::Reg<u32, _OTG_HS_DTXFSTS4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DTXFSTS4;
#[doc = "`read()` method returns [otg_hs_dtxfsts4::R](otg_hs_dtxfsts4::R) reader structure"]
impl crate::Readable for OTG_HS_DTXFSTS4 {}
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod otg_hs_dtxfsts4;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_dtxfsts5](otg_hs_dtxfsts5) module"]
pub type OTG_HS_DTXFSTS5 = crate::Reg<u32, _OTG_HS_DTXFSTS5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DTXFSTS5;
#[doc = "`read()` method returns [otg_hs_dtxfsts5::R](otg_hs_dtxfsts5::R) reader structure"]
impl crate::Readable for OTG_HS_DTXFSTS5 {}
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod otg_hs_dtxfsts5;
#[doc = "OTG_HS device endpoint transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_dieptsiz1](otg_hs_dieptsiz1) module"]
pub type OTG_HS_DIEPTSIZ1 = crate::Reg<u32, _OTG_HS_DIEPTSIZ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPTSIZ1;
#[doc = "`read()` method returns [otg_hs_dieptsiz1::R](otg_hs_dieptsiz1::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPTSIZ1 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_dieptsiz1::W](otg_hs_dieptsiz1::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPTSIZ1 {}
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod otg_hs_dieptsiz1;
#[doc = "OTG_HS device endpoint transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_dieptsiz2](otg_hs_dieptsiz2) module"]
pub type OTG_HS_DIEPTSIZ2 = crate::Reg<u32, _OTG_HS_DIEPTSIZ2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPTSIZ2;
#[doc = "`read()` method returns [otg_hs_dieptsiz2::R](otg_hs_dieptsiz2::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPTSIZ2 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_dieptsiz2::W](otg_hs_dieptsiz2::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPTSIZ2 {}
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod otg_hs_dieptsiz2;
#[doc = "OTG_HS device endpoint transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_dieptsiz3](otg_hs_dieptsiz3) module"]
pub type OTG_HS_DIEPTSIZ3 = crate::Reg<u32, _OTG_HS_DIEPTSIZ3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPTSIZ3;
#[doc = "`read()` method returns [otg_hs_dieptsiz3::R](otg_hs_dieptsiz3::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPTSIZ3 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_dieptsiz3::W](otg_hs_dieptsiz3::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPTSIZ3 {}
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod otg_hs_dieptsiz3;
#[doc = "OTG_HS device endpoint transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_dieptsiz4](otg_hs_dieptsiz4) module"]
pub type OTG_HS_DIEPTSIZ4 = crate::Reg<u32, _OTG_HS_DIEPTSIZ4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPTSIZ4;
#[doc = "`read()` method returns [otg_hs_dieptsiz4::R](otg_hs_dieptsiz4::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPTSIZ4 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_dieptsiz4::W](otg_hs_dieptsiz4::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPTSIZ4 {}
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod otg_hs_dieptsiz4;
#[doc = "OTG_HS device endpoint transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_dieptsiz5](otg_hs_dieptsiz5) module"]
pub type OTG_HS_DIEPTSIZ5 = crate::Reg<u32, _OTG_HS_DIEPTSIZ5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPTSIZ5;
#[doc = "`read()` method returns [otg_hs_dieptsiz5::R](otg_hs_dieptsiz5::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPTSIZ5 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_dieptsiz5::W](otg_hs_dieptsiz5::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPTSIZ5 {}
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod otg_hs_dieptsiz5;
#[doc = "OTG_HS device control OUT endpoint 0 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepctl0](otg_hs_doepctl0) module"]
pub type OTG_HS_DOEPCTL0 = crate::Reg<u32, _OTG_HS_DOEPCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPCTL0;
#[doc = "`read()` method returns [otg_hs_doepctl0::R](otg_hs_doepctl0::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPCTL0 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepctl0::W](otg_hs_doepctl0::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPCTL0 {}
#[doc = "OTG_HS device control OUT endpoint 0 control register"]
pub mod otg_hs_doepctl0;
#[doc = "OTG device endpoint-1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepctl1](otg_hs_doepctl1) module"]
pub type OTG_HS_DOEPCTL1 = crate::Reg<u32, _OTG_HS_DOEPCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPCTL1;
#[doc = "`read()` method returns [otg_hs_doepctl1::R](otg_hs_doepctl1::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPCTL1 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepctl1::W](otg_hs_doepctl1::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPCTL1 {}
#[doc = "OTG device endpoint-1 control register"]
pub mod otg_hs_doepctl1;
#[doc = "OTG device endpoint-2 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepctl2](otg_hs_doepctl2) module"]
pub type OTG_HS_DOEPCTL2 = crate::Reg<u32, _OTG_HS_DOEPCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPCTL2;
#[doc = "`read()` method returns [otg_hs_doepctl2::R](otg_hs_doepctl2::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPCTL2 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepctl2::W](otg_hs_doepctl2::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPCTL2 {}
#[doc = "OTG device endpoint-2 control register"]
pub mod otg_hs_doepctl2;
#[doc = "OTG device endpoint-3 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepctl3](otg_hs_doepctl3) module"]
pub type OTG_HS_DOEPCTL3 = crate::Reg<u32, _OTG_HS_DOEPCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPCTL3;
#[doc = "`read()` method returns [otg_hs_doepctl3::R](otg_hs_doepctl3::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPCTL3 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepctl3::W](otg_hs_doepctl3::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPCTL3 {}
#[doc = "OTG device endpoint-3 control register"]
pub mod otg_hs_doepctl3;
#[doc = "OTG_HS device endpoint-0 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepint0](otg_hs_doepint0) module"]
pub type OTG_HS_DOEPINT0 = crate::Reg<u32, _OTG_HS_DOEPINT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPINT0;
#[doc = "`read()` method returns [otg_hs_doepint0::R](otg_hs_doepint0::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPINT0 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepint0::W](otg_hs_doepint0::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPINT0 {}
#[doc = "OTG_HS device endpoint-0 interrupt register"]
pub mod otg_hs_doepint0;
#[doc = "OTG_HS device endpoint-1 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepint1](otg_hs_doepint1) module"]
pub type OTG_HS_DOEPINT1 = crate::Reg<u32, _OTG_HS_DOEPINT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPINT1;
#[doc = "`read()` method returns [otg_hs_doepint1::R](otg_hs_doepint1::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPINT1 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepint1::W](otg_hs_doepint1::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPINT1 {}
#[doc = "OTG_HS device endpoint-1 interrupt register"]
pub mod otg_hs_doepint1;
#[doc = "OTG_HS device endpoint-2 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepint2](otg_hs_doepint2) module"]
pub type OTG_HS_DOEPINT2 = crate::Reg<u32, _OTG_HS_DOEPINT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPINT2;
#[doc = "`read()` method returns [otg_hs_doepint2::R](otg_hs_doepint2::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPINT2 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepint2::W](otg_hs_doepint2::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPINT2 {}
#[doc = "OTG_HS device endpoint-2 interrupt register"]
pub mod otg_hs_doepint2;
#[doc = "OTG_HS device endpoint-3 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepint3](otg_hs_doepint3) module"]
pub type OTG_HS_DOEPINT3 = crate::Reg<u32, _OTG_HS_DOEPINT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPINT3;
#[doc = "`read()` method returns [otg_hs_doepint3::R](otg_hs_doepint3::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPINT3 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepint3::W](otg_hs_doepint3::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPINT3 {}
#[doc = "OTG_HS device endpoint-3 interrupt register"]
pub mod otg_hs_doepint3;
#[doc = "OTG_HS device endpoint-4 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepint4](otg_hs_doepint4) module"]
pub type OTG_HS_DOEPINT4 = crate::Reg<u32, _OTG_HS_DOEPINT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPINT4;
#[doc = "`read()` method returns [otg_hs_doepint4::R](otg_hs_doepint4::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPINT4 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepint4::W](otg_hs_doepint4::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPINT4 {}
#[doc = "OTG_HS device endpoint-4 interrupt register"]
pub mod otg_hs_doepint4;
#[doc = "OTG_HS device endpoint-5 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepint5](otg_hs_doepint5) module"]
pub type OTG_HS_DOEPINT5 = crate::Reg<u32, _OTG_HS_DOEPINT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPINT5;
#[doc = "`read()` method returns [otg_hs_doepint5::R](otg_hs_doepint5::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPINT5 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepint5::W](otg_hs_doepint5::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPINT5 {}
#[doc = "OTG_HS device endpoint-5 interrupt register"]
pub mod otg_hs_doepint5;
#[doc = "OTG_HS device endpoint-6 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepint6](otg_hs_doepint6) module"]
pub type OTG_HS_DOEPINT6 = crate::Reg<u32, _OTG_HS_DOEPINT6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPINT6;
#[doc = "`read()` method returns [otg_hs_doepint6::R](otg_hs_doepint6::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPINT6 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepint6::W](otg_hs_doepint6::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPINT6 {}
#[doc = "OTG_HS device endpoint-6 interrupt register"]
pub mod otg_hs_doepint6;
#[doc = "OTG_HS device endpoint-7 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepint7](otg_hs_doepint7) module"]
pub type OTG_HS_DOEPINT7 = crate::Reg<u32, _OTG_HS_DOEPINT7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPINT7;
#[doc = "`read()` method returns [otg_hs_doepint7::R](otg_hs_doepint7::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPINT7 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepint7::W](otg_hs_doepint7::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPINT7 {}
#[doc = "OTG_HS device endpoint-7 interrupt register"]
pub mod otg_hs_doepint7;
#[doc = "OTG_HS device endpoint-0 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doeptsiz0](otg_hs_doeptsiz0) module"]
pub type OTG_HS_DOEPTSIZ0 = crate::Reg<u32, _OTG_HS_DOEPTSIZ0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPTSIZ0;
#[doc = "`read()` method returns [otg_hs_doeptsiz0::R](otg_hs_doeptsiz0::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPTSIZ0 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doeptsiz0::W](otg_hs_doeptsiz0::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPTSIZ0 {}
#[doc = "OTG_HS device endpoint-0 transfer size register"]
pub mod otg_hs_doeptsiz0;
#[doc = "OTG_HS device endpoint-1 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doeptsiz1](otg_hs_doeptsiz1) module"]
pub type OTG_HS_DOEPTSIZ1 = crate::Reg<u32, _OTG_HS_DOEPTSIZ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPTSIZ1;
#[doc = "`read()` method returns [otg_hs_doeptsiz1::R](otg_hs_doeptsiz1::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPTSIZ1 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doeptsiz1::W](otg_hs_doeptsiz1::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPTSIZ1 {}
#[doc = "OTG_HS device endpoint-1 transfer size register"]
pub mod otg_hs_doeptsiz1;
#[doc = "OTG_HS device endpoint-2 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doeptsiz2](otg_hs_doeptsiz2) module"]
pub type OTG_HS_DOEPTSIZ2 = crate::Reg<u32, _OTG_HS_DOEPTSIZ2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPTSIZ2;
#[doc = "`read()` method returns [otg_hs_doeptsiz2::R](otg_hs_doeptsiz2::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPTSIZ2 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doeptsiz2::W](otg_hs_doeptsiz2::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPTSIZ2 {}
#[doc = "OTG_HS device endpoint-2 transfer size register"]
pub mod otg_hs_doeptsiz2;
#[doc = "OTG_HS device endpoint-3 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doeptsiz3](otg_hs_doeptsiz3) module"]
pub type OTG_HS_DOEPTSIZ3 = crate::Reg<u32, _OTG_HS_DOEPTSIZ3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPTSIZ3;
#[doc = "`read()` method returns [otg_hs_doeptsiz3::R](otg_hs_doeptsiz3::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPTSIZ3 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doeptsiz3::W](otg_hs_doeptsiz3::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPTSIZ3 {}
#[doc = "OTG_HS device endpoint-3 transfer size register"]
pub mod otg_hs_doeptsiz3;
#[doc = "OTG_HS device endpoint-4 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doeptsiz4](otg_hs_doeptsiz4) module"]
pub type OTG_HS_DOEPTSIZ4 = crate::Reg<u32, _OTG_HS_DOEPTSIZ4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPTSIZ4;
#[doc = "`read()` method returns [otg_hs_doeptsiz4::R](otg_hs_doeptsiz4::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPTSIZ4 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doeptsiz4::W](otg_hs_doeptsiz4::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPTSIZ4 {}
#[doc = "OTG_HS device endpoint-4 transfer size register"]
pub mod otg_hs_doeptsiz4;
#[doc = "OTG_HS device endpoint transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_dieptsiz6](otg_hs_dieptsiz6) module"]
pub type OTG_HS_DIEPTSIZ6 = crate::Reg<u32, _OTG_HS_DIEPTSIZ6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPTSIZ6;
#[doc = "`read()` method returns [otg_hs_dieptsiz6::R](otg_hs_dieptsiz6::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPTSIZ6 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_dieptsiz6::W](otg_hs_dieptsiz6::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPTSIZ6 {}
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod otg_hs_dieptsiz6;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_dtxfsts6](otg_hs_dtxfsts6) module"]
pub type OTG_HS_DTXFSTS6 = crate::Reg<u32, _OTG_HS_DTXFSTS6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DTXFSTS6;
#[doc = "`read()` method returns [otg_hs_dtxfsts6::R](otg_hs_dtxfsts6::R) reader structure"]
impl crate::Readable for OTG_HS_DTXFSTS6 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_dtxfsts6::W](otg_hs_dtxfsts6::W) writer structure"]
impl crate::Writable for OTG_HS_DTXFSTS6 {}
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod otg_hs_dtxfsts6;
#[doc = "OTG_HS device endpoint transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_dieptsiz7](otg_hs_dieptsiz7) module"]
pub type OTG_HS_DIEPTSIZ7 = crate::Reg<u32, _OTG_HS_DIEPTSIZ7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPTSIZ7;
#[doc = "`read()` method returns [otg_hs_dieptsiz7::R](otg_hs_dieptsiz7::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPTSIZ7 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_dieptsiz7::W](otg_hs_dieptsiz7::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPTSIZ7 {}
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod otg_hs_dieptsiz7;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_dtxfsts7](otg_hs_dtxfsts7) module"]
pub type OTG_HS_DTXFSTS7 = crate::Reg<u32, _OTG_HS_DTXFSTS7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DTXFSTS7;
#[doc = "`read()` method returns [otg_hs_dtxfsts7::R](otg_hs_dtxfsts7::R) reader structure"]
impl crate::Readable for OTG_HS_DTXFSTS7 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_dtxfsts7::W](otg_hs_dtxfsts7::W) writer structure"]
impl crate::Writable for OTG_HS_DTXFSTS7 {}
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod otg_hs_dtxfsts7;
#[doc = "OTG device endpoint-4 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepctl4](otg_hs_doepctl4) module"]
pub type OTG_HS_DOEPCTL4 = crate::Reg<u32, _OTG_HS_DOEPCTL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPCTL4;
#[doc = "`read()` method returns [otg_hs_doepctl4::R](otg_hs_doepctl4::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPCTL4 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepctl4::W](otg_hs_doepctl4::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPCTL4 {}
#[doc = "OTG device endpoint-4 control register"]
pub mod otg_hs_doepctl4;
#[doc = "OTG device endpoint-5 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepctl5](otg_hs_doepctl5) module"]
pub type OTG_HS_DOEPCTL5 = crate::Reg<u32, _OTG_HS_DOEPCTL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPCTL5;
#[doc = "`read()` method returns [otg_hs_doepctl5::R](otg_hs_doepctl5::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPCTL5 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepctl5::W](otg_hs_doepctl5::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPCTL5 {}
#[doc = "OTG device endpoint-5 control register"]
pub mod otg_hs_doepctl5;
#[doc = "OTG device endpoint-6 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepctl6](otg_hs_doepctl6) module"]
pub type OTG_HS_DOEPCTL6 = crate::Reg<u32, _OTG_HS_DOEPCTL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPCTL6;
#[doc = "`read()` method returns [otg_hs_doepctl6::R](otg_hs_doepctl6::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPCTL6 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepctl6::W](otg_hs_doepctl6::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPCTL6 {}
#[doc = "OTG device endpoint-6 control register"]
pub mod otg_hs_doepctl6;
#[doc = "OTG device endpoint-7 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepctl7](otg_hs_doepctl7) module"]
pub type OTG_HS_DOEPCTL7 = crate::Reg<u32, _OTG_HS_DOEPCTL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPCTL7;
#[doc = "`read()` method returns [otg_hs_doepctl7::R](otg_hs_doepctl7::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPCTL7 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepctl7::W](otg_hs_doepctl7::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPCTL7 {}
#[doc = "OTG device endpoint-7 control register"]
pub mod otg_hs_doepctl7;
#[doc = "OTG_HS device endpoint-5 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doeptsiz5](otg_hs_doeptsiz5) module"]
pub type OTG_HS_DOEPTSIZ5 = crate::Reg<u32, _OTG_HS_DOEPTSIZ5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPTSIZ5;
#[doc = "`read()` method returns [otg_hs_doeptsiz5::R](otg_hs_doeptsiz5::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPTSIZ5 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doeptsiz5::W](otg_hs_doeptsiz5::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPTSIZ5 {}
#[doc = "OTG_HS device endpoint-5 transfer size register"]
pub mod otg_hs_doeptsiz5;
#[doc = "OTG_HS device endpoint-6 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doeptsiz6](otg_hs_doeptsiz6) module"]
pub type OTG_HS_DOEPTSIZ6 = crate::Reg<u32, _OTG_HS_DOEPTSIZ6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPTSIZ6;
#[doc = "`read()` method returns [otg_hs_doeptsiz6::R](otg_hs_doeptsiz6::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPTSIZ6 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doeptsiz6::W](otg_hs_doeptsiz6::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPTSIZ6 {}
#[doc = "OTG_HS device endpoint-6 transfer size register"]
pub mod otg_hs_doeptsiz6;
#[doc = "OTG_HS device endpoint-7 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doeptsiz7](otg_hs_doeptsiz7) module"]
pub type OTG_HS_DOEPTSIZ7 = crate::Reg<u32, _OTG_HS_DOEPTSIZ7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPTSIZ7;
#[doc = "`read()` method returns [otg_hs_doeptsiz7::R](otg_hs_doeptsiz7::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPTSIZ7 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doeptsiz7::W](otg_hs_doeptsiz7::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPTSIZ7 {}
#[doc = "OTG_HS device endpoint-7 transfer size register"]
pub mod otg_hs_doeptsiz7;
#[doc = "OTG Device channel-x DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepdma0](otg_hs_doepdma0) module"]
pub type OTG_HS_DOEPDMA0 = crate::Reg<u32, _OTG_HS_DOEPDMA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPDMA0;
#[doc = "`read()` method returns [otg_hs_doepdma0::R](otg_hs_doepdma0::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPDMA0 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepdma0::W](otg_hs_doepdma0::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPDMA0 {}
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_doepdma0;
#[doc = "OTG Device channel-x DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepdma1](otg_hs_doepdma1) module"]
pub type OTG_HS_DOEPDMA1 = crate::Reg<u32, _OTG_HS_DOEPDMA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPDMA1;
#[doc = "`read()` method returns [otg_hs_doepdma1::R](otg_hs_doepdma1::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPDMA1 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepdma1::W](otg_hs_doepdma1::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPDMA1 {}
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_doepdma1;
#[doc = "OTG Device channel-x DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepdma2](otg_hs_doepdma2) module"]
pub type OTG_HS_DOEPDMA2 = crate::Reg<u32, _OTG_HS_DOEPDMA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPDMA2;
#[doc = "`read()` method returns [otg_hs_doepdma2::R](otg_hs_doepdma2::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPDMA2 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepdma2::W](otg_hs_doepdma2::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPDMA2 {}
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_doepdma2;
#[doc = "OTG Device channel-x DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepdma3](otg_hs_doepdma3) module"]
pub type OTG_HS_DOEPDMA3 = crate::Reg<u32, _OTG_HS_DOEPDMA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPDMA3;
#[doc = "`read()` method returns [otg_hs_doepdma3::R](otg_hs_doepdma3::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPDMA3 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepdma3::W](otg_hs_doepdma3::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPDMA3 {}
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_doepdma3;
#[doc = "OTG Device channel-x DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepdma4](otg_hs_doepdma4) module"]
pub type OTG_HS_DOEPDMA4 = crate::Reg<u32, _OTG_HS_DOEPDMA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPDMA4;
#[doc = "`read()` method returns [otg_hs_doepdma4::R](otg_hs_doepdma4::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPDMA4 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepdma4::W](otg_hs_doepdma4::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPDMA4 {}
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_doepdma4;
#[doc = "OTG Device channel-x DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepdma5](otg_hs_doepdma5) module"]
pub type OTG_HS_DOEPDMA5 = crate::Reg<u32, _OTG_HS_DOEPDMA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPDMA5;
#[doc = "`read()` method returns [otg_hs_doepdma5::R](otg_hs_doepdma5::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPDMA5 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepdma5::W](otg_hs_doepdma5::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPDMA5 {}
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_doepdma5;
#[doc = "OTG Device channel-x DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepdma6](otg_hs_doepdma6) module"]
pub type OTG_HS_DOEPDMA6 = crate::Reg<u32, _OTG_HS_DOEPDMA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPDMA6;
#[doc = "`read()` method returns [otg_hs_doepdma6::R](otg_hs_doepdma6::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPDMA6 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepdma6::W](otg_hs_doepdma6::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPDMA6 {}
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_doepdma6;
#[doc = "OTG Device channel-x DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepdma7](otg_hs_doepdma7) module"]
pub type OTG_HS_DOEPDMA7 = crate::Reg<u32, _OTG_HS_DOEPDMA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPDMA7;
#[doc = "`read()` method returns [otg_hs_doepdma7::R](otg_hs_doepdma7::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPDMA7 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepdma7::W](otg_hs_doepdma7::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPDMA7 {}
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_doepdma7;
#[doc = "OTG Device channel-x DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepdma8](otg_hs_doepdma8) module"]
pub type OTG_HS_DOEPDMA8 = crate::Reg<u32, _OTG_HS_DOEPDMA8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPDMA8;
#[doc = "`read()` method returns [otg_hs_doepdma8::R](otg_hs_doepdma8::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPDMA8 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepdma8::W](otg_hs_doepdma8::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPDMA8 {}
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_doepdma8;
#[doc = "OTG Device channel-x DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepdma9](otg_hs_doepdma9) module"]
pub type OTG_HS_DOEPDMA9 = crate::Reg<u32, _OTG_HS_DOEPDMA9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPDMA9;
#[doc = "`read()` method returns [otg_hs_doepdma9::R](otg_hs_doepdma9::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPDMA9 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepdma9::W](otg_hs_doepdma9::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPDMA9 {}
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_doepdma9;
#[doc = "OTG Device channel-x DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepdma10](otg_hs_doepdma10) module"]
pub type OTG_HS_DOEPDMA10 = crate::Reg<u32, _OTG_HS_DOEPDMA10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPDMA10;
#[doc = "`read()` method returns [otg_hs_doepdma10::R](otg_hs_doepdma10::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPDMA10 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepdma10::W](otg_hs_doepdma10::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPDMA10 {}
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_doepdma10;
#[doc = "OTG Device channel-x DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepdma11](otg_hs_doepdma11) module"]
pub type OTG_HS_DOEPDMA11 = crate::Reg<u32, _OTG_HS_DOEPDMA11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPDMA11;
#[doc = "`read()` method returns [otg_hs_doepdma11::R](otg_hs_doepdma11::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPDMA11 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepdma11::W](otg_hs_doepdma11::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPDMA11 {}
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_doepdma11;
#[doc = "OTG Device channel-x DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepdma12](otg_hs_doepdma12) module"]
pub type OTG_HS_DOEPDMA12 = crate::Reg<u32, _OTG_HS_DOEPDMA12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPDMA12;
#[doc = "`read()` method returns [otg_hs_doepdma12::R](otg_hs_doepdma12::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPDMA12 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepdma12::W](otg_hs_doepdma12::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPDMA12 {}
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_doepdma12;
#[doc = "OTG Device channel-x DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepdma13](otg_hs_doepdma13) module"]
pub type OTG_HS_DOEPDMA13 = crate::Reg<u32, _OTG_HS_DOEPDMA13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPDMA13;
#[doc = "`read()` method returns [otg_hs_doepdma13::R](otg_hs_doepdma13::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPDMA13 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepdma13::W](otg_hs_doepdma13::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPDMA13 {}
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_doepdma13;
#[doc = "OTG Device channel-x DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepdma14](otg_hs_doepdma14) module"]
pub type OTG_HS_DOEPDMA14 = crate::Reg<u32, _OTG_HS_DOEPDMA14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPDMA14;
#[doc = "`read()` method returns [otg_hs_doepdma14::R](otg_hs_doepdma14::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPDMA14 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepdma14::W](otg_hs_doepdma14::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPDMA14 {}
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_doepdma14;
#[doc = "OTG Device channel-x DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepdma15](otg_hs_doepdma15) module"]
pub type OTG_HS_DOEPDMA15 = crate::Reg<u32, _OTG_HS_DOEPDMA15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPDMA15;
#[doc = "`read()` method returns [otg_hs_doepdma15::R](otg_hs_doepdma15::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPDMA15 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepdma15::W](otg_hs_doepdma15::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPDMA15 {}
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_doepdma15;
#[doc = "OTG Device channel-x DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepdma5](otg_hs_diepdma5) module"]
pub type OTG_HS_DIEPDMA5 = crate::Reg<u32, _OTG_HS_DIEPDMA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPDMA5;
#[doc = "`read()` method returns [otg_hs_diepdma5::R](otg_hs_diepdma5::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPDMA5 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepdma5::W](otg_hs_diepdma5::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPDMA5 {}
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_diepdma5;
#[doc = "OTG Device channel-x DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepdma6](otg_hs_diepdma6) module"]
pub type OTG_HS_DIEPDMA6 = crate::Reg<u32, _OTG_HS_DIEPDMA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPDMA6;
#[doc = "`read()` method returns [otg_hs_diepdma6::R](otg_hs_diepdma6::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPDMA6 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepdma6::W](otg_hs_diepdma6::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPDMA6 {}
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_diepdma6;
#[doc = "OTG Device channel-x DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepdma7](otg_hs_diepdma7) module"]
pub type OTG_HS_DIEPDMA7 = crate::Reg<u32, _OTG_HS_DIEPDMA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPDMA7;
#[doc = "`read()` method returns [otg_hs_diepdma7::R](otg_hs_diepdma7::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPDMA7 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepdma7::W](otg_hs_diepdma7::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPDMA7 {}
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_diepdma7;
#[doc = "OTG Device channel-x DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepdma8](otg_hs_diepdma8) module"]
pub type OTG_HS_DIEPDMA8 = crate::Reg<u32, _OTG_HS_DIEPDMA8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPDMA8;
#[doc = "`read()` method returns [otg_hs_diepdma8::R](otg_hs_diepdma8::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPDMA8 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepdma8::W](otg_hs_diepdma8::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPDMA8 {}
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_diepdma8;
#[doc = "OTG Device channel-x DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepdma9](otg_hs_diepdma9) module"]
pub type OTG_HS_DIEPDMA9 = crate::Reg<u32, _OTG_HS_DIEPDMA9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPDMA9;
#[doc = "`read()` method returns [otg_hs_diepdma9::R](otg_hs_diepdma9::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPDMA9 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepdma9::W](otg_hs_diepdma9::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPDMA9 {}
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_diepdma9;
#[doc = "OTG Device channel-x DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepdma10](otg_hs_diepdma10) module"]
pub type OTG_HS_DIEPDMA10 = crate::Reg<u32, _OTG_HS_DIEPDMA10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPDMA10;
#[doc = "`read()` method returns [otg_hs_diepdma10::R](otg_hs_diepdma10::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPDMA10 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepdma10::W](otg_hs_diepdma10::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPDMA10 {}
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_diepdma10;
#[doc = "OTG Device channel-x DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepdma11](otg_hs_diepdma11) module"]
pub type OTG_HS_DIEPDMA11 = crate::Reg<u32, _OTG_HS_DIEPDMA11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPDMA11;
#[doc = "`read()` method returns [otg_hs_diepdma11::R](otg_hs_diepdma11::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPDMA11 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepdma11::W](otg_hs_diepdma11::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPDMA11 {}
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_diepdma11;
#[doc = "OTG Device channel-x DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepdma12](otg_hs_diepdma12) module"]
pub type OTG_HS_DIEPDMA12 = crate::Reg<u32, _OTG_HS_DIEPDMA12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPDMA12;
#[doc = "`read()` method returns [otg_hs_diepdma12::R](otg_hs_diepdma12::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPDMA12 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepdma12::W](otg_hs_diepdma12::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPDMA12 {}
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_diepdma12;
#[doc = "OTG Device channel-x DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepdma13](otg_hs_diepdma13) module"]
pub type OTG_HS_DIEPDMA13 = crate::Reg<u32, _OTG_HS_DIEPDMA13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPDMA13;
#[doc = "`read()` method returns [otg_hs_diepdma13::R](otg_hs_diepdma13::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPDMA13 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepdma13::W](otg_hs_diepdma13::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPDMA13 {}
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_diepdma13;
#[doc = "OTG Device channel-x DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepdma14](otg_hs_diepdma14) module"]
pub type OTG_HS_DIEPDMA14 = crate::Reg<u32, _OTG_HS_DIEPDMA14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPDMA14;
#[doc = "`read()` method returns [otg_hs_diepdma14::R](otg_hs_diepdma14::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPDMA14 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepdma14::W](otg_hs_diepdma14::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPDMA14 {}
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_diepdma14;
#[doc = "OTG Device channel-x DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepdma15](otg_hs_diepdma15) module"]
pub type OTG_HS_DIEPDMA15 = crate::Reg<u32, _OTG_HS_DIEPDMA15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPDMA15;
#[doc = "`read()` method returns [otg_hs_diepdma15::R](otg_hs_diepdma15::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPDMA15 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepdma15::W](otg_hs_diepdma15::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPDMA15 {}
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_diepdma15;
