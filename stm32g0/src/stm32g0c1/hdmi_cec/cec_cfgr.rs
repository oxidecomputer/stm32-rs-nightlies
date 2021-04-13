#[doc = "Reader of register CEC_CFGR"]
pub type R = crate::R<u32, super::CEC_CFGR>;
#[doc = "Writer for register CEC_CFGR"]
pub type W = crate::W<u32, super::CEC_CFGR>;
#[doc = "Register CEC_CFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::CEC_CFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Signal free time SFT bits are set by software. In the SFT = 0x0 configuration, the number of nominal data bit periods waited before transmission is ruled by hardware according to the transmission history. In all the other configurations the SFT number is determined by software. 0x0 2.5 data-bit periods if CEC is the last bus initiator with unsuccessful transmission (ARBLST = 1, TXERR = 1, TXUDR = 1 or TXACKE = 1) 4 data-bit periods if CEC is the new bus initiator 6 data-bit periods if CEC is the last bus initiator with successful transmission (TXEOM = 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SFT_A {
    #[doc = "1: 0.5 nominal data bit periods"]
    B_0X1 = 1,
    #[doc = "2: 1.5 nominal data bit periods"]
    B_0X2 = 2,
    #[doc = "3: 2.5 nominal data bit periods"]
    B_0X3 = 3,
    #[doc = "4: 3.5 nominal data bit periods"]
    B_0X4 = 4,
    #[doc = "5: 4.5 nominal data bit periods"]
    B_0X5 = 5,
    #[doc = "6: 5.5 nominal data bit periods"]
    B_0X6 = 6,
    #[doc = "7: 6.5 nominal data bit periods"]
    B_0X7 = 7,
}
impl From<SFT_A> for u8 {
    #[inline(always)]
    fn from(variant: SFT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SFT`"]
pub type SFT_R = crate::R<u8, SFT_A>;
impl SFT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SFT_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(SFT_A::B_0X1),
            2 => Val(SFT_A::B_0X2),
            3 => Val(SFT_A::B_0X3),
            4 => Val(SFT_A::B_0X4),
            5 => Val(SFT_A::B_0X5),
            6 => Val(SFT_A::B_0X6),
            7 => Val(SFT_A::B_0X7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SFT_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == SFT_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == SFT_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == SFT_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == SFT_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == SFT_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == SFT_A::B_0X7
    }
}
#[doc = "Write proxy for field `SFT`"]
pub struct SFT_W<'a> {
    w: &'a mut W,
}
impl<'a> SFT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SFT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "0.5 nominal data bit periods"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SFT_A::B_0X1)
    }
    #[doc = "1.5 nominal data bit periods"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(SFT_A::B_0X2)
    }
    #[doc = "2.5 nominal data bit periods"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(SFT_A::B_0X3)
    }
    #[doc = "3.5 nominal data bit periods"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(SFT_A::B_0X4)
    }
    #[doc = "4.5 nominal data bit periods"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(SFT_A::B_0X5)
    }
    #[doc = "5.5 nominal data bit periods"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(SFT_A::B_0X6)
    }
    #[doc = "6.5 nominal data bit periods"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(SFT_A::B_0X7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Rx-tolerance The RXTOL bit is set and cleared by software. Start-bit, +/- 200 µs rise, +/- 200 µs fall Data-bit: +/- 200 µs rise. +/- 350 µs fall Start-bit: +/- 400 µs rise, +/- 400 µs fall Data-bit: +/-300 µs rise, +/- 500 µs fall\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTOL_A {
    #[doc = "0: Standard tolerance margin:"]
    B_0X0 = 0,
    #[doc = "1: Extended tolerance"]
    B_0X1 = 1,
}
impl From<RXTOL_A> for bool {
    #[inline(always)]
    fn from(variant: RXTOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXTOL`"]
pub type RXTOL_R = crate::R<bool, RXTOL_A>;
impl RXTOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXTOL_A {
        match self.bits {
            false => RXTOL_A::B_0X0,
            true => RXTOL_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RXTOL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RXTOL_A::B_0X1
    }
}
#[doc = "Write proxy for field `RXTOL`"]
pub struct RXTOL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXTOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Standard tolerance margin:"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RXTOL_A::B_0X0)
    }
    #[doc = "Extended tolerance"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RXTOL_A::B_0X1)
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
#[doc = "Rx-stop on bit rising error The BRESTP bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRESTP_A {
    #[doc = "0: BRE detection does not stop reception of the CEC message. Data bit is sampled at 1.05 ms."]
    B_0X0 = 0,
    #[doc = "1: BRE detection stops message reception."]
    B_0X1 = 1,
}
impl From<BRESTP_A> for bool {
    #[inline(always)]
    fn from(variant: BRESTP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BRESTP`"]
pub type BRESTP_R = crate::R<bool, BRESTP_A>;
impl BRESTP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRESTP_A {
        match self.bits {
            false => BRESTP_A::B_0X0,
            true => BRESTP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BRESTP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BRESTP_A::B_0X1
    }
}
#[doc = "Write proxy for field `BRESTP`"]
pub struct BRESTP_W<'a> {
    w: &'a mut W,
}
impl<'a> BRESTP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRESTP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "BRE detection does not stop reception of the CEC message. Data bit is sampled at 1.05 ms."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BRESTP_A::B_0X0)
    }
    #[doc = "BRE detection stops message reception."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BRESTP_A::B_0X1)
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
#[doc = "Generate error-bit on bit rising error The BREGEN bit is set and cleared by software. Note: If BRDNOGEN = 0, an error-bit is generated upon BRE detection with BRESTP = 1 in broadcast even if BREGEN = 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BREGEN_A {
    #[doc = "0: BRE detection does not generate an error-bit on the CEC line."]
    B_0X0 = 0,
    #[doc = "1: BRE detection generates an error-bit on the CEC line (if BRESTP is set)."]
    B_0X1 = 1,
}
impl From<BREGEN_A> for bool {
    #[inline(always)]
    fn from(variant: BREGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BREGEN`"]
pub type BREGEN_R = crate::R<bool, BREGEN_A>;
impl BREGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BREGEN_A {
        match self.bits {
            false => BREGEN_A::B_0X0,
            true => BREGEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BREGEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BREGEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `BREGEN`"]
pub struct BREGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BREGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BREGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "BRE detection does not generate an error-bit on the CEC line."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BREGEN_A::B_0X0)
    }
    #[doc = "BRE detection generates an error-bit on the CEC line (if BRESTP is set)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BREGEN_A::B_0X1)
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
#[doc = "Generate error-bit on long bit period error The LBPEGEN bit is set and cleared by software. Note: If BRDNOGEN = 0, an error-bit is generated upon LBPE detection in broadcast even if LBPEGEN = 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBPEGEN_A {
    #[doc = "0: LBPE detection does not generate an error-bit on the CEC line."]
    B_0X0 = 0,
    #[doc = "1: LBPE detection generates an error-bit on the CEC line."]
    B_0X1 = 1,
}
impl From<LBPEGEN_A> for bool {
    #[inline(always)]
    fn from(variant: LBPEGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LBPEGEN`"]
pub type LBPEGEN_R = crate::R<bool, LBPEGEN_A>;
impl LBPEGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBPEGEN_A {
        match self.bits {
            false => LBPEGEN_A::B_0X0,
            true => LBPEGEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LBPEGEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LBPEGEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `LBPEGEN`"]
pub struct LBPEGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LBPEGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LBPEGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LBPE detection does not generate an error-bit on the CEC line."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LBPEGEN_A::B_0X0)
    }
    #[doc = "LBPE detection generates an error-bit on the CEC line."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LBPEGEN_A::B_0X1)
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
#[doc = "Avoid error-bit generation in broadcast The BRDNOGEN bit is set and cleared by software. error-bit on the CEC line. LBPE detection with LBPEGEN = 0 on a broadcast message generates an error-bit on the CEC line.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRDNOGEN_A {
    #[doc = "0: BRE detection with BRESTP = 1 and BREGEN = 0 on a broadcast message generates an    "]
    B_0X0 = 0,
    #[doc = "1: Error-bit is not generated in the same condition as above. An error-bit is not generated even in case of an SBPE detection in a broadcast message if listen mode is set."]
    B_0X1 = 1,
}
impl From<BRDNOGEN_A> for bool {
    #[inline(always)]
    fn from(variant: BRDNOGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BRDNOGEN`"]
pub type BRDNOGEN_R = crate::R<bool, BRDNOGEN_A>;
impl BRDNOGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRDNOGEN_A {
        match self.bits {
            false => BRDNOGEN_A::B_0X0,
            true => BRDNOGEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BRDNOGEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BRDNOGEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `BRDNOGEN`"]
pub struct BRDNOGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BRDNOGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRDNOGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "BRE detection with BRESTP = 1 and BREGEN = 0 on a broadcast message generates an"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BRDNOGEN_A::B_0X0)
    }
    #[doc = "Error-bit is not generated in the same condition as above. An error-bit is not generated even in case of an SBPE detection in a broadcast message if listen mode is set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BRDNOGEN_A::B_0X1)
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
#[doc = "SFT option bit The SFTOPT bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SFTOP_A {
    #[doc = "0: SFT timer starts when TXSOM is set by software."]
    B_0X0 = 0,
    #[doc = "1: SFT timer starts automatically at the end of message transmission/reception."]
    B_0X1 = 1,
}
impl From<SFTOP_A> for bool {
    #[inline(always)]
    fn from(variant: SFTOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SFTOP`"]
pub type SFTOP_R = crate::R<bool, SFTOP_A>;
impl SFTOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFTOP_A {
        match self.bits {
            false => SFTOP_A::B_0X0,
            true => SFTOP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SFTOP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SFTOP_A::B_0X1
    }
}
#[doc = "Write proxy for field `SFTOP`"]
pub struct SFTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SFTOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SFTOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SFT timer starts when TXSOM is set by software."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SFTOP_A::B_0X0)
    }
    #[doc = "SFT timer starts automatically at the end of message transmission/reception."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SFTOP_A::B_0X1)
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
#[doc = "Reader of field `OAR`"]
pub type OAR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OAR`"]
pub struct OAR_W<'a> {
    w: &'a mut W,
}
impl<'a> OAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 16)) | (((value as u32) & 0x7fff) << 16);
        self.w
    }
}
#[doc = "Listen mode LSTN bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSTN_A {
    #[doc = "0: CEC peripheral receives only message addressed to its own address (OAR). Messages addressed to different destination are ignored. Broadcast messages are always received."]
    B_0X0 = 0,
    #[doc = "1: CEC peripheral receives messages addressed to its own address (OAR) with positive acknowledge. Messages addressed to different destination are received, but without interfering with the CEC bus: no acknowledge sent."]
    B_0X1 = 1,
}
impl From<LSTN_A> for bool {
    #[inline(always)]
    fn from(variant: LSTN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LSTN`"]
pub type LSTN_R = crate::R<bool, LSTN_A>;
impl LSTN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSTN_A {
        match self.bits {
            false => LSTN_A::B_0X0,
            true => LSTN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSTN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSTN_A::B_0X1
    }
}
#[doc = "Write proxy for field `LSTN`"]
pub struct LSTN_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSTN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CEC peripheral receives only message addressed to its own address (OAR). Messages addressed to different destination are ignored. Broadcast messages are always received."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LSTN_A::B_0X0)
    }
    #[doc = "CEC peripheral receives messages addressed to its own address (OAR) with positive acknowledge. Messages addressed to different destination are received, but without interfering with the CEC bus: no acknowledge sent."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LSTN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Signal free time SFT bits are set by software. In the SFT = 0x0 configuration, the number of nominal data bit periods waited before transmission is ruled by hardware according to the transmission history. In all the other configurations the SFT number is determined by software. 0x0 2.5 data-bit periods if CEC is the last bus initiator with unsuccessful transmission (ARBLST = 1, TXERR = 1, TXUDR = 1 or TXACKE = 1) 4 data-bit periods if CEC is the new bus initiator 6 data-bit periods if CEC is the last bus initiator with successful transmission (TXEOM = 1)"]
    #[inline(always)]
    pub fn sft(&self) -> SFT_R {
        SFT_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Rx-tolerance The RXTOL bit is set and cleared by software. Start-bit, +/- 200 µs rise, +/- 200 µs fall Data-bit: +/- 200 µs rise. +/- 350 µs fall Start-bit: +/- 400 µs rise, +/- 400 µs fall Data-bit: +/-300 µs rise, +/- 500 µs fall"]
    #[inline(always)]
    pub fn rxtol(&self) -> RXTOL_R {
        RXTOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Rx-stop on bit rising error The BRESTP bit is set and cleared by software."]
    #[inline(always)]
    pub fn brestp(&self) -> BRESTP_R {
        BRESTP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Generate error-bit on bit rising error The BREGEN bit is set and cleared by software. Note: If BRDNOGEN = 0, an error-bit is generated upon BRE detection with BRESTP = 1 in broadcast even if BREGEN = 0."]
    #[inline(always)]
    pub fn bregen(&self) -> BREGEN_R {
        BREGEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Generate error-bit on long bit period error The LBPEGEN bit is set and cleared by software. Note: If BRDNOGEN = 0, an error-bit is generated upon LBPE detection in broadcast even if LBPEGEN = 0."]
    #[inline(always)]
    pub fn lbpegen(&self) -> LBPEGEN_R {
        LBPEGEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Avoid error-bit generation in broadcast The BRDNOGEN bit is set and cleared by software. error-bit on the CEC line. LBPE detection with LBPEGEN = 0 on a broadcast message generates an error-bit on the CEC line."]
    #[inline(always)]
    pub fn brdnogen(&self) -> BRDNOGEN_R {
        BRDNOGEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SFT option bit The SFTOPT bit is set and cleared by software."]
    #[inline(always)]
    pub fn sftop(&self) -> SFTOP_R {
        SFTOP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:30 - Own addresses configuration The OAR bits are set by software to select which destination logical addresses has to be considered in receive mode. Each bit, when set, enables the CEC logical address identified by the given bit position. At the end of HEADER reception, the received destination address is compared with the enabled addresses. In case of matching address, the incoming message is acknowledged and received. In case of non-matching address, the incoming message is received only in listen mode (LSTN = 1), but without acknowledge sent. Broadcast messages are always received. Example: OAR = 0b000 0000 0010 0001 means that CEC acknowledges addresses 0x0 and 0x5. Consequently, each message directed to one of these addresses is received."]
    #[inline(always)]
    pub fn oar(&self) -> OAR_R {
        OAR_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Listen mode LSTN bit is set and cleared by software."]
    #[inline(always)]
    pub fn lstn(&self) -> LSTN_R {
        LSTN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Signal free time SFT bits are set by software. In the SFT = 0x0 configuration, the number of nominal data bit periods waited before transmission is ruled by hardware according to the transmission history. In all the other configurations the SFT number is determined by software. 0x0 2.5 data-bit periods if CEC is the last bus initiator with unsuccessful transmission (ARBLST = 1, TXERR = 1, TXUDR = 1 or TXACKE = 1) 4 data-bit periods if CEC is the new bus initiator 6 data-bit periods if CEC is the last bus initiator with successful transmission (TXEOM = 1)"]
    #[inline(always)]
    pub fn sft(&mut self) -> SFT_W {
        SFT_W { w: self }
    }
    #[doc = "Bit 3 - Rx-tolerance The RXTOL bit is set and cleared by software. Start-bit, +/- 200 µs rise, +/- 200 µs fall Data-bit: +/- 200 µs rise. +/- 350 µs fall Start-bit: +/- 400 µs rise, +/- 400 µs fall Data-bit: +/-300 µs rise, +/- 500 µs fall"]
    #[inline(always)]
    pub fn rxtol(&mut self) -> RXTOL_W {
        RXTOL_W { w: self }
    }
    #[doc = "Bit 4 - Rx-stop on bit rising error The BRESTP bit is set and cleared by software."]
    #[inline(always)]
    pub fn brestp(&mut self) -> BRESTP_W {
        BRESTP_W { w: self }
    }
    #[doc = "Bit 5 - Generate error-bit on bit rising error The BREGEN bit is set and cleared by software. Note: If BRDNOGEN = 0, an error-bit is generated upon BRE detection with BRESTP = 1 in broadcast even if BREGEN = 0."]
    #[inline(always)]
    pub fn bregen(&mut self) -> BREGEN_W {
        BREGEN_W { w: self }
    }
    #[doc = "Bit 6 - Generate error-bit on long bit period error The LBPEGEN bit is set and cleared by software. Note: If BRDNOGEN = 0, an error-bit is generated upon LBPE detection in broadcast even if LBPEGEN = 0."]
    #[inline(always)]
    pub fn lbpegen(&mut self) -> LBPEGEN_W {
        LBPEGEN_W { w: self }
    }
    #[doc = "Bit 7 - Avoid error-bit generation in broadcast The BRDNOGEN bit is set and cleared by software. error-bit on the CEC line. LBPE detection with LBPEGEN = 0 on a broadcast message generates an error-bit on the CEC line."]
    #[inline(always)]
    pub fn brdnogen(&mut self) -> BRDNOGEN_W {
        BRDNOGEN_W { w: self }
    }
    #[doc = "Bit 8 - SFT option bit The SFTOPT bit is set and cleared by software."]
    #[inline(always)]
    pub fn sftop(&mut self) -> SFTOP_W {
        SFTOP_W { w: self }
    }
    #[doc = "Bits 16:30 - Own addresses configuration The OAR bits are set by software to select which destination logical addresses has to be considered in receive mode. Each bit, when set, enables the CEC logical address identified by the given bit position. At the end of HEADER reception, the received destination address is compared with the enabled addresses. In case of matching address, the incoming message is acknowledged and received. In case of non-matching address, the incoming message is received only in listen mode (LSTN = 1), but without acknowledge sent. Broadcast messages are always received. Example: OAR = 0b000 0000 0010 0001 means that CEC acknowledges addresses 0x0 and 0x5. Consequently, each message directed to one of these addresses is received."]
    #[inline(always)]
    pub fn oar(&mut self) -> OAR_W {
        OAR_W { w: self }
    }
    #[doc = "Bit 31 - Listen mode LSTN bit is set and cleared by software."]
    #[inline(always)]
    pub fn lstn(&mut self) -> LSTN_W {
        LSTN_W { w: self }
    }
}
