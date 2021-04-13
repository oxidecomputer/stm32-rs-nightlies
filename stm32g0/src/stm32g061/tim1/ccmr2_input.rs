#[doc = "Reader of register CCMR2_Input"]
pub type R = crate::R<u32, super::CCMR2_INPUT>;
#[doc = "Writer for register CCMR2_Input"]
pub type W = crate::W<u32, super::CCMR2_INPUT>;
#[doc = "Register CCMR2_Input `reset()`'s with value 0"]
impl crate::ResetValue for super::CCMR2_INPUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = '0â\u{80}\u{99} in TIMx_CCER).\n\nValue on reset: 0"]
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
#[doc = "Reader of field `IC3PSC`"]
pub type IC3PSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IC3PSC`"]
pub struct IC3PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> IC3PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `IC3F`"]
pub type IC3F_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IC3F`"]
pub struct IC3F_W<'a> {
    w: &'a mut W,
}
impl<'a> IC3F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
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
#[doc = "Reader of field `IC4PSC`"]
pub type IC4PSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IC4PSC`"]
pub struct IC4PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> IC4PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `IC4F`"]
pub type IC4F_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IC4F`"]
pub struct IC4F_W<'a> {
    w: &'a mut W,
}
impl<'a> IC4F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = '0â\u{80}\u{99} in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc3s(&self) -> CC3S_R {
        CC3S_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Input capture 3 prescaler Refer to IC1PSC\\[1:0\\]
description."]
    #[inline(always)]
    pub fn ic3psc(&self) -> IC3PSC_R {
        IC3PSC_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:7 - Input capture 3 filter Refer to IC1F\\[3:0\\]
description."]
    #[inline(always)]
    pub fn ic3f(&self) -> IC3F_R {
        IC3F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = '0â\u{80}\u{99} in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc4s(&self) -> CC4S_R {
        CC4S_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Input capture 4 prescaler Refer to IC1PSC\\[1:0\\]
description."]
    #[inline(always)]
    pub fn ic4psc(&self) -> IC4PSC_R {
        IC4PSC_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:15 - Input capture 4 filter Refer to IC1F\\[3:0\\]
description."]
    #[inline(always)]
    pub fn ic4f(&self) -> IC4F_R {
        IC4F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = '0â\u{80}\u{99} in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc3s(&mut self) -> CC3S_W {
        CC3S_W { w: self }
    }
    #[doc = "Bits 2:3 - Input capture 3 prescaler Refer to IC1PSC\\[1:0\\]
description."]
    #[inline(always)]
    pub fn ic3psc(&mut self) -> IC3PSC_W {
        IC3PSC_W { w: self }
    }
    #[doc = "Bits 4:7 - Input capture 3 filter Refer to IC1F\\[3:0\\]
description."]
    #[inline(always)]
    pub fn ic3f(&mut self) -> IC3F_W {
        IC3F_W { w: self }
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = '0â\u{80}\u{99} in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc4s(&mut self) -> CC4S_W {
        CC4S_W { w: self }
    }
    #[doc = "Bits 10:11 - Input capture 4 prescaler Refer to IC1PSC\\[1:0\\]
description."]
    #[inline(always)]
    pub fn ic4psc(&mut self) -> IC4PSC_W {
        IC4PSC_W { w: self }
    }
    #[doc = "Bits 12:15 - Input capture 4 filter Refer to IC1F\\[3:0\\]
description."]
    #[inline(always)]
    pub fn ic4f(&mut self) -> IC4F_W {
        IC4F_W { w: self }
    }
}
