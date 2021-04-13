#[doc = "Reader of register IWDG_PR"]
pub type R = crate::R<u32, super::IWDG_PR>;
#[doc = "Writer for register IWDG_PR"]
pub type W = crate::W<u32, super::IWDG_PR>;
#[doc = "Register IWDG_PR `reset()`'s with value 0"]
impl crate::ResetValue for super::IWDG_PR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Prescaler divider These bits are write access protected see . They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the must be reset in order to be able to change the prescaler divider. Note: Reading this register returns the prescaler value from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the status register (IWDG_SR) is reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PR_A {
    #[doc = "0: divider /4"]
    B_0X0 = 0,
    #[doc = "1: divider /8"]
    B_0X1 = 1,
    #[doc = "2: divider /16"]
    B_0X2 = 2,
    #[doc = "3: divider /32"]
    B_0X3 = 3,
    #[doc = "4: divider /64"]
    B_0X4 = 4,
    #[doc = "5: divider /128"]
    B_0X5 = 5,
    #[doc = "6: divider /256"]
    B_0X6 = 6,
    #[doc = "7: divider /256"]
    B_0X7 = 7,
}
impl From<PR_A> for u8 {
    #[inline(always)]
    fn from(variant: PR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PR`"]
pub type PR_R = crate::R<u8, PR_A>;
impl PR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PR_A {
        match self.bits {
            0 => PR_A::B_0X0,
            1 => PR_A::B_0X1,
            2 => PR_A::B_0X2,
            3 => PR_A::B_0X3,
            4 => PR_A::B_0X4,
            5 => PR_A::B_0X5,
            6 => PR_A::B_0X6,
            7 => PR_A::B_0X7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PR_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PR_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PR_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == PR_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == PR_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == PR_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == PR_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == PR_A::B_0X7
    }
}
#[doc = "Write proxy for field `PR`"]
pub struct PR_W<'a> {
    w: &'a mut W,
}
impl<'a> PR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "divider /4"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PR_A::B_0X0)
    }
    #[doc = "divider /8"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PR_A::B_0X1)
    }
    #[doc = "divider /16"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(PR_A::B_0X2)
    }
    #[doc = "divider /32"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(PR_A::B_0X3)
    }
    #[doc = "divider /64"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(PR_A::B_0X4)
    }
    #[doc = "divider /128"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(PR_A::B_0X5)
    }
    #[doc = "divider /256"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(PR_A::B_0X6)
    }
    #[doc = "divider /256"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(PR_A::B_0X7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Prescaler divider These bits are write access protected see . They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the must be reset in order to be able to change the prescaler divider. Note: Reading this register returns the prescaler value from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the status register (IWDG_SR) is reset."]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Prescaler divider These bits are write access protected see . They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the must be reset in order to be able to change the prescaler divider. Note: Reading this register returns the prescaler value from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the status register (IWDG_SR) is reset."]
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W {
        PR_W { w: self }
    }
}
