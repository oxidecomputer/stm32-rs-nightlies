#[doc = "Reader of register M4FECR"]
pub type R = crate::R<u32, super::M4FECR>;
#[doc = "Writer for register M4FECR"]
pub type W = crate::W<u32, super::M4FECR>;
#[doc = "Register M4FECR `reset()`'s with value 0"]
impl crate::ResetValue for super::M4FECR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FEC`"]
pub type FEC_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ECC failing code"]
    #[inline(always)]
    pub fn fec(&self) -> FEC_R {
        FEC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {}
