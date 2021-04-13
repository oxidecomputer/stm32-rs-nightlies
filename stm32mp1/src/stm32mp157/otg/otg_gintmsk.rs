#[doc = "Reader of register OTG_GINTMSK"]
pub type R = crate::R<u32, super::OTG_GINTMSK>;
#[doc = "Writer for register OTG_GINTMSK"]
pub type W = crate::W<u32, super::OTG_GINTMSK>;
#[doc = "Register OTG_GINTMSK `reset()`'s with value 0"]
impl crate::ResetValue for super::OTG_GINTMSK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MMISM`"]
pub type MMISM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MMISM`"]
pub struct MMISM_W<'a> {
    w: &'a mut W,
}
impl<'a> MMISM_W<'a> {
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
#[doc = "Reader of field `OTGINT`"]
pub type OTGINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OTGINT`"]
pub struct OTGINT_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGINT_W<'a> {
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
#[doc = "Reader of field `SOFM`"]
pub type SOFM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFM`"]
pub struct SOFM_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFM_W<'a> {
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
#[doc = "Reader of field `RXFLVLM`"]
pub type RXFLVLM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFLVLM`"]
pub struct RXFLVLM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFLVLM_W<'a> {
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
#[doc = "Reader of field `NPTXFEM`"]
pub type NPTXFEM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NPTXFEM`"]
pub struct NPTXFEM_W<'a> {
    w: &'a mut W,
}
impl<'a> NPTXFEM_W<'a> {
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
#[doc = "Reader of field `GINAKEFFM`"]
pub type GINAKEFFM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GINAKEFFM`"]
pub struct GINAKEFFM_W<'a> {
    w: &'a mut W,
}
impl<'a> GINAKEFFM_W<'a> {
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
#[doc = "Reader of field `GONAKEFFM`"]
pub type GONAKEFFM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GONAKEFFM`"]
pub struct GONAKEFFM_W<'a> {
    w: &'a mut W,
}
impl<'a> GONAKEFFM_W<'a> {
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
#[doc = "Reader of field `ESUSPM`"]
pub type ESUSPM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ESUSPM`"]
pub struct ESUSPM_W<'a> {
    w: &'a mut W,
}
impl<'a> ESUSPM_W<'a> {
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
#[doc = "Reader of field `USBSUSPM`"]
pub type USBSUSPM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBSUSPM`"]
pub struct USBSUSPM_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSUSPM_W<'a> {
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
#[doc = "Reader of field `USBRST`"]
pub type USBRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBRST`"]
pub struct USBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> USBRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `ENUMDNEM`"]
pub type ENUMDNEM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENUMDNEM`"]
pub struct ENUMDNEM_W<'a> {
    w: &'a mut W,
}
impl<'a> ENUMDNEM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `ISOODRPM`"]
pub type ISOODRPM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISOODRPM`"]
pub struct ISOODRPM_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOODRPM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `EOPFM`"]
pub type EOPFM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOPFM`"]
pub struct EOPFM_W<'a> {
    w: &'a mut W,
}
impl<'a> EOPFM_W<'a> {
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
#[doc = "Reader of field `IEPINT`"]
pub type IEPINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IEPINT`"]
pub struct IEPINT_W<'a> {
    w: &'a mut W,
}
impl<'a> IEPINT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `OEPINT`"]
pub type OEPINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OEPINT`"]
pub struct OEPINT_W<'a> {
    w: &'a mut W,
}
impl<'a> OEPINT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `IISOIXFRM`"]
pub type IISOIXFRM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IISOIXFRM`"]
pub struct IISOIXFRM_W<'a> {
    w: &'a mut W,
}
impl<'a> IISOIXFRM_W<'a> {
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
#[doc = "Reader of field `IPXFRM`"]
pub type IPXFRM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPXFRM`"]
pub struct IPXFRM_W<'a> {
    w: &'a mut W,
}
impl<'a> IPXFRM_W<'a> {
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
#[doc = "Reader of field `FSUSPM`"]
pub type FSUSPM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSUSPM`"]
pub struct FSUSPM_W<'a> {
    w: &'a mut W,
}
impl<'a> FSUSPM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `RSTDETM`"]
pub type RSTDETM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTDETM`"]
pub struct RSTDETM_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTDETM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `PRTIM`"]
pub type PRTIM_R = crate::R<bool, bool>;
#[doc = "Reader of field `HCIM`"]
pub type HCIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HCIM`"]
pub struct HCIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HCIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `PTXFEM`"]
pub type PTXFEM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PTXFEM`"]
pub struct PTXFEM_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXFEM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `LPMINTM`"]
pub type LPMINTM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPMINTM`"]
pub struct LPMINTM_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMINTM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `CIDSCHGM`"]
pub type CIDSCHGM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CIDSCHGM`"]
pub struct CIDSCHGM_W<'a> {
    w: &'a mut W,
}
impl<'a> CIDSCHGM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `DISCINT`"]
pub type DISCINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISCINT`"]
pub struct DISCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCINT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `SRQIM`"]
pub type SRQIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRQIM`"]
pub struct SRQIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SRQIM_W<'a> {
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
#[doc = "Reader of field `WUIM`"]
pub type WUIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUIM`"]
pub struct WUIM_W<'a> {
    w: &'a mut W,
}
impl<'a> WUIM_W<'a> {
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
    #[doc = "Bit 1 - MMISM"]
    #[inline(always)]
    pub fn mmism(&self) -> MMISM_R {
        MMISM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OTGINT"]
    #[inline(always)]
    pub fn otgint(&self) -> OTGINT_R {
        OTGINT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SOFM"]
    #[inline(always)]
    pub fn sofm(&self) -> SOFM_R {
        SOFM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RXFLVLM"]
    #[inline(always)]
    pub fn rxflvlm(&self) -> RXFLVLM_R {
        RXFLVLM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - NPTXFEM"]
    #[inline(always)]
    pub fn nptxfem(&self) -> NPTXFEM_R {
        NPTXFEM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GINAKEFFM"]
    #[inline(always)]
    pub fn ginakeffm(&self) -> GINAKEFFM_R {
        GINAKEFFM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GONAKEFFM"]
    #[inline(always)]
    pub fn gonakeffm(&self) -> GONAKEFFM_R {
        GONAKEFFM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ESUSPM"]
    #[inline(always)]
    pub fn esuspm(&self) -> ESUSPM_R {
        ESUSPM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - USBSUSPM"]
    #[inline(always)]
    pub fn usbsuspm(&self) -> USBSUSPM_R {
        USBSUSPM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - USBRST"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ENUMDNEM"]
    #[inline(always)]
    pub fn enumdnem(&self) -> ENUMDNEM_R {
        ENUMDNEM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ISOODRPM"]
    #[inline(always)]
    pub fn isoodrpm(&self) -> ISOODRPM_R {
        ISOODRPM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - EOPFM"]
    #[inline(always)]
    pub fn eopfm(&self) -> EOPFM_R {
        EOPFM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 18 - IEPINT"]
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - OEPINT"]
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - IISOIXFRM"]
    #[inline(always)]
    pub fn iisoixfrm(&self) -> IISOIXFRM_R {
        IISOIXFRM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - IPXFRM"]
    #[inline(always)]
    pub fn ipxfrm(&self) -> IPXFRM_R {
        IPXFRM_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - FSUSPM"]
    #[inline(always)]
    pub fn fsuspm(&self) -> FSUSPM_R {
        FSUSPM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - RSTDETM"]
    #[inline(always)]
    pub fn rstdetm(&self) -> RSTDETM_R {
        RSTDETM_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - PRTIM"]
    #[inline(always)]
    pub fn prtim(&self) -> PRTIM_R {
        PRTIM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - HCIM"]
    #[inline(always)]
    pub fn hcim(&self) -> HCIM_R {
        HCIM_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - PTXFEM"]
    #[inline(always)]
    pub fn ptxfem(&self) -> PTXFEM_R {
        PTXFEM_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - LPMINTM"]
    #[inline(always)]
    pub fn lpmintm(&self) -> LPMINTM_R {
        LPMINTM_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - CIDSCHGM"]
    #[inline(always)]
    pub fn cidschgm(&self) -> CIDSCHGM_R {
        CIDSCHGM_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DISCINT"]
    #[inline(always)]
    pub fn discint(&self) -> DISCINT_R {
        DISCINT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - SRQIM"]
    #[inline(always)]
    pub fn srqim(&self) -> SRQIM_R {
        SRQIM_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - WUIM"]
    #[inline(always)]
    pub fn wuim(&self) -> WUIM_R {
        WUIM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - MMISM"]
    #[inline(always)]
    pub fn mmism(&mut self) -> MMISM_W {
        MMISM_W { w: self }
    }
    #[doc = "Bit 2 - OTGINT"]
    #[inline(always)]
    pub fn otgint(&mut self) -> OTGINT_W {
        OTGINT_W { w: self }
    }
    #[doc = "Bit 3 - SOFM"]
    #[inline(always)]
    pub fn sofm(&mut self) -> SOFM_W {
        SOFM_W { w: self }
    }
    #[doc = "Bit 4 - RXFLVLM"]
    #[inline(always)]
    pub fn rxflvlm(&mut self) -> RXFLVLM_W {
        RXFLVLM_W { w: self }
    }
    #[doc = "Bit 5 - NPTXFEM"]
    #[inline(always)]
    pub fn nptxfem(&mut self) -> NPTXFEM_W {
        NPTXFEM_W { w: self }
    }
    #[doc = "Bit 6 - GINAKEFFM"]
    #[inline(always)]
    pub fn ginakeffm(&mut self) -> GINAKEFFM_W {
        GINAKEFFM_W { w: self }
    }
    #[doc = "Bit 7 - GONAKEFFM"]
    #[inline(always)]
    pub fn gonakeffm(&mut self) -> GONAKEFFM_W {
        GONAKEFFM_W { w: self }
    }
    #[doc = "Bit 10 - ESUSPM"]
    #[inline(always)]
    pub fn esuspm(&mut self) -> ESUSPM_W {
        ESUSPM_W { w: self }
    }
    #[doc = "Bit 11 - USBSUSPM"]
    #[inline(always)]
    pub fn usbsuspm(&mut self) -> USBSUSPM_W {
        USBSUSPM_W { w: self }
    }
    #[doc = "Bit 12 - USBRST"]
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W {
        USBRST_W { w: self }
    }
    #[doc = "Bit 13 - ENUMDNEM"]
    #[inline(always)]
    pub fn enumdnem(&mut self) -> ENUMDNEM_W {
        ENUMDNEM_W { w: self }
    }
    #[doc = "Bit 14 - ISOODRPM"]
    #[inline(always)]
    pub fn isoodrpm(&mut self) -> ISOODRPM_W {
        ISOODRPM_W { w: self }
    }
    #[doc = "Bit 15 - EOPFM"]
    #[inline(always)]
    pub fn eopfm(&mut self) -> EOPFM_W {
        EOPFM_W { w: self }
    }
    #[doc = "Bit 18 - IEPINT"]
    #[inline(always)]
    pub fn iepint(&mut self) -> IEPINT_W {
        IEPINT_W { w: self }
    }
    #[doc = "Bit 19 - OEPINT"]
    #[inline(always)]
    pub fn oepint(&mut self) -> OEPINT_W {
        OEPINT_W { w: self }
    }
    #[doc = "Bit 20 - IISOIXFRM"]
    #[inline(always)]
    pub fn iisoixfrm(&mut self) -> IISOIXFRM_W {
        IISOIXFRM_W { w: self }
    }
    #[doc = "Bit 21 - IPXFRM"]
    #[inline(always)]
    pub fn ipxfrm(&mut self) -> IPXFRM_W {
        IPXFRM_W { w: self }
    }
    #[doc = "Bit 22 - FSUSPM"]
    #[inline(always)]
    pub fn fsuspm(&mut self) -> FSUSPM_W {
        FSUSPM_W { w: self }
    }
    #[doc = "Bit 23 - RSTDETM"]
    #[inline(always)]
    pub fn rstdetm(&mut self) -> RSTDETM_W {
        RSTDETM_W { w: self }
    }
    #[doc = "Bit 25 - HCIM"]
    #[inline(always)]
    pub fn hcim(&mut self) -> HCIM_W {
        HCIM_W { w: self }
    }
    #[doc = "Bit 26 - PTXFEM"]
    #[inline(always)]
    pub fn ptxfem(&mut self) -> PTXFEM_W {
        PTXFEM_W { w: self }
    }
    #[doc = "Bit 27 - LPMINTM"]
    #[inline(always)]
    pub fn lpmintm(&mut self) -> LPMINTM_W {
        LPMINTM_W { w: self }
    }
    #[doc = "Bit 28 - CIDSCHGM"]
    #[inline(always)]
    pub fn cidschgm(&mut self) -> CIDSCHGM_W {
        CIDSCHGM_W { w: self }
    }
    #[doc = "Bit 29 - DISCINT"]
    #[inline(always)]
    pub fn discint(&mut self) -> DISCINT_W {
        DISCINT_W { w: self }
    }
    #[doc = "Bit 30 - SRQIM"]
    #[inline(always)]
    pub fn srqim(&mut self) -> SRQIM_W {
        SRQIM_W { w: self }
    }
    #[doc = "Bit 31 - WUIM"]
    #[inline(always)]
    pub fn wuim(&mut self) -> WUIM_W {
        WUIM_W { w: self }
    }
}