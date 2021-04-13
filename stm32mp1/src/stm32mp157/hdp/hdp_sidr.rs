#[doc = "Reader of register HDP_SIDR"]
pub type R = crate::R<u32, super::HDP_SIDR>;
#[doc = "Reader of field `SID`"]
pub type SID_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - SID"]
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
