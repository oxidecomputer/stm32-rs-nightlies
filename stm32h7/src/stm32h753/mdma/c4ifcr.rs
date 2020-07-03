#[doc = "Writer for register C4IFCR"]
pub type W = crate::W<u32, super::C4IFCR>;
#[doc = "Register C4IFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::C4IFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CTEIF4`"]
pub struct CTEIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Write proxy for field `CCTCIF4`"]
pub struct CCTCIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CCTCIF4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Write proxy for field `CBRTIF4`"]
pub struct CBRTIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CBRTIF4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Write proxy for field `CBTIF4`"]
pub struct CBTIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CBTIF4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Write proxy for field `CLTCIF4`"]
pub struct CLTCIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CLTCIF4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cteif4(&mut self) -> CTEIF4_W {
        CTEIF4_W { w: self }
    }
    #[doc = "Bit 1 - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cctcif4(&mut self) -> CCTCIF4_W {
        CCTCIF4_W { w: self }
    }
    #[doc = "Bit 2 - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbrtif4(&mut self) -> CBRTIF4_W {
        CBRTIF4_W { w: self }
    }
    #[doc = "Bit 3 - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbtif4(&mut self) -> CBTIF4_W {
        CBTIF4_W { w: self }
    }
    #[doc = "Bit 4 - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cltcif4(&mut self) -> CLTCIF4_W {
        CLTCIF4_W { w: self }
    }
}
