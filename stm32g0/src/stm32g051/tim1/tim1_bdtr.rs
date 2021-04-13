#[doc = "Reader of register TIM1_BDTR"]
pub type R = crate::R<u32, super::TIM1_BDTR>;
#[doc = "Writer for register TIM1_BDTR"]
pub type W = crate::W<u32, super::TIM1_BDTR>;
#[doc = "Register TIM1_BDTR `reset()`'s with value 0"]
impl crate::ResetValue for super::TIM1_BDTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DTG`"]
pub type DTG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DTG`"]
pub struct DTG_W<'a> {
    w: &'a mut W,
}
impl<'a> DTG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Lock configuration These bits offer a write protection against software errors. Note: The LOCK bits can be written only once after the reset. Once the TIMx_BDTR register has been written, their content is frozen until the next reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LOCK_A {
    #[doc = "0: LOCK OFF - No bit is write protected."]
    B_0X0 = 0,
    #[doc = "1: LOCK Level 1 = DTG bits in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register and BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
bits in TIMx_BDTR register can no longer be written."]
    B_0X1 = 1,
    #[doc = "2: LOCK Level 2 = LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written."]
    B_0X2 = 2,
    #[doc = "3: LOCK Level 3 = LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written."]
    B_0X3 = 3,
}
impl From<LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<u8, LOCK_A>;
impl LOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            0 => LOCK_A::B_0X0,
            1 => LOCK_A::B_0X1,
            2 => LOCK_A::B_0X2,
            3 => LOCK_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LOCK_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LOCK_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == LOCK_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == LOCK_A::B_0X3
    }
}
#[doc = "Write proxy for field `LOCK`"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "LOCK OFF - No bit is write protected."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LOCK_A::B_0X0)
    }
    #[doc = "LOCK Level 1 = DTG bits in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register and BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
bits in TIMx_BDTR register can no longer be written."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LOCK_A::B_0X1)
    }
    #[doc = "LOCK Level 2 = LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(LOCK_A::B_0X2)
    }
    #[doc = "LOCK Level 3 = LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(LOCK_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Off-state selection for Idle mode This bit is used when MOE=0 due to a break event or by a software write, on channels configured as outputs. See OC/OCN enable description for more details (enable register (TIM1_CCERTIMx_CCER)N/A). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSSI_A {
    #[doc = "0: When inactive, OC/OCN outputs are disabled (the timer releases the output control which is taken over by the GPIO logic and which imposes a Hi-Z state)."]
    B_0X0 = 0,
    #[doc = "1: When inactive, OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime. The timer maintains its control over the output."]
    B_0X1 = 1,
}
impl From<OSSI_A> for bool {
    #[inline(always)]
    fn from(variant: OSSI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OSSI`"]
pub type OSSI_R = crate::R<bool, OSSI_A>;
impl OSSI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSSI_A {
        match self.bits {
            false => OSSI_A::B_0X0,
            true => OSSI_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OSSI_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OSSI_A::B_0X1
    }
}
#[doc = "Write proxy for field `OSSI`"]
pub struct OSSI_W<'a> {
    w: &'a mut W,
}
impl<'a> OSSI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSSI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "When inactive, OC/OCN outputs are disabled (the timer releases the output control which is taken over by the GPIO logic and which imposes a Hi-Z state)."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(OSSI_A::B_0X0)
    }
    #[doc = "When inactive, OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime. The timer maintains its control over the output."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(OSSI_A::B_0X1)
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
#[doc = "Off-state selection for Run mode This bit is used when MOE=1 on channels having a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. See OC/OCN enable description for more details (enable register (TIM1_CCERTIMx_CCER)N/A). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSSR_A {
    #[doc = "0: When inactive, OC/OCN outputs are disabled (the timer releases the output control which is taken over by the GPIO logic, which forces a Hi-Z state)."]
    B_0X0 = 0,
    #[doc = "1: When inactive, OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1 (the output is still controlled by the timer)."]
    B_0X1 = 1,
}
impl From<OSSR_A> for bool {
    #[inline(always)]
    fn from(variant: OSSR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OSSR`"]
pub type OSSR_R = crate::R<bool, OSSR_A>;
impl OSSR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSSR_A {
        match self.bits {
            false => OSSR_A::B_0X0,
            true => OSSR_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OSSR_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OSSR_A::B_0X1
    }
}
#[doc = "Write proxy for field `OSSR`"]
pub struct OSSR_W<'a> {
    w: &'a mut W,
}
impl<'a> OSSR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSSR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "When inactive, OC/OCN outputs are disabled (the timer releases the output control which is taken over by the GPIO logic, which forces a Hi-Z state)."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(OSSR_A::B_0X0)
    }
    #[doc = "When inactive, OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1 (the output is still controlled by the timer)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(OSSR_A::B_0X1)
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
#[doc = "Break enable This bit enables the complete break protection (including all sources connected to bk_acth and BKIN sources, as per ). Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKE_A {
    #[doc = "0: Break function disabled"]
    B_0X0 = 0,
    #[doc = "1: Break function enabled"]
    B_0X1 = 1,
}
impl From<BKE_A> for bool {
    #[inline(always)]
    fn from(variant: BKE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BKE`"]
pub type BKE_R = crate::R<bool, BKE_A>;
impl BKE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BKE_A {
        match self.bits {
            false => BKE_A::B_0X0,
            true => BKE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BKE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BKE_A::B_0X1
    }
}
#[doc = "Write proxy for field `BKE`"]
pub struct BKE_W<'a> {
    w: &'a mut W,
}
impl<'a> BKE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BKE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Break function disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BKE_A::B_0X0)
    }
    #[doc = "Break function enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BKE_A::B_0X1)
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
#[doc = "Break polarity Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKP_A {
    #[doc = "0: Break input BRK is active low"]
    B_0X0 = 0,
    #[doc = "1: Break input BRK is active high"]
    B_0X1 = 1,
}
impl From<BKP_A> for bool {
    #[inline(always)]
    fn from(variant: BKP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BKP`"]
pub type BKP_R = crate::R<bool, BKP_A>;
impl BKP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BKP_A {
        match self.bits {
            false => BKP_A::B_0X0,
            true => BKP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BKP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BKP_A::B_0X1
    }
}
#[doc = "Write proxy for field `BKP`"]
pub struct BKP_W<'a> {
    w: &'a mut W,
}
impl<'a> BKP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BKP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Break input BRK is active low"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BKP_A::B_0X0)
    }
    #[doc = "Break input BRK is active high"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BKP_A::B_0X1)
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
#[doc = "Automatic output enable Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AOE_A {
    #[doc = "0: MOE can be set only by software"]
    B_0X0 = 0,
    #[doc = "1: MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)"]
    B_0X1 = 1,
}
impl From<AOE_A> for bool {
    #[inline(always)]
    fn from(variant: AOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AOE`"]
pub type AOE_R = crate::R<bool, AOE_A>;
impl AOE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AOE_A {
        match self.bits {
            false => AOE_A::B_0X0,
            true => AOE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AOE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AOE_A::B_0X1
    }
}
#[doc = "Write proxy for field `AOE`"]
pub struct AOE_W<'a> {
    w: &'a mut W,
}
impl<'a> AOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AOE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MOE can be set only by software"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(AOE_A::B_0X0)
    }
    #[doc = "MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(AOE_A::B_0X1)
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
#[doc = "Main output enable This bit is cleared asynchronously by hardware as soon as one of the break inputs is active (BRK or BRK2). It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. In response to a break event or if MOE is written to 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. See OC/OCN enable description for more details (enable register (TIM1_CCERTIMx_CCER)N/A).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOE_A {
    #[doc = "0: In response to a break 2 event. OC and OCN outputs are disabled"]
    B_0X0 = 0,
    #[doc = "1: OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register)."]
    B_0X1 = 1,
}
impl From<MOE_A> for bool {
    #[inline(always)]
    fn from(variant: MOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MOE`"]
pub type MOE_R = crate::R<bool, MOE_A>;
impl MOE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MOE_A {
        match self.bits {
            false => MOE_A::B_0X0,
            true => MOE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MOE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MOE_A::B_0X1
    }
}
#[doc = "Write proxy for field `MOE`"]
pub struct MOE_W<'a> {
    w: &'a mut W,
}
impl<'a> MOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MOE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "In response to a break 2 event. OC and OCN outputs are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MOE_A::B_0X0)
    }
    #[doc = "OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MOE_A::B_0X1)
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
#[doc = "Break filter This bit-field defines the frequency used to sample BRK input and the length of the digital filter applied to BRK. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BKF_A {
    #[doc = "0: No filter, BRK acts asynchronously"]
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
impl From<BKF_A> for u8 {
    #[inline(always)]
    fn from(variant: BKF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BKF`"]
pub type BKF_R = crate::R<u8, BKF_A>;
impl BKF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BKF_A {
        match self.bits {
            0 => BKF_A::B_0X0,
            1 => BKF_A::B_0X1,
            2 => BKF_A::B_0X2,
            3 => BKF_A::B_0X3,
            4 => BKF_A::B_0X4,
            5 => BKF_A::B_0X5,
            6 => BKF_A::B_0X6,
            7 => BKF_A::B_0X7,
            8 => BKF_A::B_0X8,
            9 => BKF_A::B_0X9,
            10 => BKF_A::B_0XA,
            11 => BKF_A::B_0XB,
            12 => BKF_A::B_0XC,
            13 => BKF_A::B_0XD,
            14 => BKF_A::B_0XE,
            15 => BKF_A::B_0XF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BKF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BKF_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == BKF_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == BKF_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == BKF_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == BKF_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == BKF_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == BKF_A::B_0X7
    }
    #[doc = "Checks if the value of the field is `B_0X8`"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == BKF_A::B_0X8
    }
    #[doc = "Checks if the value of the field is `B_0X9`"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == BKF_A::B_0X9
    }
    #[doc = "Checks if the value of the field is `B_0XA`"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == BKF_A::B_0XA
    }
    #[doc = "Checks if the value of the field is `B_0XB`"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == BKF_A::B_0XB
    }
    #[doc = "Checks if the value of the field is `B_0XC`"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == BKF_A::B_0XC
    }
    #[doc = "Checks if the value of the field is `B_0XD`"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == BKF_A::B_0XD
    }
    #[doc = "Checks if the value of the field is `B_0XE`"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == BKF_A::B_0XE
    }
    #[doc = "Checks if the value of the field is `B_0XF`"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == BKF_A::B_0XF
    }
}
#[doc = "Write proxy for field `BKF`"]
pub struct BKF_W<'a> {
    w: &'a mut W,
}
impl<'a> BKF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BKF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No filter, BRK acts asynchronously"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BKF_A::B_0X0)
    }
    #[doc = "fSAMPLING=fCK_INT, N=2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BKF_A::B_0X1)
    }
    #[doc = "fSAMPLING=fCK_INT, N=4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(BKF_A::B_0X2)
    }
    #[doc = "fSAMPLING=fCK_INT, N=8"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(BKF_A::B_0X3)
    }
    #[doc = "fSAMPLING=fDTS/2, N=6"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(BKF_A::B_0X4)
    }
    #[doc = "fSAMPLING=fDTS/2, N=8"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(BKF_A::B_0X5)
    }
    #[doc = "fSAMPLING=fDTS/4, N=6"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(BKF_A::B_0X6)
    }
    #[doc = "fSAMPLING=fDTS/4, N=8"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(BKF_A::B_0X7)
    }
    #[doc = "fSAMPLING=fDTS/8, N=6"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut W {
        self.variant(BKF_A::B_0X8)
    }
    #[doc = "fSAMPLING=fDTS/8, N=8"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut W {
        self.variant(BKF_A::B_0X9)
    }
    #[doc = "fSAMPLING=fDTS/16, N=5"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut W {
        self.variant(BKF_A::B_0XA)
    }
    #[doc = "fSAMPLING=fDTS/16, N=6"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut W {
        self.variant(BKF_A::B_0XB)
    }
    #[doc = "fSAMPLING=fDTS/16, N=8"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut W {
        self.variant(BKF_A::B_0XC)
    }
    #[doc = "fSAMPLING=fDTS/32, N=5"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut W {
        self.variant(BKF_A::B_0XD)
    }
    #[doc = "fSAMPLING=fDTS/32, N=6"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut W {
        self.variant(BKF_A::B_0XE)
    }
    #[doc = "fSAMPLING=fDTS/32, N=8"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut W {
        self.variant(BKF_A::B_0XF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Break 2 filter This bit-field defines the frequency used to sample BRK2 input and the length of the digital filter applied to BRK2. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BK2F_A {
    #[doc = "0: No filter, BRK2 acts asynchronously"]
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
impl From<BK2F_A> for u8 {
    #[inline(always)]
    fn from(variant: BK2F_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BK2F`"]
pub type BK2F_R = crate::R<u8, BK2F_A>;
impl BK2F_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BK2F_A {
        match self.bits {
            0 => BK2F_A::B_0X0,
            1 => BK2F_A::B_0X1,
            2 => BK2F_A::B_0X2,
            3 => BK2F_A::B_0X3,
            4 => BK2F_A::B_0X4,
            5 => BK2F_A::B_0X5,
            6 => BK2F_A::B_0X6,
            7 => BK2F_A::B_0X7,
            8 => BK2F_A::B_0X8,
            9 => BK2F_A::B_0X9,
            10 => BK2F_A::B_0XA,
            11 => BK2F_A::B_0XB,
            12 => BK2F_A::B_0XC,
            13 => BK2F_A::B_0XD,
            14 => BK2F_A::B_0XE,
            15 => BK2F_A::B_0XF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BK2F_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BK2F_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == BK2F_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == BK2F_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == BK2F_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == BK2F_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == BK2F_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == BK2F_A::B_0X7
    }
    #[doc = "Checks if the value of the field is `B_0X8`"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == BK2F_A::B_0X8
    }
    #[doc = "Checks if the value of the field is `B_0X9`"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == BK2F_A::B_0X9
    }
    #[doc = "Checks if the value of the field is `B_0XA`"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == BK2F_A::B_0XA
    }
    #[doc = "Checks if the value of the field is `B_0XB`"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == BK2F_A::B_0XB
    }
    #[doc = "Checks if the value of the field is `B_0XC`"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == BK2F_A::B_0XC
    }
    #[doc = "Checks if the value of the field is `B_0XD`"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == BK2F_A::B_0XD
    }
    #[doc = "Checks if the value of the field is `B_0XE`"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == BK2F_A::B_0XE
    }
    #[doc = "Checks if the value of the field is `B_0XF`"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == BK2F_A::B_0XF
    }
}
#[doc = "Write proxy for field `BK2F`"]
pub struct BK2F_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BK2F_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No filter, BRK2 acts asynchronously"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BK2F_A::B_0X0)
    }
    #[doc = "fSAMPLING=fCK_INT, N=2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BK2F_A::B_0X1)
    }
    #[doc = "fSAMPLING=fCK_INT, N=4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(BK2F_A::B_0X2)
    }
    #[doc = "fSAMPLING=fCK_INT, N=8"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(BK2F_A::B_0X3)
    }
    #[doc = "fSAMPLING=fDTS/2, N=6"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(BK2F_A::B_0X4)
    }
    #[doc = "fSAMPLING=fDTS/2, N=8"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(BK2F_A::B_0X5)
    }
    #[doc = "fSAMPLING=fDTS/4, N=6"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(BK2F_A::B_0X6)
    }
    #[doc = "fSAMPLING=fDTS/4, N=8"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(BK2F_A::B_0X7)
    }
    #[doc = "fSAMPLING=fDTS/8, N=6"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut W {
        self.variant(BK2F_A::B_0X8)
    }
    #[doc = "fSAMPLING=fDTS/8, N=8"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut W {
        self.variant(BK2F_A::B_0X9)
    }
    #[doc = "fSAMPLING=fDTS/16, N=5"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut W {
        self.variant(BK2F_A::B_0XA)
    }
    #[doc = "fSAMPLING=fDTS/16, N=6"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut W {
        self.variant(BK2F_A::B_0XB)
    }
    #[doc = "fSAMPLING=fDTS/16, N=8"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut W {
        self.variant(BK2F_A::B_0XC)
    }
    #[doc = "fSAMPLING=fDTS/32, N=5"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut W {
        self.variant(BK2F_A::B_0XD)
    }
    #[doc = "fSAMPLING=fDTS/32, N=6"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut W {
        self.variant(BK2F_A::B_0XE)
    }
    #[doc = "fSAMPLING=fDTS/32, N=8"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut W {
        self.variant(BK2F_A::B_0XF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Break 2 enable Note: The BRK2 must only be used with OSSR = OSSI = 1. Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BK2E_A {
    #[doc = "0: Break input BRK2 disabled"]
    B_0X0 = 0,
    #[doc = "1: Break input BRK2 enabled"]
    B_0X1 = 1,
}
impl From<BK2E_A> for bool {
    #[inline(always)]
    fn from(variant: BK2E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BK2E`"]
pub type BK2E_R = crate::R<bool, BK2E_A>;
impl BK2E_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BK2E_A {
        match self.bits {
            false => BK2E_A::B_0X0,
            true => BK2E_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BK2E_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BK2E_A::B_0X1
    }
}
#[doc = "Write proxy for field `BK2E`"]
pub struct BK2E_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BK2E_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Break input BRK2 disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BK2E_A::B_0X0)
    }
    #[doc = "Break input BRK2 enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BK2E_A::B_0X1)
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
#[doc = "Break 2 polarity Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BK2P_A {
    #[doc = "0: Break input BRK2 is active low"]
    B_0X0 = 0,
    #[doc = "1: Break input BRK2 is active high"]
    B_0X1 = 1,
}
impl From<BK2P_A> for bool {
    #[inline(always)]
    fn from(variant: BK2P_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BK2P`"]
pub type BK2P_R = crate::R<bool, BK2P_A>;
impl BK2P_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BK2P_A {
        match self.bits {
            false => BK2P_A::B_0X0,
            true => BK2P_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BK2P_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BK2P_A::B_0X1
    }
}
#[doc = "Write proxy for field `BK2P`"]
pub struct BK2P_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BK2P_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Break input BRK2 is active low"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BK2P_A::B_0X0)
    }
    #[doc = "Break input BRK2 is active high"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BK2P_A::B_0X1)
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
#[doc = "Break Disarm This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the fault condition has disappeared. Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKDSRM_A {
    #[doc = "0: Break input BRK is armed"]
    B_0X0 = 0,
    #[doc = "1: Break input BRK is disarmed"]
    B_0X1 = 1,
}
impl From<BKDSRM_A> for bool {
    #[inline(always)]
    fn from(variant: BKDSRM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BKDSRM`"]
pub type BKDSRM_R = crate::R<bool, BKDSRM_A>;
impl BKDSRM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BKDSRM_A {
        match self.bits {
            false => BKDSRM_A::B_0X0,
            true => BKDSRM_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BKDSRM_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BKDSRM_A::B_0X1
    }
}
#[doc = "Write proxy for field `BKDSRM`"]
pub struct BKDSRM_W<'a> {
    w: &'a mut W,
}
impl<'a> BKDSRM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BKDSRM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Break input BRK is armed"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BKDSRM_A::B_0X0)
    }
    #[doc = "Break input BRK is disarmed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BKDSRM_A::B_0X1)
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
#[doc = "Reader of field `BK2DSRM`"]
pub type BK2DSRM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BK2DSRM`"]
pub struct BK2DSRM_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2DSRM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Break Bidirectional In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKBID_A {
    #[doc = "0: Break input BRK in input mode"]
    B_0X0 = 0,
    #[doc = "1: Break input BRK in bidirectional mode"]
    B_0X1 = 1,
}
impl From<BKBID_A> for bool {
    #[inline(always)]
    fn from(variant: BKBID_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BKBID`"]
pub type BKBID_R = crate::R<bool, BKBID_A>;
impl BKBID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BKBID_A {
        match self.bits {
            false => BKBID_A::B_0X0,
            true => BKBID_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BKBID_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BKBID_A::B_0X1
    }
}
#[doc = "Write proxy for field `BKBID`"]
pub struct BKBID_W<'a> {
    w: &'a mut W,
}
impl<'a> BKBID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BKBID_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Break input BRK in input mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BKBID_A::B_0X0)
    }
    #[doc = "Break input BRK in bidirectional mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BKBID_A::B_0X1)
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
#[doc = "Reader of field `BK2BID`"]
pub type BK2BID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BK2BID`"]
pub struct BK2BID_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2BID_W<'a> {
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
impl R {
    #[doc = "Bits 0:7 - Dead-time generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs. DT correspond to this duration. DTG\\[7:5\\]=0xx => DT=DTG\\[7:0\\]x tDTG with tDTG=tDTS. DTG\\[7:5\\]=10x => DT=(64+DTG\\[5:0\\])xtDTG with tDTG=2xtDTS. DTG\\[7:5\\]=110 => DT=(32+DTG\\[4:0\\])xtDTG with tDTG=8xtDTS. DTG\\[7:5\\]=111 => DT=(32+DTG\\[4:0\\])xtDTG with tDTG=16xtDTS. Example if tDTS=125Â ns (8Â MHz), dead-time possible values are: 0 to 15875Â ns by 125Â ns steps, 16Â Î¼s to 31750Â nsÂ by 250Â ns steps, 32Â Î¼s to 63Â Î¼s by 1Â Î¼s steps, 64Â Î¼s to 126Â Î¼s by 2Â Î¼s steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn dtg(&self) -> DTG_R {
        DTG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Lock configuration These bits offer a write protection against software errors. Note: The LOCK bits can be written only once after the reset. Once the TIMx_BDTR register has been written, their content is frozen until the next reset."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Off-state selection for Idle mode This bit is used when MOE=0 due to a break event or by a software write, on channels configured as outputs. See OC/OCN enable description for more details (enable register (TIM1_CCERTIMx_CCER)N/A). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn ossi(&self) -> OSSI_R {
        OSSI_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Off-state selection for Run mode This bit is used when MOE=1 on channels having a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. See OC/OCN enable description for more details (enable register (TIM1_CCERTIMx_CCER)N/A). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn ossr(&self) -> OSSR_R {
        OSSR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Break enable This bit enables the complete break protection (including all sources connected to bk_acth and BKIN sources, as per ). Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bke(&self) -> BKE_R {
        BKE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Break polarity Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Automatic output enable Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn aoe(&self) -> AOE_R {
        AOE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Main output enable This bit is cleared asynchronously by hardware as soon as one of the break inputs is active (BRK or BRK2). It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. In response to a break event or if MOE is written to 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. See OC/OCN enable description for more details (enable register (TIM1_CCERTIMx_CCER)N/A)."]
    #[inline(always)]
    pub fn moe(&self) -> MOE_R {
        MOE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Break filter This bit-field defines the frequency used to sample BRK input and the length of the digital filter applied to BRK. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkf(&self) -> BKF_R {
        BKF_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Break 2 filter This bit-field defines the frequency used to sample BRK2 input and the length of the digital filter applied to BRK2. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bk2f(&self) -> BK2F_R {
        BK2F_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Break 2 enable Note: The BRK2 must only be used with OSSR = OSSI = 1. Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bk2e(&self) -> BK2E_R {
        BK2E_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Break 2 polarity Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bk2p(&self) -> BK2P_R {
        BK2P_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Break Disarm This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the fault condition has disappeared. Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bkdsrm(&self) -> BKDSRM_R {
        BKDSRM_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Break2 Disarm Refer to BKDSRM description"]
    #[inline(always)]
    pub fn bk2dsrm(&self) -> BK2DSRM_R {
        BK2DSRM_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Break Bidirectional In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bkbid(&self) -> BKBID_R {
        BKBID_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Break2 bidirectional Refer to BKBID description"]
    #[inline(always)]
    pub fn bk2bid(&self) -> BK2BID_R {
        BK2BID_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Dead-time generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs. DT correspond to this duration. DTG\\[7:5\\]=0xx => DT=DTG\\[7:0\\]x tDTG with tDTG=tDTS. DTG\\[7:5\\]=10x => DT=(64+DTG\\[5:0\\])xtDTG with tDTG=2xtDTS. DTG\\[7:5\\]=110 => DT=(32+DTG\\[4:0\\])xtDTG with tDTG=8xtDTS. DTG\\[7:5\\]=111 => DT=(32+DTG\\[4:0\\])xtDTG with tDTG=16xtDTS. Example if tDTS=125Â ns (8Â MHz), dead-time possible values are: 0 to 15875Â ns by 125Â ns steps, 16Â Î¼s to 31750Â nsÂ by 250Â ns steps, 32Â Î¼s to 63Â Î¼s by 1Â Î¼s steps, 64Â Î¼s to 126Â Î¼s by 2Â Î¼s steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn dtg(&mut self) -> DTG_W {
        DTG_W { w: self }
    }
    #[doc = "Bits 8:9 - Lock configuration These bits offer a write protection against software errors. Note: The LOCK bits can be written only once after the reset. Once the TIMx_BDTR register has been written, their content is frozen until the next reset."]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    #[doc = "Bit 10 - Off-state selection for Idle mode This bit is used when MOE=0 due to a break event or by a software write, on channels configured as outputs. See OC/OCN enable description for more details (enable register (TIM1_CCERTIMx_CCER)N/A). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn ossi(&mut self) -> OSSI_W {
        OSSI_W { w: self }
    }
    #[doc = "Bit 11 - Off-state selection for Run mode This bit is used when MOE=1 on channels having a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. See OC/OCN enable description for more details (enable register (TIM1_CCERTIMx_CCER)N/A). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn ossr(&mut self) -> OSSR_W {
        OSSR_W { w: self }
    }
    #[doc = "Bit 12 - Break enable This bit enables the complete break protection (including all sources connected to bk_acth and BKIN sources, as per ). Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bke(&mut self) -> BKE_W {
        BKE_W { w: self }
    }
    #[doc = "Bit 13 - Break polarity Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W {
        BKP_W { w: self }
    }
    #[doc = "Bit 14 - Automatic output enable Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn aoe(&mut self) -> AOE_W {
        AOE_W { w: self }
    }
    #[doc = "Bit 15 - Main output enable This bit is cleared asynchronously by hardware as soon as one of the break inputs is active (BRK or BRK2). It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. In response to a break event or if MOE is written to 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. See OC/OCN enable description for more details (enable register (TIM1_CCERTIMx_CCER)N/A)."]
    #[inline(always)]
    pub fn moe(&mut self) -> MOE_W {
        MOE_W { w: self }
    }
    #[doc = "Bits 16:19 - Break filter This bit-field defines the frequency used to sample BRK input and the length of the digital filter applied to BRK. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkf(&mut self) -> BKF_W {
        BKF_W { w: self }
    }
    #[doc = "Bits 20:23 - Break 2 filter This bit-field defines the frequency used to sample BRK2 input and the length of the digital filter applied to BRK2. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bk2f(&mut self) -> BK2F_W {
        BK2F_W { w: self }
    }
    #[doc = "Bit 24 - Break 2 enable Note: The BRK2 must only be used with OSSR = OSSI = 1. Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bk2e(&mut self) -> BK2E_W {
        BK2E_W { w: self }
    }
    #[doc = "Bit 25 - Break 2 polarity Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bk2p(&mut self) -> BK2P_W {
        BK2P_W { w: self }
    }
    #[doc = "Bit 26 - Break Disarm This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the fault condition has disappeared. Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bkdsrm(&mut self) -> BKDSRM_W {
        BKDSRM_W { w: self }
    }
    #[doc = "Bit 27 - Break2 Disarm Refer to BKDSRM description"]
    #[inline(always)]
    pub fn bk2dsrm(&mut self) -> BK2DSRM_W {
        BK2DSRM_W { w: self }
    }
    #[doc = "Bit 28 - Break Bidirectional In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bkbid(&mut self) -> BKBID_W {
        BKBID_W { w: self }
    }
    #[doc = "Bit 29 - Break2 bidirectional Refer to BKBID description"]
    #[inline(always)]
    pub fn bk2bid(&mut self) -> BK2BID_W {
        BK2BID_W { w: self }
    }
}
