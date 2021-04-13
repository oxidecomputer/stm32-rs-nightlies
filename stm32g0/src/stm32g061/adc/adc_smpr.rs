#[doc = "Reader of register ADC_SMPR"]
pub type R = crate::R<u32, super::ADC_SMPR>;
#[doc = "Writer for register ADC_SMPR"]
pub type W = crate::W<u32, super::ADC_SMPR>;
#[doc = "Register ADC_SMPR `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_SMPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMP1_A {
    #[doc = "0: 1.5 ADC clock cycles "]
    B_0X0 = 0,
    #[doc = "1: 3.5 ADC clock cycles "]
    B_0X1 = 1,
    #[doc = "2: 7.5 ADC clock cycles "]
    B_0X2 = 2,
    #[doc = "3: 12.5 ADC clock cycles "]
    B_0X3 = 3,
    #[doc = "4: 19.5 ADC clock cycles "]
    B_0X4 = 4,
    #[doc = "5: 39.5 ADC clock cycles "]
    B_0X5 = 5,
    #[doc = "6: 79.5 ADC clock cycles "]
    B_0X6 = 6,
    #[doc = "7: 160.5 ADC clock cycles "]
    B_0X7 = 7,
}
impl From<SMP1_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SMP1`"]
pub type SMP1_R = crate::R<u8, SMP1_A>;
impl SMP1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMP1_A {
        match self.bits {
            0 => SMP1_A::B_0X0,
            1 => SMP1_A::B_0X1,
            2 => SMP1_A::B_0X2,
            3 => SMP1_A::B_0X3,
            4 => SMP1_A::B_0X4,
            5 => SMP1_A::B_0X5,
            6 => SMP1_A::B_0X6,
            7 => SMP1_A::B_0X7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMP1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMP1_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == SMP1_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == SMP1_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == SMP1_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == SMP1_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == SMP1_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == SMP1_A::B_0X7
    }
}
#[doc = "Write proxy for field `SMP1`"]
pub struct SMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SMP1_A::B_0X0)
    }
    #[doc = "3.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SMP1_A::B_0X1)
    }
    #[doc = "7.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(SMP1_A::B_0X2)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(SMP1_A::B_0X3)
    }
    #[doc = "19.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(SMP1_A::B_0X4)
    }
    #[doc = "39.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(SMP1_A::B_0X5)
    }
    #[doc = "79.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(SMP1_A::B_0X6)
    }
    #[doc = "160.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(SMP1_A::B_0X7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMP2_A {
    #[doc = "0: 1.5 ADC clock cycles "]
    B_0X0 = 0,
    #[doc = "1: 3.5 ADC clock cycles "]
    B_0X1 = 1,
    #[doc = "2: 7.5 ADC clock cycles "]
    B_0X2 = 2,
    #[doc = "3: 12.5 ADC clock cycles "]
    B_0X3 = 3,
    #[doc = "4: 19.5 ADC clock cycles "]
    B_0X4 = 4,
    #[doc = "5: 39.5 ADC clock cycles "]
    B_0X5 = 5,
    #[doc = "6: 79.5 ADC clock cycles "]
    B_0X6 = 6,
    #[doc = "7: 160.5 ADC clock cycles "]
    B_0X7 = 7,
}
impl From<SMP2_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SMP2`"]
pub type SMP2_R = crate::R<u8, SMP2_A>;
impl SMP2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMP2_A {
        match self.bits {
            0 => SMP2_A::B_0X0,
            1 => SMP2_A::B_0X1,
            2 => SMP2_A::B_0X2,
            3 => SMP2_A::B_0X3,
            4 => SMP2_A::B_0X4,
            5 => SMP2_A::B_0X5,
            6 => SMP2_A::B_0X6,
            7 => SMP2_A::B_0X7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMP2_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMP2_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == SMP2_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == SMP2_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == SMP2_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == SMP2_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == SMP2_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == SMP2_A::B_0X7
    }
}
#[doc = "Write proxy for field `SMP2`"]
pub struct SMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SMP2_A::B_0X0)
    }
    #[doc = "3.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SMP2_A::B_0X1)
    }
    #[doc = "7.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(SMP2_A::B_0X2)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(SMP2_A::B_0X3)
    }
    #[doc = "19.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(SMP2_A::B_0X4)
    }
    #[doc = "39.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(SMP2_A::B_0X5)
    }
    #[doc = "79.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(SMP2_A::B_0X6)
    }
    #[doc = "160.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(SMP2_A::B_0X7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPSEL0_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register. "]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register. "]
    B_0X1 = 1,
}
impl From<SMPSEL0_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMPSEL0`"]
pub type SMPSEL0_R = crate::R<bool, SMPSEL0_A>;
impl SMPSEL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPSEL0_A {
        match self.bits {
            false => SMPSEL0_A::B_0X0,
            true => SMPSEL0_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL0_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL0_A::B_0X1
    }
}
#[doc = "Write proxy for field `SMPSEL0`"]
pub struct SMPSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SMPSEL0_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SMPSEL0_A::B_0X1)
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
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPSEL1_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register. "]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register. "]
    B_0X1 = 1,
}
impl From<SMPSEL1_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMPSEL1`"]
pub type SMPSEL1_R = crate::R<bool, SMPSEL1_A>;
impl SMPSEL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPSEL1_A {
        match self.bits {
            false => SMPSEL1_A::B_0X0,
            true => SMPSEL1_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL1_A::B_0X1
    }
}
#[doc = "Write proxy for field `SMPSEL1`"]
pub struct SMPSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SMPSEL1_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SMPSEL1_A::B_0X1)
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
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPSEL2_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register. "]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register. "]
    B_0X1 = 1,
}
impl From<SMPSEL2_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMPSEL2`"]
pub type SMPSEL2_R = crate::R<bool, SMPSEL2_A>;
impl SMPSEL2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPSEL2_A {
        match self.bits {
            false => SMPSEL2_A::B_0X0,
            true => SMPSEL2_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL2_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL2_A::B_0X1
    }
}
#[doc = "Write proxy for field `SMPSEL2`"]
pub struct SMPSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SMPSEL2_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SMPSEL2_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPSEL3_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register. "]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register. "]
    B_0X1 = 1,
}
impl From<SMPSEL3_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMPSEL3`"]
pub type SMPSEL3_R = crate::R<bool, SMPSEL3_A>;
impl SMPSEL3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPSEL3_A {
        match self.bits {
            false => SMPSEL3_A::B_0X0,
            true => SMPSEL3_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL3_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL3_A::B_0X1
    }
}
#[doc = "Write proxy for field `SMPSEL3`"]
pub struct SMPSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SMPSEL3_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SMPSEL3_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPSEL4_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register. "]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register. "]
    B_0X1 = 1,
}
impl From<SMPSEL4_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMPSEL4`"]
pub type SMPSEL4_R = crate::R<bool, SMPSEL4_A>;
impl SMPSEL4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPSEL4_A {
        match self.bits {
            false => SMPSEL4_A::B_0X0,
            true => SMPSEL4_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL4_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL4_A::B_0X1
    }
}
#[doc = "Write proxy for field `SMPSEL4`"]
pub struct SMPSEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SMPSEL4_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SMPSEL4_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPSEL5_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register. "]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register. "]
    B_0X1 = 1,
}
impl From<SMPSEL5_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMPSEL5`"]
pub type SMPSEL5_R = crate::R<bool, SMPSEL5_A>;
impl SMPSEL5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPSEL5_A {
        match self.bits {
            false => SMPSEL5_A::B_0X0,
            true => SMPSEL5_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL5_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL5_A::B_0X1
    }
}
#[doc = "Write proxy for field `SMPSEL5`"]
pub struct SMPSEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SMPSEL5_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SMPSEL5_A::B_0X1)
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
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPSEL6_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register. "]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register. "]
    B_0X1 = 1,
}
impl From<SMPSEL6_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMPSEL6`"]
pub type SMPSEL6_R = crate::R<bool, SMPSEL6_A>;
impl SMPSEL6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPSEL6_A {
        match self.bits {
            false => SMPSEL6_A::B_0X0,
            true => SMPSEL6_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL6_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL6_A::B_0X1
    }
}
#[doc = "Write proxy for field `SMPSEL6`"]
pub struct SMPSEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SMPSEL6_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SMPSEL6_A::B_0X1)
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
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPSEL7_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register. "]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register. "]
    B_0X1 = 1,
}
impl From<SMPSEL7_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMPSEL7`"]
pub type SMPSEL7_R = crate::R<bool, SMPSEL7_A>;
impl SMPSEL7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPSEL7_A {
        match self.bits {
            false => SMPSEL7_A::B_0X0,
            true => SMPSEL7_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL7_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL7_A::B_0X1
    }
}
#[doc = "Write proxy for field `SMPSEL7`"]
pub struct SMPSEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SMPSEL7_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SMPSEL7_A::B_0X1)
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
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPSEL8_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register. "]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register. "]
    B_0X1 = 1,
}
impl From<SMPSEL8_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMPSEL8`"]
pub type SMPSEL8_R = crate::R<bool, SMPSEL8_A>;
impl SMPSEL8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPSEL8_A {
        match self.bits {
            false => SMPSEL8_A::B_0X0,
            true => SMPSEL8_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL8_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL8_A::B_0X1
    }
}
#[doc = "Write proxy for field `SMPSEL8`"]
pub struct SMPSEL8_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SMPSEL8_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SMPSEL8_A::B_0X1)
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
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPSEL9_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register. "]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register. "]
    B_0X1 = 1,
}
impl From<SMPSEL9_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMPSEL9`"]
pub type SMPSEL9_R = crate::R<bool, SMPSEL9_A>;
impl SMPSEL9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPSEL9_A {
        match self.bits {
            false => SMPSEL9_A::B_0X0,
            true => SMPSEL9_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL9_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL9_A::B_0X1
    }
}
#[doc = "Write proxy for field `SMPSEL9`"]
pub struct SMPSEL9_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SMPSEL9_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SMPSEL9_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPSEL10_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register. "]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register. "]
    B_0X1 = 1,
}
impl From<SMPSEL10_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMPSEL10`"]
pub type SMPSEL10_R = crate::R<bool, SMPSEL10_A>;
impl SMPSEL10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPSEL10_A {
        match self.bits {
            false => SMPSEL10_A::B_0X0,
            true => SMPSEL10_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL10_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL10_A::B_0X1
    }
}
#[doc = "Write proxy for field `SMPSEL10`"]
pub struct SMPSEL10_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SMPSEL10_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SMPSEL10_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPSEL11_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register. "]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register. "]
    B_0X1 = 1,
}
impl From<SMPSEL11_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMPSEL11`"]
pub type SMPSEL11_R = crate::R<bool, SMPSEL11_A>;
impl SMPSEL11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPSEL11_A {
        match self.bits {
            false => SMPSEL11_A::B_0X0,
            true => SMPSEL11_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL11_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL11_A::B_0X1
    }
}
#[doc = "Write proxy for field `SMPSEL11`"]
pub struct SMPSEL11_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SMPSEL11_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SMPSEL11_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPSEL12_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register. "]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register. "]
    B_0X1 = 1,
}
impl From<SMPSEL12_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMPSEL12`"]
pub type SMPSEL12_R = crate::R<bool, SMPSEL12_A>;
impl SMPSEL12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPSEL12_A {
        match self.bits {
            false => SMPSEL12_A::B_0X0,
            true => SMPSEL12_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL12_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL12_A::B_0X1
    }
}
#[doc = "Write proxy for field `SMPSEL12`"]
pub struct SMPSEL12_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SMPSEL12_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SMPSEL12_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPSEL13_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register. "]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register. "]
    B_0X1 = 1,
}
impl From<SMPSEL13_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMPSEL13`"]
pub type SMPSEL13_R = crate::R<bool, SMPSEL13_A>;
impl SMPSEL13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPSEL13_A {
        match self.bits {
            false => SMPSEL13_A::B_0X0,
            true => SMPSEL13_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL13_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL13_A::B_0X1
    }
}
#[doc = "Write proxy for field `SMPSEL13`"]
pub struct SMPSEL13_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SMPSEL13_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SMPSEL13_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPSEL14_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register. "]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register. "]
    B_0X1 = 1,
}
impl From<SMPSEL14_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMPSEL14`"]
pub type SMPSEL14_R = crate::R<bool, SMPSEL14_A>;
impl SMPSEL14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPSEL14_A {
        match self.bits {
            false => SMPSEL14_A::B_0X0,
            true => SMPSEL14_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL14_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL14_A::B_0X1
    }
}
#[doc = "Write proxy for field `SMPSEL14`"]
pub struct SMPSEL14_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SMPSEL14_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SMPSEL14_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPSEL15_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register. "]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register. "]
    B_0X1 = 1,
}
impl From<SMPSEL15_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMPSEL15`"]
pub type SMPSEL15_R = crate::R<bool, SMPSEL15_A>;
impl SMPSEL15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPSEL15_A {
        match self.bits {
            false => SMPSEL15_A::B_0X0,
            true => SMPSEL15_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL15_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL15_A::B_0X1
    }
}
#[doc = "Write proxy for field `SMPSEL15`"]
pub struct SMPSEL15_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SMPSEL15_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SMPSEL15_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPSEL16_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register. "]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register. "]
    B_0X1 = 1,
}
impl From<SMPSEL16_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMPSEL16`"]
pub type SMPSEL16_R = crate::R<bool, SMPSEL16_A>;
impl SMPSEL16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPSEL16_A {
        match self.bits {
            false => SMPSEL16_A::B_0X0,
            true => SMPSEL16_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL16_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL16_A::B_0X1
    }
}
#[doc = "Write proxy for field `SMPSEL16`"]
pub struct SMPSEL16_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SMPSEL16_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SMPSEL16_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPSEL17_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register. "]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register. "]
    B_0X1 = 1,
}
impl From<SMPSEL17_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMPSEL17`"]
pub type SMPSEL17_R = crate::R<bool, SMPSEL17_A>;
impl SMPSEL17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPSEL17_A {
        match self.bits {
            false => SMPSEL17_A::B_0X0,
            true => SMPSEL17_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL17_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL17_A::B_0X1
    }
}
#[doc = "Write proxy for field `SMPSEL17`"]
pub struct SMPSEL17_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SMPSEL17_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SMPSEL17_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPSEL18_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register. "]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register. "]
    B_0X1 = 1,
}
impl From<SMPSEL18_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL18_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMPSEL18`"]
pub type SMPSEL18_R = crate::R<bool, SMPSEL18_A>;
impl SMPSEL18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPSEL18_A {
        match self.bits {
            false => SMPSEL18_A::B_0X0,
            true => SMPSEL18_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL18_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL18_A::B_0X1
    }
}
#[doc = "Write proxy for field `SMPSEL18`"]
pub struct SMPSEL18_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SMPSEL18_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SMPSEL18_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smp1(&self) -> SMP1_R {
        SMP1_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smp2(&self) -> SMP2_R {
        SMP2_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 8 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel0(&self) -> SMPSEL0_R {
        SMPSEL0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel1(&self) -> SMPSEL1_R {
        SMPSEL1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel2(&self) -> SMPSEL2_R {
        SMPSEL2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel3(&self) -> SMPSEL3_R {
        SMPSEL3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel4(&self) -> SMPSEL4_R {
        SMPSEL4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel5(&self) -> SMPSEL5_R {
        SMPSEL5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel6(&self) -> SMPSEL6_R {
        SMPSEL6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel7(&self) -> SMPSEL7_R {
        SMPSEL7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel8(&self) -> SMPSEL8_R {
        SMPSEL8_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel9(&self) -> SMPSEL9_R {
        SMPSEL9_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel10(&self) -> SMPSEL10_R {
        SMPSEL10_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel11(&self) -> SMPSEL11_R {
        SMPSEL11_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel12(&self) -> SMPSEL12_R {
        SMPSEL12_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel13(&self) -> SMPSEL13_R {
        SMPSEL13_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel14(&self) -> SMPSEL14_R {
        SMPSEL14_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel15(&self) -> SMPSEL15_R {
        SMPSEL15_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel16(&self) -> SMPSEL16_R {
        SMPSEL16_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel17(&self) -> SMPSEL17_R {
        SMPSEL17_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel18(&self) -> SMPSEL18_R {
        SMPSEL18_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smp1(&mut self) -> SMP1_W {
        SMP1_W { w: self }
    }
    #[doc = "Bits 4:6 - Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smp2(&mut self) -> SMP2_W {
        SMP2_W { w: self }
    }
    #[doc = "Bit 8 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel0(&mut self) -> SMPSEL0_W {
        SMPSEL0_W { w: self }
    }
    #[doc = "Bit 9 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel1(&mut self) -> SMPSEL1_W {
        SMPSEL1_W { w: self }
    }
    #[doc = "Bit 10 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel2(&mut self) -> SMPSEL2_W {
        SMPSEL2_W { w: self }
    }
    #[doc = "Bit 11 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel3(&mut self) -> SMPSEL3_W {
        SMPSEL3_W { w: self }
    }
    #[doc = "Bit 12 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel4(&mut self) -> SMPSEL4_W {
        SMPSEL4_W { w: self }
    }
    #[doc = "Bit 13 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel5(&mut self) -> SMPSEL5_W {
        SMPSEL5_W { w: self }
    }
    #[doc = "Bit 14 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel6(&mut self) -> SMPSEL6_W {
        SMPSEL6_W { w: self }
    }
    #[doc = "Bit 15 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel7(&mut self) -> SMPSEL7_W {
        SMPSEL7_W { w: self }
    }
    #[doc = "Bit 16 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel8(&mut self) -> SMPSEL8_W {
        SMPSEL8_W { w: self }
    }
    #[doc = "Bit 17 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel9(&mut self) -> SMPSEL9_W {
        SMPSEL9_W { w: self }
    }
    #[doc = "Bit 18 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel10(&mut self) -> SMPSEL10_W {
        SMPSEL10_W { w: self }
    }
    #[doc = "Bit 19 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel11(&mut self) -> SMPSEL11_W {
        SMPSEL11_W { w: self }
    }
    #[doc = "Bit 20 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel12(&mut self) -> SMPSEL12_W {
        SMPSEL12_W { w: self }
    }
    #[doc = "Bit 21 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel13(&mut self) -> SMPSEL13_W {
        SMPSEL13_W { w: self }
    }
    #[doc = "Bit 22 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel14(&mut self) -> SMPSEL14_W {
        SMPSEL14_W { w: self }
    }
    #[doc = "Bit 23 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel15(&mut self) -> SMPSEL15_W {
        SMPSEL15_W { w: self }
    }
    #[doc = "Bit 24 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel16(&mut self) -> SMPSEL16_W {
        SMPSEL16_W { w: self }
    }
    #[doc = "Bit 25 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel17(&mut self) -> SMPSEL17_W {
        SMPSEL17_W { w: self }
    }
    #[doc = "Bit 26 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel18(&mut self) -> SMPSEL18_W {
        SMPSEL18_W { w: self }
    }
}
