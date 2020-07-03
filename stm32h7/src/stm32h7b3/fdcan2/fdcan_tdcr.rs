#[doc = "Reader of register FDCAN_TDCR"]
pub type R = crate::R<u32, super::FDCAN_TDCR>;
#[doc = "Reader of field `TDCF`"]
pub type TDCF_R = crate::R<u8, u8>;
#[doc = "Reader of field `TDCO`"]
pub type TDCO_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - Transmitter Delay Compensation Filter Window Length"]
    #[inline(always)]
    pub fn tdcf(&self) -> TDCF_R {
        TDCF_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Transmitter Delay Compensation Offset"]
    #[inline(always)]
    pub fn tdco(&self) -> TDCO_R {
        TDCO_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
