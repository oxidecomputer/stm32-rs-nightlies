#[doc = "Reader of register DBG_CR"]
pub type R = crate::R<u32, super::DBG_CR>;
#[doc = "Writer for register DBG_CR"]
pub type W = crate::W<u32, super::DBG_CR>;
#[doc = "Register DBG_CR `reset()`'s with value 0"]
impl crate::ResetValue for super::DBG_CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Debug Stop mode Debug options in Stop mode. Upon Stop mode exit, the software must re-establish the desired clock configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_STOP_A {
    #[doc = "0: All clocks disabled, including FCLK and HCLK. Upon Stop mode exit, the CPU is clocked by the HSI internal RC oscillator. "]
    B_0X0 = 0,
    #[doc = "1: FCLK and HCLK running, derived from the internal RC oscillator remaining active. If Systick is enabled, it may generate periodic interrupt and wake up events."]
    B_0X1 = 1,
}
impl From<DBG_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBG_STOP`"]
pub type DBG_STOP_R = crate::R<bool, DBG_STOP_A>;
impl DBG_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_STOP_A {
        match self.bits {
            false => DBG_STOP_A::B_0X0,
            true => DBG_STOP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_STOP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_STOP_A::B_0X1
    }
}
#[doc = "Write proxy for field `DBG_STOP`"]
pub struct DBG_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "All clocks disabled, including FCLK and HCLK. Upon Stop mode exit, the CPU is clocked by the HSI internal RC oscillator."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBG_STOP_A::B_0X0)
    }
    #[doc = "FCLK and HCLK running, derived from the internal RC oscillator remaining active. If Systick is enabled, it may generate periodic interrupt and wake up events."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBG_STOP_A::B_0X1)
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
#[doc = "Debug Standby and Shutdown modes Debug options in Standby or Shutdown mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_STANDBY_A {
    #[doc = "0: Digital part powered. From software point of view, exiting Standby and Shutdown modes is identical as fetching reset vector (except for status bits indicating that the MCU exits Standby)"]
    B_0X0 = 0,
    #[doc = "1: Digital part powered and FCLK and HCLK running, derived from the internal RC oscillator remaining active. The MCU generates a system reset so that exiting Standby and Shutdown has the same effect as starting from reset."]
    B_0X1 = 1,
}
impl From<DBG_STANDBY_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_STANDBY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBG_STANDBY`"]
pub type DBG_STANDBY_R = crate::R<bool, DBG_STANDBY_A>;
impl DBG_STANDBY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_STANDBY_A {
        match self.bits {
            false => DBG_STANDBY_A::B_0X0,
            true => DBG_STANDBY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_STANDBY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_STANDBY_A::B_0X1
    }
}
#[doc = "Write proxy for field `DBG_STANDBY`"]
pub struct DBG_STANDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_STANDBY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_STANDBY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital part powered. From software point of view, exiting Standby and Shutdown modes is identical as fetching reset vector (except for status bits indicating that the MCU exits Standby)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBG_STANDBY_A::B_0X0)
    }
    #[doc = "Digital part powered and FCLK and HCLK running, derived from the internal RC oscillator remaining active. The MCU generates a system reset so that exiting Standby and Shutdown has the same effect as starting from reset."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBG_STANDBY_A::B_0X1)
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
impl R {
    #[doc = "Bit 1 - Debug Stop mode Debug options in Stop mode. Upon Stop mode exit, the software must re-establish the desired clock configuration."]
    #[inline(always)]
    pub fn dbg_stop(&self) -> DBG_STOP_R {
        DBG_STOP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Debug Standby and Shutdown modes Debug options in Standby or Shutdown mode."]
    #[inline(always)]
    pub fn dbg_standby(&self) -> DBG_STANDBY_R {
        DBG_STANDBY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Debug Stop mode Debug options in Stop mode. Upon Stop mode exit, the software must re-establish the desired clock configuration."]
    #[inline(always)]
    pub fn dbg_stop(&mut self) -> DBG_STOP_W {
        DBG_STOP_W { w: self }
    }
    #[doc = "Bit 2 - Debug Standby and Shutdown modes Debug options in Standby or Shutdown mode."]
    #[inline(always)]
    pub fn dbg_standby(&mut self) -> DBG_STANDBY_W {
        DBG_STANDBY_W { w: self }
    }
}
