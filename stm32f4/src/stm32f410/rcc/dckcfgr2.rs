#[doc = "Reader of register DCKCFGR2"]
pub type R = crate::R<u32, super::DCKCFGR2>;
#[doc = "Writer for register DCKCFGR2"]
pub type W = crate::W<u32, super::DCKCFGR2>;
#[doc = "Register DCKCFGR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::DCKCFGR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "I2C4SEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2C4SEL_A {
    #[doc = "0: APB1 clock (PCLK1) is selected as I2C clock"]
    APB1 = 0,
    #[doc = "1: System clock is selected as I2C clock"]
    SYSCLK = 1,
    #[doc = "2: HSI clock is selected as I2C clock"]
    HSI = 2,
}
impl From<I2C4SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C4SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `I2C4SEL`"]
pub type I2C4SEL_R = crate::R<u8, I2C4SEL_A>;
impl I2C4SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, I2C4SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(I2C4SEL_A::APB1),
            1 => Val(I2C4SEL_A::SYSCLK),
            2 => Val(I2C4SEL_A::HSI),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `APB1`"]
    #[inline(always)]
    pub fn is_apb1(&self) -> bool {
        *self == I2C4SEL_A::APB1
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == I2C4SEL_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == I2C4SEL_A::HSI
    }
}
#[doc = "Write proxy for field `I2C4SEL`"]
pub struct I2C4SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C4SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "APB1 clock (PCLK1) is selected as I2C clock"]
    #[inline(always)]
    pub fn apb1(self) -> &'a mut W {
        self.variant(I2C4SEL_A::APB1)
    }
    #[doc = "System clock is selected as I2C clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(I2C4SEL_A::SYSCLK)
    }
    #[doc = "HSI clock is selected as I2C clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(I2C4SEL_A::HSI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "LPTIM1SEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPTIM1SEL_A {
    #[doc = "0: APB1 clock (PCLK1) selected as LPTILM1 clock"]
    APB1 = 0,
    #[doc = "1: LSI clock is selected as LPTILM1 clock"]
    LSI = 1,
    #[doc = "2: HSI clock is selected as LPTILM1 clock"]
    HSI = 2,
    #[doc = "3: LSE clock is selected as LPTILM1 clock"]
    LSE = 3,
}
impl From<LPTIM1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPTIM1SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPTIM1SEL`"]
pub type LPTIM1SEL_R = crate::R<u8, LPTIM1SEL_A>;
impl LPTIM1SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTIM1SEL_A {
        match self.bits {
            0 => LPTIM1SEL_A::APB1,
            1 => LPTIM1SEL_A::LSI,
            2 => LPTIM1SEL_A::HSI,
            3 => LPTIM1SEL_A::LSE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `APB1`"]
    #[inline(always)]
    pub fn is_apb1(&self) -> bool {
        *self == LPTIM1SEL_A::APB1
    }
    #[doc = "Checks if the value of the field is `LSI`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LPTIM1SEL_A::LSI
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == LPTIM1SEL_A::HSI
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPTIM1SEL_A::LSE
    }
}
#[doc = "Write proxy for field `LPTIM1SEL`"]
pub struct LPTIM1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM1SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "APB1 clock (PCLK1) selected as LPTILM1 clock"]
    #[inline(always)]
    pub fn apb1(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::APB1)
    }
    #[doc = "LSI clock is selected as LPTILM1 clock"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::LSI)
    }
    #[doc = "HSI clock is selected as LPTILM1 clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::HSI)
    }
    #[doc = "LSE clock is selected as LPTILM1 clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::LSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 22:23 - I2C4SEL"]
    #[inline(always)]
    pub fn i2c4sel(&self) -> I2C4SEL_R {
        I2C4SEL_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - LPTIM1SEL"]
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 22:23 - I2C4SEL"]
    #[inline(always)]
    pub fn i2c4sel(&mut self) -> I2C4SEL_W {
        I2C4SEL_W { w: self }
    }
    #[doc = "Bits 30:31 - LPTIM1SEL"]
    #[inline(always)]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W {
        LPTIM1SEL_W { w: self }
    }
}
