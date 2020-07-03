#[doc = "Reader of register OPAMP6_TCMR"]
pub type R = crate::R<u32, super::OPAMP6_TCMR>;
#[doc = "Writer for register OPAMP6_TCMR"]
pub type W = crate::W<u32, super::OPAMP6_TCMR>;
#[doc = "Register OPAMP6_TCMR `reset()`'s with value 0"]
impl crate::ResetValue for super::OPAMP6_TCMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VMS_SEL`"]
pub type VMS_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VMS_SEL`"]
pub struct VMS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VMS_SEL_W<'a> {
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
#[doc = "Reader of field `VPS_SEL`"]
pub type VPS_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VPS_SEL`"]
pub struct VPS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VPS_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `T1CM_EN`"]
pub type T1CM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T1CM_EN`"]
pub struct T1CM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> T1CM_EN_W<'a> {
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
#[doc = "Reader of field `T8CM_EN`"]
pub type T8CM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T8CM_EN`"]
pub struct T8CM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> T8CM_EN_W<'a> {
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
#[doc = "Reader of field `T20CM_EN`"]
pub type T20CM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T20CM_EN`"]
pub struct T20CM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> T20CM_EN_W<'a> {
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
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK`"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - VMS_SEL"]
    #[inline(always)]
    pub fn vms_sel(&self) -> VMS_SEL_R {
        VMS_SEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - VPS_SEL"]
    #[inline(always)]
    pub fn vps_sel(&self) -> VPS_SEL_R {
        VPS_SEL_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - T1CM_EN"]
    #[inline(always)]
    pub fn t1cm_en(&self) -> T1CM_EN_R {
        T1CM_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - T8CM_EN"]
    #[inline(always)]
    pub fn t8cm_en(&self) -> T8CM_EN_R {
        T8CM_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - T20CM_EN"]
    #[inline(always)]
    pub fn t20cm_en(&self) -> T20CM_EN_R {
        T20CM_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VMS_SEL"]
    #[inline(always)]
    pub fn vms_sel(&mut self) -> VMS_SEL_W {
        VMS_SEL_W { w: self }
    }
    #[doc = "Bits 1:2 - VPS_SEL"]
    #[inline(always)]
    pub fn vps_sel(&mut self) -> VPS_SEL_W {
        VPS_SEL_W { w: self }
    }
    #[doc = "Bit 3 - T1CM_EN"]
    #[inline(always)]
    pub fn t1cm_en(&mut self) -> T1CM_EN_W {
        T1CM_EN_W { w: self }
    }
    #[doc = "Bit 4 - T8CM_EN"]
    #[inline(always)]
    pub fn t8cm_en(&mut self) -> T8CM_EN_W {
        T8CM_EN_W { w: self }
    }
    #[doc = "Bit 5 - T20CM_EN"]
    #[inline(always)]
    pub fn t20cm_en(&mut self) -> T20CM_EN_W {
        T20CM_EN_W { w: self }
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
}
