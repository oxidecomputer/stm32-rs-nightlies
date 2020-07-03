#[doc = "Reader of register OPAMP2_CSR"]
pub type R = crate::R<u32, super::OPAMP2_CSR>;
#[doc = "Writer for register OPAMP2_CSR"]
pub type W = crate::W<u32, super::OPAMP2_CSR>;
#[doc = "Register OPAMP2_CSR `reset()`'s with value 0"]
impl crate::ResetValue for super::OPAMP2_CSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OPAEN`"]
pub type OPAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPAEN`"]
pub struct OPAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAEN_W<'a> {
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
#[doc = "Reader of field `FORCE_VP`"]
pub type FORCE_VP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_VP`"]
pub struct FORCE_VP_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_VP_W<'a> {
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
#[doc = "Reader of field `VP_SEL`"]
pub type VP_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VP_SEL`"]
pub struct VP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VP_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `USERTRIM`"]
pub type USERTRIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USERTRIM`"]
pub struct USERTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> USERTRIM_W<'a> {
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
#[doc = "Reader of field `VM_SEL`"]
pub type VM_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VM_SEL`"]
pub struct VM_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VM_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `OPAHSM`"]
pub type OPAHSM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPAHSM`"]
pub struct OPAHSM_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAHSM_W<'a> {
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
#[doc = "Reader of field `OPAINTOEN`"]
pub type OPAINTOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPAINTOEN`"]
pub struct OPAINTOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAINTOEN_W<'a> {
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
#[doc = "Reader of field `CALON`"]
pub type CALON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CALON`"]
pub struct CALON_W<'a> {
    w: &'a mut W,
}
impl<'a> CALON_W<'a> {
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
#[doc = "Reader of field `CALSEL`"]
pub type CALSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CALSEL`"]
pub struct CALSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CALSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `PGA_GAIN`"]
pub type PGA_GAIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PGA_GAIN`"]
pub struct PGA_GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> PGA_GAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 14)) | (((value as u32) & 0x1f) << 14);
        self.w
    }
}
#[doc = "Reader of field `TRIMOFFSETP`"]
pub type TRIMOFFSETP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIMOFFSETP`"]
pub struct TRIMOFFSETP_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMOFFSETP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | (((value as u32) & 0x1f) << 19);
        self.w
    }
}
#[doc = "Reader of field `TRIMOFFSETN`"]
pub type TRIMOFFSETN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIMOFFSETN`"]
pub struct TRIMOFFSETN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMOFFSETN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
#[doc = "Reader of field `CALOUT`"]
pub type CALOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CALOUT`"]
pub struct CALOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CALOUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
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
    #[doc = "Bit 0 - Operational amplifier Enable"]
    #[inline(always)]
    pub fn opaen(&self) -> OPAEN_R {
        OPAEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FORCE_VP"]
    #[inline(always)]
    pub fn force_vp(&self) -> FORCE_VP_R {
        FORCE_VP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - VP_SEL"]
    #[inline(always)]
    pub fn vp_sel(&self) -> VP_SEL_R {
        VP_SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - USERTRIM"]
    #[inline(always)]
    pub fn usertrim(&self) -> USERTRIM_R {
        USERTRIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - VM_SEL"]
    #[inline(always)]
    pub fn vm_sel(&self) -> VM_SEL_R {
        VM_SEL_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - OPAHSM"]
    #[inline(always)]
    pub fn opahsm(&self) -> OPAHSM_R {
        OPAHSM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - OPAINTOEN"]
    #[inline(always)]
    pub fn opaintoen(&self) -> OPAINTOEN_R {
        OPAINTOEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CALON"]
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - CALSEL"]
    #[inline(always)]
    pub fn calsel(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:18 - PGA_GAIN"]
    #[inline(always)]
    pub fn pga_gain(&self) -> PGA_GAIN_R {
        PGA_GAIN_R::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - TRIMOFFSETP"]
    #[inline(always)]
    pub fn trimoffsetp(&self) -> TRIMOFFSETP_R {
        TRIMOFFSETP_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - TRIMOFFSETN"]
    #[inline(always)]
    pub fn trimoffsetn(&self) -> TRIMOFFSETN_R {
        TRIMOFFSETN_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - CALOUT"]
    #[inline(always)]
    pub fn calout(&self) -> CALOUT_R {
        CALOUT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operational amplifier Enable"]
    #[inline(always)]
    pub fn opaen(&mut self) -> OPAEN_W {
        OPAEN_W { w: self }
    }
    #[doc = "Bit 1 - FORCE_VP"]
    #[inline(always)]
    pub fn force_vp(&mut self) -> FORCE_VP_W {
        FORCE_VP_W { w: self }
    }
    #[doc = "Bits 2:3 - VP_SEL"]
    #[inline(always)]
    pub fn vp_sel(&mut self) -> VP_SEL_W {
        VP_SEL_W { w: self }
    }
    #[doc = "Bit 4 - USERTRIM"]
    #[inline(always)]
    pub fn usertrim(&mut self) -> USERTRIM_W {
        USERTRIM_W { w: self }
    }
    #[doc = "Bits 5:6 - VM_SEL"]
    #[inline(always)]
    pub fn vm_sel(&mut self) -> VM_SEL_W {
        VM_SEL_W { w: self }
    }
    #[doc = "Bit 7 - OPAHSM"]
    #[inline(always)]
    pub fn opahsm(&mut self) -> OPAHSM_W {
        OPAHSM_W { w: self }
    }
    #[doc = "Bit 8 - OPAINTOEN"]
    #[inline(always)]
    pub fn opaintoen(&mut self) -> OPAINTOEN_W {
        OPAINTOEN_W { w: self }
    }
    #[doc = "Bit 11 - CALON"]
    #[inline(always)]
    pub fn calon(&mut self) -> CALON_W {
        CALON_W { w: self }
    }
    #[doc = "Bits 12:13 - CALSEL"]
    #[inline(always)]
    pub fn calsel(&mut self) -> CALSEL_W {
        CALSEL_W { w: self }
    }
    #[doc = "Bits 14:18 - PGA_GAIN"]
    #[inline(always)]
    pub fn pga_gain(&mut self) -> PGA_GAIN_W {
        PGA_GAIN_W { w: self }
    }
    #[doc = "Bits 19:23 - TRIMOFFSETP"]
    #[inline(always)]
    pub fn trimoffsetp(&mut self) -> TRIMOFFSETP_W {
        TRIMOFFSETP_W { w: self }
    }
    #[doc = "Bits 24:28 - TRIMOFFSETN"]
    #[inline(always)]
    pub fn trimoffsetn(&mut self) -> TRIMOFFSETN_W {
        TRIMOFFSETN_W { w: self }
    }
    #[doc = "Bit 30 - CALOUT"]
    #[inline(always)]
    pub fn calout(&mut self) -> CALOUT_W {
        CALOUT_W { w: self }
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
}
