#[doc = "Reader of register ECCR"]
pub type R = crate::R<u32, super::ECCR>;
#[doc = "Reader of field `ECC`"]
pub type ECC_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ECC result This field contains the value computed by the ECC computation logic. Table167 describes the contents of these bit fields."]
    #[inline(always)]
    pub fn ecc(&self) -> ECC_R {
        ECC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
