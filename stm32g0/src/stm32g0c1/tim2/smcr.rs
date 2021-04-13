#[doc = "Reader of register SMCR"]
pub type R = crate::R<u32, super::SMCR>;
#[doc = "Writer for register SMCR"]
pub type W = crate::W<u32, super::SMCR>;
#[doc = "Register SMCR `reset()`'s with value 0"]
impl crate::ResetValue for super::SMCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. reinitializes the counter, generates an update of the registers and starts the counter. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMS1_A {
    #[doc = "0: Slave mode disabled - if CEN = '1 then the prescaler is clocked directly by the internal clock."]
    B_0X0 = 0,
    #[doc = "1: Encoder mode 1 - Counter counts up/down on TI1FP1 edge depending on TI2FP2 level."]
    B_0X1 = 1,
    #[doc = "2: Encoder mode 2 - Counter counts up/down on TI2FP2 edge depending on TI1FP1 level."]
    B_0X2 = 2,
    #[doc = "3: Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input."]
    B_0X3 = 3,
    #[doc = "4: Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers."]
    B_0X4 = 4,
    #[doc = "5: Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled."]
    B_0X5 = 5,
    #[doc = "6: Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled."]
    B_0X6 = 6,
    #[doc = "7: External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter."]
    B_0X7 = 7,
    #[doc = "8: Combined reset + trigger mode - Rising edge of the selected trigger input (TRGI)"]
    B_0X8 = 8,
}
impl From<SMS1_A> for u8 {
    #[inline(always)]
    fn from(variant: SMS1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SMS1`"]
pub type SMS1_R = crate::R<u8, SMS1_A>;
impl SMS1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SMS1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SMS1_A::B_0X0),
            1 => Val(SMS1_A::B_0X1),
            2 => Val(SMS1_A::B_0X2),
            3 => Val(SMS1_A::B_0X3),
            4 => Val(SMS1_A::B_0X4),
            5 => Val(SMS1_A::B_0X5),
            6 => Val(SMS1_A::B_0X6),
            7 => Val(SMS1_A::B_0X7),
            8 => Val(SMS1_A::B_0X8),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMS1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMS1_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == SMS1_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == SMS1_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == SMS1_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == SMS1_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == SMS1_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == SMS1_A::B_0X7
    }
    #[doc = "Checks if the value of the field is `B_0X8`"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == SMS1_A::B_0X8
    }
}
#[doc = "Write proxy for field `SMS1`"]
pub struct SMS1_W<'a> {
    w: &'a mut W,
}
impl<'a> SMS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMS1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Slave mode disabled - if CEN = '1 then the prescaler is clocked directly by the internal clock."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SMS1_A::B_0X0)
    }
    #[doc = "Encoder mode 1 - Counter counts up/down on TI1FP1 edge depending on TI2FP2 level."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SMS1_A::B_0X1)
    }
    #[doc = "Encoder mode 2 - Counter counts up/down on TI2FP2 edge depending on TI1FP1 level."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(SMS1_A::B_0X2)
    }
    #[doc = "Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(SMS1_A::B_0X3)
    }
    #[doc = "Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers."]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(SMS1_A::B_0X4)
    }
    #[doc = "Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled."]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(SMS1_A::B_0X5)
    }
    #[doc = "Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled."]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(SMS1_A::B_0X6)
    }
    #[doc = "External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter."]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(SMS1_A::B_0X7)
    }
    #[doc = "Combined reset + trigger mode - Rising edge of the selected trigger input (TRGI)"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut W {
        self.variant(SMS1_A::B_0X8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "OCREF clear selection This bit is used to select the OCREF clear source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCCS_A {
    #[doc = "0: OCREF_CLR_INT is connected to COMP1 or COMP2 output depending on TIMx_OR1.OCREF_CLR"]
    B_0X0 = 0,
    #[doc = "1: OCREF_CLR_INT is connected to ETRF"]
    B_0X1 = 1,
}
impl From<OCCS_A> for bool {
    #[inline(always)]
    fn from(variant: OCCS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OCCS`"]
pub type OCCS_R = crate::R<bool, OCCS_A>;
impl OCCS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCCS_A {
        match self.bits {
            false => OCCS_A::B_0X0,
            true => OCCS_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OCCS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OCCS_A::B_0X1
    }
}
#[doc = "Write proxy for field `OCCS`"]
pub struct OCCS_W<'a> {
    w: &'a mut W,
}
impl<'a> OCCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OCCS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "OCREF_CLR_INT is connected to COMP1 or COMP2 output depending on TIMx_OR1.OCREF_CLR"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(OCCS_A::B_0X0)
    }
    #[doc = "OCREF_CLR_INT is connected to ETRF"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(OCCS_A::B_0X1)
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
#[doc = "Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TS1_A {
    #[doc = "0: Internal Trigger 0 (ITR0)"]
    B_0X0 = 0,
    #[doc = "1: Internal Trigger 1 (ITR1)"]
    B_0X1 = 1,
    #[doc = "2: Internal Trigger 2 (ITR2)"]
    B_0X2 = 2,
    #[doc = "3: Internal Trigger 3 (ITR3)"]
    B_0X3 = 3,
    #[doc = "4: TI1 Edge Detector (TI1F_ED)"]
    B_0X4 = 4,
    #[doc = "5: Filtered Timer Input 1 (TI1FP1)"]
    B_0X5 = 5,
    #[doc = "6: Filtered Timer Input 2 (TI2FP2)"]
    B_0X6 = 6,
    #[doc = "7: External Trigger input (ETRF)"]
    B_0X7 = 7,
    #[doc = "8: Internal Trigger 4 (ITR4)"]
    B_0X8 = 8,
    #[doc = "9: Internal Trigger 5 (ITR5)"]
    B_0X9 = 9,
    #[doc = "10: Internal Trigger 6 (ITR6)"]
    B_0XA = 10,
    #[doc = "11: Internal Trigger 7 (ITR7)"]
    B_0XB = 11,
    #[doc = "12: Internal Trigger 8 (ITR8)"]
    B_0XC = 12,
}
impl From<TS1_A> for u8 {
    #[inline(always)]
    fn from(variant: TS1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TS1`"]
pub type TS1_R = crate::R<u8, TS1_A>;
impl TS1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TS1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TS1_A::B_0X0),
            1 => Val(TS1_A::B_0X1),
            2 => Val(TS1_A::B_0X2),
            3 => Val(TS1_A::B_0X3),
            4 => Val(TS1_A::B_0X4),
            5 => Val(TS1_A::B_0X5),
            6 => Val(TS1_A::B_0X6),
            7 => Val(TS1_A::B_0X7),
            8 => Val(TS1_A::B_0X8),
            9 => Val(TS1_A::B_0X9),
            10 => Val(TS1_A::B_0XA),
            11 => Val(TS1_A::B_0XB),
            12 => Val(TS1_A::B_0XC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TS1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TS1_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == TS1_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == TS1_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == TS1_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == TS1_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == TS1_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == TS1_A::B_0X7
    }
    #[doc = "Checks if the value of the field is `B_0X8`"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == TS1_A::B_0X8
    }
    #[doc = "Checks if the value of the field is `B_0X9`"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == TS1_A::B_0X9
    }
    #[doc = "Checks if the value of the field is `B_0XA`"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == TS1_A::B_0XA
    }
    #[doc = "Checks if the value of the field is `B_0XB`"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == TS1_A::B_0XB
    }
    #[doc = "Checks if the value of the field is `B_0XC`"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == TS1_A::B_0XC
    }
}
#[doc = "Write proxy for field `TS1`"]
pub struct TS1_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TS1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Internal Trigger 0 (ITR0)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TS1_A::B_0X0)
    }
    #[doc = "Internal Trigger 1 (ITR1)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TS1_A::B_0X1)
    }
    #[doc = "Internal Trigger 2 (ITR2)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(TS1_A::B_0X2)
    }
    #[doc = "Internal Trigger 3 (ITR3)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(TS1_A::B_0X3)
    }
    #[doc = "TI1 Edge Detector (TI1F_ED)"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(TS1_A::B_0X4)
    }
    #[doc = "Filtered Timer Input 1 (TI1FP1)"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(TS1_A::B_0X5)
    }
    #[doc = "Filtered Timer Input 2 (TI2FP2)"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(TS1_A::B_0X6)
    }
    #[doc = "External Trigger input (ETRF)"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(TS1_A::B_0X7)
    }
    #[doc = "Internal Trigger 4 (ITR4)"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut W {
        self.variant(TS1_A::B_0X8)
    }
    #[doc = "Internal Trigger 5 (ITR5)"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut W {
        self.variant(TS1_A::B_0X9)
    }
    #[doc = "Internal Trigger 6 (ITR6)"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut W {
        self.variant(TS1_A::B_0XA)
    }
    #[doc = "Internal Trigger 7 (ITR7)"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut W {
        self.variant(TS1_A::B_0XB)
    }
    #[doc = "Internal Trigger 8 (ITR8)"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut W {
        self.variant(TS1_A::B_0XC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Master/Slave mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSM_A {
    #[doc = "0: No action"]
    B_0X0 = 0,
    #[doc = "1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    B_0X1 = 1,
}
impl From<MSM_A> for bool {
    #[inline(always)]
    fn from(variant: MSM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSM`"]
pub type MSM_R = crate::R<bool, MSM_A>;
impl MSM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSM_A {
        match self.bits {
            false => MSM_A::B_0X0,
            true => MSM_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MSM_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MSM_A::B_0X1
    }
}
#[doc = "Write proxy for field `MSM`"]
pub struct MSM_W<'a> {
    w: &'a mut W,
}
impl<'a> MSM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MSM_A::B_0X0)
    }
    #[doc = "The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MSM_A::B_0X1)
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
#[doc = "External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ETF_A {
    #[doc = "0: No filter, sampling is done at fDTS"]
    B_0X0 = 0,
    #[doc = "1: fSAMPLING=fCK_INT, N=2"]
    B_0X1 = 1,
    #[doc = "2: fSAMPLING=fCK_INT, N=4"]
    B_0X2 = 2,
    #[doc = "3: fSAMPLING=fCK_INT, N=8"]
    B_0X3 = 3,
    #[doc = "4: fSAMPLING=fDTS/2, N=6"]
    B_0X4 = 4,
    #[doc = "5: fSAMPLING=fDTS/2, N=8"]
    B_0X5 = 5,
    #[doc = "6: fSAMPLING=fDTS/4, N=6"]
    B_0X6 = 6,
    #[doc = "7: fSAMPLING=fDTS/4, N=8"]
    B_0X7 = 7,
    #[doc = "8: fSAMPLING=fDTS/8, N=6"]
    B_0X8 = 8,
    #[doc = "9: fSAMPLING=fDTS/8, N=8"]
    B_0X9 = 9,
    #[doc = "10: fSAMPLING=fDTS/16, N=5"]
    B_0XA = 10,
    #[doc = "11: fSAMPLING=fDTS/16, N=6"]
    B_0XB = 11,
    #[doc = "12: fSAMPLING=fDTS/16, N=8"]
    B_0XC = 12,
    #[doc = "13: fSAMPLING=fDTS/32, N=5"]
    B_0XD = 13,
    #[doc = "14: fSAMPLING=fDTS/32, N=6"]
    B_0XE = 14,
    #[doc = "15: fSAMPLING=fDTS/32, N=8"]
    B_0XF = 15,
}
impl From<ETF_A> for u8 {
    #[inline(always)]
    fn from(variant: ETF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ETF`"]
pub type ETF_R = crate::R<u8, ETF_A>;
impl ETF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETF_A {
        match self.bits {
            0 => ETF_A::B_0X0,
            1 => ETF_A::B_0X1,
            2 => ETF_A::B_0X2,
            3 => ETF_A::B_0X3,
            4 => ETF_A::B_0X4,
            5 => ETF_A::B_0X5,
            6 => ETF_A::B_0X6,
            7 => ETF_A::B_0X7,
            8 => ETF_A::B_0X8,
            9 => ETF_A::B_0X9,
            10 => ETF_A::B_0XA,
            11 => ETF_A::B_0XB,
            12 => ETF_A::B_0XC,
            13 => ETF_A::B_0XD,
            14 => ETF_A::B_0XE,
            15 => ETF_A::B_0XF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ETF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ETF_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == ETF_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == ETF_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == ETF_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == ETF_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == ETF_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == ETF_A::B_0X7
    }
    #[doc = "Checks if the value of the field is `B_0X8`"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == ETF_A::B_0X8
    }
    #[doc = "Checks if the value of the field is `B_0X9`"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == ETF_A::B_0X9
    }
    #[doc = "Checks if the value of the field is `B_0XA`"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == ETF_A::B_0XA
    }
    #[doc = "Checks if the value of the field is `B_0XB`"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == ETF_A::B_0XB
    }
    #[doc = "Checks if the value of the field is `B_0XC`"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == ETF_A::B_0XC
    }
    #[doc = "Checks if the value of the field is `B_0XD`"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == ETF_A::B_0XD
    }
    #[doc = "Checks if the value of the field is `B_0XE`"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == ETF_A::B_0XE
    }
    #[doc = "Checks if the value of the field is `B_0XF`"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == ETF_A::B_0XF
    }
}
#[doc = "Write proxy for field `ETF`"]
pub struct ETF_W<'a> {
    w: &'a mut W,
}
impl<'a> ETF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No filter, sampling is done at fDTS"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ETF_A::B_0X0)
    }
    #[doc = "fSAMPLING=fCK_INT, N=2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ETF_A::B_0X1)
    }
    #[doc = "fSAMPLING=fCK_INT, N=4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(ETF_A::B_0X2)
    }
    #[doc = "fSAMPLING=fCK_INT, N=8"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(ETF_A::B_0X3)
    }
    #[doc = "fSAMPLING=fDTS/2, N=6"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(ETF_A::B_0X4)
    }
    #[doc = "fSAMPLING=fDTS/2, N=8"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(ETF_A::B_0X5)
    }
    #[doc = "fSAMPLING=fDTS/4, N=6"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(ETF_A::B_0X6)
    }
    #[doc = "fSAMPLING=fDTS/4, N=8"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(ETF_A::B_0X7)
    }
    #[doc = "fSAMPLING=fDTS/8, N=6"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut W {
        self.variant(ETF_A::B_0X8)
    }
    #[doc = "fSAMPLING=fDTS/8, N=8"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut W {
        self.variant(ETF_A::B_0X9)
    }
    #[doc = "fSAMPLING=fDTS/16, N=5"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut W {
        self.variant(ETF_A::B_0XA)
    }
    #[doc = "fSAMPLING=fDTS/16, N=6"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut W {
        self.variant(ETF_A::B_0XB)
    }
    #[doc = "fSAMPLING=fDTS/16, N=8"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut W {
        self.variant(ETF_A::B_0XC)
    }
    #[doc = "fSAMPLING=fDTS/32, N=5"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut W {
        self.variant(ETF_A::B_0XD)
    }
    #[doc = "fSAMPLING=fDTS/32, N=6"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut W {
        self.variant(ETF_A::B_0XE)
    }
    #[doc = "fSAMPLING=fDTS/32, N=8"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut W {
        self.variant(ETF_A::B_0XF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of CK_INT frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ETPS_A {
    #[doc = "0: Prescaler OFF"]
    B_0X0 = 0,
    #[doc = "1: ETRP frequency divided by 2"]
    B_0X1 = 1,
    #[doc = "2: ETRP frequency divided by 4"]
    B_0X2 = 2,
    #[doc = "3: ETRP frequency divided by 8"]
    B_0X3 = 3,
}
impl From<ETPS_A> for u8 {
    #[inline(always)]
    fn from(variant: ETPS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ETPS`"]
pub type ETPS_R = crate::R<u8, ETPS_A>;
impl ETPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETPS_A {
        match self.bits {
            0 => ETPS_A::B_0X0,
            1 => ETPS_A::B_0X1,
            2 => ETPS_A::B_0X2,
            3 => ETPS_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ETPS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ETPS_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == ETPS_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == ETPS_A::B_0X3
    }
}
#[doc = "Write proxy for field `ETPS`"]
pub struct ETPS_W<'a> {
    w: &'a mut W,
}
impl<'a> ETPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETPS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Prescaler OFF"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ETPS_A::B_0X0)
    }
    #[doc = "ETRP frequency divided by 2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ETPS_A::B_0X1)
    }
    #[doc = "ETRP frequency divided by 4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(ETPS_A::B_0X2)
    }
    #[doc = "ETRP frequency divided by 8"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(ETPS_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with TRGI connected to ETRF (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 00111). If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECE_A {
    #[doc = "0: External clock mode 2 disabled"]
    B_0X0 = 0,
    #[doc = "1: External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal."]
    B_0X1 = 1,
}
impl From<ECE_A> for bool {
    #[inline(always)]
    fn from(variant: ECE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ECE`"]
pub type ECE_R = crate::R<bool, ECE_A>;
impl ECE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECE_A {
        match self.bits {
            false => ECE_A::B_0X0,
            true => ECE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ECE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ECE_A::B_0X1
    }
}
#[doc = "Write proxy for field `ECE`"]
pub struct ECE_W<'a> {
    w: &'a mut W,
}
impl<'a> ECE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External clock mode 2 disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ECE_A::B_0X0)
    }
    #[doc = "External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ECE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "External trigger polarity This bit selects whether ETR or ETR is used for trigger operations\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETP_A {
    #[doc = "0: ETR is non-inverted, active at high level or rising edge"]
    B_0X0 = 0,
    #[doc = "1: ETR is inverted, active at low level or falling edge"]
    B_0X1 = 1,
}
impl From<ETP_A> for bool {
    #[inline(always)]
    fn from(variant: ETP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ETP`"]
pub type ETP_R = crate::R<bool, ETP_A>;
impl ETP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETP_A {
        match self.bits {
            false => ETP_A::B_0X0,
            true => ETP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ETP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ETP_A::B_0X1
    }
}
#[doc = "Write proxy for field `ETP`"]
pub struct ETP_W<'a> {
    w: &'a mut W,
}
impl<'a> ETP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ETR is non-inverted, active at high level or rising edge"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ETP_A::B_0X0)
    }
    #[doc = "ETR is inverted, active at low level or falling edge"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ETP_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. reinitializes the counter, generates an update of the registers and starts the counter. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMS2_A {
    #[doc = "0: Slave mode disabled - if CEN = '1 then the prescaler is clocked directly by the internal clock."]
    B_0X0 = 0,
    #[doc = "1: Encoder mode 1 - Counter counts up/down on TI1FP1 edge depending on TI2FP2 level."]
    B_0X1 = 1,
    #[doc = "2: Encoder mode 2 - Counter counts up/down on TI2FP2 edge depending on TI1FP1 level."]
    B_0X2 = 2,
    #[doc = "3: Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input."]
    B_0X3 = 3,
    #[doc = "4: Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers."]
    B_0X4 = 4,
    #[doc = "5: Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled."]
    B_0X5 = 5,
    #[doc = "6: Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled."]
    B_0X6 = 6,
    #[doc = "7: External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter."]
    B_0X7 = 7,
    #[doc = "8: Combined reset + trigger mode - Rising edge of the selected trigger input (TRGI)"]
    B_0X8 = 8,
}
impl From<SMS2_A> for bool {
    #[inline(always)]
    fn from(variant: SMS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMS2`"]
pub type SMS2_R = crate::R<bool, SMS2_A>;
impl SMS2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SMS2_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(SMS2_A::B_0X0),
            true => Val(SMS2_A::B_0X1),
            true => Val(SMS2_A::B_0X2),
            true => Val(SMS2_A::B_0X3),
            true => Val(SMS2_A::B_0X4),
            true => Val(SMS2_A::B_0X5),
            true => Val(SMS2_A::B_0X6),
            true => Val(SMS2_A::B_0X7),
            true => Val(SMS2_A::B_0X8),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMS2_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMS2_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == SMS2_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == SMS2_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == SMS2_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == SMS2_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == SMS2_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == SMS2_A::B_0X7
    }
    #[doc = "Checks if the value of the field is `B_0X8`"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == SMS2_A::B_0X8
    }
}
#[doc = "Write proxy for field `SMS2`"]
pub struct SMS2_W<'a> {
    w: &'a mut W,
}
impl<'a> SMS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMS2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Slave mode disabled - if CEN = '1 then the prescaler is clocked directly by the internal clock."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SMS2_A::B_0X0)
    }
    #[doc = "Encoder mode 1 - Counter counts up/down on TI1FP1 edge depending on TI2FP2 level."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SMS2_A::B_0X1)
    }
    #[doc = "Encoder mode 2 - Counter counts up/down on TI2FP2 edge depending on TI1FP1 level."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(SMS2_A::B_0X2)
    }
    #[doc = "Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(SMS2_A::B_0X3)
    }
    #[doc = "Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers."]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(SMS2_A::B_0X4)
    }
    #[doc = "Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled."]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(SMS2_A::B_0X5)
    }
    #[doc = "Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled."]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(SMS2_A::B_0X6)
    }
    #[doc = "External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter."]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(SMS2_A::B_0X7)
    }
    #[doc = "Combined reset + trigger mode - Rising edge of the selected trigger input (TRGI)"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut W {
        self.variant(SMS2_A::B_0X8)
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
#[doc = "Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TS2_A {
    #[doc = "0: Internal Trigger 0 (ITR0)"]
    B_0X0 = 0,
    #[doc = "1: Internal Trigger 1 (ITR1)"]
    B_0X1 = 1,
    #[doc = "2: Internal Trigger 2 (ITR2)"]
    B_0X2 = 2,
    #[doc = "3: Internal Trigger 3 (ITR3)"]
    B_0X3 = 3,
    #[doc = "4: TI1 Edge Detector (TI1F_ED)"]
    B_0X4 = 4,
    #[doc = "5: Filtered Timer Input 1 (TI1FP1)"]
    B_0X5 = 5,
    #[doc = "6: Filtered Timer Input 2 (TI2FP2)"]
    B_0X6 = 6,
    #[doc = "7: External Trigger input (ETRF)"]
    B_0X7 = 7,
    #[doc = "8: Internal Trigger 4 (ITR4)"]
    B_0X8 = 8,
    #[doc = "9: Internal Trigger 5 (ITR5)"]
    B_0X9 = 9,
    #[doc = "10: Internal Trigger 6 (ITR6)"]
    B_0XA = 10,
    #[doc = "11: Internal Trigger 7 (ITR7)"]
    B_0XB = 11,
    #[doc = "12: Internal Trigger 8 (ITR8)"]
    B_0XC = 12,
}
impl From<TS2_A> for u8 {
    #[inline(always)]
    fn from(variant: TS2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TS2`"]
pub type TS2_R = crate::R<u8, TS2_A>;
impl TS2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TS2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TS2_A::B_0X0),
            1 => Val(TS2_A::B_0X1),
            2 => Val(TS2_A::B_0X2),
            3 => Val(TS2_A::B_0X3),
            4 => Val(TS2_A::B_0X4),
            5 => Val(TS2_A::B_0X5),
            6 => Val(TS2_A::B_0X6),
            7 => Val(TS2_A::B_0X7),
            8 => Val(TS2_A::B_0X8),
            9 => Val(TS2_A::B_0X9),
            10 => Val(TS2_A::B_0XA),
            11 => Val(TS2_A::B_0XB),
            12 => Val(TS2_A::B_0XC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TS2_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TS2_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == TS2_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == TS2_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == TS2_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == TS2_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == TS2_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == TS2_A::B_0X7
    }
    #[doc = "Checks if the value of the field is `B_0X8`"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == TS2_A::B_0X8
    }
    #[doc = "Checks if the value of the field is `B_0X9`"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == TS2_A::B_0X9
    }
    #[doc = "Checks if the value of the field is `B_0XA`"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == TS2_A::B_0XA
    }
    #[doc = "Checks if the value of the field is `B_0XB`"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == TS2_A::B_0XB
    }
    #[doc = "Checks if the value of the field is `B_0XC`"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == TS2_A::B_0XC
    }
}
#[doc = "Write proxy for field `TS2`"]
pub struct TS2_W<'a> {
    w: &'a mut W,
}
impl<'a> TS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TS2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Internal Trigger 0 (ITR0)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TS2_A::B_0X0)
    }
    #[doc = "Internal Trigger 1 (ITR1)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TS2_A::B_0X1)
    }
    #[doc = "Internal Trigger 2 (ITR2)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(TS2_A::B_0X2)
    }
    #[doc = "Internal Trigger 3 (ITR3)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(TS2_A::B_0X3)
    }
    #[doc = "TI1 Edge Detector (TI1F_ED)"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(TS2_A::B_0X4)
    }
    #[doc = "Filtered Timer Input 1 (TI1FP1)"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(TS2_A::B_0X5)
    }
    #[doc = "Filtered Timer Input 2 (TI2FP2)"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(TS2_A::B_0X6)
    }
    #[doc = "External Trigger input (ETRF)"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(TS2_A::B_0X7)
    }
    #[doc = "Internal Trigger 4 (ITR4)"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut W {
        self.variant(TS2_A::B_0X8)
    }
    #[doc = "Internal Trigger 5 (ITR5)"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut W {
        self.variant(TS2_A::B_0X9)
    }
    #[doc = "Internal Trigger 6 (ITR6)"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut W {
        self.variant(TS2_A::B_0XA)
    }
    #[doc = "Internal Trigger 7 (ITR7)"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut W {
        self.variant(TS2_A::B_0XB)
    }
    #[doc = "Internal Trigger 8 (ITR8)"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut W {
        self.variant(TS2_A::B_0XC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. reinitializes the counter, generates an update of the registers and starts the counter. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn sms1(&self) -> SMS1_R {
        SMS1_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - OCREF clear selection This bit is used to select the OCREF clear source"]
    #[inline(always)]
    pub fn occs(&self) -> OCCS_R {
        OCCS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    pub fn ts1(&self) -> TS1_R {
        TS1_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
    #[inline(always)]
    pub fn etf(&self) -> ETF_R {
        ETF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of CK_INT frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks."]
    #[inline(always)]
    pub fn etps(&self) -> ETPS_R {
        ETPS_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with TRGI connected to ETRF (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 00111). If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF."]
    #[inline(always)]
    pub fn ece(&self) -> ECE_R {
        ECE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - External trigger polarity This bit selects whether ETR or ETR is used for trigger operations"]
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. reinitializes the counter, generates an update of the registers and starts the counter. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn sms2(&self) -> SMS2_R {
        SMS2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    pub fn ts2(&self) -> TS2_R {
        TS2_R::new(((self.bits >> 20) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. reinitializes the counter, generates an update of the registers and starts the counter. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn sms1(&mut self) -> SMS1_W {
        SMS1_W { w: self }
    }
    #[doc = "Bit 3 - OCREF clear selection This bit is used to select the OCREF clear source"]
    #[inline(always)]
    pub fn occs(&mut self) -> OCCS_W {
        OCCS_W { w: self }
    }
    #[doc = "Bits 4:6 - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    pub fn ts1(&mut self) -> TS1_W {
        TS1_W { w: self }
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    pub fn msm(&mut self) -> MSM_W {
        MSM_W { w: self }
    }
    #[doc = "Bits 8:11 - External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
    #[inline(always)]
    pub fn etf(&mut self) -> ETF_W {
        ETF_W { w: self }
    }
    #[doc = "Bits 12:13 - External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of CK_INT frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks."]
    #[inline(always)]
    pub fn etps(&mut self) -> ETPS_W {
        ETPS_W { w: self }
    }
    #[doc = "Bit 14 - External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with TRGI connected to ETRF (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 00111). If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF."]
    #[inline(always)]
    pub fn ece(&mut self) -> ECE_W {
        ECE_W { w: self }
    }
    #[doc = "Bit 15 - External trigger polarity This bit selects whether ETR or ETR is used for trigger operations"]
    #[inline(always)]
    pub fn etp(&mut self) -> ETP_W {
        ETP_W { w: self }
    }
    #[doc = "Bit 16 - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. reinitializes the counter, generates an update of the registers and starts the counter. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn sms2(&mut self) -> SMS2_W {
        SMS2_W { w: self }
    }
    #[doc = "Bits 20:21 - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    pub fn ts2(&mut self) -> TS2_W {
        TS2_W { w: self }
    }
}
