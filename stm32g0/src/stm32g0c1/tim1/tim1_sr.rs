#[doc = "Reader of register TIM1_SR"]
pub type R = crate::R<u32, super::TIM1_SR>;
#[doc = "Writer for register TIM1_SR"]
pub type W = crate::W<u32, super::TIM1_SR>;
#[doc = "Register TIM1_SR `reset()`'s with value 0"]
impl crate::ResetValue for super::TIM1_SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to control register (TIM1_SMCRTIMx_SMCR)N/A), if URS=0 and UDIS=0 in the TIMx_CR1 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UIF_A {
    #[doc = "0: No update occurred."]
    B_0X0 = 0,
    #[doc = "1: Update interrupt pending. This bit is set by hardware when the registers are updated:"]
    B_0X1 = 1,
}
impl From<UIF_A> for bool {
    #[inline(always)]
    fn from(variant: UIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UIF`"]
pub type UIF_R = crate::R<bool, UIF_A>;
impl UIF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UIF_A {
        match self.bits {
            false => UIF_A::B_0X0,
            true => UIF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UIF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == UIF_A::B_0X1
    }
}
#[doc = "Write proxy for field `UIF`"]
pub struct UIF_W<'a> {
    w: &'a mut W,
}
impl<'a> UIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No update occurred."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(UIF_A::B_0X0)
    }
    #[doc = "Update interrupt pending. This bit is set by hardware when the registers are updated:"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(UIF_A::B_0X1)
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
#[doc = "Capture/Compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when he content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in down-counting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1IF_A {
    #[doc = "0: No compare match / No input capture occurred"]
    B_0X0 = 0,
    #[doc = "1: A compare match or an input capture occurred."]
    B_0X1 = 1,
}
impl From<CC1IF_A> for bool {
    #[inline(always)]
    fn from(variant: CC1IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CC1IF`"]
pub type CC1IF_R = crate::R<bool, CC1IF_A>;
impl CC1IF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC1IF_A {
        match self.bits {
            false => CC1IF_A::B_0X0,
            true => CC1IF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC1IF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC1IF_A::B_0X1
    }
}
#[doc = "Write proxy for field `CC1IF`"]
pub struct CC1IF_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1IF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC1IF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No compare match / No input capture occurred"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CC1IF_A::B_0X0)
    }
    #[doc = "A compare match or an input capture occurred."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CC1IF_A::B_0X1)
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
#[doc = "Reader of field `CC2IF`"]
pub type CC2IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CC2IF`"]
pub struct CC2IF_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2IF_W<'a> {
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
#[doc = "Reader of field `CC3IF`"]
pub type CC3IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CC3IF`"]
pub struct CC3IF_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3IF_W<'a> {
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
#[doc = "Reader of field `CC4IF`"]
pub type CC4IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CC4IF`"]
pub struct CC4IF_W<'a> {
    w: &'a mut W,
}
impl<'a> CC4IF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "COM interrupt flag This flag is set by hardware on COM event (when Capture/compare Control bits - CCxE, CCxNE, OCxM - have been updated). It is cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMIF_A {
    #[doc = "0: No COM event occurred."]
    B_0X0 = 0,
    #[doc = "1: COM interrupt pending."]
    B_0X1 = 1,
}
impl From<COMIF_A> for bool {
    #[inline(always)]
    fn from(variant: COMIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMIF`"]
pub type COMIF_R = crate::R<bool, COMIF_A>;
impl COMIF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMIF_A {
        match self.bits {
            false => COMIF_A::B_0X0,
            true => COMIF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == COMIF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == COMIF_A::B_0X1
    }
}
#[doc = "Write proxy for field `COMIF`"]
pub struct COMIF_W<'a> {
    w: &'a mut W,
}
impl<'a> COMIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No COM event occurred."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(COMIF_A::B_0X0)
    }
    #[doc = "COM interrupt pending."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(COMIF_A::B_0X1)
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
#[doc = "Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIF_A {
    #[doc = "0: No trigger event occurred."]
    B_0X0 = 0,
    #[doc = "1: Trigger interrupt pending."]
    B_0X1 = 1,
}
impl From<TIF_A> for bool {
    #[inline(always)]
    fn from(variant: TIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIF`"]
pub type TIF_R = crate::R<bool, TIF_A>;
impl TIF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIF_A {
        match self.bits {
            false => TIF_A::B_0X0,
            true => TIF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIF_A::B_0X1
    }
}
#[doc = "Write proxy for field `TIF`"]
pub struct TIF_W<'a> {
    w: &'a mut W,
}
impl<'a> TIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No trigger event occurred."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIF_A::B_0X0)
    }
    #[doc = "Trigger interrupt pending."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIF_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIF_A {
    #[doc = "0: No break event occurred."]
    B_0X0 = 0,
    #[doc = "1: An active level has been detected on the break input. An interrupt is generated if BIE=1 in the TIMx_DIER register."]
    B_0X1 = 1,
}
impl From<BIF_A> for bool {
    #[inline(always)]
    fn from(variant: BIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BIF`"]
pub type BIF_R = crate::R<bool, BIF_A>;
impl BIF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIF_A {
        match self.bits {
            false => BIF_A::B_0X0,
            true => BIF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BIF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BIF_A::B_0X1
    }
}
#[doc = "Write proxy for field `BIF`"]
pub struct BIF_W<'a> {
    w: &'a mut W,
}
impl<'a> BIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No break event occurred."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BIF_A::B_0X0)
    }
    #[doc = "An active level has been detected on the break input. An interrupt is generated if BIE=1 in the TIMx_DIER register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BIF_A::B_0X1)
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
#[doc = "Break 2 interrupt flag This flag is set by hardware as soon as the break 2 input goes active. It can be cleared by software if the break 2 input is not active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B2IF_A {
    #[doc = "0: No break event occurred."]
    B_0X0 = 0,
    #[doc = "1: An active level has been detected on the break 2 input. An interrupt is generated if BIE=1 in the TIMx_DIER register."]
    B_0X1 = 1,
}
impl From<B2IF_A> for bool {
    #[inline(always)]
    fn from(variant: B2IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `B2IF`"]
pub type B2IF_R = crate::R<bool, B2IF_A>;
impl B2IF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B2IF_A {
        match self.bits {
            false => B2IF_A::B_0X0,
            true => B2IF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == B2IF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == B2IF_A::B_0X1
    }
}
#[doc = "Write proxy for field `B2IF`"]
pub struct B2IF_W<'a> {
    w: &'a mut W,
}
impl<'a> B2IF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: B2IF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No break event occurred."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(B2IF_A::B_0X0)
    }
    #[doc = "An active level has been detected on the break 2 input. An interrupt is generated if BIE=1 in the TIMx_DIER register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(B2IF_A::B_0X1)
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
#[doc = "Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0â\u{80}\u{99}.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1OF_A {
    #[doc = "0: No overcapture has been detected."]
    B_0X0 = 0,
    #[doc = "1: The counter value has been captured in TIMx_CCR1 register while CC1IF flag was already set"]
    B_0X1 = 1,
}
impl From<CC1OF_A> for bool {
    #[inline(always)]
    fn from(variant: CC1OF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CC1OF`"]
pub type CC1OF_R = crate::R<bool, CC1OF_A>;
impl CC1OF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC1OF_A {
        match self.bits {
            false => CC1OF_A::B_0X0,
            true => CC1OF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC1OF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC1OF_A::B_0X1
    }
}
#[doc = "Write proxy for field `CC1OF`"]
pub struct CC1OF_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1OF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC1OF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No overcapture has been detected."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CC1OF_A::B_0X0)
    }
    #[doc = "The counter value has been captured in TIMx_CCR1 register while CC1IF flag was already set"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CC1OF_A::B_0X1)
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
#[doc = "Reader of field `CC2OF`"]
pub type CC2OF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CC2OF`"]
pub struct CC2OF_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2OF_W<'a> {
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
#[doc = "Reader of field `CC3OF`"]
pub type CC3OF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CC3OF`"]
pub struct CC3OF_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3OF_W<'a> {
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
#[doc = "Reader of field `CC4OF`"]
pub type CC4OF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CC4OF`"]
pub struct CC4OF_W<'a> {
    w: &'a mut W,
}
impl<'a> CC4OF_W<'a> {
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
#[doc = "System Break interrupt flag This flag is set by hardware as soon as the system break input goes active. It can be cleared by software if the system break input is not active. This flag must be reset to re-start PWM operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBIF_A {
    #[doc = "0: No break event occurred."]
    B_0X0 = 0,
    #[doc = "1: An active level has been detected on the system break input. An interrupt is generated if BIE=1 in the TIMx_DIER register."]
    B_0X1 = 1,
}
impl From<SBIF_A> for bool {
    #[inline(always)]
    fn from(variant: SBIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SBIF`"]
pub type SBIF_R = crate::R<bool, SBIF_A>;
impl SBIF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBIF_A {
        match self.bits {
            false => SBIF_A::B_0X0,
            true => SBIF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SBIF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SBIF_A::B_0X1
    }
}
#[doc = "Write proxy for field `SBIF`"]
pub struct SBIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SBIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SBIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No break event occurred."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SBIF_A::B_0X0)
    }
    #[doc = "An active level has been detected on the system break input. An interrupt is generated if BIE=1 in the TIMx_DIER register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SBIF_A::B_0X1)
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
#[doc = "Reader of field `CC5IF`"]
pub type CC5IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CC5IF`"]
pub struct CC5IF_W<'a> {
    w: &'a mut W,
}
impl<'a> CC5IF_W<'a> {
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
#[doc = "Reader of field `CC6IF`"]
pub type CC6IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CC6IF`"]
pub struct CC6IF_W<'a> {
    w: &'a mut W,
}
impl<'a> CC6IF_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to control register (TIM1_SMCRTIMx_SMCR)N/A), if URS=0 and UDIS=0 in the TIMx_CR1 register."]
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when he content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in down-counting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    pub fn cc2if(&self) -> CC2IF_R {
        CC2IF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    pub fn cc3if(&self) -> CC3IF_R {
        CC3IF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    pub fn cc4if(&self) -> CC4IF_R {
        CC4IF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - COM interrupt flag This flag is set by hardware on COM event (when Capture/compare Control bits - CCxE, CCxNE, OCxM - have been updated). It is cleared by software."]
    #[inline(always)]
    pub fn comif(&self) -> COMIF_R {
        COMIF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software."]
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active."]
    #[inline(always)]
    pub fn bif(&self) -> BIF_R {
        BIF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Break 2 interrupt flag This flag is set by hardware as soon as the break 2 input goes active. It can be cleared by software if the break 2 input is not active."]
    #[inline(always)]
    pub fn b2if(&self) -> B2IF_R {
        B2IF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0â\u{80}\u{99}."]
    #[inline(always)]
    pub fn cc1of(&self) -> CC1OF_R {
        CC1OF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Capture/Compare 2 overcapture flag Refer to CC1OF description"]
    #[inline(always)]
    pub fn cc2of(&self) -> CC2OF_R {
        CC2OF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Capture/Compare 3 overcapture flag Refer to CC1OF description"]
    #[inline(always)]
    pub fn cc3of(&self) -> CC3OF_R {
        CC3OF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Capture/Compare 4 overcapture flag Refer to CC1OF description"]
    #[inline(always)]
    pub fn cc4of(&self) -> CC4OF_R {
        CC4OF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - System Break interrupt flag This flag is set by hardware as soon as the system break input goes active. It can be cleared by software if the system break input is not active. This flag must be reset to re-start PWM operation."]
    #[inline(always)]
    pub fn sbif(&self) -> SBIF_R {
        SBIF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Compare 5 interrupt flag Refer to CC1IF description (Note: Channel 5 can only be configured as output)"]
    #[inline(always)]
    pub fn cc5if(&self) -> CC5IF_R {
        CC5IF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Compare 6 interrupt flag Refer to CC1IF description (Note: Channel 6 can only be configured as output)"]
    #[inline(always)]
    pub fn cc6if(&self) -> CC6IF_R {
        CC6IF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to control register (TIM1_SMCRTIMx_SMCR)N/A), if URS=0 and UDIS=0 in the TIMx_CR1 register."]
    #[inline(always)]
    pub fn uif(&mut self) -> UIF_W {
        UIF_W { w: self }
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when he content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in down-counting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc1if(&mut self) -> CC1IF_W {
        CC1IF_W { w: self }
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    pub fn cc2if(&mut self) -> CC2IF_W {
        CC2IF_W { w: self }
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    pub fn cc3if(&mut self) -> CC3IF_W {
        CC3IF_W { w: self }
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    pub fn cc4if(&mut self) -> CC4IF_W {
        CC4IF_W { w: self }
    }
    #[doc = "Bit 5 - COM interrupt flag This flag is set by hardware on COM event (when Capture/compare Control bits - CCxE, CCxNE, OCxM - have been updated). It is cleared by software."]
    #[inline(always)]
    pub fn comif(&mut self) -> COMIF_W {
        COMIF_W { w: self }
    }
    #[doc = "Bit 6 - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software."]
    #[inline(always)]
    pub fn tif(&mut self) -> TIF_W {
        TIF_W { w: self }
    }
    #[doc = "Bit 7 - Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active."]
    #[inline(always)]
    pub fn bif(&mut self) -> BIF_W {
        BIF_W { w: self }
    }
    #[doc = "Bit 8 - Break 2 interrupt flag This flag is set by hardware as soon as the break 2 input goes active. It can be cleared by software if the break 2 input is not active."]
    #[inline(always)]
    pub fn b2if(&mut self) -> B2IF_W {
        B2IF_W { w: self }
    }
    #[doc = "Bit 9 - Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0â\u{80}\u{99}."]
    #[inline(always)]
    pub fn cc1of(&mut self) -> CC1OF_W {
        CC1OF_W { w: self }
    }
    #[doc = "Bit 10 - Capture/Compare 2 overcapture flag Refer to CC1OF description"]
    #[inline(always)]
    pub fn cc2of(&mut self) -> CC2OF_W {
        CC2OF_W { w: self }
    }
    #[doc = "Bit 11 - Capture/Compare 3 overcapture flag Refer to CC1OF description"]
    #[inline(always)]
    pub fn cc3of(&mut self) -> CC3OF_W {
        CC3OF_W { w: self }
    }
    #[doc = "Bit 12 - Capture/Compare 4 overcapture flag Refer to CC1OF description"]
    #[inline(always)]
    pub fn cc4of(&mut self) -> CC4OF_W {
        CC4OF_W { w: self }
    }
    #[doc = "Bit 13 - System Break interrupt flag This flag is set by hardware as soon as the system break input goes active. It can be cleared by software if the system break input is not active. This flag must be reset to re-start PWM operation."]
    #[inline(always)]
    pub fn sbif(&mut self) -> SBIF_W {
        SBIF_W { w: self }
    }
    #[doc = "Bit 16 - Compare 5 interrupt flag Refer to CC1IF description (Note: Channel 5 can only be configured as output)"]
    #[inline(always)]
    pub fn cc5if(&mut self) -> CC5IF_W {
        CC5IF_W { w: self }
    }
    #[doc = "Bit 17 - Compare 6 interrupt flag Refer to CC1IF description (Note: Channel 6 can only be configured as output)"]
    #[inline(always)]
    pub fn cc6if(&mut self) -> CC6IF_W {
        CC6IF_W { w: self }
    }
}
