#[doc = "Reader of register CFGR"]
pub type R = crate::R<u32, super::CFGR>;
#[doc = "Writer for register CFGR"]
pub type W = crate::W<u32, super::CFGR>;
#[doc = "Register CFGR `reset()`'s with value 0x0007_0000"]
impl crate::ResetValue for super::CFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0007_0000
    }
}
#[doc = "Reader of field `MCOPRE`"]
pub type MCOPRE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCOPRE`"]
pub struct MCOPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCOPRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Reader of field `MCOSEL`"]
pub type MCOSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCOSEL`"]
pub struct MCOSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCOSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `PPRE2F`"]
pub type PPRE2F_R = crate::R<bool, bool>;
#[doc = "Reader of field `PPRE1F`"]
pub type PPRE1F_R = crate::R<bool, bool>;
#[doc = "Reader of field `HPREF`"]
pub type HPREF_R = crate::R<bool, bool>;
#[doc = "Reader of field `STOPWUCK`"]
pub type STOPWUCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOPWUCK`"]
pub struct STOPWUCK_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPWUCK_W<'a> {
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
#[doc = "Reader of field `PPRE2`"]
pub type PPRE2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PPRE2`"]
pub struct PPRE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PPRE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Reader of field `PPRE1`"]
pub type PPRE1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PPRE1`"]
pub struct PPRE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PPRE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `HPRE`"]
pub type HPRE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HPRE`"]
pub struct HPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> HPRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `SWS`"]
pub type SWS_R = crate::R<u8, u8>;
#[doc = "Reader of field `SW`"]
pub type SW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SW`"]
pub struct SW_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:30 - Microcontroller clock output prescaler"]
    #[inline(always)]
    pub fn mcopre(&self) -> MCOPRE_R {
        MCOPRE_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 24:27 - Microcontroller clock output"]
    #[inline(always)]
    pub fn mcosel(&self) -> MCOSEL_R {
        MCOSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - PCLK2 prescaler flag (APB2)"]
    #[inline(always)]
    pub fn ppre2f(&self) -> PPRE2F_R {
        PPRE2F_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PCLK1 prescaler flag (APB1)"]
    #[inline(always)]
    pub fn ppre1f(&self) -> PPRE1F_R {
        PPRE1F_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - HCLK1 prescaler flag (CPU1, AHB1, AHB2, and SRAM1)"]
    #[inline(always)]
    pub fn hpref(&self) -> HPREF_R {
        HPREF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Wakeup from Stop and CSS backup clock selection"]
    #[inline(always)]
    pub fn stopwuck(&self) -> STOPWUCK_R {
        STOPWUCK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - PCLK2 high-speed prescaler (APB2)"]
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - PCLK1 low-speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:7 - HCLK1 prescaler (CPU1, AHB1, AHB2, and SRAM1.)"]
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 2:3 - System clock switch status"]
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 28:30 - Microcontroller clock output prescaler"]
    #[inline(always)]
    pub fn mcopre(&mut self) -> MCOPRE_W {
        MCOPRE_W { w: self }
    }
    #[doc = "Bits 24:27 - Microcontroller clock output"]
    #[inline(always)]
    pub fn mcosel(&mut self) -> MCOSEL_W {
        MCOSEL_W { w: self }
    }
    #[doc = "Bit 15 - Wakeup from Stop and CSS backup clock selection"]
    #[inline(always)]
    pub fn stopwuck(&mut self) -> STOPWUCK_W {
        STOPWUCK_W { w: self }
    }
    #[doc = "Bits 11:13 - PCLK2 high-speed prescaler (APB2)"]
    #[inline(always)]
    pub fn ppre2(&mut self) -> PPRE2_W {
        PPRE2_W { w: self }
    }
    #[doc = "Bits 8:10 - PCLK1 low-speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre1(&mut self) -> PPRE1_W {
        PPRE1_W { w: self }
    }
    #[doc = "Bits 4:7 - HCLK1 prescaler (CPU1, AHB1, AHB2, and SRAM1.)"]
    #[inline(always)]
    pub fn hpre(&mut self) -> HPRE_W {
        HPRE_W { w: self }
    }
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W {
        SW_W { w: self }
    }
}