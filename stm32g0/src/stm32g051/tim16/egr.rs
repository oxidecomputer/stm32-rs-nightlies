#[doc = "Writer for register EGR"]
pub type W = crate::W<u32, super::EGR>;
#[doc = "Register EGR `reset()`'s with value 0"]
impl crate::ResetValue for super::EGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Update generation This bit can be set by software, it is automatically cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UG_AW {
    #[doc = "0: No action."]
    B_0X0 = 0,
    #[doc = "1: Reinitialize the counter and generates an update of the registers. Note that the prescaler counter is cleared too (anyway the prescaler ratio is not affected). "]
    B_0X1 = 1,
}
impl From<UG_AW> for bool {
    #[inline(always)]
    fn from(variant: UG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `UG`"]
pub struct UG_W<'a> {
    w: &'a mut W,
}
impl<'a> UG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UG_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(UG_AW::B_0X0)
    }
    #[doc = "Reinitialize the counter and generates an update of the registers. Note that the prescaler counter is cleared too (anyway the prescaler ratio is not affected)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(UG_AW::B_0X1)
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
#[doc = "Capture/Compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in TIMx_CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1G_AW {
    #[doc = "0: No action."]
    B_0X0 = 0,
    #[doc = "1: A capture/compare event is generated on channel 1:"]
    B_0X1 = 1,
}
impl From<CC1G_AW> for bool {
    #[inline(always)]
    fn from(variant: CC1G_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CC1G`"]
pub struct CC1G_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1G_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC1G_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CC1G_AW::B_0X0)
    }
    #[doc = "A capture/compare event is generated on channel 1:"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CC1G_AW::B_0X1)
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
#[doc = "Capture/Compare control update generation This bit can be set by software, it is automatically cleared by hardware. Note: This bit acts only on channels that have a complementary output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMG_AW {
    #[doc = "0: No action"]
    B_0X0 = 0,
    #[doc = "1: When the CCPC bit is set, it is possible to update the CCxE, CCxNE and OCxM bits"]
    B_0X1 = 1,
}
impl From<COMG_AW> for bool {
    #[inline(always)]
    fn from(variant: COMG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `COMG`"]
pub struct COMG_W<'a> {
    w: &'a mut W,
}
impl<'a> COMG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMG_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(COMG_AW::B_0X0)
    }
    #[doc = "When the CCPC bit is set, it is possible to update the CCxE, CCxNE and OCxM bits"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(COMG_AW::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BG_AW {
    #[doc = "0: No action."]
    B_0X0 = 0,
    #[doc = "1: A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled."]
    B_0X1 = 1,
}
impl From<BG_AW> for bool {
    #[inline(always)]
    fn from(variant: BG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `BG`"]
pub struct BG_W<'a> {
    w: &'a mut W,
}
impl<'a> BG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BG_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BG_AW::B_0X0)
    }
    #[doc = "A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BG_AW::B_0X1)
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
impl W {
    #[doc = "Bit 0 - Update generation This bit can be set by software, it is automatically cleared by hardware."]
    #[inline(always)]
    pub fn ug(&mut self) -> UG_W {
        UG_W { w: self }
    }
    #[doc = "Bit 1 - Capture/Compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in TIMx_CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high."]
    #[inline(always)]
    pub fn cc1g(&mut self) -> CC1G_W {
        CC1G_W { w: self }
    }
    #[doc = "Bit 5 - Capture/Compare control update generation This bit can be set by software, it is automatically cleared by hardware. Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    pub fn comg(&mut self) -> COMG_W {
        COMG_W { w: self }
    }
    #[doc = "Bit 7 - Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware."]
    #[inline(always)]
    pub fn bg(&mut self) -> BG_W {
        BG_W { w: self }
    }
}
