#[doc = "Reader of register DBTP"]
pub type R = crate::R<u32, super::DBTP>;
#[doc = "Reader of field `DSJW`"]
pub type DSJW_R = crate::R<u8, u8>;
#[doc = "Reader of field `DTSEG2`"]
pub type DTSEG2_R = crate::R<u8, u8>;
#[doc = "Reader of field `DTSEG1`"]
pub type DTSEG1_R = crate::R<u8, u8>;
#[doc = "Reader of field `DBRP`"]
pub type DBRP_R = crate::R<u8, u8>;
#[doc = "Reader of field `TDC`"]
pub type TDC_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:3 - Synchronization Jump Width"]
    #[inline(always)]
    pub fn dsjw(&self) -> DSJW_R {
        DSJW_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Data time segment after sample point"]
    #[inline(always)]
    pub fn dtseg2(&self) -> DTSEG2_R {
        DTSEG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Data time segment after sample point"]
    #[inline(always)]
    pub fn dtseg1(&self) -> DTSEG1_R {
        DTSEG1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Data BIt Rate Prescaler"]
    #[inline(always)]
    pub fn dbrp(&self) -> DBRP_R {
        DBRP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Transceiver Delay Compensation"]
    #[inline(always)]
    pub fn tdc(&self) -> TDC_R {
        TDC_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
