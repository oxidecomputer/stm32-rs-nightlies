#[doc = "Reader of register DBG_APB_FZ2"]
pub type R = crate::R<u32, super::DBG_APB_FZ2>;
#[doc = "Writer for register DBG_APB_FZ2"]
pub type W = crate::W<u32, super::DBG_APB_FZ2>;
#[doc = "Register DBG_APB_FZ2 `reset()`'s with value 0"]
impl crate::ResetValue for super::DBG_APB_FZ2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_TIM1_STOP_A {
    #[doc = "0: Enable"]
    B_0X0 = 0,
    #[doc = "1: Disable"]
    B_0X1 = 1,
}
impl From<DBG_TIM1_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM1_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBG_TIM1_STOP`"]
pub type DBG_TIM1_STOP_R = crate::R<bool, DBG_TIM1_STOP_A>;
impl DBG_TIM1_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_TIM1_STOP_A {
        match self.bits {
            false => DBG_TIM1_STOP_A::B_0X0,
            true => DBG_TIM1_STOP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_TIM1_STOP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_TIM1_STOP_A::B_0X1
    }
}
#[doc = "Write proxy for field `DBG_TIM1_STOP`"]
pub struct DBG_TIM1_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM1_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_TIM1_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBG_TIM1_STOP_A::B_0X0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBG_TIM1_STOP_A::B_0X1)
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
#[doc = "Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_TIM14_STOP_A {
    #[doc = "0: Enable"]
    B_0X0 = 0,
    #[doc = "1: Disable"]
    B_0X1 = 1,
}
impl From<DBG_TIM14_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM14_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBG_TIM14_STOP`"]
pub type DBG_TIM14_STOP_R = crate::R<bool, DBG_TIM14_STOP_A>;
impl DBG_TIM14_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_TIM14_STOP_A {
        match self.bits {
            false => DBG_TIM14_STOP_A::B_0X0,
            true => DBG_TIM14_STOP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_TIM14_STOP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_TIM14_STOP_A::B_0X1
    }
}
#[doc = "Write proxy for field `DBG_TIM14_STOP`"]
pub struct DBG_TIM14_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM14_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_TIM14_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBG_TIM14_STOP_A::B_0X0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBG_TIM14_STOP_A::B_0X1)
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
#[doc = "Clocking of TIM15 counter when the core is halted This bit enables/disables the clock to the counter of TIM15 when the core is halted: Only available on STM32G071xx and STM32G081xx, reserved on STM32G031xx and STM32G041xx.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_TIM15_STOP_A {
    #[doc = "0: Enable"]
    B_0X0 = 0,
    #[doc = "1: Disable"]
    B_0X1 = 1,
}
impl From<DBG_TIM15_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM15_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBG_TIM15_STOP`"]
pub type DBG_TIM15_STOP_R = crate::R<bool, DBG_TIM15_STOP_A>;
impl DBG_TIM15_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_TIM15_STOP_A {
        match self.bits {
            false => DBG_TIM15_STOP_A::B_0X0,
            true => DBG_TIM15_STOP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_TIM15_STOP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_TIM15_STOP_A::B_0X1
    }
}
#[doc = "Write proxy for field `DBG_TIM15_STOP`"]
pub struct DBG_TIM15_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM15_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_TIM15_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBG_TIM15_STOP_A::B_0X0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBG_TIM15_STOP_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_TIM16_STOP_A {
    #[doc = "0: Enable"]
    B_0X0 = 0,
    #[doc = "1: Disable"]
    B_0X1 = 1,
}
impl From<DBG_TIM16_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM16_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBG_TIM16_STOP`"]
pub type DBG_TIM16_STOP_R = crate::R<bool, DBG_TIM16_STOP_A>;
impl DBG_TIM16_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_TIM16_STOP_A {
        match self.bits {
            false => DBG_TIM16_STOP_A::B_0X0,
            true => DBG_TIM16_STOP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_TIM16_STOP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_TIM16_STOP_A::B_0X1
    }
}
#[doc = "Write proxy for field `DBG_TIM16_STOP`"]
pub struct DBG_TIM16_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM16_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_TIM16_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBG_TIM16_STOP_A::B_0X0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBG_TIM16_STOP_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_TIM17_STOP_A {
    #[doc = "0: Enable"]
    B_0X0 = 0,
    #[doc = "1: Disable"]
    B_0X1 = 1,
}
impl From<DBG_TIM17_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM17_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBG_TIM17_STOP`"]
pub type DBG_TIM17_STOP_R = crate::R<bool, DBG_TIM17_STOP_A>;
impl DBG_TIM17_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_TIM17_STOP_A {
        match self.bits {
            false => DBG_TIM17_STOP_A::B_0X0,
            true => DBG_TIM17_STOP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_TIM17_STOP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_TIM17_STOP_A::B_0X1
    }
}
#[doc = "Write proxy for field `DBG_TIM17_STOP`"]
pub struct DBG_TIM17_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM17_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_TIM17_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBG_TIM17_STOP_A::B_0X0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBG_TIM17_STOP_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 11 - Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DBG_TIM1_STOP_R {
        DBG_TIM1_STOP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim14_stop(&self) -> DBG_TIM14_STOP_R {
        DBG_TIM14_STOP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Clocking of TIM15 counter when the core is halted This bit enables/disables the clock to the counter of TIM15 when the core is halted: Only available on STM32G071xx and STM32G081xx, reserved on STM32G031xx and STM32G041xx."]
    #[inline(always)]
    pub fn dbg_tim15_stop(&self) -> DBG_TIM15_STOP_R {
        DBG_TIM15_STOP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DBG_TIM16_STOP_R {
        DBG_TIM16_STOP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim17_stop(&self) -> DBG_TIM17_STOP_R {
        DBG_TIM17_STOP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim1_stop(&mut self) -> DBG_TIM1_STOP_W {
        DBG_TIM1_STOP_W { w: self }
    }
    #[doc = "Bit 15 - Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim14_stop(&mut self) -> DBG_TIM14_STOP_W {
        DBG_TIM14_STOP_W { w: self }
    }
    #[doc = "Bit 16 - Clocking of TIM15 counter when the core is halted This bit enables/disables the clock to the counter of TIM15 when the core is halted: Only available on STM32G071xx and STM32G081xx, reserved on STM32G031xx and STM32G041xx."]
    #[inline(always)]
    pub fn dbg_tim15_stop(&mut self) -> DBG_TIM15_STOP_W {
        DBG_TIM15_STOP_W { w: self }
    }
    #[doc = "Bit 17 - Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim16_stop(&mut self) -> DBG_TIM16_STOP_W {
        DBG_TIM16_STOP_W { w: self }
    }
    #[doc = "Bit 18 - Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim17_stop(&mut self) -> DBG_TIM17_STOP_W {
        DBG_TIM17_STOP_W { w: self }
    }
}
