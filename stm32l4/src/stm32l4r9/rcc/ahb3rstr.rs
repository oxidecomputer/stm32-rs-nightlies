#[doc = "Reader of register AHB3RSTR"]
pub type R = crate::R<u32, super::AHB3RSTR>;
#[doc = "Writer for register AHB3RSTR"]
pub type W = crate::W<u32, super::AHB3RSTR>;
#[doc = "Register AHB3RSTR `reset()`'s with value 0"]
impl crate::ResetValue for super::AHB3RSTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FMCRST`"]
pub type FMCRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMCRST`"]
pub struct FMCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> FMCRST_W<'a> {
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
#[doc = "Reader of field `OSPI2RST`"]
pub type OSPI2RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSPI2RST`"]
pub struct OSPI2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPI2RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Flexible memory controller reset"]
    #[inline(always)]
    pub fn fmcrst(&self) -> FMCRST_R {
        FMCRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 9 - OctOSPI2 memory interface reset"]
    #[inline(always)]
    pub fn ospi2rst(&self) -> OSPI2RST_R {
        OSPI2RST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flexible memory controller reset"]
    #[inline(always)]
    pub fn fmcrst(&mut self) -> FMCRST_W {
        FMCRST_W { w: self }
    }
    #[doc = "Bit 9 - OctOSPI2 memory interface reset"]
    #[inline(always)]
    pub fn ospi2rst(&mut self) -> OSPI2RST_W {
        OSPI2RST_W { w: self }
    }
}
