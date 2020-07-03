#[doc = "Reader of register AHB2ENR"]
pub type R = crate::R<u32, super::AHB2ENR>;
#[doc = "Writer for register AHB2ENR"]
pub type W = crate::W<u32, super::AHB2ENR>;
#[doc = "Register AHB2ENR `reset()`'s with value 0"]
impl crate::ResetValue for super::AHB2ENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "USB OTG FS clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTGFSEN_A {
    #[doc = "0: The selected clock is disabled"]
    DISABLED = 0,
    #[doc = "1: The selected clock is enabled"]
    ENABLED = 1,
}
impl From<OTGFSEN_A> for bool {
    #[inline(always)]
    fn from(variant: OTGFSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OTGFSEN`"]
pub type OTGFSEN_R = crate::R<bool, OTGFSEN_A>;
impl OTGFSEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OTGFSEN_A {
        match self.bits {
            false => OTGFSEN_A::DISABLED,
            true => OTGFSEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OTGFSEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OTGFSEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `OTGFSEN`"]
pub struct OTGFSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGFSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OTGFSEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGFSEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGFSEN_A::ENABLED)
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
#[doc = "Camera interface enable"]
pub type DCMIEN_A = OTGFSEN_A;
#[doc = "Reader of field `DCMIEN`"]
pub type DCMIEN_R = crate::R<bool, OTGFSEN_A>;
#[doc = "Write proxy for field `DCMIEN`"]
pub struct DCMIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCMIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGFSEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGFSEN_A::ENABLED)
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
impl R {
    #[doc = "Bit 7 - USB OTG FS clock enable"]
    #[inline(always)]
    pub fn otgfsen(&self) -> OTGFSEN_R {
        OTGFSEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Camera interface enable"]
    #[inline(always)]
    pub fn dcmien(&self) -> DCMIEN_R {
        DCMIEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - USB OTG FS clock enable"]
    #[inline(always)]
    pub fn otgfsen(&mut self) -> OTGFSEN_W {
        OTGFSEN_W { w: self }
    }
    #[doc = "Bit 0 - Camera interface enable"]
    #[inline(always)]
    pub fn dcmien(&mut self) -> DCMIEN_W {
        DCMIEN_W { w: self }
    }
}
