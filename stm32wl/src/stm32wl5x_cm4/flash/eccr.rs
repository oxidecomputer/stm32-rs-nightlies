#[doc = "Reader of register ECCR"]
pub type R = crate::R<u32, super::ECCR>;
#[doc = "Writer for register ECCR"]
pub type W = crate::W<u32, super::ECCR>;
#[doc = "Register ECCR `reset()`'s with value 0"]
impl crate::ResetValue for super::ECCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDR_ECC`"]
pub type ADDR_ECC_R = crate::R<u32, u32>;
#[doc = "Reader of field `SYSF_ECC`"]
pub type SYSF_ECC_R = crate::R<bool, bool>;
#[doc = "Reader of field `ECCCIE`"]
pub type ECCCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECCCIE`"]
pub struct ECCCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCCIE_W<'a> {
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
#[doc = "Reader of field `CPUID`"]
pub type CPUID_R = crate::R<u8, u8>;
#[doc = "Reader of field `ECCC`"]
pub type ECCC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECCC`"]
pub struct ECCC_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCC_W<'a> {
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
#[doc = "Reader of field `ECCD`"]
pub type ECCD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECCD`"]
pub struct ECCD_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCD_W<'a> {
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
    #[doc = "Bits 0:16 - ECC fail address"]
    #[inline(always)]
    pub fn addr_ecc(&self) -> ADDR_ECC_R {
        ADDR_ECC_R::new((self.bits & 0x0001_ffff) as u32)
    }
    #[doc = "Bit 20 - System Flash ECC fail"]
    #[inline(always)]
    pub fn sysf_ecc(&self) -> SYSF_ECC_R {
        SYSF_ECC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ECC correction interrupt enable"]
    #[inline(always)]
    pub fn ecccie(&self) -> ECCCIE_R {
        ECCCIE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 26:28 - CPU identification"]
    #[inline(always)]
    pub fn cpuid(&self) -> CPUID_R {
        CPUID_R::new(((self.bits >> 26) & 0x07) as u8)
    }
    #[doc = "Bit 30 - ECC correction"]
    #[inline(always)]
    pub fn eccc(&self) -> ECCC_R {
        ECCC_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - ECC detection"]
    #[inline(always)]
    pub fn eccd(&self) -> ECCD_R {
        ECCD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - ECC correction interrupt enable"]
    #[inline(always)]
    pub fn ecccie(&mut self) -> ECCCIE_W {
        ECCCIE_W { w: self }
    }
    #[doc = "Bit 30 - ECC correction"]
    #[inline(always)]
    pub fn eccc(&mut self) -> ECCC_W {
        ECCC_W { w: self }
    }
    #[doc = "Bit 31 - ECC detection"]
    #[inline(always)]
    pub fn eccd(&mut self) -> ECCD_W {
        ECCD_W { w: self }
    }
}
