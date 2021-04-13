#[doc = "Reader of register CCMR2_Output"]
pub type R = crate::R<u32, super::CCMR2_OUTPUT>;
#[doc = "Writer for register CCMR2_Output"]
pub type W = crate::W<u32, super::CCMR2_OUTPUT>;
#[doc = "Register CCMR2_Output `reset()`'s with value 0"]
impl crate::ResetValue for super::CCMR2_OUTPUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = '0â\u{80}\u{99} in TIMx_CCER).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CC3S_A {
    #[doc = "0: CC3 channel is configured as output"]
    B_0X0 = 0,
    #[doc = "1: CC3 channel is configured as input, IC3 is mapped on TI3"]
    B_0X1 = 1,
    #[doc = "2: CC3 channel is configured as input, IC3 is mapped on TI4"]
    B_0X2 = 2,
    #[doc = "3: CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    B_0X3 = 3,
}
impl From<CC3S_A> for u8 {
    #[inline(always)]
    fn from(variant: CC3S_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CC3S`"]
pub type CC3S_R = crate::R<u8, CC3S_A>;
impl CC3S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC3S_A {
        match self.bits {
            0 => CC3S_A::B_0X0,
            1 => CC3S_A::B_0X1,
            2 => CC3S_A::B_0X2,
            3 => CC3S_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC3S_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC3S_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == CC3S_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == CC3S_A::B_0X3
    }
}
#[doc = "Write proxy for field `CC3S`"]
pub struct CC3S_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC3S_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CC3 channel is configured as output"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CC3S_A::B_0X0)
    }
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TI3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CC3S_A::B_0X1)
    }
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TI4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(CC3S_A::B_0X2)
    }
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(CC3S_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `OC3FE`"]
pub type OC3FE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC3FE`"]
pub struct OC3FE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC3FE_W<'a> {
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
#[doc = "Reader of field `OC3PE`"]
pub type OC3PE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC3PE`"]
pub struct OC3PE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC3PE_W<'a> {
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
#[doc = "Reader of field `OC3M1`"]
pub type OC3M1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OC3M1`"]
pub struct OC3M1_W<'a> {
    w: &'a mut W,
}
impl<'a> OC3M1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `OC3CE`"]
pub type OC3CE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC3CE`"]
pub struct OC3CE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC3CE_W<'a> {
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
#[doc = "Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = '0â\u{80}\u{99} in TIMx_CCER).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CC4S_A {
    #[doc = "0: CC4 channel is configured as output"]
    B_0X0 = 0,
    #[doc = "1: CC4 channel is configured as input, IC4 is mapped on TI4"]
    B_0X1 = 1,
    #[doc = "2: CC4 channel is configured as input, IC4 is mapped on TI3"]
    B_0X2 = 2,
    #[doc = "3: CC4 channel is configured as input, IC4 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    B_0X3 = 3,
}
impl From<CC4S_A> for u8 {
    #[inline(always)]
    fn from(variant: CC4S_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CC4S`"]
pub type CC4S_R = crate::R<u8, CC4S_A>;
impl CC4S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC4S_A {
        match self.bits {
            0 => CC4S_A::B_0X0,
            1 => CC4S_A::B_0X1,
            2 => CC4S_A::B_0X2,
            3 => CC4S_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC4S_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC4S_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == CC4S_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == CC4S_A::B_0X3
    }
}
#[doc = "Write proxy for field `CC4S`"]
pub struct CC4S_W<'a> {
    w: &'a mut W,
}
impl<'a> CC4S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC4S_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CC4 channel is configured as output"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CC4S_A::B_0X0)
    }
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TI4"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CC4S_A::B_0X1)
    }
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TI3"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(CC4S_A::B_0X2)
    }
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(CC4S_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `OC4FE`"]
pub type OC4FE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC4FE`"]
pub struct OC4FE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC4FE_W<'a> {
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
#[doc = "Reader of field `OC4PE`"]
pub type OC4PE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC4PE`"]
pub struct OC4PE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC4PE_W<'a> {
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
#[doc = "Reader of field `OC4M1`"]
pub type OC4M1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OC4M1`"]
pub struct OC4M1_W<'a> {
    w: &'a mut W,
}
impl<'a> OC4M1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `OC4CE`"]
pub type OC4CE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC4CE`"]
pub struct OC4CE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC4CE_W<'a> {
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
#[doc = "Reader of field `OC3M2`"]
pub type OC3M2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC3M2`"]
pub struct OC3M2_W<'a> {
    w: &'a mut W,
}
impl<'a> OC3M2_W<'a> {
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
#[doc = "Reader of field `OC4M2`"]
pub type OC4M2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC4M2`"]
pub struct OC4M2_W<'a> {
    w: &'a mut W,
}
impl<'a> OC4M2_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = '0â\u{80}\u{99} in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc3s(&self) -> CC3S_R {
        CC3S_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Output compare 3 fast enable Refer to OC1FE description."]
    #[inline(always)]
    pub fn oc3fe(&self) -> OC3FE_R {
        OC3FE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Output compare 3 preload enable Refer to OC1PE description."]
    #[inline(always)]
    pub fn oc3pe(&self) -> OC3PE_R {
        OC3PE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Output compare 3 mode Refer to OC1M\\[3:0\\]
description."]
    #[inline(always)]
    pub fn oc3m1(&self) -> OC3M1_R {
        OC3M1_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Output compare 3 clear enable Refer to OC1CE description."]
    #[inline(always)]
    pub fn oc3ce(&self) -> OC3CE_R {
        OC3CE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = '0â\u{80}\u{99} in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc4s(&self) -> CC4S_R {
        CC4S_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Output compare 4 fast enable Refer to OC1FE description."]
    #[inline(always)]
    pub fn oc4fe(&self) -> OC4FE_R {
        OC4FE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Output compare 4 preload enable Refer to OC1PE description."]
    #[inline(always)]
    pub fn oc4pe(&self) -> OC4PE_R {
        OC4PE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - Output compare 4 mode Refer to OC3M\\[3:0\\]
description."]
    #[inline(always)]
    pub fn oc4m1(&self) -> OC4M1_R {
        OC4M1_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Output compare 4 clear enable Refer to OC1CE description."]
    #[inline(always)]
    pub fn oc4ce(&self) -> OC4CE_R {
        OC4CE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Output compare 3 mode Refer to OC1M\\[3:0\\]
description."]
    #[inline(always)]
    pub fn oc3m2(&self) -> OC3M2_R {
        OC3M2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Output compare 4 mode Refer to OC3M\\[3:0\\]
description."]
    #[inline(always)]
    pub fn oc4m2(&self) -> OC4M2_R {
        OC4M2_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = '0â\u{80}\u{99} in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc3s(&mut self) -> CC3S_W {
        CC3S_W { w: self }
    }
    #[doc = "Bit 2 - Output compare 3 fast enable Refer to OC1FE description."]
    #[inline(always)]
    pub fn oc3fe(&mut self) -> OC3FE_W {
        OC3FE_W { w: self }
    }
    #[doc = "Bit 3 - Output compare 3 preload enable Refer to OC1PE description."]
    #[inline(always)]
    pub fn oc3pe(&mut self) -> OC3PE_W {
        OC3PE_W { w: self }
    }
    #[doc = "Bits 4:6 - Output compare 3 mode Refer to OC1M\\[3:0\\]
description."]
    #[inline(always)]
    pub fn oc3m1(&mut self) -> OC3M1_W {
        OC3M1_W { w: self }
    }
    #[doc = "Bit 7 - Output compare 3 clear enable Refer to OC1CE description."]
    #[inline(always)]
    pub fn oc3ce(&mut self) -> OC3CE_W {
        OC3CE_W { w: self }
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = '0â\u{80}\u{99} in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc4s(&mut self) -> CC4S_W {
        CC4S_W { w: self }
    }
    #[doc = "Bit 10 - Output compare 4 fast enable Refer to OC1FE description."]
    #[inline(always)]
    pub fn oc4fe(&mut self) -> OC4FE_W {
        OC4FE_W { w: self }
    }
    #[doc = "Bit 11 - Output compare 4 preload enable Refer to OC1PE description."]
    #[inline(always)]
    pub fn oc4pe(&mut self) -> OC4PE_W {
        OC4PE_W { w: self }
    }
    #[doc = "Bits 12:14 - Output compare 4 mode Refer to OC3M\\[3:0\\]
description."]
    #[inline(always)]
    pub fn oc4m1(&mut self) -> OC4M1_W {
        OC4M1_W { w: self }
    }
    #[doc = "Bit 15 - Output compare 4 clear enable Refer to OC1CE description."]
    #[inline(always)]
    pub fn oc4ce(&mut self) -> OC4CE_W {
        OC4CE_W { w: self }
    }
    #[doc = "Bit 16 - Output compare 3 mode Refer to OC1M\\[3:0\\]
description."]
    #[inline(always)]
    pub fn oc3m2(&mut self) -> OC3M2_W {
        OC3M2_W { w: self }
    }
    #[doc = "Bit 24 - Output compare 4 mode Refer to OC3M\\[3:0\\]
description."]
    #[inline(always)]
    pub fn oc4m2(&mut self) -> OC4M2_W {
        OC4M2_W { w: self }
    }
}
