#[doc = "Reader of register OAR2"]
pub type R = crate::R<u32, super::OAR2>;
#[doc = "Writer for register OAR2"]
pub type W = crate::W<u32, super::OAR2>;
#[doc = "Register OAR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::OAR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OA21_7`"]
pub type OA21_7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OA21_7`"]
pub struct OA21_7_W<'a> {
    w: &'a mut W,
}
impl<'a> OA21_7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u32) & 0x7f) << 1);
        self.w
    }
}
#[doc = "Reader of field `OA2MSK`"]
pub type OA2MSK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OA2MSK`"]
pub struct OA2MSK_W<'a> {
    w: &'a mut W,
}
impl<'a> OA2MSK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `OA2EN`"]
pub type OA2EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OA2EN`"]
pub struct OA2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OA2EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:7 - OA21_7"]
    #[inline(always)]
    pub fn oa21_7(&self) -> OA21_7_R {
        OA21_7_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:10 - OA2MSK"]
    #[inline(always)]
    pub fn oa2msk(&self) -> OA2MSK_R {
        OA2MSK_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 15 - OA2EN"]
    #[inline(always)]
    pub fn oa2en(&self) -> OA2EN_R {
        OA2EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:7 - OA21_7"]
    #[inline(always)]
    pub fn oa21_7(&mut self) -> OA21_7_W {
        OA21_7_W { w: self }
    }
    #[doc = "Bits 8:10 - OA2MSK"]
    #[inline(always)]
    pub fn oa2msk(&mut self) -> OA2MSK_W {
        OA2MSK_W { w: self }
    }
    #[doc = "Bit 15 - OA2EN"]
    #[inline(always)]
    pub fn oa2en(&mut self) -> OA2EN_W {
        OA2EN_W { w: self }
    }
}
