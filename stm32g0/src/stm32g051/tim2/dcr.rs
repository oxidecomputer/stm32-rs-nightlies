#[doc = "Reader of register DCR"]
pub type R = crate::R<u32, super::DCR>;
#[doc = "Writer for register DCR"]
pub type W = crate::W<u32, super::DCR>;
#[doc = "Register DCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DMA base address This 5-bit vector defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the address of the TIMx_CR1 register. Example: ... Example: Let us consider the following transfer: DBL = 7 transfers & DBA = TIMx_CR1. In this case the transfer is done to/from 7 registers starting from the TIMx_CR1 address.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DBA_A {
    #[doc = "0: TIMx_CR1"]
    B_0X0 = 0,
    #[doc = "1: TIMx_CR2"]
    B_0X1 = 1,
    #[doc = "2: TIMx_SMCR"]
    B_0X2 = 2,
}
impl From<DBA_A> for u8 {
    #[inline(always)]
    fn from(variant: DBA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DBA`"]
pub type DBA_R = crate::R<u8, DBA_A>;
impl DBA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DBA_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DBA_A::B_0X0),
            1 => Val(DBA_A::B_0X1),
            2 => Val(DBA_A::B_0X2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBA_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBA_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == DBA_A::B_0X2
    }
}
#[doc = "Write proxy for field `DBA`"]
pub struct DBA_W<'a> {
    w: &'a mut W,
}
impl<'a> DBA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "TIMx_CR1"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBA_A::B_0X0)
    }
    #[doc = "TIMx_CR2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBA_A::B_0X1)
    }
    #[doc = "TIMx_SMCR"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(DBA_A::B_0X2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "DMA burst length This 5-bit vector defines the number of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address). ...\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DBL_A {
    #[doc = "0: 1 transfer,"]
    B_0X0 = 0,
    #[doc = "1: 2 transfers,"]
    B_0X1 = 1,
    #[doc = "2: 3 transfers,"]
    B_0X2 = 2,
    #[doc = "17: 18 transfers."]
    B_0X11 = 17,
}
impl From<DBL_A> for u8 {
    #[inline(always)]
    fn from(variant: DBL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DBL`"]
pub type DBL_R = crate::R<u8, DBL_A>;
impl DBL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DBL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DBL_A::B_0X0),
            1 => Val(DBL_A::B_0X1),
            2 => Val(DBL_A::B_0X2),
            17 => Val(DBL_A::B_0X11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBL_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == DBL_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X11`"]
    #[inline(always)]
    pub fn is_b_0x11(&self) -> bool {
        *self == DBL_A::B_0X11
    }
}
#[doc = "Write proxy for field `DBL`"]
pub struct DBL_W<'a> {
    w: &'a mut W,
}
impl<'a> DBL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 transfer,"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBL_A::B_0X0)
    }
    #[doc = "2 transfers,"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBL_A::B_0X1)
    }
    #[doc = "3 transfers,"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(DBL_A::B_0X2)
    }
    #[doc = "18 transfers."]
    #[inline(always)]
    pub fn b_0x11(self) -> &'a mut W {
        self.variant(DBL_A::B_0X11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - DMA base address This 5-bit vector defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the address of the TIMx_CR1 register. Example: ... Example: Let us consider the following transfer: DBL = 7 transfers & DBA = TIMx_CR1. In this case the transfer is done to/from 7 registers starting from the TIMx_CR1 address."]
    #[inline(always)]
    pub fn dba(&self) -> DBA_R {
        DBA_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DMA burst length This 5-bit vector defines the number of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address). ..."]
    #[inline(always)]
    pub fn dbl(&self) -> DBL_R {
        DBL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA base address This 5-bit vector defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the address of the TIMx_CR1 register. Example: ... Example: Let us consider the following transfer: DBL = 7 transfers & DBA = TIMx_CR1. In this case the transfer is done to/from 7 registers starting from the TIMx_CR1 address."]
    #[inline(always)]
    pub fn dba(&mut self) -> DBA_W {
        DBA_W { w: self }
    }
    #[doc = "Bits 8:12 - DMA burst length This 5-bit vector defines the number of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address). ..."]
    #[inline(always)]
    pub fn dbl(&mut self) -> DBL_W {
        DBL_W { w: self }
    }
}
