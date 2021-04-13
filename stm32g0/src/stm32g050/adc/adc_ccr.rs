#[doc = "Reader of register ADC_CCR"]
pub type R = crate::R<u32, super::ADC_CCR>;
#[doc = "Writer for register ADC_CCR"]
pub type W = crate::W<u32, super::ADC_CCR>;
#[doc = "Register ADC_CCR `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_CCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ADC prescaler Set and cleared by software to select the frequency of the clock to the ADC. Other: Reserved Note: Software is allowed to write these bits only when the ADC is disabled (ADCALÂ =Â 0, ADSTARTÂ =Â 0, ADSTPÂ =Â 0, ADDISÂ =Â 0 and ADENÂ =Â 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESC_A {
    #[doc = "0: input ADC clock not divided"]
    B_0X0 = 0,
    #[doc = "1: input ADC clock divided by 2"]
    B_0X1 = 1,
    #[doc = "2: input ADC clock divided by 4"]
    B_0X2 = 2,
    #[doc = "3: input ADC clock divided by 6"]
    B_0X3 = 3,
    #[doc = "4: input ADC clock divided by 8"]
    B_0X4 = 4,
    #[doc = "5: input ADC clock divided by 10"]
    B_0X5 = 5,
    #[doc = "6: input ADC clock divided by 12"]
    B_0X6 = 6,
    #[doc = "7: input ADC clock divided by 16"]
    B_0X7 = 7,
    #[doc = "8: input ADC clock divided by 32"]
    B_0X8 = 8,
    #[doc = "9: input ADC clock divided by 64"]
    B_0X9 = 9,
    #[doc = "10: input ADC clock divided by 128"]
    B_0XA = 10,
    #[doc = "11: input ADC clock divided by 256"]
    B_0XB = 11,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRESC`"]
pub type PRESC_R = crate::R<u8, PRESC_A>;
impl PRESC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRESC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRESC_A::B_0X0),
            1 => Val(PRESC_A::B_0X1),
            2 => Val(PRESC_A::B_0X2),
            3 => Val(PRESC_A::B_0X3),
            4 => Val(PRESC_A::B_0X4),
            5 => Val(PRESC_A::B_0X5),
            6 => Val(PRESC_A::B_0X6),
            7 => Val(PRESC_A::B_0X7),
            8 => Val(PRESC_A::B_0X8),
            9 => Val(PRESC_A::B_0X9),
            10 => Val(PRESC_A::B_0XA),
            11 => Val(PRESC_A::B_0XB),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PRESC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PRESC_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PRESC_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == PRESC_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == PRESC_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == PRESC_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == PRESC_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == PRESC_A::B_0X7
    }
    #[doc = "Checks if the value of the field is `B_0X8`"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == PRESC_A::B_0X8
    }
    #[doc = "Checks if the value of the field is `B_0X9`"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == PRESC_A::B_0X9
    }
    #[doc = "Checks if the value of the field is `B_0XA`"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == PRESC_A::B_0XA
    }
    #[doc = "Checks if the value of the field is `B_0XB`"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == PRESC_A::B_0XB
    }
}
#[doc = "Write proxy for field `PRESC`"]
pub struct PRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "input ADC clock not divided"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PRESC_A::B_0X0)
    }
    #[doc = "input ADC clock divided by 2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PRESC_A::B_0X1)
    }
    #[doc = "input ADC clock divided by 4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(PRESC_A::B_0X2)
    }
    #[doc = "input ADC clock divided by 6"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(PRESC_A::B_0X3)
    }
    #[doc = "input ADC clock divided by 8"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(PRESC_A::B_0X4)
    }
    #[doc = "input ADC clock divided by 10"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(PRESC_A::B_0X5)
    }
    #[doc = "input ADC clock divided by 12"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(PRESC_A::B_0X6)
    }
    #[doc = "input ADC clock divided by 16"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(PRESC_A::B_0X7)
    }
    #[doc = "input ADC clock divided by 32"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut W {
        self.variant(PRESC_A::B_0X8)
    }
    #[doc = "input ADC clock divided by 64"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut W {
        self.variant(PRESC_A::B_0X9)
    }
    #[doc = "input ADC clock divided by 128"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut W {
        self.variant(PRESC_A::B_0XA)
    }
    #[doc = "input ADC clock divided by 256"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut W {
        self.variant(PRESC_A::B_0XB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | (((value as u32) & 0x0f) << 18);
        self.w
    }
}
#[doc = "VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT. Note: Software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREFEN_A {
    #[doc = "0: VREFINT disabled"]
    B_0X0 = 0,
    #[doc = "1: VREFINT enabled"]
    B_0X1 = 1,
}
impl From<VREFEN_A> for bool {
    #[inline(always)]
    fn from(variant: VREFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VREFEN`"]
pub type VREFEN_R = crate::R<bool, VREFEN_A>;
impl VREFEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREFEN_A {
        match self.bits {
            false => VREFEN_A::B_0X0,
            true => VREFEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == VREFEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == VREFEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `VREFEN`"]
pub struct VREFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREFEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "VREFINT disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(VREFEN_A::B_0X0)
    }
    #[doc = "VREFINT enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(VREFEN_A::B_0X1)
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
#[doc = "Temperature sensor enable This bit is set and cleared by software to enable/disable the temperature sensor. Note: Software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSEN_A {
    #[doc = "0: Temperature sensor disabled, DAC_OUT1 connected to ADC channel 12"]
    B_0X0 = 0,
    #[doc = "1: Temperature sensor enabled"]
    B_0X1 = 1,
}
impl From<TSEN_A> for bool {
    #[inline(always)]
    fn from(variant: TSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TSEN`"]
pub type TSEN_R = crate::R<bool, TSEN_A>;
impl TSEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSEN_A {
        match self.bits {
            false => TSEN_A::B_0X0,
            true => TSEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TSEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TSEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `TSEN`"]
pub struct TSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Temperature sensor disabled, DAC_OUT1 connected to ADC channel 12"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TSEN_A::B_0X0)
    }
    #[doc = "Temperature sensor enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TSEN_A::B_0X1)
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
#[doc = "VBAT enable This bit is set and cleared by software to enable/disable the VBAT channel. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBATEN_A {
    #[doc = "0: VBAT channel disabled, DAC_OUT2 connected to ADC channel 14"]
    B_0X0 = 0,
    #[doc = "1: VBAT channel enabled"]
    B_0X1 = 1,
}
impl From<VBATEN_A> for bool {
    #[inline(always)]
    fn from(variant: VBATEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VBATEN`"]
pub type VBATEN_R = crate::R<bool, VBATEN_A>;
impl VBATEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBATEN_A {
        match self.bits {
            false => VBATEN_A::B_0X0,
            true => VBATEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == VBATEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == VBATEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `VBATEN`"]
pub struct VBATEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VBATEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBATEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "VBAT channel disabled, DAC_OUT2 connected to ADC channel 14"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(VBATEN_A::B_0X0)
    }
    #[doc = "VBAT channel enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(VBATEN_A::B_0X1)
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
impl R {
    #[doc = "Bits 18:21 - ADC prescaler Set and cleared by software to select the frequency of the clock to the ADC. Other: Reserved Note: Software is allowed to write these bits only when the ADC is disabled (ADCALÂ =Â 0, ADSTARTÂ =Â 0, ADSTPÂ =Â 0, ADDISÂ =Â 0 and ADENÂ =Â 0)."]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT. Note: Software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Temperature sensor enable This bit is set and cleared by software to enable/disable the temperature sensor. Note: Software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - VBAT enable This bit is set and cleared by software to enable/disable the VBAT channel. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)"]
    #[inline(always)]
    pub fn vbaten(&self) -> VBATEN_R {
        VBATEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 18:21 - ADC prescaler Set and cleared by software to select the frequency of the clock to the ADC. Other: Reserved Note: Software is allowed to write these bits only when the ADC is disabled (ADCALÂ =Â 0, ADSTARTÂ =Â 0, ADSTPÂ =Â 0, ADDISÂ =Â 0 and ADENÂ =Â 0)."]
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W {
        PRESC_W { w: self }
    }
    #[doc = "Bit 22 - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT. Note: Software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn vrefen(&mut self) -> VREFEN_W {
        VREFEN_W { w: self }
    }
    #[doc = "Bit 23 - Temperature sensor enable This bit is set and cleared by software to enable/disable the temperature sensor. Note: Software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn tsen(&mut self) -> TSEN_W {
        TSEN_W { w: self }
    }
    #[doc = "Bit 24 - VBAT enable This bit is set and cleared by software to enable/disable the VBAT channel. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)"]
    #[inline(always)]
    pub fn vbaten(&mut self) -> VBATEN_W {
        VBATEN_W { w: self }
    }
}
