#[doc = "Reader of register ETH_MACATSNR"]
pub type R = crate::R<u32, super::ETH_MACATSNR>;
#[doc = "Reader of field `AUXTSLO`"]
pub type AUXTSLO_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:30 - AUXTSLO"]
    #[inline(always)]
    pub fn auxtslo(&self) -> AUXTSLO_R {
        AUXTSLO_R::new((self.bits & 0x7fff_ffff) as u32)
    }
}
