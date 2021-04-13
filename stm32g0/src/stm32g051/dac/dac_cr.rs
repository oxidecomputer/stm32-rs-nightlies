#[doc = "Reader of register DAC_CR"]
pub type R = crate::R<u32, super::DAC_CR>;
#[doc = "Writer for register DAC_CR"]
pub type W = crate::W<u32, super::DAC_CR>;
#[doc = "Register DAC_CR `reset()`'s with value 0"]
impl crate::ResetValue for super::DAC_CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DAC channel1 enable This bit is set and cleared by software to enable/disable DAC channel1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN1_A {
    #[doc = "0: DAC channel1 disabled"]
    B_0X0 = 0,
    #[doc = "1: DAC channel1 enabled"]
    B_0X1 = 1,
}
impl From<EN1_A> for bool {
    #[inline(always)]
    fn from(variant: EN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN1`"]
pub type EN1_R = crate::R<bool, EN1_A>;
impl EN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN1_A {
        match self.bits {
            false => EN1_A::B_0X0,
            true => EN1_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EN1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EN1_A::B_0X1
    }
}
#[doc = "Write proxy for field `EN1`"]
pub struct EN1_W<'a> {
    w: &'a mut W,
}
impl<'a> EN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DAC channel1 disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EN1_A::B_0X0)
    }
    #[doc = "DAC channel1 enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EN1_A::B_0X1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "DAC channel1 trigger enable This bit is set and cleared by software to enable/disable DAC channel1 trigger. Note: When software trigger is selected, the transfer from the DAC_DHR1 register to the DAC_DOR1 register takes only one dac_pclk clock cycle.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEN1_A {
    #[doc = "0: DAC channel1 trigger disabled and data written into the DAC_DHR1 register are transferred one dac_pclk clock cycle later to the DAC_DOR1 register"]
    B_0X0 = 0,
    #[doc = "1: DAC channel1 trigger enabled and data from the DAC_DHR1 register are transferred three dac_pclk clock cycles later to the DAC_DOR1 register"]
    B_0X1 = 1,
}
impl From<TEN1_A> for bool {
    #[inline(always)]
    fn from(variant: TEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEN1`"]
pub type TEN1_R = crate::R<bool, TEN1_A>;
impl TEN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEN1_A {
        match self.bits {
            false => TEN1_A::B_0X0,
            true => TEN1_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEN1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEN1_A::B_0X1
    }
}
#[doc = "Write proxy for field `TEN1`"]
pub struct TEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DAC channel1 trigger disabled and data written into the DAC_DHR1 register are transferred one dac_pclk clock cycle later to the DAC_DOR1 register"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TEN1_A::B_0X0)
    }
    #[doc = "DAC channel1 trigger enabled and data from the DAC_DHR1 register are transferred three dac_pclk clock cycles later to the DAC_DOR1 register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TEN1_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "DAC channel1 trigger selection These bits select the external event used to trigger DAC channel1 ... Refer to the trigger selection tables in for details on trigger configuration and mapping. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSEL1_A {
    #[doc = "0: SWTRIG1"]
    B_0X0 = 0,
    #[doc = "1: dac_ch1_trg1"]
    B_0X1 = 1,
    #[doc = "2: dac_ch1_trg2"]
    B_0X2 = 2,
    #[doc = "15: dac_ch1_trg15"]
    B_0XF = 15,
}
impl From<TSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: TSEL1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TSEL1`"]
pub type TSEL1_R = crate::R<u8, TSEL1_A>;
impl TSEL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TSEL1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TSEL1_A::B_0X0),
            1 => Val(TSEL1_A::B_0X1),
            2 => Val(TSEL1_A::B_0X2),
            15 => Val(TSEL1_A::B_0XF),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TSEL1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TSEL1_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == TSEL1_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0XF`"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == TSEL1_A::B_0XF
    }
}
#[doc = "Write proxy for field `TSEL1`"]
pub struct TSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSEL1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SWTRIG1"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TSEL1_A::B_0X0)
    }
    #[doc = "dac_ch1_trg1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TSEL1_A::B_0X1)
    }
    #[doc = "dac_ch1_trg2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(TSEL1_A::B_0X2)
    }
    #[doc = "dac_ch1_trg15"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut W {
        self.variant(TSEL1_A::B_0XF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
#[doc = "DAC channel1 noise/triangle wave generation enable These bits are set and cleared by software. 1x: Triangle wave generation enabled Only used if bit TEN1 = 1 (DAC channel1 trigger enabled).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WAVE1_A {
    #[doc = "0: wave generation disabled"]
    B_0X0 = 0,
    #[doc = "1: Noise wave generation enabled"]
    B_0X1 = 1,
}
impl From<WAVE1_A> for u8 {
    #[inline(always)]
    fn from(variant: WAVE1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WAVE1`"]
pub type WAVE1_R = crate::R<u8, WAVE1_A>;
impl WAVE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WAVE1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WAVE1_A::B_0X0),
            1 => Val(WAVE1_A::B_0X1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WAVE1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WAVE1_A::B_0X1
    }
}
#[doc = "Write proxy for field `WAVE1`"]
pub struct WAVE1_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAVE1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "wave generation disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WAVE1_A::B_0X0)
    }
    #[doc = "Noise wave generation enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(WAVE1_A::B_0X1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "DAC channel1 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. ≥ 1011: Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MAMP1_A {
    #[doc = "0: Unmask bit0 of LFSR/ triangle amplitude equal to 1"]
    B_0X0 = 0,
    #[doc = "1: Unmask bits\\[1:0\\]
of LFSR/ triangle amplitude equal to 3"]
    B_0X1 = 1,
    #[doc = "2: Unmask bits\\[2:0\\]
of LFSR/ triangle amplitude equal to 7"]
    B_0X2 = 2,
    #[doc = "3: Unmask bits\\[3:0\\]
of LFSR/ triangle amplitude equal to 15"]
    B_0X3 = 3,
    #[doc = "4: Unmask bits\\[4:0\\]
of LFSR/ triangle amplitude equal to 31"]
    B_0X4 = 4,
    #[doc = "5: Unmask bits\\[5:0\\]
of LFSR/ triangle amplitude equal to 63"]
    B_0X5 = 5,
    #[doc = "6: Unmask bits\\[6:0\\]
of LFSR/ triangle amplitude equal to 127"]
    B_0X6 = 6,
    #[doc = "7: Unmask bits\\[7:0\\]
of LFSR/ triangle amplitude equal to 255"]
    B_0X7 = 7,
    #[doc = "8: Unmask bits\\[8:0\\]
of LFSR/ triangle amplitude equal to 511"]
    B_0X8 = 8,
    #[doc = "9: Unmask bits\\[9:0\\]
of LFSR/ triangle amplitude equal to 1023"]
    B_0X9 = 9,
    #[doc = "10: Unmask bits\\[10:0\\]
of LFSR/ triangle amplitude equal to 2047"]
    B_0XA = 10,
}
impl From<MAMP1_A> for u8 {
    #[inline(always)]
    fn from(variant: MAMP1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MAMP1`"]
pub type MAMP1_R = crate::R<u8, MAMP1_A>;
impl MAMP1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MAMP1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MAMP1_A::B_0X0),
            1 => Val(MAMP1_A::B_0X1),
            2 => Val(MAMP1_A::B_0X2),
            3 => Val(MAMP1_A::B_0X3),
            4 => Val(MAMP1_A::B_0X4),
            5 => Val(MAMP1_A::B_0X5),
            6 => Val(MAMP1_A::B_0X6),
            7 => Val(MAMP1_A::B_0X7),
            8 => Val(MAMP1_A::B_0X8),
            9 => Val(MAMP1_A::B_0X9),
            10 => Val(MAMP1_A::B_0XA),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MAMP1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MAMP1_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MAMP1_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MAMP1_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == MAMP1_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == MAMP1_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == MAMP1_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == MAMP1_A::B_0X7
    }
    #[doc = "Checks if the value of the field is `B_0X8`"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == MAMP1_A::B_0X8
    }
    #[doc = "Checks if the value of the field is `B_0X9`"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == MAMP1_A::B_0X9
    }
    #[doc = "Checks if the value of the field is `B_0XA`"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == MAMP1_A::B_0XA
    }
}
#[doc = "Write proxy for field `MAMP1`"]
pub struct MAMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> MAMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAMP1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Unmask bit0 of LFSR/ triangle amplitude equal to 1"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MAMP1_A::B_0X0)
    }
    #[doc = "Unmask bits\\[1:0\\]
of LFSR/ triangle amplitude equal to 3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MAMP1_A::B_0X1)
    }
    #[doc = "Unmask bits\\[2:0\\]
of LFSR/ triangle amplitude equal to 7"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(MAMP1_A::B_0X2)
    }
    #[doc = "Unmask bits\\[3:0\\]
of LFSR/ triangle amplitude equal to 15"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(MAMP1_A::B_0X3)
    }
    #[doc = "Unmask bits\\[4:0\\]
of LFSR/ triangle amplitude equal to 31"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(MAMP1_A::B_0X4)
    }
    #[doc = "Unmask bits\\[5:0\\]
of LFSR/ triangle amplitude equal to 63"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(MAMP1_A::B_0X5)
    }
    #[doc = "Unmask bits\\[6:0\\]
of LFSR/ triangle amplitude equal to 127"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(MAMP1_A::B_0X6)
    }
    #[doc = "Unmask bits\\[7:0\\]
of LFSR/ triangle amplitude equal to 255"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(MAMP1_A::B_0X7)
    }
    #[doc = "Unmask bits\\[8:0\\]
of LFSR/ triangle amplitude equal to 511"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut W {
        self.variant(MAMP1_A::B_0X8)
    }
    #[doc = "Unmask bits\\[9:0\\]
of LFSR/ triangle amplitude equal to 1023"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut W {
        self.variant(MAMP1_A::B_0X9)
    }
    #[doc = "Unmask bits\\[10:0\\]
of LFSR/ triangle amplitude equal to 2047"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut W {
        self.variant(MAMP1_A::B_0XA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "DAC channel1 DMA enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN1_A {
    #[doc = "0: DAC channel1 DMA mode disabled"]
    B_0X0 = 0,
    #[doc = "1: DAC channel1 DMA mode enabled"]
    B_0X1 = 1,
}
impl From<DMAEN1_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAEN1`"]
pub type DMAEN1_R = crate::R<bool, DMAEN1_A>;
impl DMAEN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN1_A {
        match self.bits {
            false => DMAEN1_A::B_0X0,
            true => DMAEN1_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DMAEN1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DMAEN1_A::B_0X1
    }
}
#[doc = "Write proxy for field `DMAEN1`"]
pub struct DMAEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAEN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DAC channel1 DMA mode disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DMAEN1_A::B_0X0)
    }
    #[doc = "DAC channel1 DMA mode enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DMAEN1_A::B_0X1)
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
#[doc = "DAC channel1 DMA Underrun Interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAUDRIE1_A {
    #[doc = "0: DAC channel1 DMA Underrun Interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: DAC channel1 DMA Underrun Interrupt enabled"]
    B_0X1 = 1,
}
impl From<DMAUDRIE1_A> for bool {
    #[inline(always)]
    fn from(variant: DMAUDRIE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAUDRIE1`"]
pub type DMAUDRIE1_R = crate::R<bool, DMAUDRIE1_A>;
impl DMAUDRIE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAUDRIE1_A {
        match self.bits {
            false => DMAUDRIE1_A::B_0X0,
            true => DMAUDRIE1_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DMAUDRIE1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DMAUDRIE1_A::B_0X1
    }
}
#[doc = "Write proxy for field `DMAUDRIE1`"]
pub struct DMAUDRIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAUDRIE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAUDRIE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DAC channel1 DMA Underrun Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DMAUDRIE1_A::B_0X0)
    }
    #[doc = "DAC channel1 DMA Underrun Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DMAUDRIE1_A::B_0X1)
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
#[doc = "DAC channel1 calibration enable This bit is set and cleared by software to enable/disable DAC channel1 calibration, it can be written only if bit EN1=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEN1_A {
    #[doc = "0: DAC channel1 in Normal operating mode"]
    B_0X0 = 0,
    #[doc = "1: DAC channel1 in calibration mode"]
    B_0X1 = 1,
}
impl From<CEN1_A> for bool {
    #[inline(always)]
    fn from(variant: CEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CEN1`"]
pub type CEN1_R = crate::R<bool, CEN1_A>;
impl CEN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEN1_A {
        match self.bits {
            false => CEN1_A::B_0X0,
            true => CEN1_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CEN1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CEN1_A::B_0X1
    }
}
#[doc = "Write proxy for field `CEN1`"]
pub struct CEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DAC channel1 in Normal operating mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CEN1_A::B_0X0)
    }
    #[doc = "DAC channel1 in calibration mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CEN1_A::B_0X1)
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
#[doc = "DAC channel2 enable This bit is set and cleared by software to enable/disable DAC channel2. Note: These bits are available only on dual-channel DACs. Refer to implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN2_A {
    #[doc = "0: DAC channel2 disabled"]
    B_0X0 = 0,
    #[doc = "1: DAC channel2 enabled"]
    B_0X1 = 1,
}
impl From<EN2_A> for bool {
    #[inline(always)]
    fn from(variant: EN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN2`"]
pub type EN2_R = crate::R<bool, EN2_A>;
impl EN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN2_A {
        match self.bits {
            false => EN2_A::B_0X0,
            true => EN2_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EN2_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EN2_A::B_0X1
    }
}
#[doc = "Write proxy for field `EN2`"]
pub struct EN2_W<'a> {
    w: &'a mut W,
}
impl<'a> EN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DAC channel2 disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EN2_A::B_0X0)
    }
    #[doc = "DAC channel2 enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EN2_A::B_0X1)
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
#[doc = "DAC channel2 trigger enable This bit is set and cleared by software to enable/disable DAC channel2 trigger Note: When software trigger is selected, the transfer from the DAC_DHR2 register to the DAC_DOR2 register takes only one dac_pclk clock cycle. These bits are available only on dual-channel DACs. Refer to implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEN2_A {
    #[doc = "0: DAC channel2 trigger disabled and data written into the DAC_DHR2 register are transferred one dac_pclk clock cycle later to the DAC_DOR2 register"]
    B_0X0 = 0,
    #[doc = "1: DAC channel2 trigger enabled and data from the DAC_DHR2 register are transferred three dac_pclk clock cycles later to the DAC_DOR2 register"]
    B_0X1 = 1,
}
impl From<TEN2_A> for bool {
    #[inline(always)]
    fn from(variant: TEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEN2`"]
pub type TEN2_R = crate::R<bool, TEN2_A>;
impl TEN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEN2_A {
        match self.bits {
            false => TEN2_A::B_0X0,
            true => TEN2_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEN2_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEN2_A::B_0X1
    }
}
#[doc = "Write proxy for field `TEN2`"]
pub struct TEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DAC channel2 trigger disabled and data written into the DAC_DHR2 register are transferred one dac_pclk clock cycle later to the DAC_DOR2 register"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TEN2_A::B_0X0)
    }
    #[doc = "DAC channel2 trigger enabled and data from the DAC_DHR2 register are transferred three dac_pclk clock cycles later to the DAC_DOR2 register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TEN2_A::B_0X1)
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
#[doc = "DAC channel2 trigger selection These bits select the external event used to trigger DAC channel2 ... Refer to the trigger selection tables in for details on trigger configuration and mapping. Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled). These bits are available only on dual-channel DACs. Refer to implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSEL2_A {
    #[doc = "0: SWTRIG2"]
    B_0X0 = 0,
    #[doc = "1: dac_ch2_trg1"]
    B_0X1 = 1,
    #[doc = "2: dac_ch2_trg2"]
    B_0X2 = 2,
    #[doc = "15: dac_ch2_trg15"]
    B_0XF = 15,
}
impl From<TSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: TSEL2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TSEL2`"]
pub type TSEL2_R = crate::R<u8, TSEL2_A>;
impl TSEL2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TSEL2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TSEL2_A::B_0X0),
            1 => Val(TSEL2_A::B_0X1),
            2 => Val(TSEL2_A::B_0X2),
            15 => Val(TSEL2_A::B_0XF),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TSEL2_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TSEL2_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == TSEL2_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0XF`"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == TSEL2_A::B_0XF
    }
}
#[doc = "Write proxy for field `TSEL2`"]
pub struct TSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSEL2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SWTRIG2"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TSEL2_A::B_0X0)
    }
    #[doc = "dac_ch2_trg1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TSEL2_A::B_0X1)
    }
    #[doc = "dac_ch2_trg2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(TSEL2_A::B_0X2)
    }
    #[doc = "dac_ch2_trg15"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut W {
        self.variant(TSEL2_A::B_0XF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | (((value as u32) & 0x0f) << 18);
        self.w
    }
}
#[doc = "DAC channel2 noise/triangle wave generation enable These bits are set/reset by software. 1x: Triangle wave generation enabled Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled) These bits are available only on dual-channel DACs. Refer to implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WAVE2_A {
    #[doc = "0: wave generation disabled "]
    B_0X0 = 0,
    #[doc = "1: Noise wave generation enabled "]
    B_0X1 = 1,
}
impl From<WAVE2_A> for u8 {
    #[inline(always)]
    fn from(variant: WAVE2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WAVE2`"]
pub type WAVE2_R = crate::R<u8, WAVE2_A>;
impl WAVE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WAVE2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WAVE2_A::B_0X0),
            1 => Val(WAVE2_A::B_0X1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WAVE2_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WAVE2_A::B_0X1
    }
}
#[doc = "Write proxy for field `WAVE2`"]
pub struct WAVE2_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAVE2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "wave generation disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WAVE2_A::B_0X0)
    }
    #[doc = "Noise wave generation enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(WAVE2_A::B_0X1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "DAC channel2 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. ≥ 1011: Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095 Note: These bits are available only on dual-channel DACs. Refer to implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MAMP2_A {
    #[doc = "0: Unmask bit0 of LFSR/ triangle amplitude equal to 1"]
    B_0X0 = 0,
    #[doc = "1: Unmask bits\\[1:0\\]
of LFSR/ triangle amplitude equal to 3"]
    B_0X1 = 1,
    #[doc = "2: Unmask bits\\[2:0\\]
of LFSR/ triangle amplitude equal to 7"]
    B_0X2 = 2,
    #[doc = "3: Unmask bits\\[3:0\\]
of LFSR/ triangle amplitude equal to 15"]
    B_0X3 = 3,
    #[doc = "4: Unmask bits\\[4:0\\]
of LFSR/ triangle amplitude equal to 31"]
    B_0X4 = 4,
    #[doc = "5: Unmask bits\\[5:0\\]
of LFSR/ triangle amplitude equal to 63"]
    B_0X5 = 5,
    #[doc = "6: Unmask bits\\[6:0\\]
of LFSR/ triangle amplitude equal to 127"]
    B_0X6 = 6,
    #[doc = "7: Unmask bits\\[7:0\\]
of LFSR/ triangle amplitude equal to 255"]
    B_0X7 = 7,
    #[doc = "8: Unmask bits\\[8:0\\]
of LFSR/ triangle amplitude equal to 511"]
    B_0X8 = 8,
    #[doc = "9: Unmask bits\\[9:0\\]
of LFSR/ triangle amplitude equal to 1023"]
    B_0X9 = 9,
    #[doc = "10: Unmask bits\\[10:0\\]
of LFSR/ triangle amplitude equal to 2047"]
    B_0XA = 10,
}
impl From<MAMP2_A> for u8 {
    #[inline(always)]
    fn from(variant: MAMP2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MAMP2`"]
pub type MAMP2_R = crate::R<u8, MAMP2_A>;
impl MAMP2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MAMP2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MAMP2_A::B_0X0),
            1 => Val(MAMP2_A::B_0X1),
            2 => Val(MAMP2_A::B_0X2),
            3 => Val(MAMP2_A::B_0X3),
            4 => Val(MAMP2_A::B_0X4),
            5 => Val(MAMP2_A::B_0X5),
            6 => Val(MAMP2_A::B_0X6),
            7 => Val(MAMP2_A::B_0X7),
            8 => Val(MAMP2_A::B_0X8),
            9 => Val(MAMP2_A::B_0X9),
            10 => Val(MAMP2_A::B_0XA),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MAMP2_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MAMP2_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MAMP2_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MAMP2_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == MAMP2_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == MAMP2_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == MAMP2_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == MAMP2_A::B_0X7
    }
    #[doc = "Checks if the value of the field is `B_0X8`"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == MAMP2_A::B_0X8
    }
    #[doc = "Checks if the value of the field is `B_0X9`"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == MAMP2_A::B_0X9
    }
    #[doc = "Checks if the value of the field is `B_0XA`"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == MAMP2_A::B_0XA
    }
}
#[doc = "Write proxy for field `MAMP2`"]
pub struct MAMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> MAMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAMP2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Unmask bit0 of LFSR/ triangle amplitude equal to 1"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MAMP2_A::B_0X0)
    }
    #[doc = "Unmask bits\\[1:0\\]
of LFSR/ triangle amplitude equal to 3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MAMP2_A::B_0X1)
    }
    #[doc = "Unmask bits\\[2:0\\]
of LFSR/ triangle amplitude equal to 7"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(MAMP2_A::B_0X2)
    }
    #[doc = "Unmask bits\\[3:0\\]
of LFSR/ triangle amplitude equal to 15"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(MAMP2_A::B_0X3)
    }
    #[doc = "Unmask bits\\[4:0\\]
of LFSR/ triangle amplitude equal to 31"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(MAMP2_A::B_0X4)
    }
    #[doc = "Unmask bits\\[5:0\\]
of LFSR/ triangle amplitude equal to 63"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(MAMP2_A::B_0X5)
    }
    #[doc = "Unmask bits\\[6:0\\]
of LFSR/ triangle amplitude equal to 127"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(MAMP2_A::B_0X6)
    }
    #[doc = "Unmask bits\\[7:0\\]
of LFSR/ triangle amplitude equal to 255"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(MAMP2_A::B_0X7)
    }
    #[doc = "Unmask bits\\[8:0\\]
of LFSR/ triangle amplitude equal to 511"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut W {
        self.variant(MAMP2_A::B_0X8)
    }
    #[doc = "Unmask bits\\[9:0\\]
of LFSR/ triangle amplitude equal to 1023"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut W {
        self.variant(MAMP2_A::B_0X9)
    }
    #[doc = "Unmask bits\\[10:0\\]
of LFSR/ triangle amplitude equal to 2047"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut W {
        self.variant(MAMP2_A::B_0XA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "DAC channel2 DMA enable This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN2_A {
    #[doc = "0: DAC channel2 DMA mode disabled"]
    B_0X0 = 0,
    #[doc = "1: DAC channel2 DMA mode enabled"]
    B_0X1 = 1,
}
impl From<DMAEN2_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAEN2`"]
pub type DMAEN2_R = crate::R<bool, DMAEN2_A>;
impl DMAEN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN2_A {
        match self.bits {
            false => DMAEN2_A::B_0X0,
            true => DMAEN2_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DMAEN2_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DMAEN2_A::B_0X1
    }
}
#[doc = "Write proxy for field `DMAEN2`"]
pub struct DMAEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAEN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DAC channel2 DMA mode disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DMAEN2_A::B_0X0)
    }
    #[doc = "DAC channel2 DMA mode enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DMAEN2_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "DAC channel2 DMA underrun interrupt enable This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAUDRIE2_A {
    #[doc = "0: DAC channel2 DMA underrun interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: DAC channel2 DMA underrun interrupt enabled"]
    B_0X1 = 1,
}
impl From<DMAUDRIE2_A> for bool {
    #[inline(always)]
    fn from(variant: DMAUDRIE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAUDRIE2`"]
pub type DMAUDRIE2_R = crate::R<bool, DMAUDRIE2_A>;
impl DMAUDRIE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAUDRIE2_A {
        match self.bits {
            false => DMAUDRIE2_A::B_0X0,
            true => DMAUDRIE2_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DMAUDRIE2_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DMAUDRIE2_A::B_0X1
    }
}
#[doc = "Write proxy for field `DMAUDRIE2`"]
pub struct DMAUDRIE2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAUDRIE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAUDRIE2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DAC channel2 DMA underrun interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DMAUDRIE2_A::B_0X0)
    }
    #[doc = "DAC channel2 DMA underrun interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DMAUDRIE2_A::B_0X1)
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
#[doc = "DAC channel2 calibration enable This bit is set and cleared by software to enable/disable DAC channel2 calibration, it can be written only if EN2 bit is set to 0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored. Note: This bit is available only on dual-channel DACs. Refer to implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEN2_A {
    #[doc = "0: DAC channel2 in Normal operating mode"]
    B_0X0 = 0,
    #[doc = "1: DAC channel2 in calibration mode"]
    B_0X1 = 1,
}
impl From<CEN2_A> for bool {
    #[inline(always)]
    fn from(variant: CEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CEN2`"]
pub type CEN2_R = crate::R<bool, CEN2_A>;
impl CEN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEN2_A {
        match self.bits {
            false => CEN2_A::B_0X0,
            true => CEN2_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CEN2_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CEN2_A::B_0X1
    }
}
#[doc = "Write proxy for field `CEN2`"]
pub struct CEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> CEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DAC channel2 in Normal operating mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CEN2_A::B_0X0)
    }
    #[doc = "DAC channel2 in calibration mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CEN2_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DAC channel1 enable This bit is set and cleared by software to enable/disable DAC channel1."]
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DAC channel1 trigger enable This bit is set and cleared by software to enable/disable DAC channel1 trigger. Note: When software trigger is selected, the transfer from the DAC_DHR1 register to the DAC_DOR1 register takes only one dac_pclk clock cycle."]
    #[inline(always)]
    pub fn ten1(&self) -> TEN1_R {
        TEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:5 - DAC channel1 trigger selection These bits select the external event used to trigger DAC channel1 ... Refer to the trigger selection tables in for details on trigger configuration and mapping. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled)."]
    #[inline(always)]
    pub fn tsel1(&self) -> TSEL1_R {
        TSEL1_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - DAC channel1 noise/triangle wave generation enable These bits are set and cleared by software. 1x: Triangle wave generation enabled Only used if bit TEN1 = 1 (DAC channel1 trigger enabled)."]
    #[inline(always)]
    pub fn wave1(&self) -> WAVE1_R {
        WAVE1_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - DAC channel1 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. ≥ 1011: Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095"]
    #[inline(always)]
    pub fn mamp1(&self) -> MAMP1_R {
        MAMP1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DAC channel1 DMA enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn dmaen1(&self) -> DMAEN1_R {
        DMAEN1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DAC channel1 DMA Underrun Interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn dmaudrie1(&self) -> DMAUDRIE1_R {
        DMAUDRIE1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - DAC channel1 calibration enable This bit is set and cleared by software to enable/disable DAC channel1 calibration, it can be written only if bit EN1=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored."]
    #[inline(always)]
    pub fn cen1(&self) -> CEN1_R {
        CEN1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DAC channel2 enable This bit is set and cleared by software to enable/disable DAC channel2. Note: These bits are available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn en2(&self) -> EN2_R {
        EN2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DAC channel2 trigger enable This bit is set and cleared by software to enable/disable DAC channel2 trigger Note: When software trigger is selected, the transfer from the DAC_DHR2 register to the DAC_DOR2 register takes only one dac_pclk clock cycle. These bits are available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn ten2(&self) -> TEN2_R {
        TEN2_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:21 - DAC channel2 trigger selection These bits select the external event used to trigger DAC channel2 ... Refer to the trigger selection tables in for details on trigger configuration and mapping. Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled). These bits are available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn tsel2(&self) -> TSEL2_R {
        TSEL2_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:23 - DAC channel2 noise/triangle wave generation enable These bits are set/reset by software. 1x: Triangle wave generation enabled Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled) These bits are available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn wave2(&self) -> WAVE2_R {
        WAVE2_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:27 - DAC channel2 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. ≥ 1011: Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095 Note: These bits are available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn mamp2(&self) -> MAMP2_R {
        MAMP2_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - DAC channel2 DMA enable This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn dmaen2(&self) -> DMAEN2_R {
        DMAEN2_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DAC channel2 DMA underrun interrupt enable This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn dmaudrie2(&self) -> DMAUDRIE2_R {
        DMAUDRIE2_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DAC channel2 calibration enable This bit is set and cleared by software to enable/disable DAC channel2 calibration, it can be written only if EN2 bit is set to 0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn cen2(&self) -> CEN2_R {
        CEN2_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC channel1 enable This bit is set and cleared by software to enable/disable DAC channel1."]
    #[inline(always)]
    pub fn en1(&mut self) -> EN1_W {
        EN1_W { w: self }
    }
    #[doc = "Bit 1 - DAC channel1 trigger enable This bit is set and cleared by software to enable/disable DAC channel1 trigger. Note: When software trigger is selected, the transfer from the DAC_DHR1 register to the DAC_DOR1 register takes only one dac_pclk clock cycle."]
    #[inline(always)]
    pub fn ten1(&mut self) -> TEN1_W {
        TEN1_W { w: self }
    }
    #[doc = "Bits 2:5 - DAC channel1 trigger selection These bits select the external event used to trigger DAC channel1 ... Refer to the trigger selection tables in for details on trigger configuration and mapping. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled)."]
    #[inline(always)]
    pub fn tsel1(&mut self) -> TSEL1_W {
        TSEL1_W { w: self }
    }
    #[doc = "Bits 6:7 - DAC channel1 noise/triangle wave generation enable These bits are set and cleared by software. 1x: Triangle wave generation enabled Only used if bit TEN1 = 1 (DAC channel1 trigger enabled)."]
    #[inline(always)]
    pub fn wave1(&mut self) -> WAVE1_W {
        WAVE1_W { w: self }
    }
    #[doc = "Bits 8:11 - DAC channel1 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. ≥ 1011: Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095"]
    #[inline(always)]
    pub fn mamp1(&mut self) -> MAMP1_W {
        MAMP1_W { w: self }
    }
    #[doc = "Bit 12 - DAC channel1 DMA enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn dmaen1(&mut self) -> DMAEN1_W {
        DMAEN1_W { w: self }
    }
    #[doc = "Bit 13 - DAC channel1 DMA Underrun Interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn dmaudrie1(&mut self) -> DMAUDRIE1_W {
        DMAUDRIE1_W { w: self }
    }
    #[doc = "Bit 14 - DAC channel1 calibration enable This bit is set and cleared by software to enable/disable DAC channel1 calibration, it can be written only if bit EN1=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored."]
    #[inline(always)]
    pub fn cen1(&mut self) -> CEN1_W {
        CEN1_W { w: self }
    }
    #[doc = "Bit 16 - DAC channel2 enable This bit is set and cleared by software to enable/disable DAC channel2. Note: These bits are available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn en2(&mut self) -> EN2_W {
        EN2_W { w: self }
    }
    #[doc = "Bit 17 - DAC channel2 trigger enable This bit is set and cleared by software to enable/disable DAC channel2 trigger Note: When software trigger is selected, the transfer from the DAC_DHR2 register to the DAC_DOR2 register takes only one dac_pclk clock cycle. These bits are available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn ten2(&mut self) -> TEN2_W {
        TEN2_W { w: self }
    }
    #[doc = "Bits 18:21 - DAC channel2 trigger selection These bits select the external event used to trigger DAC channel2 ... Refer to the trigger selection tables in for details on trigger configuration and mapping. Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled). These bits are available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn tsel2(&mut self) -> TSEL2_W {
        TSEL2_W { w: self }
    }
    #[doc = "Bits 22:23 - DAC channel2 noise/triangle wave generation enable These bits are set/reset by software. 1x: Triangle wave generation enabled Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled) These bits are available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn wave2(&mut self) -> WAVE2_W {
        WAVE2_W { w: self }
    }
    #[doc = "Bits 24:27 - DAC channel2 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. ≥ 1011: Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095 Note: These bits are available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn mamp2(&mut self) -> MAMP2_W {
        MAMP2_W { w: self }
    }
    #[doc = "Bit 28 - DAC channel2 DMA enable This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn dmaen2(&mut self) -> DMAEN2_W {
        DMAEN2_W { w: self }
    }
    #[doc = "Bit 29 - DAC channel2 DMA underrun interrupt enable This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn dmaudrie2(&mut self) -> DMAUDRIE2_W {
        DMAUDRIE2_W { w: self }
    }
    #[doc = "Bit 30 - DAC channel2 calibration enable This bit is set and cleared by software to enable/disable DAC channel2 calibration, it can be written only if EN2 bit is set to 0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn cen2(&mut self) -> CEN2_W {
        CEN2_W { w: self }
    }
}
