#[doc = "Reader of register CCMR1_Output"]
pub type R = crate::R<u32, super::CCMR1_OUTPUT>;
#[doc = "Writer for register CCMR1_Output"]
pub type W = crate::W<u32, super::CCMR1_OUTPUT>;
#[doc = "Register CCMR1_Output `reset()`'s with value 0"]
impl crate::ResetValue for super::CCMR1_OUTPUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CC1S_A {
    #[doc = "0: CC1 channel is configured as output."]
    B_0X0 = 0,
    #[doc = "1: CC1 channel is configured as input, IC1 is mapped on TI1."]
    B_0X1 = 1,
}
impl From<CC1S_A> for u8 {
    #[inline(always)]
    fn from(variant: CC1S_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CC1S`"]
pub type CC1S_R = crate::R<u8, CC1S_A>;
impl CC1S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CC1S_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CC1S_A::B_0X0),
            1 => Val(CC1S_A::B_0X1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC1S_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC1S_A::B_0X1
    }
}
#[doc = "Write proxy for field `CC1S`"]
pub struct CC1S_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC1S_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CC1 channel is configured as output."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CC1S_A::B_0X0)
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI1."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CC1S_A::B_0X1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Output compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OC1FE_A {
    #[doc = "0: CC1 behaves normally depending on counter and CCR1 values even when the trigger is ON. The minimum delay to activate CC1 output when an edge occurs on the trigger input is 5 clock cycles."]
    B_0X0 = 0,
    #[doc = "1: An active edge on the trigger input acts like a compare match on CC1 output. OC is then set to the compare level independently of the result of the comparison. Delay to sample the trigger input and to activate CC1 output is reduced to 3 clock cycles. OC1FE acts only if the channel is configured in PWM1 or PWM2 mode."]
    B_0X1 = 1,
}
impl From<OC1FE_A> for bool {
    #[inline(always)]
    fn from(variant: OC1FE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OC1FE`"]
pub type OC1FE_R = crate::R<bool, OC1FE_A>;
impl OC1FE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OC1FE_A {
        match self.bits {
            false => OC1FE_A::B_0X0,
            true => OC1FE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OC1FE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OC1FE_A::B_0X1
    }
}
#[doc = "Write proxy for field `OC1FE`"]
pub struct OC1FE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC1FE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OC1FE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CC1 behaves normally depending on counter and CCR1 values even when the trigger is ON. The minimum delay to activate CC1 output when an edge occurs on the trigger input is 5 clock cycles."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(OC1FE_A::B_0X0)
    }
    #[doc = "An active edge on the trigger input acts like a compare match on CC1 output. OC is then set to the compare level independently of the result of the comparison. Delay to sample the trigger input and to activate CC1 output is reduced to 3 clock cycles. OC1FE acts only if the channel is configured in PWM1 or PWM2 mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(OC1FE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Output compare 1 preload enable Note: The PWM mode can be used without validating the preload register only in one pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OC1PE_A {
    #[doc = "0: Preload register on TIMx_CCR1 disabled. TIMx_CCR1 can be written at anytime, the new value is taken in account immediately. "]
    B_0X0 = 0,
    #[doc = "1: Preload register on TIMx_CCR1 enabled. Read/Write operations access the preload register. TIMx_CCR1 preload value is loaded in the active register at each update event."]
    B_0X1 = 1,
}
impl From<OC1PE_A> for bool {
    #[inline(always)]
    fn from(variant: OC1PE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OC1PE`"]
pub type OC1PE_R = crate::R<bool, OC1PE_A>;
impl OC1PE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OC1PE_A {
        match self.bits {
            false => OC1PE_A::B_0X0,
            true => OC1PE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OC1PE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OC1PE_A::B_0X1
    }
}
#[doc = "Write proxy for field `OC1PE`"]
pub struct OC1PE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC1PE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OC1PE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Preload register on TIMx_CCR1 disabled. TIMx_CCR1 can be written at anytime, the new value is taken in account immediately."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(OC1PE_A::B_0X0)
    }
    #[doc = "Preload register on TIMx_CCR1 enabled. Read/Write operations access the preload register. TIMx_CCR1 preload value is loaded in the active register at each update event."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(OC1PE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Output compare 1 mode (refer to bit 16 for OC1M\\[3\\]) These bits define the behavior of the output reference signal OC1REF from which OC1 is derived. OC1REF is active high whereas OC1 active level depends on CC1P bit. Others: Reserved Note: In PWM mode 1 or 2, the OCREF level changes when the result of the comparison changes or when the output compare mode switches from frozen to PWM mode. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OC1M1_A {
    #[doc = "0: Frozen. The comparison between the output compare register TIMx_CCR1 and the counter TIMx_CNT has no effect on the outputs. "]
    B_0X0 = 0,
    #[doc = "1: Set channel 1 to active level on match. OC1REF signal is forced high when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1). "]
    B_0X1 = 1,
    #[doc = "2: Set channel 1 to inactive level on match. OC1REF signal is forced low when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1). "]
    B_0X2 = 2,
    #[doc = "3: Toggle - OC1REF toggles when TIMx_CNT = TIMx_CCR1. "]
    B_0X3 = 3,
    #[doc = "4: Force inactive level - OC1REF is forced low. "]
    B_0X4 = 4,
    #[doc = "5: Force active level - OC1REF is forced high."]
    B_0X5 = 5,
    #[doc = "6: PWM mode 1 - Channel 1 is active as long as TIMx_CNT < TIMx_CCR1 else inactive."]
    B_0X6 = 6,
    #[doc = "7: PWM mode 2 - Channel 1 is inactive as long as TIMx_CNT < TIMx_CCR1 else active"]
    B_0X7 = 7,
}
impl From<OC1M1_A> for u8 {
    #[inline(always)]
    fn from(variant: OC1M1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OC1M1`"]
pub type OC1M1_R = crate::R<u8, OC1M1_A>;
impl OC1M1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OC1M1_A {
        match self.bits {
            0 => OC1M1_A::B_0X0,
            1 => OC1M1_A::B_0X1,
            2 => OC1M1_A::B_0X2,
            3 => OC1M1_A::B_0X3,
            4 => OC1M1_A::B_0X4,
            5 => OC1M1_A::B_0X5,
            6 => OC1M1_A::B_0X6,
            7 => OC1M1_A::B_0X7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OC1M1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OC1M1_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == OC1M1_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == OC1M1_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == OC1M1_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == OC1M1_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == OC1M1_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == OC1M1_A::B_0X7
    }
}
#[doc = "Write proxy for field `OC1M1`"]
pub struct OC1M1_W<'a> {
    w: &'a mut W,
}
impl<'a> OC1M1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OC1M1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Frozen. The comparison between the output compare register TIMx_CCR1 and the counter TIMx_CNT has no effect on the outputs."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(OC1M1_A::B_0X0)
    }
    #[doc = "Set channel 1 to active level on match. OC1REF signal is forced high when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(OC1M1_A::B_0X1)
    }
    #[doc = "Set channel 1 to inactive level on match. OC1REF signal is forced low when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1)."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(OC1M1_A::B_0X2)
    }
    #[doc = "Toggle - OC1REF toggles when TIMx_CNT = TIMx_CCR1."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(OC1M1_A::B_0X3)
    }
    #[doc = "Force inactive level - OC1REF is forced low."]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(OC1M1_A::B_0X4)
    }
    #[doc = "Force active level - OC1REF is forced high."]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(OC1M1_A::B_0X5)
    }
    #[doc = "PWM mode 1 - Channel 1 is active as long as TIMx_CNT < TIMx_CCR1 else inactive."]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(OC1M1_A::B_0X6)
    }
    #[doc = "PWM mode 2 - Channel 1 is inactive as long as TIMx_CNT < TIMx_CCR1 else active"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(OC1M1_A::B_0X7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Output compare 1 mode (refer to bit 16 for OC1M\\[3\\]) These bits define the behavior of the output reference signal OC1REF from which OC1 is derived. OC1REF is active high whereas OC1 active level depends on CC1P bit. Others: Reserved Note: In PWM mode 1 or 2, the OCREF level changes when the result of the comparison changes or when the output compare mode switches from frozen to PWM mode. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OC1M2_A {
    #[doc = "0: Frozen. The comparison between the output compare register TIMx_CCR1 and the counter TIMx_CNT has no effect on the outputs. "]
    B_0X0 = 0,
    #[doc = "1: Set channel 1 to active level on match. OC1REF signal is forced high when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1). "]
    B_0X1 = 1,
    #[doc = "2: Set channel 1 to inactive level on match. OC1REF signal is forced low when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1). "]
    B_0X2 = 2,
    #[doc = "3: Toggle - OC1REF toggles when TIMx_CNT = TIMx_CCR1. "]
    B_0X3 = 3,
    #[doc = "4: Force inactive level - OC1REF is forced low. "]
    B_0X4 = 4,
    #[doc = "5: Force active level - OC1REF is forced high."]
    B_0X5 = 5,
    #[doc = "6: PWM mode 1 - Channel 1 is active as long as TIMx_CNT < TIMx_CCR1 else inactive."]
    B_0X6 = 6,
    #[doc = "7: PWM mode 2 - Channel 1 is inactive as long as TIMx_CNT < TIMx_CCR1 else active"]
    B_0X7 = 7,
}
impl From<OC1M2_A> for bool {
    #[inline(always)]
    fn from(variant: OC1M2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OC1M2`"]
pub type OC1M2_R = crate::R<bool, OC1M2_A>;
impl OC1M2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, OC1M2_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(OC1M2_A::B_0X0),
            true => Val(OC1M2_A::B_0X1),
            true => Val(OC1M2_A::B_0X2),
            true => Val(OC1M2_A::B_0X3),
            true => Val(OC1M2_A::B_0X4),
            true => Val(OC1M2_A::B_0X5),
            true => Val(OC1M2_A::B_0X6),
            true => Val(OC1M2_A::B_0X7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OC1M2_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OC1M2_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == OC1M2_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == OC1M2_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == OC1M2_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == OC1M2_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == OC1M2_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == OC1M2_A::B_0X7
    }
}
#[doc = "Write proxy for field `OC1M2`"]
pub struct OC1M2_W<'a> {
    w: &'a mut W,
}
impl<'a> OC1M2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OC1M2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Frozen. The comparison between the output compare register TIMx_CCR1 and the counter TIMx_CNT has no effect on the outputs."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(OC1M2_A::B_0X0)
    }
    #[doc = "Set channel 1 to active level on match. OC1REF signal is forced high when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(OC1M2_A::B_0X1)
    }
    #[doc = "Set channel 1 to inactive level on match. OC1REF signal is forced low when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1)."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(OC1M2_A::B_0X2)
    }
    #[doc = "Toggle - OC1REF toggles when TIMx_CNT = TIMx_CCR1."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(OC1M2_A::B_0X3)
    }
    #[doc = "Force inactive level - OC1REF is forced low."]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(OC1M2_A::B_0X4)
    }
    #[doc = "Force active level - OC1REF is forced high."]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(OC1M2_A::B_0X5)
    }
    #[doc = "PWM mode 1 - Channel 1 is active as long as TIMx_CNT < TIMx_CCR1 else inactive."]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(OC1M2_A::B_0X6)
    }
    #[doc = "PWM mode 2 - Channel 1 is inactive as long as TIMx_CNT < TIMx_CCR1 else active"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(OC1M2_A::B_0X7)
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
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Output compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger."]
    #[inline(always)]
    pub fn oc1fe(&self) -> OC1FE_R {
        OC1FE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Output compare 1 preload enable Note: The PWM mode can be used without validating the preload register only in one pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed."]
    #[inline(always)]
    pub fn oc1pe(&self) -> OC1PE_R {
        OC1PE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Output compare 1 mode (refer to bit 16 for OC1M\\[3\\]) These bits define the behavior of the output reference signal OC1REF from which OC1 is derived. OC1REF is active high whereas OC1 active level depends on CC1P bit. Others: Reserved Note: In PWM mode 1 or 2, the OCREF level changes when the result of the comparison changes or when the output compare mode switches from frozen to PWM mode. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
    #[inline(always)]
    pub fn oc1m1(&self) -> OC1M1_R {
        OC1M1_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 16 - Output compare 1 mode (refer to bit 16 for OC1M\\[3\\]) These bits define the behavior of the output reference signal OC1REF from which OC1 is derived. OC1REF is active high whereas OC1 active level depends on CC1P bit. Others: Reserved Note: In PWM mode 1 or 2, the OCREF level changes when the result of the comparison changes or when the output compare mode switches from frozen to PWM mode. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
    #[inline(always)]
    pub fn oc1m2(&self) -> OC1M2_R {
        OC1M2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc1s(&mut self) -> CC1S_W {
        CC1S_W { w: self }
    }
    #[doc = "Bit 2 - Output compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger."]
    #[inline(always)]
    pub fn oc1fe(&mut self) -> OC1FE_W {
        OC1FE_W { w: self }
    }
    #[doc = "Bit 3 - Output compare 1 preload enable Note: The PWM mode can be used without validating the preload register only in one pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed."]
    #[inline(always)]
    pub fn oc1pe(&mut self) -> OC1PE_W {
        OC1PE_W { w: self }
    }
    #[doc = "Bits 4:6 - Output compare 1 mode (refer to bit 16 for OC1M\\[3\\]) These bits define the behavior of the output reference signal OC1REF from which OC1 is derived. OC1REF is active high whereas OC1 active level depends on CC1P bit. Others: Reserved Note: In PWM mode 1 or 2, the OCREF level changes when the result of the comparison changes or when the output compare mode switches from frozen to PWM mode. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
    #[inline(always)]
    pub fn oc1m1(&mut self) -> OC1M1_W {
        OC1M1_W { w: self }
    }
    #[doc = "Bit 16 - Output compare 1 mode (refer to bit 16 for OC1M\\[3\\]) These bits define the behavior of the output reference signal OC1REF from which OC1 is derived. OC1REF is active high whereas OC1 active level depends on CC1P bit. Others: Reserved Note: In PWM mode 1 or 2, the OCREF level changes when the result of the comparison changes or when the output compare mode switches from frozen to PWM mode. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
    #[inline(always)]
    pub fn oc1m2(&mut self) -> OC1M2_W {
        OC1M2_W { w: self }
    }
}
