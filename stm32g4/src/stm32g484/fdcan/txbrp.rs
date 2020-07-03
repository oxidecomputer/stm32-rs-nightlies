#[doc = "Reader of register TXBRP"]
pub type R = crate::R<u32, super::TXBRP>;
#[doc = "Reader of field `TRP`"]
pub type TRP_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - TRP"]
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
