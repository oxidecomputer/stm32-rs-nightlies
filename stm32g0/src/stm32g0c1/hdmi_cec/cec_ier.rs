#[doc = "Reader of register CEC_IER"]
pub type R = crate::R<u32, super::CEC_IER>;
#[doc = "Writer for register CEC_IER"]
pub type W = crate::W<u32, super::CEC_IER>;
#[doc = "Register CEC_IER `reset()`'s with value 0"]
impl crate::ResetValue for super::CEC_IER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Rx-byte received interrupt enable The RXBRIE bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXBRIE_A {
    #[doc = "0: RXBR interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: RXBR interrupt enabled"]
    B_0X1 = 1,
}
impl From<RXBRIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXBRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXBRIE`"]
pub type RXBRIE_R = crate::R<bool, RXBRIE_A>;
impl RXBRIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXBRIE_A {
        match self.bits {
            false => RXBRIE_A::B_0X0,
            true => RXBRIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RXBRIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RXBRIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `RXBRIE`"]
pub struct RXBRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXBRIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RXBR interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RXBRIE_A::B_0X0)
    }
    #[doc = "RXBR interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RXBRIE_A::B_0X1)
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
#[doc = "End of reception interrupt enable The RXENDIE bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXENDIE_A {
    #[doc = "0: RXEND interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: RXEND interrupt enabled"]
    B_0X1 = 1,
}
impl From<RXENDIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXENDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXENDIE`"]
pub type RXENDIE_R = crate::R<bool, RXENDIE_A>;
impl RXENDIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXENDIE_A {
        match self.bits {
            false => RXENDIE_A::B_0X0,
            true => RXENDIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RXENDIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RXENDIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `RXENDIE`"]
pub struct RXENDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXENDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXENDIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RXEND interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RXENDIE_A::B_0X0)
    }
    #[doc = "RXEND interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RXENDIE_A::B_0X1)
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
#[doc = "Rx-buffer overrun interrupt enable The RXOVRIE bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOVRIE_A {
    #[doc = "0: RXOVR interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: RXOVR interrupt enabled"]
    B_0X1 = 1,
}
impl From<RXOVRIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXOVRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXOVRIE`"]
pub type RXOVRIE_R = crate::R<bool, RXOVRIE_A>;
impl RXOVRIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXOVRIE_A {
        match self.bits {
            false => RXOVRIE_A::B_0X0,
            true => RXOVRIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RXOVRIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RXOVRIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `RXOVRIE`"]
pub struct RXOVRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXOVRIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RXOVR interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RXOVRIE_A::B_0X0)
    }
    #[doc = "RXOVR interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RXOVRIE_A::B_0X1)
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
#[doc = "Bit rising error interrupt enable The BREIE bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BREIE_A {
    #[doc = "0: BRE interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: BRE interrupt enabled"]
    B_0X1 = 1,
}
impl From<BREIE_A> for bool {
    #[inline(always)]
    fn from(variant: BREIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BREIE`"]
pub type BREIE_R = crate::R<bool, BREIE_A>;
impl BREIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BREIE_A {
        match self.bits {
            false => BREIE_A::B_0X0,
            true => BREIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BREIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BREIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `BREIE`"]
pub struct BREIE_W<'a> {
    w: &'a mut W,
}
impl<'a> BREIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BREIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "BRE interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BREIE_A::B_0X0)
    }
    #[doc = "BRE interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BREIE_A::B_0X1)
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
#[doc = "Short bit period error interrupt enable The SBPEIE bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBPEIE_A {
    #[doc = "0: SBPE interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: SBPE interrupt enabled"]
    B_0X1 = 1,
}
impl From<SBPEIE_A> for bool {
    #[inline(always)]
    fn from(variant: SBPEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SBPEIE`"]
pub type SBPEIE_R = crate::R<bool, SBPEIE_A>;
impl SBPEIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBPEIE_A {
        match self.bits {
            false => SBPEIE_A::B_0X0,
            true => SBPEIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SBPEIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SBPEIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `SBPEIE`"]
pub struct SBPEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SBPEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SBPEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SBPE interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SBPEIE_A::B_0X0)
    }
    #[doc = "SBPE interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SBPEIE_A::B_0X1)
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
#[doc = "Long bit period error interrupt enable The LBPEIE bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBPEIE_A {
    #[doc = "0: LBPE interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: LBPE interrupt enabled"]
    B_0X1 = 1,
}
impl From<LBPEIE_A> for bool {
    #[inline(always)]
    fn from(variant: LBPEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LBPEIE`"]
pub type LBPEIE_R = crate::R<bool, LBPEIE_A>;
impl LBPEIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBPEIE_A {
        match self.bits {
            false => LBPEIE_A::B_0X0,
            true => LBPEIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LBPEIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LBPEIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `LBPEIE`"]
pub struct LBPEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LBPEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LBPEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LBPE interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LBPEIE_A::B_0X0)
    }
    #[doc = "LBPE interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LBPEIE_A::B_0X1)
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
#[doc = "Rx-missing acknowledge error interrupt enable The RXACKIE bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXACKIE_A {
    #[doc = "0: RXACKE interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: RXACKE interrupt enabled"]
    B_0X1 = 1,
}
impl From<RXACKIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXACKIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXACKIE`"]
pub type RXACKIE_R = crate::R<bool, RXACKIE_A>;
impl RXACKIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXACKIE_A {
        match self.bits {
            false => RXACKIE_A::B_0X0,
            true => RXACKIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RXACKIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RXACKIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `RXACKIE`"]
pub struct RXACKIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXACKIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXACKIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RXACKE interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RXACKIE_A::B_0X0)
    }
    #[doc = "RXACKE interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RXACKIE_A::B_0X1)
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
#[doc = "Arbitration lost interrupt enable The ARBLSTIE bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARBLSTIE_A {
    #[doc = "0: ARBLST interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: ARBLST interrupt enabled"]
    B_0X1 = 1,
}
impl From<ARBLSTIE_A> for bool {
    #[inline(always)]
    fn from(variant: ARBLSTIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ARBLSTIE`"]
pub type ARBLSTIE_R = crate::R<bool, ARBLSTIE_A>;
impl ARBLSTIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARBLSTIE_A {
        match self.bits {
            false => ARBLSTIE_A::B_0X0,
            true => ARBLSTIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ARBLSTIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ARBLSTIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `ARBLSTIE`"]
pub struct ARBLSTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBLSTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARBLSTIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ARBLST interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ARBLSTIE_A::B_0X0)
    }
    #[doc = "ARBLST interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ARBLSTIE_A::B_0X1)
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
#[doc = "Tx-byte request interrupt enable The TXBRIE bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXBRIE_A {
    #[doc = "0: TXBR interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: TXBR interrupt enabled"]
    B_0X1 = 1,
}
impl From<TXBRIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXBRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXBRIE`"]
pub type TXBRIE_R = crate::R<bool, TXBRIE_A>;
impl TXBRIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXBRIE_A {
        match self.bits {
            false => TXBRIE_A::B_0X0,
            true => TXBRIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TXBRIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TXBRIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `TXBRIE`"]
pub struct TXBRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXBRIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TXBR interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TXBRIE_A::B_0X0)
    }
    #[doc = "TXBR interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TXBRIE_A::B_0X1)
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
#[doc = "Tx-end of message interrupt enable The TXENDIE bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXENDIE_A {
    #[doc = "0: TXEND interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: TXEND interrupt enabled"]
    B_0X1 = 1,
}
impl From<TXENDIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXENDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXENDIE`"]
pub type TXENDIE_R = crate::R<bool, TXENDIE_A>;
impl TXENDIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXENDIE_A {
        match self.bits {
            false => TXENDIE_A::B_0X0,
            true => TXENDIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TXENDIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TXENDIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `TXENDIE`"]
pub struct TXENDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXENDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXENDIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TXEND interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TXENDIE_A::B_0X0)
    }
    #[doc = "TXEND interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TXENDIE_A::B_0X1)
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
#[doc = "Tx-underrun interrupt enable The TXUDRIE bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXUDRIE_A {
    #[doc = "0: TXUDR interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: TXUDR interrupt enabled"]
    B_0X1 = 1,
}
impl From<TXUDRIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXUDRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXUDRIE`"]
pub type TXUDRIE_R = crate::R<bool, TXUDRIE_A>;
impl TXUDRIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXUDRIE_A {
        match self.bits {
            false => TXUDRIE_A::B_0X0,
            true => TXUDRIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TXUDRIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TXUDRIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `TXUDRIE`"]
pub struct TXUDRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUDRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXUDRIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TXUDR interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TXUDRIE_A::B_0X0)
    }
    #[doc = "TXUDR interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TXUDRIE_A::B_0X1)
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
#[doc = "Tx-error interrupt enable The TXERRIE bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXERRIE_A {
    #[doc = "0: TXERR interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: TXERR interrupt enabled"]
    B_0X1 = 1,
}
impl From<TXERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXERRIE`"]
pub type TXERRIE_R = crate::R<bool, TXERRIE_A>;
impl TXERRIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXERRIE_A {
        match self.bits {
            false => TXERRIE_A::B_0X0,
            true => TXERRIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TXERRIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TXERRIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `TXERRIE`"]
pub struct TXERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXERRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXERRIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TXERR interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TXERRIE_A::B_0X0)
    }
    #[doc = "TXERR interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TXERRIE_A::B_0X1)
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
#[doc = "Tx-missing acknowledge error interrupt enable The TXACKEIE bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXACKIE_A {
    #[doc = "0: TXACKE interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: TXACKE interrupt enabled"]
    B_0X1 = 1,
}
impl From<TXACKIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXACKIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXACKIE`"]
pub type TXACKIE_R = crate::R<bool, TXACKIE_A>;
impl TXACKIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXACKIE_A {
        match self.bits {
            false => TXACKIE_A::B_0X0,
            true => TXACKIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TXACKIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TXACKIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `TXACKIE`"]
pub struct TXACKIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXACKIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXACKIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TXACKE interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TXACKIE_A::B_0X0)
    }
    #[doc = "TXACKE interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TXACKIE_A::B_0X1)
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
impl R {
    #[doc = "Bit 0 - Rx-byte received interrupt enable The RXBRIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn rxbrie(&self) -> RXBRIE_R {
        RXBRIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - End of reception interrupt enable The RXENDIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn rxendie(&self) -> RXENDIE_R {
        RXENDIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Rx-buffer overrun interrupt enable The RXOVRIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn rxovrie(&self) -> RXOVRIE_R {
        RXOVRIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Bit rising error interrupt enable The BREIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn breie(&self) -> BREIE_R {
        BREIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Short bit period error interrupt enable The SBPEIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn sbpeie(&self) -> SBPEIE_R {
        SBPEIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Long bit period error interrupt enable The LBPEIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn lbpeie(&self) -> LBPEIE_R {
        LBPEIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Rx-missing acknowledge error interrupt enable The RXACKIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn rxackie(&self) -> RXACKIE_R {
        RXACKIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Arbitration lost interrupt enable The ARBLSTIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn arblstie(&self) -> ARBLSTIE_R {
        ARBLSTIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Tx-byte request interrupt enable The TXBRIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn txbrie(&self) -> TXBRIE_R {
        TXBRIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Tx-end of message interrupt enable The TXENDIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn txendie(&self) -> TXENDIE_R {
        TXENDIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Tx-underrun interrupt enable The TXUDRIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn txudrie(&self) -> TXUDRIE_R {
        TXUDRIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Tx-error interrupt enable The TXERRIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn txerrie(&self) -> TXERRIE_R {
        TXERRIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Tx-missing acknowledge error interrupt enable The TXACKEIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn txackie(&self) -> TXACKIE_R {
        TXACKIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx-byte received interrupt enable The RXBRIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn rxbrie(&mut self) -> RXBRIE_W {
        RXBRIE_W { w: self }
    }
    #[doc = "Bit 1 - End of reception interrupt enable The RXENDIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn rxendie(&mut self) -> RXENDIE_W {
        RXENDIE_W { w: self }
    }
    #[doc = "Bit 2 - Rx-buffer overrun interrupt enable The RXOVRIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn rxovrie(&mut self) -> RXOVRIE_W {
        RXOVRIE_W { w: self }
    }
    #[doc = "Bit 3 - Bit rising error interrupt enable The BREIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn breie(&mut self) -> BREIE_W {
        BREIE_W { w: self }
    }
    #[doc = "Bit 4 - Short bit period error interrupt enable The SBPEIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn sbpeie(&mut self) -> SBPEIE_W {
        SBPEIE_W { w: self }
    }
    #[doc = "Bit 5 - Long bit period error interrupt enable The LBPEIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn lbpeie(&mut self) -> LBPEIE_W {
        LBPEIE_W { w: self }
    }
    #[doc = "Bit 6 - Rx-missing acknowledge error interrupt enable The RXACKIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn rxackie(&mut self) -> RXACKIE_W {
        RXACKIE_W { w: self }
    }
    #[doc = "Bit 7 - Arbitration lost interrupt enable The ARBLSTIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn arblstie(&mut self) -> ARBLSTIE_W {
        ARBLSTIE_W { w: self }
    }
    #[doc = "Bit 8 - Tx-byte request interrupt enable The TXBRIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn txbrie(&mut self) -> TXBRIE_W {
        TXBRIE_W { w: self }
    }
    #[doc = "Bit 9 - Tx-end of message interrupt enable The TXENDIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn txendie(&mut self) -> TXENDIE_W {
        TXENDIE_W { w: self }
    }
    #[doc = "Bit 10 - Tx-underrun interrupt enable The TXUDRIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn txudrie(&mut self) -> TXUDRIE_W {
        TXUDRIE_W { w: self }
    }
    #[doc = "Bit 11 - Tx-error interrupt enable The TXERRIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn txerrie(&mut self) -> TXERRIE_W {
        TXERRIE_W { w: self }
    }
    #[doc = "Bit 12 - Tx-missing acknowledge error interrupt enable The TXACKEIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn txackie(&mut self) -> TXACKIE_W {
        TXACKIE_W { w: self }
    }
}
