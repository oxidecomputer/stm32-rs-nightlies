#[doc = "Reader of register PTPTSSR"]
pub type R = crate::R<u32, super::PTPTSSR>;
#[doc = "Reader of field `TSSO`"]
pub type TSSO_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSTTR`"]
pub type TSTTR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - TSSO"]
    #[inline(always)]
    pub fn tsso(&self) -> TSSO_R {
        TSSO_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TSSO"]
    #[inline(always)]
    pub fn tsttr(&self) -> TSTTR_R {
        TSTTR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
