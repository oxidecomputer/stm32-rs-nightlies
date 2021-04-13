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
#[doc = "Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Other codes: reserved. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=â\u{80}\u{99}00100â\u{80}\u{99}). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMS1_A {
    #[doc = "0: Slave mode disabled - if CEN = '1â\u{80}\u{99} then the prescaler is clocked directly by the internal clock."]
    B_0X0 = 0,
    #[doc = "4: Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers."]
    B_0X4 = 4,
    #[doc = "5: Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled."]
    B_0X5 = 5,
    #[doc = "6: Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled."]
    B_0X6 = 6,
    #[doc = "7: External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter."]
    B_0X7 = 7,
    #[doc = "8: Combined reset + trigger mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter, generates an update of the registers and starts the counter."]
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
    #[doc = "Slave mode disabled - if CEN = '1â\u{80}\u{99} then the prescaler is clocked directly by the internal clock."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SMS1_A::B_0X0)
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
    #[doc = "Combined reset + trigger mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter, generates an update of the registers and starts the counter."]
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
#[doc = "Trigger selection This bit field selects the trigger input to be used to synchronize the counter. Other: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.\n\nValue on reset: 0"]
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Master/slave mode\n\nValue on reset: 0"]
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
#[doc = "Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Other codes: reserved. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=â\u{80}\u{99}00100â\u{80}\u{99}). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMS2_A {
    #[doc = "0: Slave mode disabled - if CEN = '1â\u{80}\u{99} then the prescaler is clocked directly by the internal clock."]
    B_0X0 = 0,
    #[doc = "4: Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers."]
    B_0X4 = 4,
    #[doc = "5: Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled."]
    B_0X5 = 5,
    #[doc = "6: Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled."]
    B_0X6 = 6,
    #[doc = "7: External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter."]
    B_0X7 = 7,
    #[doc = "8: Combined reset + trigger mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter, generates an update of the registers and starts the counter."]
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
    #[doc = "Slave mode disabled - if CEN = '1â\u{80}\u{99} then the prescaler is clocked directly by the internal clock."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SMS2_A::B_0X0)
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
    #[doc = "Combined reset + trigger mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter, generates an update of the registers and starts the counter."]
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
#[doc = "Trigger selection This bit field selects the trigger input to be used to synchronize the counter. Other: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.\n\nValue on reset: 0"]
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Other codes: reserved. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=â\u{80}\u{99}00100â\u{80}\u{99}). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn sms1(&self) -> SMS1_R {
        SMS1_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Trigger selection This bit field selects the trigger input to be used to synchronize the counter. Other: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    pub fn ts1(&self) -> TS1_R {
        TS1_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Master/slave mode"]
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Other codes: reserved. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=â\u{80}\u{99}00100â\u{80}\u{99}). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn sms2(&self) -> SMS2_R {
        SMS2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - Trigger selection This bit field selects the trigger input to be used to synchronize the counter. Other: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    pub fn ts2(&self) -> TS2_R {
        TS2_R::new(((self.bits >> 20) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Other codes: reserved. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=â\u{80}\u{99}00100â\u{80}\u{99}). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn sms1(&mut self) -> SMS1_W {
        SMS1_W { w: self }
    }
    #[doc = "Bits 4:6 - Trigger selection This bit field selects the trigger input to be used to synchronize the counter. Other: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    pub fn ts1(&mut self) -> TS1_W {
        TS1_W { w: self }
    }
    #[doc = "Bit 7 - Master/slave mode"]
    #[inline(always)]
    pub fn msm(&mut self) -> MSM_W {
        MSM_W { w: self }
    }
    #[doc = "Bit 16 - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Other codes: reserved. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=â\u{80}\u{99}00100â\u{80}\u{99}). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn sms2(&mut self) -> SMS2_W {
        SMS2_W { w: self }
    }
    #[doc = "Bits 20:21 - Trigger selection This bit field selects the trigger input to be used to synchronize the counter. Other: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    pub fn ts2(&mut self) -> TS2_W {
        TS2_W { w: self }
    }
}
