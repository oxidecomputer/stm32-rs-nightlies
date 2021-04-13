#[doc = "Reader of register DAC_SR"]
pub type R = crate::R<u32, super::DAC_SR>;
#[doc = "Writer for register DAC_SR"]
pub type W = crate::W<u32, super::DAC_SR>;
#[doc = "Register DAC_SR `reset()`'s with value 0"]
impl crate::ResetValue for super::DAC_SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAUDR1_A {
    #[doc = "0: No DMA underrun error condition occurred for DAC channel1"]
    B_0X0 = 0,
    #[doc = "1: DMA underrun error condition occurred for DAC channel1 (the currently selected trigger is driving DAC channel1 conversion at a frequency higher than the DMA service capability rate)"]
    B_0X1 = 1,
}
impl From<DMAUDR1_A> for bool {
    #[inline(always)]
    fn from(variant: DMAUDR1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAUDR1`"]
pub type DMAUDR1_R = crate::R<bool, DMAUDR1_A>;
impl DMAUDR1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAUDR1_A {
        match self.bits {
            false => DMAUDR1_A::B_0X0,
            true => DMAUDR1_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DMAUDR1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DMAUDR1_A::B_0X1
    }
}
#[doc = "Write proxy for field `DMAUDR1`"]
pub struct DMAUDR1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAUDR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAUDR1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No DMA underrun error condition occurred for DAC channel1"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DMAUDR1_A::B_0X0)
    }
    #[doc = "DMA underrun error condition occurred for DAC channel1 (the currently selected trigger is driving DAC channel1 conversion at a frequency higher than the DMA service capability rate)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DMAUDR1_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "DAC channel1 calibration offset status This bit is set and cleared by hardware\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAL_FLAG1_A {
    #[doc = "0: calibration trimming value is lower than the offset correction value"]
    B_0X0 = 0,
    #[doc = "1: calibration trimming value is equal or greater than the offset correction value"]
    B_0X1 = 1,
}
impl From<CAL_FLAG1_A> for bool {
    #[inline(always)]
    fn from(variant: CAL_FLAG1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAL_FLAG1`"]
pub type CAL_FLAG1_R = crate::R<bool, CAL_FLAG1_A>;
impl CAL_FLAG1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAL_FLAG1_A {
        match self.bits {
            false => CAL_FLAG1_A::B_0X0,
            true => CAL_FLAG1_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CAL_FLAG1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CAL_FLAG1_A::B_0X1
    }
}
#[doc = "DAC channel1 busy writing sample time flag This bit is systematically set just after Sample and hold mode enable and is set each time the software writes the register DAC_SHSR1, It is cleared by hardware when the write operation of DAC_SHSR1 is complete. (It takes about 3 LSI periods of synchronization).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWST1_A {
    #[doc = "0: There is no write operation of DAC_SHSR1 ongoing: DAC_SHSR1 can be written"]
    B_0X0 = 0,
    #[doc = "1: There is a write operation of DAC_SHSR1 ongoing: DAC_SHSR1 cannot be written"]
    B_0X1 = 1,
}
impl From<BWST1_A> for bool {
    #[inline(always)]
    fn from(variant: BWST1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BWST1`"]
pub type BWST1_R = crate::R<bool, BWST1_A>;
impl BWST1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWST1_A {
        match self.bits {
            false => BWST1_A::B_0X0,
            true => BWST1_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BWST1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BWST1_A::B_0X1
    }
}
#[doc = "DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1). Note: This bit is available only on dual-channel DACs. Refer to implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAUDR2_A {
    #[doc = "0: No DMA underrun error condition occurred for DAC channel2"]
    B_0X0 = 0,
    #[doc = "1: DMA underrun error condition occurred for DAC channel2 (the currently selected trigger is driving DAC channel2 conversion at a frequency higher than the DMA service capability rate)."]
    B_0X1 = 1,
}
impl From<DMAUDR2_A> for bool {
    #[inline(always)]
    fn from(variant: DMAUDR2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAUDR2`"]
pub type DMAUDR2_R = crate::R<bool, DMAUDR2_A>;
impl DMAUDR2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAUDR2_A {
        match self.bits {
            false => DMAUDR2_A::B_0X0,
            true => DMAUDR2_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DMAUDR2_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DMAUDR2_A::B_0X1
    }
}
#[doc = "Write proxy for field `DMAUDR2`"]
pub struct DMAUDR2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAUDR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAUDR2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No DMA underrun error condition occurred for DAC channel2"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DMAUDR2_A::B_0X0)
    }
    #[doc = "DMA underrun error condition occurred for DAC channel2 (the currently selected trigger is driving DAC channel2 conversion at a frequency higher than the DMA service capability rate)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DMAUDR2_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "DAC channel2 calibration offset status This bit is set and cleared by hardware Note: This bit is available only on dual-channel DACs. Refer to implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAL_FLAG2_A {
    #[doc = "0: calibration trimming value is lower than the offset correction value"]
    B_0X0 = 0,
    #[doc = "1: calibration trimming value is equal or greater than the offset correction value"]
    B_0X1 = 1,
}
impl From<CAL_FLAG2_A> for bool {
    #[inline(always)]
    fn from(variant: CAL_FLAG2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAL_FLAG2`"]
pub type CAL_FLAG2_R = crate::R<bool, CAL_FLAG2_A>;
impl CAL_FLAG2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAL_FLAG2_A {
        match self.bits {
            false => CAL_FLAG2_A::B_0X0,
            true => CAL_FLAG2_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CAL_FLAG2_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CAL_FLAG2_A::B_0X1
    }
}
#[doc = "DAC channel2 busy writing sample time flag This bit is systematically set just after Sample and hold mode enable. It is set each time the software writes the register DAC_SHSR2, It is cleared by hardware when the write operation of DAC_SHSR2 is complete. (It takes about 3 LSI periods of synchronization). Note: This bit is available only on dual-channel DACs. Refer to implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWST2_A {
    #[doc = "0: There is no write operation of DAC_SHSR2 ongoing: DAC_SHSR2 can be written"]
    B_0X0 = 0,
    #[doc = "1: There is a write operation of DAC_SHSR2 ongoing: DAC_SHSR2 cannot be written"]
    B_0X1 = 1,
}
impl From<BWST2_A> for bool {
    #[inline(always)]
    fn from(variant: BWST2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BWST2`"]
pub type BWST2_R = crate::R<bool, BWST2_A>;
impl BWST2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWST2_A {
        match self.bits {
            false => BWST2_A::B_0X0,
            true => BWST2_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BWST2_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BWST2_A::B_0X1
    }
}
impl R {
    #[doc = "Bit 13 - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
    #[inline(always)]
    pub fn dmaudr1(&self) -> DMAUDR1_R {
        DMAUDR1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - DAC channel1 calibration offset status This bit is set and cleared by hardware"]
    #[inline(always)]
    pub fn cal_flag1(&self) -> CAL_FLAG1_R {
        CAL_FLAG1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DAC channel1 busy writing sample time flag This bit is systematically set just after Sample and hold mode enable and is set each time the software writes the register DAC_SHSR1, It is cleared by hardware when the write operation of DAC_SHSR1 is complete. (It takes about 3 LSI periods of synchronization)."]
    #[inline(always)]
    pub fn bwst1(&self) -> BWST1_R {
        BWST1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1). Note: This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn dmaudr2(&self) -> DMAUDR2_R {
        DMAUDR2_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DAC channel2 calibration offset status This bit is set and cleared by hardware Note: This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn cal_flag2(&self) -> CAL_FLAG2_R {
        CAL_FLAG2_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DAC channel2 busy writing sample time flag This bit is systematically set just after Sample and hold mode enable. It is set each time the software writes the register DAC_SHSR2, It is cleared by hardware when the write operation of DAC_SHSR2 is complete. (It takes about 3 LSI periods of synchronization). Note: This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn bwst2(&self) -> BWST2_R {
        BWST2_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
    #[inline(always)]
    pub fn dmaudr1(&mut self) -> DMAUDR1_W {
        DMAUDR1_W { w: self }
    }
    #[doc = "Bit 29 - DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1). Note: This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn dmaudr2(&mut self) -> DMAUDR2_W {
        DMAUDR2_W { w: self }
    }
}
