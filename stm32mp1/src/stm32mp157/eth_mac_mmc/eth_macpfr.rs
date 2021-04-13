#[doc = "Reader of register ETH_MACPFR"]
pub type R = crate::R<u32, super::ETH_MACPFR>;
#[doc = "Writer for register ETH_MACPFR"]
pub type W = crate::W<u32, super::ETH_MACPFR>;
#[doc = "Register ETH_MACPFR `reset()`'s with value 0"]
impl crate::ResetValue for super::ETH_MACPFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PR`"]
pub type PR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PR`"]
pub struct PR_W<'a> {
    w: &'a mut W,
}
impl<'a> PR_W<'a> {
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
#[doc = "Reader of field `HUC`"]
pub type HUC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HUC`"]
pub struct HUC_W<'a> {
    w: &'a mut W,
}
impl<'a> HUC_W<'a> {
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
#[doc = "Reader of field `HMC`"]
pub type HMC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HMC`"]
pub struct HMC_W<'a> {
    w: &'a mut W,
}
impl<'a> HMC_W<'a> {
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
#[doc = "Reader of field `DAIF`"]
pub type DAIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAIF`"]
pub struct DAIF_W<'a> {
    w: &'a mut W,
}
impl<'a> DAIF_W<'a> {
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
#[doc = "Reader of field `PM`"]
pub type PM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PM`"]
pub struct PM_W<'a> {
    w: &'a mut W,
}
impl<'a> PM_W<'a> {
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
#[doc = "Reader of field `DBF`"]
pub type DBF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBF`"]
pub struct DBF_W<'a> {
    w: &'a mut W,
}
impl<'a> DBF_W<'a> {
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
#[doc = "Reader of field `PCF`"]
pub type PCF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCF`"]
pub struct PCF_W<'a> {
    w: &'a mut W,
}
impl<'a> PCF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `SAIF`"]
pub type SAIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAIF`"]
pub struct SAIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SAIF_W<'a> {
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
#[doc = "Reader of field `SAF`"]
pub type SAF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAF`"]
pub struct SAF_W<'a> {
    w: &'a mut W,
}
impl<'a> SAF_W<'a> {
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
#[doc = "Reader of field `HPF`"]
pub type HPF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HPF`"]
pub struct HPF_W<'a> {
    w: &'a mut W,
}
impl<'a> HPF_W<'a> {
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
#[doc = "Reader of field `VTFE`"]
pub type VTFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VTFE`"]
pub struct VTFE_W<'a> {
    w: &'a mut W,
}
impl<'a> VTFE_W<'a> {
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
#[doc = "Reader of field `IPFE`"]
pub type IPFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPFE`"]
pub struct IPFE_W<'a> {
    w: &'a mut W,
}
impl<'a> IPFE_W<'a> {
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
#[doc = "Reader of field `DNTU`"]
pub type DNTU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DNTU`"]
pub struct DNTU_W<'a> {
    w: &'a mut W,
}
impl<'a> DNTU_W<'a> {
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
#[doc = "Reader of field `RA`"]
pub type RA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RA`"]
pub struct RA_W<'a> {
    w: &'a mut W,
}
impl<'a> RA_W<'a> {
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
    #[doc = "Bit 0 - PR"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HUC"]
    #[inline(always)]
    pub fn huc(&self) -> HUC_R {
        HUC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HMC"]
    #[inline(always)]
    pub fn hmc(&self) -> HMC_R {
        HMC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DAIF"]
    #[inline(always)]
    pub fn daif(&self) -> DAIF_R {
        DAIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PM"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DBF"]
    #[inline(always)]
    pub fn dbf(&self) -> DBF_R {
        DBF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - PCF"]
    #[inline(always)]
    pub fn pcf(&self) -> PCF_R {
        PCF_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - SAIF"]
    #[inline(always)]
    pub fn saif(&self) -> SAIF_R {
        SAIF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SAF"]
    #[inline(always)]
    pub fn saf(&self) -> SAF_R {
        SAF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - HPF"]
    #[inline(always)]
    pub fn hpf(&self) -> HPF_R {
        HPF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 16 - VTFE"]
    #[inline(always)]
    pub fn vtfe(&self) -> VTFE_R {
        VTFE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - IPFE"]
    #[inline(always)]
    pub fn ipfe(&self) -> IPFE_R {
        IPFE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - DNTU"]
    #[inline(always)]
    pub fn dntu(&self) -> DNTU_R {
        DNTU_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 31 - RA"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PR"]
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W {
        PR_W { w: self }
    }
    #[doc = "Bit 1 - HUC"]
    #[inline(always)]
    pub fn huc(&mut self) -> HUC_W {
        HUC_W { w: self }
    }
    #[doc = "Bit 2 - HMC"]
    #[inline(always)]
    pub fn hmc(&mut self) -> HMC_W {
        HMC_W { w: self }
    }
    #[doc = "Bit 3 - DAIF"]
    #[inline(always)]
    pub fn daif(&mut self) -> DAIF_W {
        DAIF_W { w: self }
    }
    #[doc = "Bit 4 - PM"]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W {
        PM_W { w: self }
    }
    #[doc = "Bit 5 - DBF"]
    #[inline(always)]
    pub fn dbf(&mut self) -> DBF_W {
        DBF_W { w: self }
    }
    #[doc = "Bits 6:7 - PCF"]
    #[inline(always)]
    pub fn pcf(&mut self) -> PCF_W {
        PCF_W { w: self }
    }
    #[doc = "Bit 8 - SAIF"]
    #[inline(always)]
    pub fn saif(&mut self) -> SAIF_W {
        SAIF_W { w: self }
    }
    #[doc = "Bit 9 - SAF"]
    #[inline(always)]
    pub fn saf(&mut self) -> SAF_W {
        SAF_W { w: self }
    }
    #[doc = "Bit 10 - HPF"]
    #[inline(always)]
    pub fn hpf(&mut self) -> HPF_W {
        HPF_W { w: self }
    }
    #[doc = "Bit 16 - VTFE"]
    #[inline(always)]
    pub fn vtfe(&mut self) -> VTFE_W {
        VTFE_W { w: self }
    }
    #[doc = "Bit 20 - IPFE"]
    #[inline(always)]
    pub fn ipfe(&mut self) -> IPFE_W {
        IPFE_W { w: self }
    }
    #[doc = "Bit 21 - DNTU"]
    #[inline(always)]
    pub fn dntu(&mut self) -> DNTU_W {
        DNTU_W { w: self }
    }
    #[doc = "Bit 31 - RA"]
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W {
        RA_W { w: self }
    }
}
