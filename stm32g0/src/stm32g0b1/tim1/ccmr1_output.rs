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
#[doc = "Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = '0â\u{80}\u{99} in TIMx_CCER).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CC1S_A {
    #[doc = "0: CC1 channel is configured as output"]
    B_0X0 = 0,
    #[doc = "1: CC1 channel is configured as input, IC1 is mapped on TI1"]
    B_0X1 = 1,
    #[doc = "2: CC1 channel is configured as input, IC1 is mapped on TI2"]
    B_0X2 = 2,
    #[doc = "3: CC1 channel is configured as input, IC1 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    B_0X3 = 3,
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
    pub fn variant(&self) -> CC1S_A {
        match self.bits {
            0 => CC1S_A::B_0X0,
            1 => CC1S_A::B_0X1,
            2 => CC1S_A::B_0X2,
            3 => CC1S_A::B_0X3,
            _ => unreachable!(),
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
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == CC1S_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == CC1S_A::B_0X3
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
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CC1 channel is configured as output"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CC1S_A::B_0X0)
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CC1S_A::B_0X1)
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(CC1S_A::B_0X2)
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(CC1S_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Output Compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OC1FE_A {
    #[doc = "0: CC1 behaves normally depending on counter and CCR1 values even when the trigger is ON. The minimum delay to activate CC1 output when an edge occurs on the trigger input is 5 clock cycles."]
    B_0X0 = 0,
    #[doc = "1: An active edge on the trigger input acts like a compare match on CC1 output. Then, OC is set to the compare level independently from the result of the comparison. Delay to sample the trigger input and to activate CC1 output is reduced to 3 clock cycles. OCFE acts only if the channel is configured in PWM1 or PWM2 mode."]
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
    #[doc = "An active edge on the trigger input acts like a compare match on CC1 output. Then, OC is set to the compare level independently from the result of the comparison. Delay to sample the trigger input and to activate CC1 output is reduced to 3 clock cycles. OCFE acts only if the channel is configured in PWM1 or PWM2 mode."]
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
#[doc = "Output Compare 1 preload enable Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=â\u{80}\u{99}00â\u{80}\u{99} (the channel is configured in output). The PWM mode can be used without validating the preload register only in one pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OC1PE_A {
    #[doc = "0: Preload register on TIMx_CCR1 disabled. TIMx_CCR1 can be written at anytime, the new value is taken in account immediately."]
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
#[doc = "Output Compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=â\u{80}\u{99}00â\u{80}\u{99} (the channel is configured in output). Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from â\u{80}\u{9c}frozenâ\u{80}\u{9d} mode to â\u{80}\u{9c}PWMâ\u{80}\u{9d} mode. Note: On channels having a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OC1M1_A {
    #[doc = "0: Frozen - The comparison between the output compare register TIMx_CCR1 and the counter TIMx_CNT has no effect on the outputs.(this mode is used to generate a timing base)."]
    B_0X0 = 0,
    #[doc = "1: Set channel 1 to active level on match. OC1REF signal is forced high when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1)."]
    B_0X1 = 1,
    #[doc = "2: Set channel 1 to inactive level on match. OC1REF signal is forced low when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1)."]
    B_0X2 = 2,
    #[doc = "3: Toggle - OC1REF toggles when TIMx_CNT=TIMx_CCR1."]
    B_0X3 = 3,
    #[doc = "4: Force inactive level - OC1REF is forced low."]
    B_0X4 = 4,
    #[doc = "5: Force active level - OC1REF is forced high."]
    B_0X5 = 5,
    #[doc = "6: PWM mode 1 - In upcounting, channel 1 is active as long as TIMx_CNT<TIMx_CCR1 else inactive. In downcounting, channel 1 is inactive (OC1REF='0â\u{80}\u{99}) as long as TIMx_CNT>TIMx_CCR1 else active (OC1REF=â\u{80}\u{99}1â\u{80}\u{99})."]
    B_0X6 = 6,
    #[doc = "7: PWM mode 2 - In upcounting, channel 1 is inactive as long as TIMx_CNT<TIMx_CCR1 else active. In downcounting, channel 1 is active as long as TIMx_CNT>TIMx_CCR1 else inactive."]
    B_0X7 = 7,
    #[doc = "8: Retrigerrable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). Then, a comparison is performed as in PWM mode 1 and the channels becomes active again at the next update. In down-counting mode, the channel is inactive until a trigger event is detected (on TRGI signal). Then, a comparison is performed as in PWM mode 1 and the channels becomes inactive again at the next update."]
    B_0X8 = 8,
    #[doc = "9: Retrigerrable OPM mode 2 - In up-counting mode, the channel is inactive until a trigger event is detected (on TRGI signal). Then, a comparison is performed as in PWM mode 2 and the channels becomes inactive again at the next update. In down-counting mode, the channel is active until a trigger event is detected (on TRGI signal). Then, a comparison is performed as in PWM mode 1 and the channels becomes active again at the next update."]
    B_0X9 = 9,
    #[doc = "12: Combined PWM mode 1 - OC1REF has the same behavior as in PWM mode 1. OC1REFC is the logical OR between OC1REF and OC2REF."]
    B_0XC = 12,
    #[doc = "13: Combined PWM mode 2 - OC1REF has the same behavior as in PWM mode 2. OC1REFC is the logical AND between OC1REF and OC2REF."]
    B_0XD = 13,
    #[doc = "14: Asymmetric PWM mode 1 - OC1REF has the same behavior as in PWM mode 1. OC1REFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down."]
    B_0XE = 14,
    #[doc = "15: Asymmetric PWM mode 2 - OC1REF has the same behavior as in PWM mode 2. OC1REFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down."]
    B_0XF = 15,
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
    pub fn variant(&self) -> crate::Variant<u8, OC1M1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OC1M1_A::B_0X0),
            1 => Val(OC1M1_A::B_0X1),
            2 => Val(OC1M1_A::B_0X2),
            3 => Val(OC1M1_A::B_0X3),
            4 => Val(OC1M1_A::B_0X4),
            5 => Val(OC1M1_A::B_0X5),
            6 => Val(OC1M1_A::B_0X6),
            7 => Val(OC1M1_A::B_0X7),
            8 => Val(OC1M1_A::B_0X8),
            9 => Val(OC1M1_A::B_0X9),
            12 => Val(OC1M1_A::B_0XC),
            13 => Val(OC1M1_A::B_0XD),
            14 => Val(OC1M1_A::B_0XE),
            15 => Val(OC1M1_A::B_0XF),
            i => Res(i),
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
    #[doc = "Checks if the value of the field is `B_0X8`"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == OC1M1_A::B_0X8
    }
    #[doc = "Checks if the value of the field is `B_0X9`"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == OC1M1_A::B_0X9
    }
    #[doc = "Checks if the value of the field is `B_0XC`"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == OC1M1_A::B_0XC
    }
    #[doc = "Checks if the value of the field is `B_0XD`"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == OC1M1_A::B_0XD
    }
    #[doc = "Checks if the value of the field is `B_0XE`"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == OC1M1_A::B_0XE
    }
    #[doc = "Checks if the value of the field is `B_0XF`"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == OC1M1_A::B_0XF
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
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Frozen - The comparison between the output compare register TIMx_CCR1 and the counter TIMx_CNT has no effect on the outputs.(this mode is used to generate a timing base)."]
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
    #[doc = "Toggle - OC1REF toggles when TIMx_CNT=TIMx_CCR1."]
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
    #[doc = "PWM mode 1 - In upcounting, channel 1 is active as long as TIMx_CNT<TIMx_CCR1 else inactive. In downcounting, channel 1 is inactive (OC1REF='0â\u{80}\u{99}) as long as TIMx_CNT>TIMx_CCR1 else active (OC1REF=â\u{80}\u{99}1â\u{80}\u{99})."]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(OC1M1_A::B_0X6)
    }
    #[doc = "PWM mode 2 - In upcounting, channel 1 is inactive as long as TIMx_CNT<TIMx_CCR1 else active. In downcounting, channel 1 is active as long as TIMx_CNT>TIMx_CCR1 else inactive."]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(OC1M1_A::B_0X7)
    }
    #[doc = "Retrigerrable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). Then, a comparison is performed as in PWM mode 1 and the channels becomes active again at the next update. In down-counting mode, the channel is inactive until a trigger event is detected (on TRGI signal). Then, a comparison is performed as in PWM mode 1 and the channels becomes inactive again at the next update."]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut W {
        self.variant(OC1M1_A::B_0X8)
    }
    #[doc = "Retrigerrable OPM mode 2 - In up-counting mode, the channel is inactive until a trigger event is detected (on TRGI signal). Then, a comparison is performed as in PWM mode 2 and the channels becomes inactive again at the next update. In down-counting mode, the channel is active until a trigger event is detected (on TRGI signal). Then, a comparison is performed as in PWM mode 1 and the channels becomes active again at the next update."]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut W {
        self.variant(OC1M1_A::B_0X9)
    }
    #[doc = "Combined PWM mode 1 - OC1REF has the same behavior as in PWM mode 1. OC1REFC is the logical OR between OC1REF and OC2REF."]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut W {
        self.variant(OC1M1_A::B_0XC)
    }
    #[doc = "Combined PWM mode 2 - OC1REF has the same behavior as in PWM mode 2. OC1REFC is the logical AND between OC1REF and OC2REF."]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut W {
        self.variant(OC1M1_A::B_0XD)
    }
    #[doc = "Asymmetric PWM mode 1 - OC1REF has the same behavior as in PWM mode 1. OC1REFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down."]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut W {
        self.variant(OC1M1_A::B_0XE)
    }
    #[doc = "Asymmetric PWM mode 2 - OC1REF has the same behavior as in PWM mode 2. OC1REFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down."]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut W {
        self.variant(OC1M1_A::B_0XF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Output Compare 1 clear enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OC1CE_A {
    #[doc = "0: OC1Ref is not affected by the ocref_clr_int signal"]
    B_0X0 = 0,
    #[doc = "1: OC1Ref is cleared as soon as a High level is detected on ocref_clr_int signal (OCREF_CLR input or ETRF input)"]
    B_0X1 = 1,
}
impl From<OC1CE_A> for bool {
    #[inline(always)]
    fn from(variant: OC1CE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OC1CE`"]
pub type OC1CE_R = crate::R<bool, OC1CE_A>;
impl OC1CE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OC1CE_A {
        match self.bits {
            false => OC1CE_A::B_0X0,
            true => OC1CE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OC1CE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OC1CE_A::B_0X1
    }
}
#[doc = "Write proxy for field `OC1CE`"]
pub struct OC1CE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC1CE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OC1CE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "OC1Ref is not affected by the ocref_clr_int signal"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(OC1CE_A::B_0X0)
    }
    #[doc = "OC1Ref is cleared as soon as a High level is detected on ocref_clr_int signal (OCREF_CLR input or ETRF input)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(OC1CE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = '0â\u{80}\u{99} in TIMx_CCER).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CC2S_A {
    #[doc = "0: CC2 channel is configured as output"]
    B_0X0 = 0,
    #[doc = "1: CC2 channel is configured as input, IC2 is mapped on TI2"]
    B_0X1 = 1,
    #[doc = "2: CC2 channel is configured as input, IC2 is mapped on TI1"]
    B_0X2 = 2,
    #[doc = "3: CC2 channel is configured as input, IC2 is mapped on TRC. This mode is working only if an internal trigger input is selected through the TS bit (TIMx_SMCR register)"]
    B_0X3 = 3,
}
impl From<CC2S_A> for u8 {
    #[inline(always)]
    fn from(variant: CC2S_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CC2S`"]
pub type CC2S_R = crate::R<u8, CC2S_A>;
impl CC2S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC2S_A {
        match self.bits {
            0 => CC2S_A::B_0X0,
            1 => CC2S_A::B_0X1,
            2 => CC2S_A::B_0X2,
            3 => CC2S_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC2S_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC2S_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == CC2S_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == CC2S_A::B_0X3
    }
}
#[doc = "Write proxy for field `CC2S`"]
pub struct CC2S_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC2S_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CC2 channel is configured as output"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CC2S_A::B_0X0)
    }
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TI2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CC2S_A::B_0X1)
    }
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TI1"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(CC2S_A::B_0X2)
    }
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TRC. This mode is working only if an internal trigger input is selected through the TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(CC2S_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `OC2FE`"]
pub type OC2FE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC2FE`"]
pub struct OC2FE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC2FE_W<'a> {
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
#[doc = "Reader of field `OC2PE`"]
pub type OC2PE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC2PE`"]
pub struct OC2PE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC2PE_W<'a> {
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
#[doc = "Reader of field `OC2M1`"]
pub type OC2M1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OC2M1`"]
pub struct OC2M1_W<'a> {
    w: &'a mut W,
}
impl<'a> OC2M1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `OC2CE`"]
pub type OC2CE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC2CE`"]
pub struct OC2CE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC2CE_W<'a> {
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
#[doc = "Output Compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=â\u{80}\u{99}00â\u{80}\u{99} (the channel is configured in output). Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from â\u{80}\u{9c}frozenâ\u{80}\u{9d} mode to â\u{80}\u{9c}PWMâ\u{80}\u{9d} mode. Note: On channels having a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OC1M2_A {
    #[doc = "0: Frozen - The comparison between the output compare register TIMx_CCR1 and the counter TIMx_CNT has no effect on the outputs.(this mode is used to generate a timing base)."]
    B_0X0 = 0,
    #[doc = "1: Set channel 1 to active level on match. OC1REF signal is forced high when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1)."]
    B_0X1 = 1,
    #[doc = "2: Set channel 1 to inactive level on match. OC1REF signal is forced low when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1)."]
    B_0X2 = 2,
    #[doc = "3: Toggle - OC1REF toggles when TIMx_CNT=TIMx_CCR1."]
    B_0X3 = 3,
    #[doc = "4: Force inactive level - OC1REF is forced low."]
    B_0X4 = 4,
    #[doc = "5: Force active level - OC1REF is forced high."]
    B_0X5 = 5,
    #[doc = "6: PWM mode 1 - In upcounting, channel 1 is active as long as TIMx_CNT<TIMx_CCR1 else inactive. In downcounting, channel 1 is inactive (OC1REF='0â\u{80}\u{99}) as long as TIMx_CNT>TIMx_CCR1 else active (OC1REF=â\u{80}\u{99}1â\u{80}\u{99})."]
    B_0X6 = 6,
    #[doc = "7: PWM mode 2 - In upcounting, channel 1 is inactive as long as TIMx_CNT<TIMx_CCR1 else active. In downcounting, channel 1 is active as long as TIMx_CNT>TIMx_CCR1 else inactive."]
    B_0X7 = 7,
    #[doc = "8: Retrigerrable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). Then, a comparison is performed as in PWM mode 1 and the channels becomes active again at the next update. In down-counting mode, the channel is inactive until a trigger event is detected (on TRGI signal). Then, a comparison is performed as in PWM mode 1 and the channels becomes inactive again at the next update."]
    B_0X8 = 8,
    #[doc = "9: Retrigerrable OPM mode 2 - In up-counting mode, the channel is inactive until a trigger event is detected (on TRGI signal). Then, a comparison is performed as in PWM mode 2 and the channels becomes inactive again at the next update. In down-counting mode, the channel is active until a trigger event is detected (on TRGI signal). Then, a comparison is performed as in PWM mode 1 and the channels becomes active again at the next update."]
    B_0X9 = 9,
    #[doc = "12: Combined PWM mode 1 - OC1REF has the same behavior as in PWM mode 1. OC1REFC is the logical OR between OC1REF and OC2REF."]
    B_0XC = 12,
    #[doc = "13: Combined PWM mode 2 - OC1REF has the same behavior as in PWM mode 2. OC1REFC is the logical AND between OC1REF and OC2REF."]
    B_0XD = 13,
    #[doc = "14: Asymmetric PWM mode 1 - OC1REF has the same behavior as in PWM mode 1. OC1REFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down."]
    B_0XE = 14,
    #[doc = "15: Asymmetric PWM mode 2 - OC1REF has the same behavior as in PWM mode 2. OC1REFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down."]
    B_0XF = 15,
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
            true => Val(OC1M2_A::B_0X8),
            true => Val(OC1M2_A::B_0X9),
            true => Val(OC1M2_A::B_0XC),
            true => Val(OC1M2_A::B_0XD),
            true => Val(OC1M2_A::B_0XE),
            true => Val(OC1M2_A::B_0XF),
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
    #[doc = "Checks if the value of the field is `B_0X8`"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == OC1M2_A::B_0X8
    }
    #[doc = "Checks if the value of the field is `B_0X9`"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == OC1M2_A::B_0X9
    }
    #[doc = "Checks if the value of the field is `B_0XC`"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == OC1M2_A::B_0XC
    }
    #[doc = "Checks if the value of the field is `B_0XD`"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == OC1M2_A::B_0XD
    }
    #[doc = "Checks if the value of the field is `B_0XE`"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == OC1M2_A::B_0XE
    }
    #[doc = "Checks if the value of the field is `B_0XF`"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == OC1M2_A::B_0XF
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
    #[doc = "Frozen - The comparison between the output compare register TIMx_CCR1 and the counter TIMx_CNT has no effect on the outputs.(this mode is used to generate a timing base)."]
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
    #[doc = "Toggle - OC1REF toggles when TIMx_CNT=TIMx_CCR1."]
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
    #[doc = "PWM mode 1 - In upcounting, channel 1 is active as long as TIMx_CNT<TIMx_CCR1 else inactive. In downcounting, channel 1 is inactive (OC1REF='0â\u{80}\u{99}) as long as TIMx_CNT>TIMx_CCR1 else active (OC1REF=â\u{80}\u{99}1â\u{80}\u{99})."]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(OC1M2_A::B_0X6)
    }
    #[doc = "PWM mode 2 - In upcounting, channel 1 is inactive as long as TIMx_CNT<TIMx_CCR1 else active. In downcounting, channel 1 is active as long as TIMx_CNT>TIMx_CCR1 else inactive."]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(OC1M2_A::B_0X7)
    }
    #[doc = "Retrigerrable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). Then, a comparison is performed as in PWM mode 1 and the channels becomes active again at the next update. In down-counting mode, the channel is inactive until a trigger event is detected (on TRGI signal). Then, a comparison is performed as in PWM mode 1 and the channels becomes inactive again at the next update."]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut W {
        self.variant(OC1M2_A::B_0X8)
    }
    #[doc = "Retrigerrable OPM mode 2 - In up-counting mode, the channel is inactive until a trigger event is detected (on TRGI signal). Then, a comparison is performed as in PWM mode 2 and the channels becomes inactive again at the next update. In down-counting mode, the channel is active until a trigger event is detected (on TRGI signal). Then, a comparison is performed as in PWM mode 1 and the channels becomes active again at the next update."]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut W {
        self.variant(OC1M2_A::B_0X9)
    }
    #[doc = "Combined PWM mode 1 - OC1REF has the same behavior as in PWM mode 1. OC1REFC is the logical OR between OC1REF and OC2REF."]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut W {
        self.variant(OC1M2_A::B_0XC)
    }
    #[doc = "Combined PWM mode 2 - OC1REF has the same behavior as in PWM mode 2. OC1REFC is the logical AND between OC1REF and OC2REF."]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut W {
        self.variant(OC1M2_A::B_0XD)
    }
    #[doc = "Asymmetric PWM mode 1 - OC1REF has the same behavior as in PWM mode 1. OC1REFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down."]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut W {
        self.variant(OC1M2_A::B_0XE)
    }
    #[doc = "Asymmetric PWM mode 2 - OC1REF has the same behavior as in PWM mode 2. OC1REFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down."]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut W {
        self.variant(OC1M2_A::B_0XF)
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
#[doc = "Reader of field `OC2M2`"]
pub type OC2M2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC2M2`"]
pub struct OC2M2_W<'a> {
    w: &'a mut W,
}
impl<'a> OC2M2_W<'a> {
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
    #[doc = "Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = '0â\u{80}\u{99} in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Output Compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger."]
    #[inline(always)]
    pub fn oc1fe(&self) -> OC1FE_R {
        OC1FE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Output Compare 1 preload enable Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=â\u{80}\u{99}00â\u{80}\u{99} (the channel is configured in output). The PWM mode can be used without validating the preload register only in one pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed."]
    #[inline(always)]
    pub fn oc1pe(&self) -> OC1PE_R {
        OC1PE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Output Compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=â\u{80}\u{99}00â\u{80}\u{99} (the channel is configured in output). Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from â\u{80}\u{9c}frozenâ\u{80}\u{9d} mode to â\u{80}\u{9c}PWMâ\u{80}\u{9d} mode. Note: On channels having a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
    #[inline(always)]
    pub fn oc1m1(&self) -> OC1M1_R {
        OC1M1_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Output Compare 1 clear enable"]
    #[inline(always)]
    pub fn oc1ce(&self) -> OC1CE_R {
        OC1CE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = '0â\u{80}\u{99} in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc2s(&self) -> CC2S_R {
        CC2S_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Output Compare 2 fast enable Refer to OC1FE description."]
    #[inline(always)]
    pub fn oc2fe(&self) -> OC2FE_R {
        OC2FE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Output Compare 2 preload enable Refer to OC1PE description."]
    #[inline(always)]
    pub fn oc2pe(&self) -> OC2PE_R {
        OC2PE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - Output Compare 2 mode Refer to OC1M\\[3:0\\]
description."]
    #[inline(always)]
    pub fn oc2m1(&self) -> OC2M1_R {
        OC2M1_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Output Compare 2 clear enable Refer to OC1CE description."]
    #[inline(always)]
    pub fn oc2ce(&self) -> OC2CE_R {
        OC2CE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Output Compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=â\u{80}\u{99}00â\u{80}\u{99} (the channel is configured in output). Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from â\u{80}\u{9c}frozenâ\u{80}\u{9d} mode to â\u{80}\u{9c}PWMâ\u{80}\u{9d} mode. Note: On channels having a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
    #[inline(always)]
    pub fn oc1m2(&self) -> OC1M2_R {
        OC1M2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Output Compare 2 mode Refer to OC1M\\[3:0\\]
description."]
    #[inline(always)]
    pub fn oc2m2(&self) -> OC2M2_R {
        OC2M2_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = '0â\u{80}\u{99} in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc1s(&mut self) -> CC1S_W {
        CC1S_W { w: self }
    }
    #[doc = "Bit 2 - Output Compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger."]
    #[inline(always)]
    pub fn oc1fe(&mut self) -> OC1FE_W {
        OC1FE_W { w: self }
    }
    #[doc = "Bit 3 - Output Compare 1 preload enable Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=â\u{80}\u{99}00â\u{80}\u{99} (the channel is configured in output). The PWM mode can be used without validating the preload register only in one pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed."]
    #[inline(always)]
    pub fn oc1pe(&mut self) -> OC1PE_W {
        OC1PE_W { w: self }
    }
    #[doc = "Bits 4:6 - Output Compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=â\u{80}\u{99}00â\u{80}\u{99} (the channel is configured in output). Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from â\u{80}\u{9c}frozenâ\u{80}\u{9d} mode to â\u{80}\u{9c}PWMâ\u{80}\u{9d} mode. Note: On channels having a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
    #[inline(always)]
    pub fn oc1m1(&mut self) -> OC1M1_W {
        OC1M1_W { w: self }
    }
    #[doc = "Bit 7 - Output Compare 1 clear enable"]
    #[inline(always)]
    pub fn oc1ce(&mut self) -> OC1CE_W {
        OC1CE_W { w: self }
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = '0â\u{80}\u{99} in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc2s(&mut self) -> CC2S_W {
        CC2S_W { w: self }
    }
    #[doc = "Bit 10 - Output Compare 2 fast enable Refer to OC1FE description."]
    #[inline(always)]
    pub fn oc2fe(&mut self) -> OC2FE_W {
        OC2FE_W { w: self }
    }
    #[doc = "Bit 11 - Output Compare 2 preload enable Refer to OC1PE description."]
    #[inline(always)]
    pub fn oc2pe(&mut self) -> OC2PE_W {
        OC2PE_W { w: self }
    }
    #[doc = "Bits 12:14 - Output Compare 2 mode Refer to OC1M\\[3:0\\]
description."]
    #[inline(always)]
    pub fn oc2m1(&mut self) -> OC2M1_W {
        OC2M1_W { w: self }
    }
    #[doc = "Bit 15 - Output Compare 2 clear enable Refer to OC1CE description."]
    #[inline(always)]
    pub fn oc2ce(&mut self) -> OC2CE_W {
        OC2CE_W { w: self }
    }
    #[doc = "Bit 16 - Output Compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=â\u{80}\u{99}00â\u{80}\u{99} (the channel is configured in output). Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from â\u{80}\u{9c}frozenâ\u{80}\u{9d} mode to â\u{80}\u{9c}PWMâ\u{80}\u{9d} mode. Note: On channels having a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
    #[inline(always)]
    pub fn oc1m2(&mut self) -> OC1M2_W {
        OC1M2_W { w: self }
    }
    #[doc = "Bit 24 - Output Compare 2 mode Refer to OC1M\\[3:0\\]
description."]
    #[inline(always)]
    pub fn oc2m2(&mut self) -> OC2M2_W {
        OC2M2_W { w: self }
    }
}
