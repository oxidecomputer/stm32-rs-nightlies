#[doc = "Reader of register ADC_ISR"]
pub type R = crate::R<u32, super::ADC_ISR>;
#[doc = "Writer for register ADC_ISR"]
pub type W = crate::W<u32, super::ADC_ISR>;
#[doc = "Register ADC_ISR `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_ISR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ADC ready This bit is set by hardware after the ADC has been enabled (ADENÂ =Â 1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADRDY_A {
    #[doc = "0: ADC not yet ready to start conversion (or the flag event was already acknowledged and cleared by software)"]
    B_0X0 = 0,
    #[doc = "1: ADC is ready to start conversion"]
    B_0X1 = 1,
}
impl From<ADRDY_A> for bool {
    #[inline(always)]
    fn from(variant: ADRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADRDY`"]
pub type ADRDY_R = crate::R<bool, ADRDY_A>;
impl ADRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADRDY_A {
        match self.bits {
            false => ADRDY_A::B_0X0,
            true => ADRDY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ADRDY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ADRDY_A::B_0X1
    }
}
#[doc = "Write proxy for field `ADRDY`"]
pub struct ADRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> ADRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADRDY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC not yet ready to start conversion (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ADRDY_A::B_0X0)
    }
    #[doc = "ADC is ready to start conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ADRDY_A::B_0X1)
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
#[doc = "End of sampling flag This bit is set by hardware during the conversion, at the end of the sampling phase.It is cleared by software by programming it to '1â\u{80}\u{99}.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOSMP_A {
    #[doc = "0: Not at the end of the sampling phase (or the flag event was already acknowledged and cleared by software)"]
    B_0X0 = 0,
    #[doc = "1: End of sampling phase reached"]
    B_0X1 = 1,
}
impl From<EOSMP_A> for bool {
    #[inline(always)]
    fn from(variant: EOSMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EOSMP`"]
pub type EOSMP_R = crate::R<bool, EOSMP_A>;
impl EOSMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOSMP_A {
        match self.bits {
            false => EOSMP_A::B_0X0,
            true => EOSMP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EOSMP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EOSMP_A::B_0X1
    }
}
#[doc = "Write proxy for field `EOSMP`"]
pub struct EOSMP_W<'a> {
    w: &'a mut W,
}
impl<'a> EOSMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOSMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not at the end of the sampling phase (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EOSMP_A::B_0X0)
    }
    #[doc = "End of sampling phase reached"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EOSMP_A::B_0X1)
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
#[doc = "End of conversion flag This bit is set by hardware at the end of each conversion of a channel when a new data result is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOC_A {
    #[doc = "0: Channel conversion not complete (or the flag event was already acknowledged and cleared by software)"]
    B_0X0 = 0,
    #[doc = "1: Channel conversion complete"]
    B_0X1 = 1,
}
impl From<EOC_A> for bool {
    #[inline(always)]
    fn from(variant: EOC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EOC`"]
pub type EOC_R = crate::R<bool, EOC_A>;
impl EOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOC_A {
        match self.bits {
            false => EOC_A::B_0X0,
            true => EOC_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EOC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EOC_A::B_0X1
    }
}
#[doc = "Write proxy for field `EOC`"]
pub struct EOC_W<'a> {
    w: &'a mut W,
}
impl<'a> EOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel conversion not complete (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EOC_A::B_0X0)
    }
    #[doc = "Channel conversion complete"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EOC_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "End of sequence flag This bit is set by hardware at the end of the conversion of a sequence of channels selected by the CHSEL bits. It is cleared by software writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOS_A {
    #[doc = "0: Conversion sequence not complete (or the flag event was already acknowledged and cleared by software)"]
    B_0X0 = 0,
    #[doc = "1: Conversion sequence complete"]
    B_0X1 = 1,
}
impl From<EOS_A> for bool {
    #[inline(always)]
    fn from(variant: EOS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EOS`"]
pub type EOS_R = crate::R<bool, EOS_A>;
impl EOS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOS_A {
        match self.bits {
            false => EOS_A::B_0X0,
            true => EOS_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EOS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EOS_A::B_0X1
    }
}
#[doc = "Write proxy for field `EOS`"]
pub struct EOS_W<'a> {
    w: &'a mut W,
}
impl<'a> EOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Conversion sequence not complete (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EOS_A::B_0X0)
    }
    #[doc = "Conversion sequence complete"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EOS_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "ADC overrun This bit is set by hardware when an overrun occurs, meaning that a new conversion has complete while the EOC flag was already set. It is cleared by software writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVR_A {
    #[doc = "0: No overrun occurred (or the flag event was already acknowledged and cleared by software)"]
    B_0X0 = 0,
    #[doc = "1: Overrun has occurred"]
    B_0X1 = 1,
}
impl From<OVR_A> for bool {
    #[inline(always)]
    fn from(variant: OVR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OVR`"]
pub type OVR_R = crate::R<bool, OVR_A>;
impl OVR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVR_A {
        match self.bits {
            false => OVR_A::B_0X0,
            true => OVR_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OVR_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OVR_A::B_0X1
    }
}
#[doc = "Write proxy for field `OVR`"]
pub struct OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No overrun occurred (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(OVR_A::B_0X0)
    }
    #[doc = "Overrun has occurred"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(OVR_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_TR1 and ADC_HR1 registers. It is cleared by software by programming it to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWD1_A {
    #[doc = "0: No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)"]
    B_0X0 = 0,
    #[doc = "1: Analog watchdog event occurred"]
    B_0X1 = 1,
}
impl From<AWD1_A> for bool {
    #[inline(always)]
    fn from(variant: AWD1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AWD1`"]
pub type AWD1_R = crate::R<bool, AWD1_A>;
impl AWD1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWD1_A {
        match self.bits {
            false => AWD1_A::B_0X0,
            true => AWD1_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD1_A::B_0X1
    }
}
#[doc = "Write proxy for field `AWD1`"]
pub struct AWD1_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(AWD1_A::B_0X0)
    }
    #[doc = "Analog watchdog event occurred"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(AWD1_A::B_0X1)
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
#[doc = "Analog watchdog 2 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD2TR and ADC_AWD2TR registers. It is cleared by software programming it it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWD2_A {
    #[doc = "0: No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)"]
    B_0X0 = 0,
    #[doc = "1: Analog watchdog event occurred"]
    B_0X1 = 1,
}
impl From<AWD2_A> for bool {
    #[inline(always)]
    fn from(variant: AWD2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AWD2`"]
pub type AWD2_R = crate::R<bool, AWD2_A>;
impl AWD2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWD2_A {
        match self.bits {
            false => AWD2_A::B_0X0,
            true => AWD2_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD2_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD2_A::B_0X1
    }
}
#[doc = "Write proxy for field `AWD2`"]
pub struct AWD2_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(AWD2_A::B_0X0)
    }
    #[doc = "Analog watchdog event occurred"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(AWD2_A::B_0X1)
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
#[doc = "Analog watchdog 3 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD3TR and ADC_AWD3TR registers. It is cleared by software by programming it to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWD3_A {
    #[doc = "0: No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)"]
    B_0X0 = 0,
    #[doc = "1: Analog watchdog event occurred"]
    B_0X1 = 1,
}
impl From<AWD3_A> for bool {
    #[inline(always)]
    fn from(variant: AWD3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AWD3`"]
pub type AWD3_R = crate::R<bool, AWD3_A>;
impl AWD3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWD3_A {
        match self.bits {
            false => AWD3_A::B_0X0,
            true => AWD3_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD3_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD3_A::B_0X1
    }
}
#[doc = "Write proxy for field `AWD3`"]
pub struct AWD3_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(AWD3_A::B_0X0)
    }
    #[doc = "Analog watchdog event occurred"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(AWD3_A::B_0X1)
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
#[doc = "End Of Calibration flag This bit is set by hardware when calibration is complete. It is cleared by software writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOCAL_A {
    #[doc = "0: Calibration is not complete"]
    B_0X0 = 0,
    #[doc = "1: Calibration is complete"]
    B_0X1 = 1,
}
impl From<EOCAL_A> for bool {
    #[inline(always)]
    fn from(variant: EOCAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EOCAL`"]
pub type EOCAL_R = crate::R<bool, EOCAL_A>;
impl EOCAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOCAL_A {
        match self.bits {
            false => EOCAL_A::B_0X0,
            true => EOCAL_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EOCAL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EOCAL_A::B_0X1
    }
}
#[doc = "Write proxy for field `EOCAL`"]
pub struct EOCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> EOCAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOCAL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Calibration is not complete"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EOCAL_A::B_0X0)
    }
    #[doc = "Calibration is complete"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EOCAL_A::B_0X1)
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
#[doc = "Channel Configuration Ready flag This flag bit is set by hardware when the channel configuration is applied after programming to ADC_CHSELR register or changing CHSELRMOD or SCANDIR. It is cleared by software by programming it to it. Note: When the software configures the channels (by programming ADC_CHSELR or changing CHSELRMOD or SCANDIR), it must wait until the CCRDY flag rises before configuring again or starting conversions, otherwise the new configuration (or the START bit) is ignored. Once the flag is asserted, if the software needs to configure again the channels, it must clear the CCRDY flag before proceeding with a new configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCRDY_A {
    #[doc = "0: Channel configuration update not applied. "]
    B_0X0 = 0,
    #[doc = "1: Channel configuration update is applied."]
    B_0X1 = 1,
}
impl From<CCRDY_A> for bool {
    #[inline(always)]
    fn from(variant: CCRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCRDY`"]
pub type CCRDY_R = crate::R<bool, CCRDY_A>;
impl CCRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCRDY_A {
        match self.bits {
            false => CCRDY_A::B_0X0,
            true => CCRDY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CCRDY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CCRDY_A::B_0X1
    }
}
#[doc = "Write proxy for field `CCRDY`"]
pub struct CCRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> CCRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCRDY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel configuration update not applied."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CCRDY_A::B_0X0)
    }
    #[doc = "Channel configuration update is applied."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CCRDY_A::B_0X1)
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
impl R {
    #[doc = "Bit 0 - ADC ready This bit is set by hardware after the ADC has been enabled (ADENÂ =Â 1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn adrdy(&self) -> ADRDY_R {
        ADRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - End of sampling flag This bit is set by hardware during the conversion, at the end of the sampling phase.It is cleared by software by programming it to '1â\u{80}\u{99}."]
    #[inline(always)]
    pub fn eosmp(&self) -> EOSMP_R {
        EOSMP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - End of conversion flag This bit is set by hardware at the end of each conversion of a channel when a new data result is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register."]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - End of sequence flag This bit is set by hardware at the end of the conversion of a sequence of channels selected by the CHSEL bits. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn eos(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC overrun This bit is set by hardware when an overrun occurs, meaning that a new conversion has complete while the EOC flag was already set. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_TR1 and ADC_HR1 registers. It is cleared by software by programming it to 1."]
    #[inline(always)]
    pub fn awd1(&self) -> AWD1_R {
        AWD1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Analog watchdog 2 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD2TR and ADC_AWD2TR registers. It is cleared by software programming it it."]
    #[inline(always)]
    pub fn awd2(&self) -> AWD2_R {
        AWD2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Analog watchdog 3 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD3TR and ADC_AWD3TR registers. It is cleared by software by programming it to 1."]
    #[inline(always)]
    pub fn awd3(&self) -> AWD3_R {
        AWD3_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - End Of Calibration flag This bit is set by hardware when calibration is complete. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn eocal(&self) -> EOCAL_R {
        EOCAL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Channel Configuration Ready flag This flag bit is set by hardware when the channel configuration is applied after programming to ADC_CHSELR register or changing CHSELRMOD or SCANDIR. It is cleared by software by programming it to it. Note: When the software configures the channels (by programming ADC_CHSELR or changing CHSELRMOD or SCANDIR), it must wait until the CCRDY flag rises before configuring again or starting conversions, otherwise the new configuration (or the START bit) is ignored. Once the flag is asserted, if the software needs to configure again the channels, it must clear the CCRDY flag before proceeding with a new configuration."]
    #[inline(always)]
    pub fn ccrdy(&self) -> CCRDY_R {
        CCRDY_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC ready This bit is set by hardware after the ADC has been enabled (ADENÂ =Â 1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn adrdy(&mut self) -> ADRDY_W {
        ADRDY_W { w: self }
    }
    #[doc = "Bit 1 - End of sampling flag This bit is set by hardware during the conversion, at the end of the sampling phase.It is cleared by software by programming it to '1â\u{80}\u{99}."]
    #[inline(always)]
    pub fn eosmp(&mut self) -> EOSMP_W {
        EOSMP_W { w: self }
    }
    #[doc = "Bit 2 - End of conversion flag This bit is set by hardware at the end of each conversion of a channel when a new data result is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register."]
    #[inline(always)]
    pub fn eoc(&mut self) -> EOC_W {
        EOC_W { w: self }
    }
    #[doc = "Bit 3 - End of sequence flag This bit is set by hardware at the end of the conversion of a sequence of channels selected by the CHSEL bits. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn eos(&mut self) -> EOS_W {
        EOS_W { w: self }
    }
    #[doc = "Bit 4 - ADC overrun This bit is set by hardware when an overrun occurs, meaning that a new conversion has complete while the EOC flag was already set. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn ovr(&mut self) -> OVR_W {
        OVR_W { w: self }
    }
    #[doc = "Bit 7 - Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_TR1 and ADC_HR1 registers. It is cleared by software by programming it to 1."]
    #[inline(always)]
    pub fn awd1(&mut self) -> AWD1_W {
        AWD1_W { w: self }
    }
    #[doc = "Bit 8 - Analog watchdog 2 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD2TR and ADC_AWD2TR registers. It is cleared by software programming it it."]
    #[inline(always)]
    pub fn awd2(&mut self) -> AWD2_W {
        AWD2_W { w: self }
    }
    #[doc = "Bit 9 - Analog watchdog 3 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD3TR and ADC_AWD3TR registers. It is cleared by software by programming it to 1."]
    #[inline(always)]
    pub fn awd3(&mut self) -> AWD3_W {
        AWD3_W { w: self }
    }
    #[doc = "Bit 11 - End Of Calibration flag This bit is set by hardware when calibration is complete. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn eocal(&mut self) -> EOCAL_W {
        EOCAL_W { w: self }
    }
    #[doc = "Bit 13 - Channel Configuration Ready flag This flag bit is set by hardware when the channel configuration is applied after programming to ADC_CHSELR register or changing CHSELRMOD or SCANDIR. It is cleared by software by programming it to it. Note: When the software configures the channels (by programming ADC_CHSELR or changing CHSELRMOD or SCANDIR), it must wait until the CCRDY flag rises before configuring again or starting conversions, otherwise the new configuration (or the START bit) is ignored. Once the flag is asserted, if the software needs to configure again the channels, it must clear the CCRDY flag before proceeding with a new configuration."]
    #[inline(always)]
    pub fn ccrdy(&mut self) -> CCRDY_W {
        CCRDY_W { w: self }
    }
}
