#[doc = "Reader of register DMAMUX_RG2CR"]
pub type R = crate::R<u32, super::DMAMUX_RG2CR>;
#[doc = "Writer for register DMAMUX_RG2CR"]
pub type W = crate::W<u32, super::DMAMUX_RG2CR>;
#[doc = "Register DMAMUX_RG2CR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMAMUX_RG2CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SIG_ID`"]
pub type SIG_ID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIG_ID`"]
pub struct SIG_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> SIG_ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Trigger overrun interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OIE_A {
    #[doc = "0: interrupt on a trigger overrun event occurrence is disabled"]
    B_0X0 = 0,
    #[doc = "1: interrupt on a trigger overrun event occurrence is enabled"]
    B_0X1 = 1,
}
impl From<OIE_A> for bool {
    #[inline(always)]
    fn from(variant: OIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OIE`"]
pub type OIE_R = crate::R<bool, OIE_A>;
impl OIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OIE_A {
        match self.bits {
            false => OIE_A::B_0X0,
            true => OIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `OIE`"]
pub struct OIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "interrupt on a trigger overrun event occurrence is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(OIE_A::B_0X0)
    }
    #[doc = "interrupt on a trigger overrun event occurrence is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(OIE_A::B_0X1)
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
#[doc = "DMA request generator channel x enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GE_A {
    #[doc = "0: DMA request generator channel x disabled"]
    B_0X0 = 0,
    #[doc = "1: DMA request generator channel x enabled"]
    B_0X1 = 1,
}
impl From<GE_A> for bool {
    #[inline(always)]
    fn from(variant: GE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GE`"]
pub type GE_R = crate::R<bool, GE_A>;
impl GE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GE_A {
        match self.bits {
            false => GE_A::B_0X0,
            true => GE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GE_A::B_0X1
    }
}
#[doc = "Write proxy for field `GE`"]
pub struct GE_W<'a> {
    w: &'a mut W,
}
impl<'a> GE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA request generator channel x disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GE_A::B_0X0)
    }
    #[doc = "DMA request generator channel x enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GE_A::B_0X1)
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
#[doc = "DMA request generator trigger polarity Defines the edge polarity of the selected trigger input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPOL_A {
    #[doc = "0: no event. I.e. none trigger detection nor generation."]
    B_0X0 = 0,
    #[doc = "1: rising edge"]
    B_0X1 = 1,
    #[doc = "2: falling edge"]
    B_0X2 = 2,
    #[doc = "3: rising and falling edge"]
    B_0X3 = 3,
}
impl From<GPOL_A> for u8 {
    #[inline(always)]
    fn from(variant: GPOL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPOL`"]
pub type GPOL_R = crate::R<u8, GPOL_A>;
impl GPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPOL_A {
        match self.bits {
            0 => GPOL_A::B_0X0,
            1 => GPOL_A::B_0X1,
            2 => GPOL_A::B_0X2,
            3 => GPOL_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPOL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPOL_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == GPOL_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == GPOL_A::B_0X3
    }
}
#[doc = "Write proxy for field `GPOL`"]
pub struct GPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPOL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "no event. I.e. none trigger detection nor generation."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPOL_A::B_0X0)
    }
    #[doc = "rising edge"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPOL_A::B_0X1)
    }
    #[doc = "falling edge"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(GPOL_A::B_0X2)
    }
    #[doc = "rising and falling edge"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(GPOL_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "Reader of field `GNBREQ`"]
pub type GNBREQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GNBREQ`"]
pub struct GNBREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> GNBREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | (((value as u32) & 0x1f) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Signal identification Selects the DMA request trigger input used for the channel x of the DMA request generator"]
    #[inline(always)]
    pub fn sig_id(&self) -> SIG_ID_R {
        SIG_ID_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Trigger overrun interrupt enable"]
    #[inline(always)]
    pub fn oie(&self) -> OIE_R {
        OIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DMA request generator channel x enable"]
    #[inline(always)]
    pub fn ge(&self) -> GE_R {
        GE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger polarity Defines the edge polarity of the selected trigger input"]
    #[inline(always)]
    pub fn gpol(&self) -> GPOL_R {
        GPOL_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to be generated (minus 1) Defines the number of DMA requests to be generated after a trigger event. The actual number of generated DMA requests is GNBREQ +1. Note: This field shall only be written when GE bit is disabled."]
    #[inline(always)]
    pub fn gnbreq(&self) -> GNBREQ_R {
        GNBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Signal identification Selects the DMA request trigger input used for the channel x of the DMA request generator"]
    #[inline(always)]
    pub fn sig_id(&mut self) -> SIG_ID_W {
        SIG_ID_W { w: self }
    }
    #[doc = "Bit 8 - Trigger overrun interrupt enable"]
    #[inline(always)]
    pub fn oie(&mut self) -> OIE_W {
        OIE_W { w: self }
    }
    #[doc = "Bit 16 - DMA request generator channel x enable"]
    #[inline(always)]
    pub fn ge(&mut self) -> GE_W {
        GE_W { w: self }
    }
    #[doc = "Bits 17:18 - DMA request generator trigger polarity Defines the edge polarity of the selected trigger input"]
    #[inline(always)]
    pub fn gpol(&mut self) -> GPOL_W {
        GPOL_W { w: self }
    }
    #[doc = "Bits 19:23 - Number of DMA requests to be generated (minus 1) Defines the number of DMA requests to be generated after a trigger event. The actual number of generated DMA requests is GNBREQ +1. Note: This field shall only be written when GE bit is disabled."]
    #[inline(always)]
    pub fn gnbreq(&mut self) -> GNBREQ_W {
        GNBREQ_W { w: self }
    }
}
