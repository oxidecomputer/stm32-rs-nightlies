#[doc = "Reader of register CCMR2_Output"]
pub type R = crate::R<u32, super::CCMR2_OUTPUT>;
#[doc = "Writer for register CCMR2_Output"]
pub type W = crate::W<u32, super::CCMR2_OUTPUT>;
#[doc = "Register CCMR2_Output `reset()`'s with value 0"]
impl crate::ResetValue for super::CCMR2_OUTPUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OC4M_3`"]
pub type OC4M_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC4M_3`"]
pub struct OC4M_3_W<'a> {
    w: &'a mut W,
}
impl<'a> OC4M_3_W<'a> {
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
#[doc = "Reader of field `OC3M_3`"]
pub type OC3M_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC3M_3`"]
pub struct OC3M_3_W<'a> {
    w: &'a mut W,
}
impl<'a> OC3M_3_W<'a> {
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
#[doc = "Reader of field `O24CE`"]
pub type O24CE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `O24CE`"]
pub struct O24CE_W<'a> {
    w: &'a mut W,
}
impl<'a> O24CE_W<'a> {
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
#[doc = "OC4M\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OC4M_A {
    #[doc = "0: The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs"]
    FROZEN = 0,
    #[doc = "1: Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register"]
    ACTIVEONMATCH = 1,
    #[doc = "2: Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register"]
    INACTIVEONMATCH = 2,
    #[doc = "3: OCyREF toggles when TIMx_CNT=TIMx_CCRy"]
    TOGGLE = 3,
    #[doc = "4: OCyREF is forced low"]
    FORCEINACTIVE = 4,
    #[doc = "5: OCyREF is forced high"]
    FORCEACTIVE = 5,
    #[doc = "6: In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active"]
    PWMMODE1 = 6,
    #[doc = "7: Inversely to PwmMode1"]
    PWMMODE2 = 7,
    #[doc = "8: Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). In down-counting mode, the channel is inactive"]
    OPMMODE1 = 8,
    #[doc = "9: Inversely to OpmMode1"]
    OPMMODE2 = 9,
    #[doc = "12: OCyREF has the same behavior as in PWM mode 1. OCyREFC is the logical OR between OC1REF and OC2REF"]
    COMBINEDPWMMODE1 = 12,
    #[doc = "13: OCyREF has the same behavior as in PWM mode 2. OCyREFC is the logical AND between OC1REF and OC2REF"]
    COMBINEDPWMMODE2 = 13,
    #[doc = "14: OCyREF has the same behavior as in PWM mode 1. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down"]
    ASYMMETRICPWMMODE1 = 14,
    #[doc = "15: OCyREF has the same behavior as in PWM mode 2. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down"]
    ASYMMETRICPWMMODE2 = 15,
}
impl From<OC4M_A> for u8 {
    #[inline(always)]
    fn from(variant: OC4M_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OC4M`"]
pub type OC4M_R = crate::R<u8, OC4M_A>;
impl OC4M_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OC4M_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OC4M_A::FROZEN),
            1 => Val(OC4M_A::ACTIVEONMATCH),
            2 => Val(OC4M_A::INACTIVEONMATCH),
            3 => Val(OC4M_A::TOGGLE),
            4 => Val(OC4M_A::FORCEINACTIVE),
            5 => Val(OC4M_A::FORCEACTIVE),
            6 => Val(OC4M_A::PWMMODE1),
            7 => Val(OC4M_A::PWMMODE2),
            8 => Val(OC4M_A::OPMMODE1),
            9 => Val(OC4M_A::OPMMODE2),
            12 => Val(OC4M_A::COMBINEDPWMMODE1),
            13 => Val(OC4M_A::COMBINEDPWMMODE2),
            14 => Val(OC4M_A::ASYMMETRICPWMMODE1),
            15 => Val(OC4M_A::ASYMMETRICPWMMODE2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FROZEN`"]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == OC4M_A::FROZEN
    }
    #[doc = "Checks if the value of the field is `ACTIVEONMATCH`"]
    #[inline(always)]
    pub fn is_active_on_match(&self) -> bool {
        *self == OC4M_A::ACTIVEONMATCH
    }
    #[doc = "Checks if the value of the field is `INACTIVEONMATCH`"]
    #[inline(always)]
    pub fn is_inactive_on_match(&self) -> bool {
        *self == OC4M_A::INACTIVEONMATCH
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OC4M_A::TOGGLE
    }
    #[doc = "Checks if the value of the field is `FORCEINACTIVE`"]
    #[inline(always)]
    pub fn is_force_inactive(&self) -> bool {
        *self == OC4M_A::FORCEINACTIVE
    }
    #[doc = "Checks if the value of the field is `FORCEACTIVE`"]
    #[inline(always)]
    pub fn is_force_active(&self) -> bool {
        *self == OC4M_A::FORCEACTIVE
    }
    #[doc = "Checks if the value of the field is `PWMMODE1`"]
    #[inline(always)]
    pub fn is_pwm_mode1(&self) -> bool {
        *self == OC4M_A::PWMMODE1
    }
    #[doc = "Checks if the value of the field is `PWMMODE2`"]
    #[inline(always)]
    pub fn is_pwm_mode2(&self) -> bool {
        *self == OC4M_A::PWMMODE2
    }
    #[doc = "Checks if the value of the field is `OPMMODE1`"]
    #[inline(always)]
    pub fn is_opm_mode1(&self) -> bool {
        *self == OC4M_A::OPMMODE1
    }
    #[doc = "Checks if the value of the field is `OPMMODE2`"]
    #[inline(always)]
    pub fn is_opm_mode2(&self) -> bool {
        *self == OC4M_A::OPMMODE2
    }
    #[doc = "Checks if the value of the field is `COMBINEDPWMMODE1`"]
    #[inline(always)]
    pub fn is_combined_pwm_mode1(&self) -> bool {
        *self == OC4M_A::COMBINEDPWMMODE1
    }
    #[doc = "Checks if the value of the field is `COMBINEDPWMMODE2`"]
    #[inline(always)]
    pub fn is_combined_pwm_mode2(&self) -> bool {
        *self == OC4M_A::COMBINEDPWMMODE2
    }
    #[doc = "Checks if the value of the field is `ASYMMETRICPWMMODE1`"]
    #[inline(always)]
    pub fn is_asymmetric_pwm_mode1(&self) -> bool {
        *self == OC4M_A::ASYMMETRICPWMMODE1
    }
    #[doc = "Checks if the value of the field is `ASYMMETRICPWMMODE2`"]
    #[inline(always)]
    pub fn is_asymmetric_pwm_mode2(&self) -> bool {
        *self == OC4M_A::ASYMMETRICPWMMODE2
    }
}
#[doc = "Write proxy for field `OC4M`"]
pub struct OC4M_W<'a> {
    w: &'a mut W,
}
impl<'a> OC4M_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OC4M_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs"]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(OC4M_A::FROZEN)
    }
    #[doc = "Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register"]
    #[inline(always)]
    pub fn active_on_match(self) -> &'a mut W {
        self.variant(OC4M_A::ACTIVEONMATCH)
    }
    #[doc = "Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register"]
    #[inline(always)]
    pub fn inactive_on_match(self) -> &'a mut W {
        self.variant(OC4M_A::INACTIVEONMATCH)
    }
    #[doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OC4M_A::TOGGLE)
    }
    #[doc = "OCyREF is forced low"]
    #[inline(always)]
    pub fn force_inactive(self) -> &'a mut W {
        self.variant(OC4M_A::FORCEINACTIVE)
    }
    #[doc = "OCyREF is forced high"]
    #[inline(always)]
    pub fn force_active(self) -> &'a mut W {
        self.variant(OC4M_A::FORCEACTIVE)
    }
    #[doc = "In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active"]
    #[inline(always)]
    pub fn pwm_mode1(self) -> &'a mut W {
        self.variant(OC4M_A::PWMMODE1)
    }
    #[doc = "Inversely to PwmMode1"]
    #[inline(always)]
    pub fn pwm_mode2(self) -> &'a mut W {
        self.variant(OC4M_A::PWMMODE2)
    }
    #[doc = "Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). In down-counting mode, the channel is inactive"]
    #[inline(always)]
    pub fn opm_mode1(self) -> &'a mut W {
        self.variant(OC4M_A::OPMMODE1)
    }
    #[doc = "Inversely to OpmMode1"]
    #[inline(always)]
    pub fn opm_mode2(self) -> &'a mut W {
        self.variant(OC4M_A::OPMMODE2)
    }
    #[doc = "OCyREF has the same behavior as in PWM mode 1. OCyREFC is the logical OR between OC1REF and OC2REF"]
    #[inline(always)]
    pub fn combined_pwm_mode1(self) -> &'a mut W {
        self.variant(OC4M_A::COMBINEDPWMMODE1)
    }
    #[doc = "OCyREF has the same behavior as in PWM mode 2. OCyREFC is the logical AND between OC1REF and OC2REF"]
    #[inline(always)]
    pub fn combined_pwm_mode2(self) -> &'a mut W {
        self.variant(OC4M_A::COMBINEDPWMMODE2)
    }
    #[doc = "OCyREF has the same behavior as in PWM mode 1. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down"]
    #[inline(always)]
    pub fn asymmetric_pwm_mode1(self) -> &'a mut W {
        self.variant(OC4M_A::ASYMMETRICPWMMODE1)
    }
    #[doc = "OCyREF has the same behavior as in PWM mode 2. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down"]
    #[inline(always)]
    pub fn asymmetric_pwm_mode2(self) -> &'a mut W {
        self.variant(OC4M_A::ASYMMETRICPWMMODE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `OC4PE`"]
pub type OC4PE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC4PE`"]
pub struct OC4PE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC4PE_W<'a> {
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
#[doc = "Reader of field `OC4FE`"]
pub type OC4FE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC4FE`"]
pub struct OC4FE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC4FE_W<'a> {
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
#[doc = "Reader of field `CC4S`"]
pub type CC4S_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CC4S`"]
pub struct CC4S_W<'a> {
    w: &'a mut W,
}
impl<'a> CC4S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `OC3CE`"]
pub type OC3CE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC3CE`"]
pub struct OC3CE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC3CE_W<'a> {
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
#[doc = "OC3M"]
pub type OC3M_A = OC4M_A;
#[doc = "Reader of field `OC3M`"]
pub type OC3M_R = crate::R<u8, OC4M_A>;
#[doc = "Write proxy for field `OC3M`"]
pub struct OC3M_W<'a> {
    w: &'a mut W,
}
impl<'a> OC3M_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OC3M_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs"]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(OC4M_A::FROZEN)
    }
    #[doc = "Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register"]
    #[inline(always)]
    pub fn active_on_match(self) -> &'a mut W {
        self.variant(OC4M_A::ACTIVEONMATCH)
    }
    #[doc = "Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register"]
    #[inline(always)]
    pub fn inactive_on_match(self) -> &'a mut W {
        self.variant(OC4M_A::INACTIVEONMATCH)
    }
    #[doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OC4M_A::TOGGLE)
    }
    #[doc = "OCyREF is forced low"]
    #[inline(always)]
    pub fn force_inactive(self) -> &'a mut W {
        self.variant(OC4M_A::FORCEINACTIVE)
    }
    #[doc = "OCyREF is forced high"]
    #[inline(always)]
    pub fn force_active(self) -> &'a mut W {
        self.variant(OC4M_A::FORCEACTIVE)
    }
    #[doc = "In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active"]
    #[inline(always)]
    pub fn pwm_mode1(self) -> &'a mut W {
        self.variant(OC4M_A::PWMMODE1)
    }
    #[doc = "Inversely to PwmMode1"]
    #[inline(always)]
    pub fn pwm_mode2(self) -> &'a mut W {
        self.variant(OC4M_A::PWMMODE2)
    }
    #[doc = "Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). In down-counting mode, the channel is inactive"]
    #[inline(always)]
    pub fn opm_mode1(self) -> &'a mut W {
        self.variant(OC4M_A::OPMMODE1)
    }
    #[doc = "Inversely to OpmMode1"]
    #[inline(always)]
    pub fn opm_mode2(self) -> &'a mut W {
        self.variant(OC4M_A::OPMMODE2)
    }
    #[doc = "OCyREF has the same behavior as in PWM mode 1. OCyREFC is the logical OR between OC1REF and OC2REF"]
    #[inline(always)]
    pub fn combined_pwm_mode1(self) -> &'a mut W {
        self.variant(OC4M_A::COMBINEDPWMMODE1)
    }
    #[doc = "OCyREF has the same behavior as in PWM mode 2. OCyREFC is the logical AND between OC1REF and OC2REF"]
    #[inline(always)]
    pub fn combined_pwm_mode2(self) -> &'a mut W {
        self.variant(OC4M_A::COMBINEDPWMMODE2)
    }
    #[doc = "OCyREF has the same behavior as in PWM mode 1. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down"]
    #[inline(always)]
    pub fn asymmetric_pwm_mode1(self) -> &'a mut W {
        self.variant(OC4M_A::ASYMMETRICPWMMODE1)
    }
    #[doc = "OCyREF has the same behavior as in PWM mode 2. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down"]
    #[inline(always)]
    pub fn asymmetric_pwm_mode2(self) -> &'a mut W {
        self.variant(OC4M_A::ASYMMETRICPWMMODE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `OC3PE`"]
pub type OC3PE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC3PE`"]
pub struct OC3PE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC3PE_W<'a> {
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
#[doc = "Reader of field `OC3FE`"]
pub type OC3FE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC3FE`"]
pub struct OC3FE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC3FE_W<'a> {
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
#[doc = "Reader of field `CC3S`"]
pub type CC3S_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CC3S`"]
pub struct CC3S_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 24 - Output Compare 2 mode - bit 3"]
    #[inline(always)]
    pub fn oc4m_3(&self) -> OC4M_3_R {
        OC4M_3_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Output Compare 1 mode - bit 3"]
    #[inline(always)]
    pub fn oc3m_3(&self) -> OC3M_3_R {
        OC3M_3_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - O24CE"]
    #[inline(always)]
    pub fn o24ce(&self) -> O24CE_R {
        O24CE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - OC4M"]
    #[inline(always)]
    pub fn oc4m(&self) -> OC4M_R {
        OC4M_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 11 - OC4PE"]
    #[inline(always)]
    pub fn oc4pe(&self) -> OC4PE_R {
        OC4PE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - OC4FE"]
    #[inline(always)]
    pub fn oc4fe(&self) -> OC4FE_R {
        OC4FE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - CC4S"]
    #[inline(always)]
    pub fn cc4s(&self) -> CC4S_R {
        CC4S_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 7 - OC3CE"]
    #[inline(always)]
    pub fn oc3ce(&self) -> OC3CE_R {
        OC3CE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - OC3M"]
    #[inline(always)]
    pub fn oc3m(&self) -> OC3M_R {
        OC3M_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3 - OC3PE"]
    #[inline(always)]
    pub fn oc3pe(&self) -> OC3PE_R {
        OC3PE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OC3FE"]
    #[inline(always)]
    pub fn oc3fe(&self) -> OC3FE_R {
        OC3FE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - CC3S"]
    #[inline(always)]
    pub fn cc3s(&self) -> CC3S_R {
        CC3S_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 24 - Output Compare 2 mode - bit 3"]
    #[inline(always)]
    pub fn oc4m_3(&mut self) -> OC4M_3_W {
        OC4M_3_W { w: self }
    }
    #[doc = "Bit 16 - Output Compare 1 mode - bit 3"]
    #[inline(always)]
    pub fn oc3m_3(&mut self) -> OC3M_3_W {
        OC3M_3_W { w: self }
    }
    #[doc = "Bit 15 - O24CE"]
    #[inline(always)]
    pub fn o24ce(&mut self) -> O24CE_W {
        O24CE_W { w: self }
    }
    #[doc = "Bits 12:14 - OC4M"]
    #[inline(always)]
    pub fn oc4m(&mut self) -> OC4M_W {
        OC4M_W { w: self }
    }
    #[doc = "Bit 11 - OC4PE"]
    #[inline(always)]
    pub fn oc4pe(&mut self) -> OC4PE_W {
        OC4PE_W { w: self }
    }
    #[doc = "Bit 10 - OC4FE"]
    #[inline(always)]
    pub fn oc4fe(&mut self) -> OC4FE_W {
        OC4FE_W { w: self }
    }
    #[doc = "Bits 8:9 - CC4S"]
    #[inline(always)]
    pub fn cc4s(&mut self) -> CC4S_W {
        CC4S_W { w: self }
    }
    #[doc = "Bit 7 - OC3CE"]
    #[inline(always)]
    pub fn oc3ce(&mut self) -> OC3CE_W {
        OC3CE_W { w: self }
    }
    #[doc = "Bits 4:6 - OC3M"]
    #[inline(always)]
    pub fn oc3m(&mut self) -> OC3M_W {
        OC3M_W { w: self }
    }
    #[doc = "Bit 3 - OC3PE"]
    #[inline(always)]
    pub fn oc3pe(&mut self) -> OC3PE_W {
        OC3PE_W { w: self }
    }
    #[doc = "Bit 2 - OC3FE"]
    #[inline(always)]
    pub fn oc3fe(&mut self) -> OC3FE_W {
        OC3FE_W { w: self }
    }
    #[doc = "Bits 0:1 - CC3S"]
    #[inline(always)]
    pub fn cc3s(&mut self) -> CC3S_W {
        CC3S_W { w: self }
    }
}
