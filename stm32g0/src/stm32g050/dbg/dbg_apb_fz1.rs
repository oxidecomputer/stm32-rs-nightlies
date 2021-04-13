#[doc = "Reader of register DBG_APB_FZ1"]
pub type R = crate::R<u32, super::DBG_APB_FZ1>;
#[doc = "Writer for register DBG_APB_FZ1"]
pub type W = crate::W<u32, super::DBG_APB_FZ1>;
#[doc = "Register DBG_APB_FZ1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DBG_APB_FZ1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_TIM3_STOP_A {
    #[doc = "0: Enable"]
    B_0X0 = 0,
    #[doc = "1: Disable"]
    B_0X1 = 1,
}
impl From<DBG_TIM3_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM3_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBG_TIM3_STOP`"]
pub type DBG_TIM3_STOP_R = crate::R<bool, DBG_TIM3_STOP_A>;
impl DBG_TIM3_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_TIM3_STOP_A {
        match self.bits {
            false => DBG_TIM3_STOP_A::B_0X0,
            true => DBG_TIM3_STOP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_TIM3_STOP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_TIM3_STOP_A::B_0X1
    }
}
#[doc = "Write proxy for field `DBG_TIM3_STOP`"]
pub struct DBG_TIM3_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM3_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_TIM3_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBG_TIM3_STOP_A::B_0X0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBG_TIM3_STOP_A::B_0X1)
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
#[doc = "Clocking of TIM6 counter when the core is halted This bit enables/disables the clock to the counter of TIM6 when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_TIM6_STOP_A {
    #[doc = "0: Enable"]
    B_0X0 = 0,
    #[doc = "1: Disable"]
    B_0X1 = 1,
}
impl From<DBG_TIM6_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM6_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBG_TIM6_STOP`"]
pub type DBG_TIM6_STOP_R = crate::R<bool, DBG_TIM6_STOP_A>;
impl DBG_TIM6_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_TIM6_STOP_A {
        match self.bits {
            false => DBG_TIM6_STOP_A::B_0X0,
            true => DBG_TIM6_STOP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_TIM6_STOP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_TIM6_STOP_A::B_0X1
    }
}
#[doc = "Write proxy for field `DBG_TIM6_STOP`"]
pub struct DBG_TIM6_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM6_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_TIM6_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBG_TIM6_STOP_A::B_0X0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBG_TIM6_STOP_A::B_0X1)
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
#[doc = "Clocking of TIM7 counter when the core is halted. This bit enables/disables the clock to the counter of ITIM7 when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_TIM7_STOP_A {
    #[doc = "0: Enable"]
    B_0X0 = 0,
    #[doc = "1: Disable"]
    B_0X1 = 1,
}
impl From<DBG_TIM7_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM7_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBG_TIM7_STOP`"]
pub type DBG_TIM7_STOP_R = crate::R<bool, DBG_TIM7_STOP_A>;
impl DBG_TIM7_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_TIM7_STOP_A {
        match self.bits {
            false => DBG_TIM7_STOP_A::B_0X0,
            true => DBG_TIM7_STOP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_TIM7_STOP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_TIM7_STOP_A::B_0X1
    }
}
#[doc = "Write proxy for field `DBG_TIM7_STOP`"]
pub struct DBG_TIM7_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM7_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_TIM7_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBG_TIM7_STOP_A::B_0X0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBG_TIM7_STOP_A::B_0X1)
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
#[doc = "Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_RTC_STOP_A {
    #[doc = "0: Enable"]
    B_0X0 = 0,
    #[doc = "1: Disable"]
    B_0X1 = 1,
}
impl From<DBG_RTC_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_RTC_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBG_RTC_STOP`"]
pub type DBG_RTC_STOP_R = crate::R<bool, DBG_RTC_STOP_A>;
impl DBG_RTC_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_RTC_STOP_A {
        match self.bits {
            false => DBG_RTC_STOP_A::B_0X0,
            true => DBG_RTC_STOP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_RTC_STOP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_RTC_STOP_A::B_0X1
    }
}
#[doc = "Write proxy for field `DBG_RTC_STOP`"]
pub struct DBG_RTC_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_RTC_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_RTC_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBG_RTC_STOP_A::B_0X0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBG_RTC_STOP_A::B_0X1)
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
#[doc = "Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_WWDG_STOP_A {
    #[doc = "0: Enable"]
    B_0X0 = 0,
    #[doc = "1: Disable"]
    B_0X1 = 1,
}
impl From<DBG_WWDG_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_WWDG_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBG_WWDG_STOP`"]
pub type DBG_WWDG_STOP_R = crate::R<bool, DBG_WWDG_STOP_A>;
impl DBG_WWDG_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_WWDG_STOP_A {
        match self.bits {
            false => DBG_WWDG_STOP_A::B_0X0,
            true => DBG_WWDG_STOP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_WWDG_STOP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_WWDG_STOP_A::B_0X1
    }
}
#[doc = "Write proxy for field `DBG_WWDG_STOP`"]
pub struct DBG_WWDG_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_WWDG_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_WWDG_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBG_WWDG_STOP_A::B_0X0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBG_WWDG_STOP_A::B_0X1)
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
#[doc = "Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_IWDG_STOP_A {
    #[doc = "0: Enable"]
    B_0X0 = 0,
    #[doc = "1: Disable"]
    B_0X1 = 1,
}
impl From<DBG_IWDG_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_IWDG_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBG_IWDG_STOP`"]
pub type DBG_IWDG_STOP_R = crate::R<bool, DBG_IWDG_STOP_A>;
impl DBG_IWDG_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_IWDG_STOP_A {
        match self.bits {
            false => DBG_IWDG_STOP_A::B_0X0,
            true => DBG_IWDG_STOP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_IWDG_STOP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_IWDG_STOP_A::B_0X1
    }
}
#[doc = "Write proxy for field `DBG_IWDG_STOP`"]
pub struct DBG_IWDG_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_IWDG_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_IWDG_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBG_IWDG_STOP_A::B_0X0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBG_IWDG_STOP_A::B_0X1)
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
#[doc = "SMBUS timeout when core is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_I2C1_SMBUS_TIMEOUT_A {
    #[doc = "0: Same behavior as in normal mode"]
    B_0X0 = 0,
    #[doc = "1: The SMBUS timeout is frozen"]
    B_0X1 = 1,
}
impl From<DBG_I2C1_SMBUS_TIMEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_I2C1_SMBUS_TIMEOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBG_I2C1_SMBUS_TIMEOUT`"]
pub type DBG_I2C1_SMBUS_TIMEOUT_R = crate::R<bool, DBG_I2C1_SMBUS_TIMEOUT_A>;
impl DBG_I2C1_SMBUS_TIMEOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_I2C1_SMBUS_TIMEOUT_A {
        match self.bits {
            false => DBG_I2C1_SMBUS_TIMEOUT_A::B_0X0,
            true => DBG_I2C1_SMBUS_TIMEOUT_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_I2C1_SMBUS_TIMEOUT_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_I2C1_SMBUS_TIMEOUT_A::B_0X1
    }
}
#[doc = "Write proxy for field `DBG_I2C1_SMBUS_TIMEOUT`"]
pub struct DBG_I2C1_SMBUS_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_I2C1_SMBUS_TIMEOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_I2C1_SMBUS_TIMEOUT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Same behavior as in normal mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBG_I2C1_SMBUS_TIMEOUT_A::B_0X0)
    }
    #[doc = "The SMBUS timeout is frozen"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBG_I2C1_SMBUS_TIMEOUT_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `DBG_I2C2_SMBUS_TIMEOUT`"]
pub type DBG_I2C2_SMBUS_TIMEOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBG_I2C2_SMBUS_TIMEOUT`"]
pub struct DBG_I2C2_SMBUS_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_I2C2_SMBUS_TIMEOUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DBG_TIM3_STOP_R {
        DBG_TIM3_STOP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Clocking of TIM6 counter when the core is halted This bit enables/disables the clock to the counter of TIM6 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim6_stop(&self) -> DBG_TIM6_STOP_R {
        DBG_TIM6_STOP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clocking of TIM7 counter when the core is halted. This bit enables/disables the clock to the counter of ITIM7 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim7_stop(&self) -> DBG_TIM7_STOP_R {
        DBG_TIM7_STOP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 21 - SMBUS timeout when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c1_smbus_timeout(&self) -> DBG_I2C1_SMBUS_TIMEOUT_R {
        DBG_I2C1_SMBUS_TIMEOUT_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - SMBUS timeout when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c2_smbus_timeout(&self) -> DBG_I2C2_SMBUS_TIMEOUT_R {
        DBG_I2C2_SMBUS_TIMEOUT_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&mut self) -> DBG_TIM3_STOP_W {
        DBG_TIM3_STOP_W { w: self }
    }
    #[doc = "Bit 4 - Clocking of TIM6 counter when the core is halted This bit enables/disables the clock to the counter of TIM6 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim6_stop(&mut self) -> DBG_TIM6_STOP_W {
        DBG_TIM6_STOP_W { w: self }
    }
    #[doc = "Bit 5 - Clocking of TIM7 counter when the core is halted. This bit enables/disables the clock to the counter of ITIM7 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim7_stop(&mut self) -> DBG_TIM7_STOP_W {
        DBG_TIM7_STOP_W { w: self }
    }
    #[doc = "Bit 10 - Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W {
        DBG_RTC_STOP_W { w: self }
    }
    #[doc = "Bit 11 - Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W {
        DBG_WWDG_STOP_W { w: self }
    }
    #[doc = "Bit 12 - Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W {
        DBG_IWDG_STOP_W { w: self }
    }
    #[doc = "Bit 21 - SMBUS timeout when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c1_smbus_timeout(&mut self) -> DBG_I2C1_SMBUS_TIMEOUT_W {
        DBG_I2C1_SMBUS_TIMEOUT_W { w: self }
    }
    #[doc = "Bit 22 - SMBUS timeout when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c2_smbus_timeout(&mut self) -> DBG_I2C2_SMBUS_TIMEOUT_W {
        DBG_I2C2_SMBUS_TIMEOUT_W { w: self }
    }
}
