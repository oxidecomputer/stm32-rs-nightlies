#[doc = "Writer for register CICR"]
pub type W = crate::W<u32, super::CICR>;
#[doc = "Register CICR `reset()`'s with value 0"]
impl crate::ResetValue for super::CICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `LSI2RDYC`"]
pub struct LSI2RDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> LSI2RDYC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Write proxy for field `HSI48RDYC`"]
pub struct HSI48RDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI48RDYC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Write proxy for field `LSECSSC`"]
pub struct LSECSSC_W<'a> {
    w: &'a mut W,
}
impl<'a> LSECSSC_W<'a> {
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
#[doc = "Write proxy for field `HSECSSC`"]
pub struct HSECSSC_W<'a> {
    w: &'a mut W,
}
impl<'a> HSECSSC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `PLLSAI1RDYC`"]
pub struct PLLSAI1RDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI1RDYC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Write proxy for field `PLLRDYC`"]
pub struct PLLRDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLRDYC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Write proxy for field `HSERDYC`"]
pub struct HSERDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> HSERDYC_W<'a> {
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
#[doc = "Write proxy for field `HSIRDYC`"]
pub struct HSIRDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIRDYC_W<'a> {
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
#[doc = "Write proxy for field `MSIRDYC`"]
pub struct MSIRDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> MSIRDYC_W<'a> {
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
#[doc = "Write proxy for field `LSERDYC`"]
pub struct LSERDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> LSERDYC_W<'a> {
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
#[doc = "Write proxy for field `LSI1RDYC`"]
pub struct LSI1RDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> LSI1RDYC_W<'a> {
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
impl W {
    #[doc = "Bit 11 - LSI2 ready interrupt clear"]
    #[inline(always)]
    pub fn lsi2rdyc(&mut self) -> LSI2RDYC_W {
        LSI2RDYC_W { w: self }
    }
    #[doc = "Bit 10 - HSI48 ready interrupt clear"]
    #[inline(always)]
    pub fn hsi48rdyc(&mut self) -> HSI48RDYC_W {
        HSI48RDYC_W { w: self }
    }
    #[doc = "Bit 9 - LSE Clock security system interrupt clear"]
    #[inline(always)]
    pub fn lsecssc(&mut self) -> LSECSSC_W {
        LSECSSC_W { w: self }
    }
    #[doc = "Bit 8 - HSE Clock security system interrupt clear"]
    #[inline(always)]
    pub fn hsecssc(&mut self) -> HSECSSC_W {
        HSECSSC_W { w: self }
    }
    #[doc = "Bit 6 - PLLSAI1 ready interrupt clear"]
    #[inline(always)]
    pub fn pllsai1rdyc(&mut self) -> PLLSAI1RDYC_W {
        PLLSAI1RDYC_W { w: self }
    }
    #[doc = "Bit 5 - PLL ready interrupt clear"]
    #[inline(always)]
    pub fn pllrdyc(&mut self) -> PLLRDYC_W {
        PLLRDYC_W { w: self }
    }
    #[doc = "Bit 4 - HSE ready interrupt clear"]
    #[inline(always)]
    pub fn hserdyc(&mut self) -> HSERDYC_W {
        HSERDYC_W { w: self }
    }
    #[doc = "Bit 3 - HSI ready interrupt clear"]
    #[inline(always)]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W {
        HSIRDYC_W { w: self }
    }
    #[doc = "Bit 2 - MSI ready interrupt clear"]
    #[inline(always)]
    pub fn msirdyc(&mut self) -> MSIRDYC_W {
        MSIRDYC_W { w: self }
    }
    #[doc = "Bit 1 - LSE ready interrupt clear"]
    #[inline(always)]
    pub fn lserdyc(&mut self) -> LSERDYC_W {
        LSERDYC_W { w: self }
    }
    #[doc = "Bit 0 - LSI1 ready interrupt clear"]
    #[inline(always)]
    pub fn lsi1rdyc(&mut self) -> LSI1RDYC_W {
        LSI1RDYC_W { w: self }
    }
}
