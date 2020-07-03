#[doc = "Reader of register D3PMR3"]
pub type R = crate::R<u32, super::D3PMR3>;
#[doc = "Writer for register D3PMR3"]
pub type W = crate::W<u32, super::D3PMR3>;
#[doc = "Register D3PMR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::D3PMR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MR88`"]
pub type MR88_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR88`"]
pub struct MR88_W<'a> {
    w: &'a mut W,
}
impl<'a> MR88_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 24 - D3 Pending Mask on Event input x+64"]
    #[inline(always)]
    pub fn mr88(&self) -> MR88_R {
        MR88_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - D3 Pending Mask on Event input x+64"]
    #[inline(always)]
    pub fn mr88(&mut self) -> MR88_W {
        MR88_W { w: self }
    }
}
