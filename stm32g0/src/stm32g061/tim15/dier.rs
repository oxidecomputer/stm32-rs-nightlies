#[doc = "Reader of register DIER"]
pub type R = crate::R<u32, super::DIER>;
#[doc = "Writer for register DIER"]
pub type W = crate::W<u32, super::DIER>;
#[doc = "Register DIER `reset()`'s with value 0"]
impl crate::ResetValue for super::DIER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Update interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UIE_A {
    #[doc = "0: Update interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: Update interrupt enabled"]
    B_0X1 = 1,
}
impl From<UIE_A> for bool {
    #[inline(always)]
    fn from(variant: UIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UIE`"]
pub type UIE_R = crate::R<bool, UIE_A>;
impl UIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UIE_A {
        match self.bits {
            false => UIE_A::B_0X0,
            true => UIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == UIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `UIE`"]
pub struct UIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Update interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(UIE_A::B_0X0)
    }
    #[doc = "Update interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(UIE_A::B_0X1)
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
#[doc = "Capture/Compare 1 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1IE_A {
    #[doc = "0: CC1 interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: CC1 interrupt enabled"]
    B_0X1 = 1,
}
impl From<CC1IE_A> for bool {
    #[inline(always)]
    fn from(variant: CC1IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CC1IE`"]
pub type CC1IE_R = crate::R<bool, CC1IE_A>;
impl CC1IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC1IE_A {
        match self.bits {
            false => CC1IE_A::B_0X0,
            true => CC1IE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC1IE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC1IE_A::B_0X1
    }
}
#[doc = "Write proxy for field `CC1IE`"]
pub struct CC1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC1IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CC1 interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CC1IE_A::B_0X0)
    }
    #[doc = "CC1 interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CC1IE_A::B_0X1)
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
#[doc = "Capture/Compare 2 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC2IE_A {
    #[doc = "0: CC2 interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: CC2 interrupt enabled"]
    B_0X1 = 1,
}
impl From<CC2IE_A> for bool {
    #[inline(always)]
    fn from(variant: CC2IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CC2IE`"]
pub type CC2IE_R = crate::R<bool, CC2IE_A>;
impl CC2IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC2IE_A {
        match self.bits {
            false => CC2IE_A::B_0X0,
            true => CC2IE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC2IE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC2IE_A::B_0X1
    }
}
#[doc = "Write proxy for field `CC2IE`"]
pub struct CC2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC2IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CC2 interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CC2IE_A::B_0X0)
    }
    #[doc = "CC2 interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CC2IE_A::B_0X1)
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
#[doc = "COM interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMIE_A {
    #[doc = "0: COM interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: COM interrupt enabled"]
    B_0X1 = 1,
}
impl From<COMIE_A> for bool {
    #[inline(always)]
    fn from(variant: COMIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMIE`"]
pub type COMIE_R = crate::R<bool, COMIE_A>;
impl COMIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMIE_A {
        match self.bits {
            false => COMIE_A::B_0X0,
            true => COMIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == COMIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == COMIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `COMIE`"]
pub struct COMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "COM interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(COMIE_A::B_0X0)
    }
    #[doc = "COM interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(COMIE_A::B_0X1)
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
#[doc = "Trigger interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIE_A {
    #[doc = "0: Trigger interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: Trigger interrupt enabled"]
    B_0X1 = 1,
}
impl From<TIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIE`"]
pub type TIE_R = crate::R<bool, TIE_A>;
impl TIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIE_A {
        match self.bits {
            false => TIE_A::B_0X0,
            true => TIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `TIE`"]
pub struct TIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIE_A::B_0X0)
    }
    #[doc = "Trigger interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIE_A::B_0X1)
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
#[doc = "Break interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIE_A {
    #[doc = "0: Break interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: Break interrupt enabled"]
    B_0X1 = 1,
}
impl From<BIE_A> for bool {
    #[inline(always)]
    fn from(variant: BIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BIE`"]
pub type BIE_R = crate::R<bool, BIE_A>;
impl BIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIE_A {
        match self.bits {
            false => BIE_A::B_0X0,
            true => BIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `BIE`"]
pub struct BIE_W<'a> {
    w: &'a mut W,
}
impl<'a> BIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Break interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BIE_A::B_0X0)
    }
    #[doc = "Break interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BIE_A::B_0X1)
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
#[doc = "Update DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UDE_A {
    #[doc = "0: Update DMA request disabled"]
    B_0X0 = 0,
    #[doc = "1: Update DMA request enabled"]
    B_0X1 = 1,
}
impl From<UDE_A> for bool {
    #[inline(always)]
    fn from(variant: UDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UDE`"]
pub type UDE_R = crate::R<bool, UDE_A>;
impl UDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UDE_A {
        match self.bits {
            false => UDE_A::B_0X0,
            true => UDE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UDE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == UDE_A::B_0X1
    }
}
#[doc = "Write proxy for field `UDE`"]
pub struct UDE_W<'a> {
    w: &'a mut W,
}
impl<'a> UDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Update DMA request disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(UDE_A::B_0X0)
    }
    #[doc = "Update DMA request enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(UDE_A::B_0X1)
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
#[doc = "Capture/Compare 1 DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1DE_A {
    #[doc = "0: CC1 DMA request disabled"]
    B_0X0 = 0,
    #[doc = "1: CC1 DMA request enabled"]
    B_0X1 = 1,
}
impl From<CC1DE_A> for bool {
    #[inline(always)]
    fn from(variant: CC1DE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CC1DE`"]
pub type CC1DE_R = crate::R<bool, CC1DE_A>;
impl CC1DE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC1DE_A {
        match self.bits {
            false => CC1DE_A::B_0X0,
            true => CC1DE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC1DE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC1DE_A::B_0X1
    }
}
#[doc = "Write proxy for field `CC1DE`"]
pub struct CC1DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1DE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC1DE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CC1 DMA request disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CC1DE_A::B_0X0)
    }
    #[doc = "CC1 DMA request enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CC1DE_A::B_0X1)
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
#[doc = "Capture/Compare 2 DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC2DE_A {
    #[doc = "0: CC2 DMA request disabled"]
    B_0X0 = 0,
    #[doc = "1: CC2 DMA request enabled"]
    B_0X1 = 1,
}
impl From<CC2DE_A> for bool {
    #[inline(always)]
    fn from(variant: CC2DE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CC2DE`"]
pub type CC2DE_R = crate::R<bool, CC2DE_A>;
impl CC2DE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC2DE_A {
        match self.bits {
            false => CC2DE_A::B_0X0,
            true => CC2DE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC2DE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC2DE_A::B_0X1
    }
}
#[doc = "Write proxy for field `CC2DE`"]
pub struct CC2DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2DE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC2DE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CC2 DMA request disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CC2DE_A::B_0X0)
    }
    #[doc = "CC2 DMA request enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CC2DE_A::B_0X1)
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
#[doc = "COM DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMDE_A {
    #[doc = "0: COM DMA request disabled"]
    B_0X0 = 0,
    #[doc = "1: COM DMA request enabled"]
    B_0X1 = 1,
}
impl From<COMDE_A> for bool {
    #[inline(always)]
    fn from(variant: COMDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMDE`"]
pub type COMDE_R = crate::R<bool, COMDE_A>;
impl COMDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMDE_A {
        match self.bits {
            false => COMDE_A::B_0X0,
            true => COMDE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == COMDE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == COMDE_A::B_0X1
    }
}
#[doc = "Write proxy for field `COMDE`"]
pub struct COMDE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "COM DMA request disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(COMDE_A::B_0X0)
    }
    #[doc = "COM DMA request enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(COMDE_A::B_0X1)
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
#[doc = "Trigger DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDE_A {
    #[doc = "0: Trigger DMA request disabled"]
    B_0X0 = 0,
    #[doc = "1: Trigger DMA request enabled"]
    B_0X1 = 1,
}
impl From<TDE_A> for bool {
    #[inline(always)]
    fn from(variant: TDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TDE`"]
pub type TDE_R = crate::R<bool, TDE_A>;
impl TDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDE_A {
        match self.bits {
            false => TDE_A::B_0X0,
            true => TDE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TDE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TDE_A::B_0X1
    }
}
#[doc = "Write proxy for field `TDE`"]
pub struct TDE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger DMA request disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TDE_A::B_0X0)
    }
    #[doc = "Trigger DMA request enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TDE_A::B_0X1)
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
impl R {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt enable"]
    #[inline(always)]
    pub fn cc1ie(&self) -> CC1IE_R {
        CC1IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt enable"]
    #[inline(always)]
    pub fn cc2ie(&self) -> CC2IE_R {
        CC2IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - COM interrupt enable"]
    #[inline(always)]
    pub fn comie(&self) -> COMIE_R {
        COMIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline(always)]
    pub fn bie(&self) -> BIE_R {
        BIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn ude(&self) -> UDE_R {
        UDE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 1 DMA request enable"]
    #[inline(always)]
    pub fn cc1de(&self) -> CC1DE_R {
        CC1DE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Capture/Compare 2 DMA request enable"]
    #[inline(always)]
    pub fn cc2de(&self) -> CC2DE_R {
        CC2DE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 13 - COM DMA request enable"]
    #[inline(always)]
    pub fn comde(&self) -> COMDE_R {
        COMDE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    pub fn tde(&self) -> TDE_R {
        TDE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn uie(&mut self) -> UIE_W {
        UIE_W { w: self }
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt enable"]
    #[inline(always)]
    pub fn cc1ie(&mut self) -> CC1IE_W {
        CC1IE_W { w: self }
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt enable"]
    #[inline(always)]
    pub fn cc2ie(&mut self) -> CC2IE_W {
        CC2IE_W { w: self }
    }
    #[doc = "Bit 5 - COM interrupt enable"]
    #[inline(always)]
    pub fn comie(&mut self) -> COMIE_W {
        COMIE_W { w: self }
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W {
        TIE_W { w: self }
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline(always)]
    pub fn bie(&mut self) -> BIE_W {
        BIE_W { w: self }
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn ude(&mut self) -> UDE_W {
        UDE_W { w: self }
    }
    #[doc = "Bit 9 - Capture/Compare 1 DMA request enable"]
    #[inline(always)]
    pub fn cc1de(&mut self) -> CC1DE_W {
        CC1DE_W { w: self }
    }
    #[doc = "Bit 10 - Capture/Compare 2 DMA request enable"]
    #[inline(always)]
    pub fn cc2de(&mut self) -> CC2DE_W {
        CC2DE_W { w: self }
    }
    #[doc = "Bit 13 - COM DMA request enable"]
    #[inline(always)]
    pub fn comde(&mut self) -> COMDE_W {
        COMDE_W { w: self }
    }
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    pub fn tde(&mut self) -> TDE_W {
        TDE_W { w: self }
    }
}