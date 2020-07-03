#[doc = "Reader of register CSR"]
pub type R = crate::R<u32, super::CSR>;
#[doc = "Writer for register CSR"]
pub type W = crate::W<u32, super::CSR>;
#[doc = "Register CSR `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FUNC`"]
pub type FUNC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FUNC`"]
pub struct FUNC_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `PRECISION`"]
pub type PRECISION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRECISION`"]
pub struct PRECISION_W<'a> {
    w: &'a mut W,
}
impl<'a> PRECISION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `SCALE`"]
pub type SCALE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCALE`"]
pub struct SCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `IEN`"]
pub type IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IEN`"]
pub struct IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IEN_W<'a> {
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
#[doc = "Reader of field `DMAREN`"]
pub type DMAREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAREN`"]
pub struct DMAREN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAREN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `DMAWEN`"]
pub type DMAWEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAWEN`"]
pub struct DMAWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAWEN_W<'a> {
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
#[doc = "Reader of field `NRES`"]
pub type NRES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NRES`"]
pub struct NRES_W<'a> {
    w: &'a mut W,
}
impl<'a> NRES_W<'a> {
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
#[doc = "Reader of field `NARGS`"]
pub type NARGS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NARGS`"]
pub struct NARGS_W<'a> {
    w: &'a mut W,
}
impl<'a> NARGS_W<'a> {
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
#[doc = "Reader of field `RESSIZE`"]
pub type RESSIZE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESSIZE`"]
pub struct RESSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> RESSIZE_W<'a> {
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
#[doc = "Reader of field `ARGSIZE`"]
pub type ARGSIZE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARGSIZE`"]
pub struct ARGSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> ARGSIZE_W<'a> {
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
#[doc = "Reader of field `RRDY`"]
pub type RRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RRDY`"]
pub struct RRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> RRDY_W<'a> {
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
    #[doc = "Bits 0:3 - FUNC"]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PRECISION"]
    #[inline(always)]
    pub fn precision(&self) -> PRECISION_R {
        PRECISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - SCALE"]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 16 - IEN"]
    #[inline(always)]
    pub fn ien(&self) -> IEN_R {
        IEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DMAREN"]
    #[inline(always)]
    pub fn dmaren(&self) -> DMAREN_R {
        DMAREN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DMAWEN"]
    #[inline(always)]
    pub fn dmawen(&self) -> DMAWEN_R {
        DMAWEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - NRES"]
    #[inline(always)]
    pub fn nres(&self) -> NRES_R {
        NRES_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - NARGS"]
    #[inline(always)]
    pub fn nargs(&self) -> NARGS_R {
        NARGS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - RESSIZE"]
    #[inline(always)]
    pub fn ressize(&self) -> RESSIZE_R {
        RESSIZE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - ARGSIZE"]
    #[inline(always)]
    pub fn argsize(&self) -> ARGSIZE_R {
        ARGSIZE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 31 - RRDY"]
    #[inline(always)]
    pub fn rrdy(&self) -> RRDY_R {
        RRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - FUNC"]
    #[inline(always)]
    pub fn func(&mut self) -> FUNC_W {
        FUNC_W { w: self }
    }
    #[doc = "Bits 4:7 - PRECISION"]
    #[inline(always)]
    pub fn precision(&mut self) -> PRECISION_W {
        PRECISION_W { w: self }
    }
    #[doc = "Bits 8:10 - SCALE"]
    #[inline(always)]
    pub fn scale(&mut self) -> SCALE_W {
        SCALE_W { w: self }
    }
    #[doc = "Bit 16 - IEN"]
    #[inline(always)]
    pub fn ien(&mut self) -> IEN_W {
        IEN_W { w: self }
    }
    #[doc = "Bit 17 - DMAREN"]
    #[inline(always)]
    pub fn dmaren(&mut self) -> DMAREN_W {
        DMAREN_W { w: self }
    }
    #[doc = "Bit 18 - DMAWEN"]
    #[inline(always)]
    pub fn dmawen(&mut self) -> DMAWEN_W {
        DMAWEN_W { w: self }
    }
    #[doc = "Bit 19 - NRES"]
    #[inline(always)]
    pub fn nres(&mut self) -> NRES_W {
        NRES_W { w: self }
    }
    #[doc = "Bit 20 - NARGS"]
    #[inline(always)]
    pub fn nargs(&mut self) -> NARGS_W {
        NARGS_W { w: self }
    }
    #[doc = "Bit 21 - RESSIZE"]
    #[inline(always)]
    pub fn ressize(&mut self) -> RESSIZE_W {
        RESSIZE_W { w: self }
    }
    #[doc = "Bit 22 - ARGSIZE"]
    #[inline(always)]
    pub fn argsize(&mut self) -> ARGSIZE_W {
        ARGSIZE_W { w: self }
    }
    #[doc = "Bit 31 - RRDY"]
    #[inline(always)]
    pub fn rrdy(&mut self) -> RRDY_W {
        RRDY_W { w: self }
    }
}
