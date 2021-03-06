#[doc = "Reader of register BGPFCCR"]
pub type R = crate::R<u32, super::BGPFCCR>;
#[doc = "Writer for register BGPFCCR"]
pub type W = crate::W<u32, super::BGPFCCR>;
#[doc = "Register BGPFCCR `reset()`'s with value 0"]
impl crate::ResetValue for super::BGPFCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Color mode These bits define the color format of the foreground image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CM_A {
    #[doc = "0: Color mode ARGB8888"]
    ARGB8888 = 0,
    #[doc = "1: Color mode RGB888"]
    RGB888 = 1,
    #[doc = "2: Color mode RGB565"]
    RGB565 = 2,
    #[doc = "3: Color mode ARGB1555"]
    ARGB1555 = 3,
    #[doc = "4: Color mode ARGB4444"]
    ARGB4444 = 4,
    #[doc = "5: Color mode L8"]
    L8 = 5,
    #[doc = "6: Color mode AL44"]
    AL44 = 6,
    #[doc = "7: Color mode AL88"]
    AL88 = 7,
    #[doc = "8: Color mode L4"]
    L4 = 8,
    #[doc = "9: Color mode A8"]
    A8 = 9,
    #[doc = "10: Color mode A4"]
    A4 = 10,
}
impl From<CM_A> for u8 {
    #[inline(always)]
    fn from(variant: CM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CM`"]
pub type CM_R = crate::R<u8, CM_A>;
impl CM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CM_A::ARGB8888),
            1 => Val(CM_A::RGB888),
            2 => Val(CM_A::RGB565),
            3 => Val(CM_A::ARGB1555),
            4 => Val(CM_A::ARGB4444),
            5 => Val(CM_A::L8),
            6 => Val(CM_A::AL44),
            7 => Val(CM_A::AL88),
            8 => Val(CM_A::L4),
            9 => Val(CM_A::A8),
            10 => Val(CM_A::A4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ARGB8888`"]
    #[inline(always)]
    pub fn is_argb8888(&self) -> bool {
        *self == CM_A::ARGB8888
    }
    #[doc = "Checks if the value of the field is `RGB888`"]
    #[inline(always)]
    pub fn is_rgb888(&self) -> bool {
        *self == CM_A::RGB888
    }
    #[doc = "Checks if the value of the field is `RGB565`"]
    #[inline(always)]
    pub fn is_rgb565(&self) -> bool {
        *self == CM_A::RGB565
    }
    #[doc = "Checks if the value of the field is `ARGB1555`"]
    #[inline(always)]
    pub fn is_argb1555(&self) -> bool {
        *self == CM_A::ARGB1555
    }
    #[doc = "Checks if the value of the field is `ARGB4444`"]
    #[inline(always)]
    pub fn is_argb4444(&self) -> bool {
        *self == CM_A::ARGB4444
    }
    #[doc = "Checks if the value of the field is `L8`"]
    #[inline(always)]
    pub fn is_l8(&self) -> bool {
        *self == CM_A::L8
    }
    #[doc = "Checks if the value of the field is `AL44`"]
    #[inline(always)]
    pub fn is_al44(&self) -> bool {
        *self == CM_A::AL44
    }
    #[doc = "Checks if the value of the field is `AL88`"]
    #[inline(always)]
    pub fn is_al88(&self) -> bool {
        *self == CM_A::AL88
    }
    #[doc = "Checks if the value of the field is `L4`"]
    #[inline(always)]
    pub fn is_l4(&self) -> bool {
        *self == CM_A::L4
    }
    #[doc = "Checks if the value of the field is `A8`"]
    #[inline(always)]
    pub fn is_a8(&self) -> bool {
        *self == CM_A::A8
    }
    #[doc = "Checks if the value of the field is `A4`"]
    #[inline(always)]
    pub fn is_a4(&self) -> bool {
        *self == CM_A::A4
    }
}
#[doc = "Write proxy for field `CM`"]
pub struct CM_W<'a> {
    w: &'a mut W,
}
impl<'a> CM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Color mode ARGB8888"]
    #[inline(always)]
    pub fn argb8888(self) -> &'a mut W {
        self.variant(CM_A::ARGB8888)
    }
    #[doc = "Color mode RGB888"]
    #[inline(always)]
    pub fn rgb888(self) -> &'a mut W {
        self.variant(CM_A::RGB888)
    }
    #[doc = "Color mode RGB565"]
    #[inline(always)]
    pub fn rgb565(self) -> &'a mut W {
        self.variant(CM_A::RGB565)
    }
    #[doc = "Color mode ARGB1555"]
    #[inline(always)]
    pub fn argb1555(self) -> &'a mut W {
        self.variant(CM_A::ARGB1555)
    }
    #[doc = "Color mode ARGB4444"]
    #[inline(always)]
    pub fn argb4444(self) -> &'a mut W {
        self.variant(CM_A::ARGB4444)
    }
    #[doc = "Color mode L8"]
    #[inline(always)]
    pub fn l8(self) -> &'a mut W {
        self.variant(CM_A::L8)
    }
    #[doc = "Color mode AL44"]
    #[inline(always)]
    pub fn al44(self) -> &'a mut W {
        self.variant(CM_A::AL44)
    }
    #[doc = "Color mode AL88"]
    #[inline(always)]
    pub fn al88(self) -> &'a mut W {
        self.variant(CM_A::AL88)
    }
    #[doc = "Color mode L4"]
    #[inline(always)]
    pub fn l4(self) -> &'a mut W {
        self.variant(CM_A::L4)
    }
    #[doc = "Color mode A8"]
    #[inline(always)]
    pub fn a8(self) -> &'a mut W {
        self.variant(CM_A::A8)
    }
    #[doc = "Color mode A4"]
    #[inline(always)]
    pub fn a4(self) -> &'a mut W {
        self.variant(CM_A::A4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "CLUT Color mode These bits define the color format of the CLUT. This register can only be written when the transfer is disabled. Once the CLUT transfer has started, this bit is read-only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCM_A {
    #[doc = "0: CLUT color format ARGB8888"]
    ARGB8888 = 0,
    #[doc = "1: CLUT color format RGB888"]
    RGB888 = 1,
}
impl From<CCM_A> for bool {
    #[inline(always)]
    fn from(variant: CCM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCM`"]
pub type CCM_R = crate::R<bool, CCM_A>;
impl CCM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCM_A {
        match self.bits {
            false => CCM_A::ARGB8888,
            true => CCM_A::RGB888,
        }
    }
    #[doc = "Checks if the value of the field is `ARGB8888`"]
    #[inline(always)]
    pub fn is_argb8888(&self) -> bool {
        *self == CCM_A::ARGB8888
    }
    #[doc = "Checks if the value of the field is `RGB888`"]
    #[inline(always)]
    pub fn is_rgb888(&self) -> bool {
        *self == CCM_A::RGB888
    }
}
#[doc = "Write proxy for field `CCM`"]
pub struct CCM_W<'a> {
    w: &'a mut W,
}
impl<'a> CCM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CLUT color format ARGB8888"]
    #[inline(always)]
    pub fn argb8888(self) -> &'a mut W {
        self.variant(CCM_A::ARGB8888)
    }
    #[doc = "CLUT color format RGB888"]
    #[inline(always)]
    pub fn rgb888(self) -> &'a mut W {
        self.variant(CCM_A::RGB888)
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
#[doc = "Start This bit is set to start the automatic loading of the CLUT. This bit is automatically reset: ** at the end of the transfer ** when the transfer is aborted by the user application by setting the ABORT bit in the DMA2D_CR ** when a transfer error occurs ** when the transfer has not started due to a configuration error or another transfer operation already on going (data transfer or automatic BackGround CLUT transfer).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_A {
    #[doc = "1: Start the automatic loading of the CLUT"]
    START = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `START`"]
pub type START_R = crate::R<bool, START_A>;
impl START_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, START_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(START_A::START),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == START_A::START
    }
}
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: START_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Start the automatic loading of the CLUT"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(START_A::START)
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
#[doc = "Reader of field `CS`"]
pub type CS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CS`"]
pub struct CS_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Alpha mode These bits define which alpha channel value to be used for the background image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AM_A {
    #[doc = "0: No modification of alpha channel"]
    NOMODIFY = 0,
    #[doc = "1: Replace with value in ALPHA\\[7:0\\]"]
    REPLACE = 1,
    #[doc = "2: Multiply with value in ALPHA\\[7:0\\]"]
    MULTIPLY = 2,
}
impl From<AM_A> for u8 {
    #[inline(always)]
    fn from(variant: AM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AM`"]
pub type AM_R = crate::R<u8, AM_A>;
impl AM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AM_A::NOMODIFY),
            1 => Val(AM_A::REPLACE),
            2 => Val(AM_A::MULTIPLY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOMODIFY`"]
    #[inline(always)]
    pub fn is_no_modify(&self) -> bool {
        *self == AM_A::NOMODIFY
    }
    #[doc = "Checks if the value of the field is `REPLACE`"]
    #[inline(always)]
    pub fn is_replace(&self) -> bool {
        *self == AM_A::REPLACE
    }
    #[doc = "Checks if the value of the field is `MULTIPLY`"]
    #[inline(always)]
    pub fn is_multiply(&self) -> bool {
        *self == AM_A::MULTIPLY
    }
}
#[doc = "Write proxy for field `AM`"]
pub struct AM_W<'a> {
    w: &'a mut W,
}
impl<'a> AM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No modification of alpha channel"]
    #[inline(always)]
    pub fn no_modify(self) -> &'a mut W {
        self.variant(AM_A::NOMODIFY)
    }
    #[doc = "Replace with value in ALPHA\\[7:0\\]"]
    #[inline(always)]
    pub fn replace(self) -> &'a mut W {
        self.variant(AM_A::REPLACE)
    }
    #[doc = "Multiply with value in ALPHA\\[7:0\\]"]
    #[inline(always)]
    pub fn multiply(self) -> &'a mut W {
        self.variant(AM_A::MULTIPLY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AI_A {
    #[doc = "0: Regular alpha"]
    REGULARALPHA = 0,
    #[doc = "1: Inverted alpha"]
    INVERTEDALPHA = 1,
}
impl From<AI_A> for bool {
    #[inline(always)]
    fn from(variant: AI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AI`"]
pub type AI_R = crate::R<bool, AI_A>;
impl AI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AI_A {
        match self.bits {
            false => AI_A::REGULARALPHA,
            true => AI_A::INVERTEDALPHA,
        }
    }
    #[doc = "Checks if the value of the field is `REGULARALPHA`"]
    #[inline(always)]
    pub fn is_regular_alpha(&self) -> bool {
        *self == AI_A::REGULARALPHA
    }
    #[doc = "Checks if the value of the field is `INVERTEDALPHA`"]
    #[inline(always)]
    pub fn is_inverted_alpha(&self) -> bool {
        *self == AI_A::INVERTEDALPHA
    }
}
#[doc = "Write proxy for field `AI`"]
pub struct AI_W<'a> {
    w: &'a mut W,
}
impl<'a> AI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Regular alpha"]
    #[inline(always)]
    pub fn regular_alpha(self) -> &'a mut W {
        self.variant(AI_A::REGULARALPHA)
    }
    #[doc = "Inverted alpha"]
    #[inline(always)]
    pub fn inverted_alpha(self) -> &'a mut W {
        self.variant(AI_A::INVERTEDALPHA)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Red Blue Swap This bit allows to swap the R &amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBS_A {
    #[doc = "0: No Red Blue Swap (RGB or ARGB)"]
    REGULAR = 0,
    #[doc = "1: Red Blue Swap (BGR or ABGR)"]
    SWAP = 1,
}
impl From<RBS_A> for bool {
    #[inline(always)]
    fn from(variant: RBS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RBS`"]
pub type RBS_R = crate::R<bool, RBS_A>;
impl RBS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBS_A {
        match self.bits {
            false => RBS_A::REGULAR,
            true => RBS_A::SWAP,
        }
    }
    #[doc = "Checks if the value of the field is `REGULAR`"]
    #[inline(always)]
    pub fn is_regular(&self) -> bool {
        *self == RBS_A::REGULAR
    }
    #[doc = "Checks if the value of the field is `SWAP`"]
    #[inline(always)]
    pub fn is_swap(&self) -> bool {
        *self == RBS_A::SWAP
    }
}
#[doc = "Write proxy for field `RBS`"]
pub struct RBS_W<'a> {
    w: &'a mut W,
}
impl<'a> RBS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RBS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Red Blue Swap (RGB or ARGB)"]
    #[inline(always)]
    pub fn regular(self) -> &'a mut W {
        self.variant(RBS_A::REGULAR)
    }
    #[doc = "Red Blue Swap (BGR or ABGR)"]
    #[inline(always)]
    pub fn swap(self) -> &'a mut W {
        self.variant(RBS_A::SWAP)
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
#[doc = "Reader of field `ALPHA`"]
pub type ALPHA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALPHA`"]
pub struct ALPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> ALPHA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Color mode These bits define the color format of the foreground image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - CLUT Color mode These bits define the color format of the CLUT. This register can only be written when the transfer is disabled. Once the CLUT transfer has started, this bit is read-only."]
    #[inline(always)]
    pub fn ccm(&self) -> CCM_R {
        CCM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Start This bit is set to start the automatic loading of the CLUT. This bit is automatically reset: ** at the end of the transfer ** when the transfer is aborted by the user application by setting the ABORT bit in the DMA2D_CR ** when a transfer error occurs ** when the transfer has not started due to a configuration error or another transfer operation already on going (data transfer or automatic BackGround CLUT transfer)."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - CLUT size These bits define the size of the CLUT used for the BG. Once the CLUT transfer has started, this field is read-only. The number of CLUT entries is equal to CS\\[7:0\\]
+ 1."]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Alpha mode These bits define which alpha channel value to be used for the background image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
    #[inline(always)]
    pub fn am(&self) -> AM_R {
        AM_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 20 - Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only."]
    #[inline(always)]
    pub fn ai(&self) -> AI_R {
        AI_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Red Blue Swap This bit allows to swap the R &amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only."]
    #[inline(always)]
    pub fn rbs(&self) -> RBS_R {
        RBS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - Alpha value These bits define a fixed alpha channel value which can replace the original alpha value or be multiplied with the original alpha value according to the alpha mode selected with bits AM\\[1: 0\\]. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    pub fn alpha(&self) -> ALPHA_R {
        ALPHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Color mode These bits define the color format of the foreground image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W {
        CM_W { w: self }
    }
    #[doc = "Bit 4 - CLUT Color mode These bits define the color format of the CLUT. This register can only be written when the transfer is disabled. Once the CLUT transfer has started, this bit is read-only."]
    #[inline(always)]
    pub fn ccm(&mut self) -> CCM_W {
        CCM_W { w: self }
    }
    #[doc = "Bit 5 - Start This bit is set to start the automatic loading of the CLUT. This bit is automatically reset: ** at the end of the transfer ** when the transfer is aborted by the user application by setting the ABORT bit in the DMA2D_CR ** when a transfer error occurs ** when the transfer has not started due to a configuration error or another transfer operation already on going (data transfer or automatic BackGround CLUT transfer)."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bits 8:15 - CLUT size These bits define the size of the CLUT used for the BG. Once the CLUT transfer has started, this field is read-only. The number of CLUT entries is equal to CS\\[7:0\\]
+ 1."]
    #[inline(always)]
    pub fn cs(&mut self) -> CS_W {
        CS_W { w: self }
    }
    #[doc = "Bits 16:17 - Alpha mode These bits define which alpha channel value to be used for the background image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
    #[inline(always)]
    pub fn am(&mut self) -> AM_W {
        AM_W { w: self }
    }
    #[doc = "Bit 20 - Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only."]
    #[inline(always)]
    pub fn ai(&mut self) -> AI_W {
        AI_W { w: self }
    }
    #[doc = "Bit 21 - Red Blue Swap This bit allows to swap the R &amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only."]
    #[inline(always)]
    pub fn rbs(&mut self) -> RBS_W {
        RBS_W { w: self }
    }
    #[doc = "Bits 24:31 - Alpha value These bits define a fixed alpha channel value which can replace the original alpha value or be multiplied with the original alpha value according to the alpha mode selected with bits AM\\[1: 0\\]. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    pub fn alpha(&mut self) -> ALPHA_W {
        ALPHA_W { w: self }
    }
}
