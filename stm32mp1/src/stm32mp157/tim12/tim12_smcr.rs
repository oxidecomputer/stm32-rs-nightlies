#[doc = "Reader of register TIM12_SMCR"]
pub type R = crate::R<u32, super::TIM12_SMCR>;
#[doc = "Writer for register TIM12_SMCR"]
pub type W = crate::W<u32, super::TIM12_SMCR>;
#[doc = "Register TIM12_SMCR `reset()`'s with value 0"]
impl crate::ResetValue for super::TIM12_SMCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SMS`"]
pub type SMS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SMS`"]
pub struct SMS_W<'a> {
    w: &'a mut W,
}
impl<'a> SMS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `TS`"]
pub type TS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TS`"]
pub struct TS_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `MSM`"]
pub type MSM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSM`"]
pub struct MSM_W<'a> {
    w: &'a mut W,
}
impl<'a> MSM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `SMS_3`"]
pub type SMS_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMS_3`"]
pub struct SMS_3_W<'a> {
    w: &'a mut W,
}
impl<'a> SMS_3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `TS_3`"]
pub type TS_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_3`"]
pub struct TS_3_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `TS_4`"]
pub type TS_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_4`"]
pub struct TS_4_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - SMS"]
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - TS"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - MSM"]
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SMS_3"]
    #[inline(always)]
    pub fn sms_3(&self) -> SMS_3_R {
        SMS_3_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - TS_3"]
    #[inline(always)]
    pub fn ts_3(&self) -> TS_3_R {
        TS_3_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - TS_4"]
    #[inline(always)]
    pub fn ts_4(&self) -> TS_4_R {
        TS_4_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SMS"]
    #[inline(always)]
    pub fn sms(&mut self) -> SMS_W {
        SMS_W { w: self }
    }
    #[doc = "Bits 4:6 - TS"]
    #[inline(always)]
    pub fn ts(&mut self) -> TS_W {
        TS_W { w: self }
    }
    #[doc = "Bit 7 - MSM"]
    #[inline(always)]
    pub fn msm(&mut self) -> MSM_W {
        MSM_W { w: self }
    }
    #[doc = "Bit 16 - SMS_3"]
    #[inline(always)]
    pub fn sms_3(&mut self) -> SMS_3_W {
        SMS_3_W { w: self }
    }
    #[doc = "Bit 20 - TS_3"]
    #[inline(always)]
    pub fn ts_3(&mut self) -> TS_3_W {
        TS_3_W { w: self }
    }
    #[doc = "Bit 21 - TS_4"]
    #[inline(always)]
    pub fn ts_4(&mut self) -> TS_4_W {
        TS_4_W { w: self }
    }
}
