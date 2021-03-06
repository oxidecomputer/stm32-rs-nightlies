#[doc = "Reader of register FDCAN_ENDN"]
pub type R = crate::R<u32, super::FDCAN_ENDN>;
#[doc = "Reader of field `ETV`"]
pub type ETV_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Endiannes Test Value"]
    #[inline(always)]
    pub fn etv(&self) -> ETV_R {
        ETV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
