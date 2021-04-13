#[doc = "Reader of register CFGR1"]
pub type R = crate::R<u32, super::CFGR1>;
#[doc = "Writer for register CFGR1"]
pub type W = crate::W<u32, super::CFGR1>;
#[doc = "Register CFGR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Boot mode selected by the boot pins status bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BOOT_MODE_A {
    #[doc = "0: Main Flash memory boot mode"]
    MAINFLASH = 0,
    #[doc = "1: System Flash memory boot mode"]
    SYSTEMFLASH = 1,
    #[doc = "3: Embedded SRAM boot mode"]
    SRAM = 3,
}
impl From<BOOT_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: BOOT_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BOOT_MODE`"]
pub type BOOT_MODE_R = crate::R<u8, BOOT_MODE_A>;
impl BOOT_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BOOT_MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BOOT_MODE_A::MAINFLASH),
            1 => Val(BOOT_MODE_A::SYSTEMFLASH),
            3 => Val(BOOT_MODE_A::SRAM),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MAINFLASH`"]
    #[inline(always)]
    pub fn is_main_flash(&self) -> bool {
        *self == BOOT_MODE_A::MAINFLASH
    }
    #[doc = "Checks if the value of the field is `SYSTEMFLASH`"]
    #[inline(always)]
    pub fn is_system_flash(&self) -> bool {
        *self == BOOT_MODE_A::SYSTEMFLASH
    }
    #[doc = "Checks if the value of the field is `SRAM`"]
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == BOOT_MODE_A::SRAM
    }
}
#[doc = "Memory mapping selection bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MEM_MODE_A {
    #[doc = "0: Main Flash memory mapped at 0x0000_0000"]
    MAINFLASH = 0,
    #[doc = "1: System Flash memory mapped at 0x0000_0000"]
    SYSTEMFLASH = 1,
    #[doc = "3: Embedded SRAM mapped at 0x0000_0000"]
    SRAM = 3,
}
impl From<MEM_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MEM_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MEM_MODE`"]
pub type MEM_MODE_R = crate::R<u8, MEM_MODE_A>;
impl MEM_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MEM_MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MEM_MODE_A::MAINFLASH),
            1 => Val(MEM_MODE_A::SYSTEMFLASH),
            3 => Val(MEM_MODE_A::SRAM),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MAINFLASH`"]
    #[inline(always)]
    pub fn is_main_flash(&self) -> bool {
        *self == MEM_MODE_A::MAINFLASH
    }
    #[doc = "Checks if the value of the field is `SYSTEMFLASH`"]
    #[inline(always)]
    pub fn is_system_flash(&self) -> bool {
        *self == MEM_MODE_A::SYSTEMFLASH
    }
    #[doc = "Checks if the value of the field is `SRAM`"]
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == MEM_MODE_A::SRAM
    }
}
#[doc = "Write proxy for field `MEM_MODE`"]
pub struct MEM_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEM_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Main Flash memory mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn main_flash(self) -> &'a mut W {
        self.variant(MEM_MODE_A::MAINFLASH)
    }
    #[doc = "System Flash memory mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn system_flash(self) -> &'a mut W {
        self.variant(MEM_MODE_A::SYSTEMFLASH)
    }
    #[doc = "Embedded SRAM mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn sram(self) -> &'a mut W {
        self.variant(MEM_MODE_A::SRAM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:9 - Boot mode selected by the boot pins status bits"]
    #[inline(always)]
    pub fn boot_mode(&self) -> BOOT_MODE_R {
        BOOT_MODE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MEM_MODE_W {
        MEM_MODE_W { w: self }
    }
}
