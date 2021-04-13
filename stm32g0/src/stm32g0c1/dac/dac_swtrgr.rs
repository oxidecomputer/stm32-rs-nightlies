#[doc = "Writer for register DAC_SWTRGR"]
pub type W = crate::W<u32, super::DAC_SWTRGR>;
#[doc = "Register DAC_SWTRGR `reset()`'s with value 0"]
impl crate::ResetValue for super::DAC_SWTRGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DAC channel1 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one dac_pclk clock cycle later) once the DAC_DHR1 register value has been loaded into the DAC_DOR1 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWTRIG1_AW {
    #[doc = "0: No trigger"]
    B_0X0 = 0,
    #[doc = "1: Trigger"]
    B_0X1 = 1,
}
impl From<SWTRIG1_AW> for bool {
    #[inline(always)]
    fn from(variant: SWTRIG1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SWTRIG1`"]
pub struct SWTRIG1_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWTRIG1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No trigger"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SWTRIG1_AW::B_0X0)
    }
    #[doc = "Trigger"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SWTRIG1_AW::B_0X1)
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
#[doc = "DAC channel2 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one dac_pclk clock cycle later) once the DAC_DHR2 register value has been loaded into the DAC_DOR2 register. This bit is available only on dual-channel DACs. Refer to implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWTRIG2_AW {
    #[doc = "0: No trigger"]
    B_0X0 = 0,
    #[doc = "1: Trigger"]
    B_0X1 = 1,
}
impl From<SWTRIG2_AW> for bool {
    #[inline(always)]
    fn from(variant: SWTRIG2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SWTRIG2`"]
pub struct SWTRIG2_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWTRIG2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No trigger"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SWTRIG2_AW::B_0X0)
    }
    #[doc = "Trigger"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SWTRIG2_AW::B_0X1)
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
impl W {
    #[doc = "Bit 0 - DAC channel1 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one dac_pclk clock cycle later) once the DAC_DHR1 register value has been loaded into the DAC_DOR1 register."]
    #[inline(always)]
    pub fn swtrig1(&mut self) -> SWTRIG1_W {
        SWTRIG1_W { w: self }
    }
    #[doc = "Bit 1 - DAC channel2 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one dac_pclk clock cycle later) once the DAC_DHR2 register value has been loaded into the DAC_DOR2 register. This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn swtrig2(&mut self) -> SWTRIG2_W {
        SWTRIG2_W { w: self }
    }
}
