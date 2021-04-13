#[doc = "Reader of register DMAMUX_C11CR"]
pub type R = crate::R<u32, super::DMAMUX_C11CR>;
#[doc = "Writer for register DMAMUX_C11CR"]
pub type W = crate::W<u32, super::DMAMUX_C11CR>;
#[doc = "Register DMAMUX_C11CR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMAMUX_C11CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMAREQ_ID`"]
pub type DMAREQ_ID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMAREQ_ID`"]
pub struct DMAREQ_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAREQ_ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Synchronization overrun interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOIE_A {
    #[doc = "0: interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: interrupt enabled"]
    B_0X1 = 1,
}
impl From<SOIE_A> for bool {
    #[inline(always)]
    fn from(variant: SOIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SOIE`"]
pub type SOIE_R = crate::R<bool, SOIE_A>;
impl SOIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOIE_A {
        match self.bits {
            false => SOIE_A::B_0X0,
            true => SOIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SOIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SOIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `SOIE`"]
pub struct SOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SOIE_A::B_0X0)
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SOIE_A::B_0X1)
    }
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
#[doc = "Event generation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EGE_A {
    #[doc = "0: event generation disabled"]
    B_0X0 = 0,
    #[doc = "1: event generation enabled"]
    B_0X1 = 1,
}
impl From<EGE_A> for bool {
    #[inline(always)]
    fn from(variant: EGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EGE`"]
pub type EGE_R = crate::R<bool, EGE_A>;
impl EGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EGE_A {
        match self.bits {
            false => EGE_A::B_0X0,
            true => EGE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EGE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EGE_A::B_0X1
    }
}
#[doc = "Write proxy for field `EGE`"]
pub struct EGE_W<'a> {
    w: &'a mut W,
}
impl<'a> EGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EGE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "event generation disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EGE_A::B_0X0)
    }
    #[doc = "event generation enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EGE_A::B_0X1)
    }
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
#[doc = "Synchronization enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SE_A {
    #[doc = "0: synchronization disabled"]
    B_0X0 = 0,
    #[doc = "1: synchronization enabled"]
    B_0X1 = 1,
}
impl From<SE_A> for bool {
    #[inline(always)]
    fn from(variant: SE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SE`"]
pub type SE_R = crate::R<bool, SE_A>;
impl SE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SE_A {
        match self.bits {
            false => SE_A::B_0X0,
            true => SE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SE_A::B_0X1
    }
}
#[doc = "Write proxy for field `SE`"]
pub struct SE_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "synchronization disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SE_A::B_0X0)
    }
    #[doc = "synchronization enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SE_A::B_0X1)
    }
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
#[doc = "Synchronization polarity Defines the edge polarity of the selected synchronization input:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPOL_A {
    #[doc = "0: no event, i.e. no synchronization nor detection."]
    B_0X0 = 0,
    #[doc = "1: rising edge"]
    B_0X1 = 1,
    #[doc = "2: falling edge"]
    B_0X2 = 2,
    #[doc = "3: rising and falling edge"]
    B_0X3 = 3,
}
impl From<SPOL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPOL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SPOL`"]
pub type SPOL_R = crate::R<u8, SPOL_A>;
impl SPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPOL_A {
        match self.bits {
            0 => SPOL_A::B_0X0,
            1 => SPOL_A::B_0X1,
            2 => SPOL_A::B_0X2,
            3 => SPOL_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPOL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPOL_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == SPOL_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == SPOL_A::B_0X3
    }
}
#[doc = "Write proxy for field `SPOL`"]
pub struct SPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPOL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "no event, i.e. no synchronization nor detection."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SPOL_A::B_0X0)
    }
    #[doc = "rising edge"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SPOL_A::B_0X1)
    }
    #[doc = "falling edge"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(SPOL_A::B_0X2)
    }
    #[doc = "rising and falling edge"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(SPOL_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "Reader of field `NBREQ`"]
pub type NBREQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NBREQ`"]
pub struct NBREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> NBREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | (((value as u32) & 0x1f) << 19);
        self.w
    }
}
#[doc = "Reader of field `SYNC_ID`"]
pub type SYNC_ID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYNC_ID`"]
pub struct SYNC_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - DMA request identification Selects the input DMA request. See the DMAMUX table about assignments of multiplexer inputs to resources."]
    #[inline(always)]
    pub fn dmareq_id(&self) -> DMAREQ_ID_R {
        DMAREQ_ID_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Synchronization overrun interrupt enable"]
    #[inline(always)]
    pub fn soie(&self) -> SOIE_R {
        SOIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Event generation enable"]
    #[inline(always)]
    pub fn ege(&self) -> EGE_R {
        EGE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Synchronization enable"]
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - Synchronization polarity Defines the edge polarity of the selected synchronization input:"]
    #[inline(always)]
    pub fn spol(&self) -> SPOL_R {
        SPOL_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bits 19:23 - Number of DMA requests minus 1 to forward Defines the number of DMA requests to forward to the DMA controller after a synchronization event, and/or the number of DMA requests before an output event is generated. This field shall only be written when both SE and EGE bits are low."]
    #[inline(always)]
    pub fn nbreq(&self) -> NBREQ_R {
        NBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Synchronization identification Selects the synchronization input (see inputs to resources STM32G0)."]
    #[inline(always)]
    pub fn sync_id(&self) -> SYNC_ID_R {
        SYNC_ID_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - DMA request identification Selects the input DMA request. See the DMAMUX table about assignments of multiplexer inputs to resources."]
    #[inline(always)]
    pub fn dmareq_id(&mut self) -> DMAREQ_ID_W {
        DMAREQ_ID_W { w: self }
    }
    #[doc = "Bit 8 - Synchronization overrun interrupt enable"]
    #[inline(always)]
    pub fn soie(&mut self) -> SOIE_W {
        SOIE_W { w: self }
    }
    #[doc = "Bit 9 - Event generation enable"]
    #[inline(always)]
    pub fn ege(&mut self) -> EGE_W {
        EGE_W { w: self }
    }
    #[doc = "Bit 16 - Synchronization enable"]
    #[inline(always)]
    pub fn se(&mut self) -> SE_W {
        SE_W { w: self }
    }
    #[doc = "Bits 17:18 - Synchronization polarity Defines the edge polarity of the selected synchronization input:"]
    #[inline(always)]
    pub fn spol(&mut self) -> SPOL_W {
        SPOL_W { w: self }
    }
    #[doc = "Bits 19:23 - Number of DMA requests minus 1 to forward Defines the number of DMA requests to forward to the DMA controller after a synchronization event, and/or the number of DMA requests before an output event is generated. This field shall only be written when both SE and EGE bits are low."]
    #[inline(always)]
    pub fn nbreq(&mut self) -> NBREQ_W {
        NBREQ_W { w: self }
    }
    #[doc = "Bits 24:28 - Synchronization identification Selects the synchronization input (see inputs to resources STM32G0)."]
    #[inline(always)]
    pub fn sync_id(&mut self) -> SYNC_ID_W {
        SYNC_ID_W { w: self }
    }
}
