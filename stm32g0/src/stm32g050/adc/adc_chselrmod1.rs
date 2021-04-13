#[doc = "Reader of register ADC_CHSELRMOD1"]
pub type R = crate::R<u32, super::ADC_CHSELRMOD1>;
#[doc = "Writer for register ADC_CHSELRMOD1"]
pub type W = crate::W<u32, super::ADC_CHSELRMOD1>;
#[doc = "Register ADC_CHSELRMOD1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_CHSELRMOD1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SQ1`"]
pub type SQ1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SQ1`"]
pub struct SQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `SQ2`"]
pub type SQ2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SQ2`"]
pub struct SQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `SQ3`"]
pub type SQ3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SQ3`"]
pub struct SQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `SQ4`"]
pub type SQ4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SQ4`"]
pub struct SQ4_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `SQ5`"]
pub type SQ5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SQ5`"]
pub struct SQ5_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `SQ6`"]
pub type SQ6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SQ6`"]
pub struct SQ6_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `SQ7`"]
pub type SQ7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SQ7`"]
pub struct SQ7_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "8th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates the end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. ... Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SQ8_A {
    #[doc = "0: CH0 "]
    B_0X0 = 0,
    #[doc = "1: CH1"]
    B_0X1 = 1,
    #[doc = "12: CH12"]
    B_0XC = 12,
    #[doc = "13: CH13"]
    B_0XD = 13,
    #[doc = "14: CH14"]
    B_0XE = 14,
    #[doc = "15: No channel selected (End of sequence)"]
    B_0XF = 15,
}
impl From<SQ8_A> for u8 {
    #[inline(always)]
    fn from(variant: SQ8_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SQ8`"]
pub type SQ8_R = crate::R<u8, SQ8_A>;
impl SQ8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SQ8_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SQ8_A::B_0X0),
            1 => Val(SQ8_A::B_0X1),
            12 => Val(SQ8_A::B_0XC),
            13 => Val(SQ8_A::B_0XD),
            14 => Val(SQ8_A::B_0XE),
            15 => Val(SQ8_A::B_0XF),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SQ8_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SQ8_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0XC`"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == SQ8_A::B_0XC
    }
    #[doc = "Checks if the value of the field is `B_0XD`"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == SQ8_A::B_0XD
    }
    #[doc = "Checks if the value of the field is `B_0XE`"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == SQ8_A::B_0XE
    }
    #[doc = "Checks if the value of the field is `B_0XF`"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == SQ8_A::B_0XF
    }
}
#[doc = "Write proxy for field `SQ8`"]
pub struct SQ8_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SQ8_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CH0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SQ8_A::B_0X0)
    }
    #[doc = "CH1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SQ8_A::B_0X1)
    }
    #[doc = "CH12"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut W {
        self.variant(SQ8_A::B_0XC)
    }
    #[doc = "CH13"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut W {
        self.variant(SQ8_A::B_0XD)
    }
    #[doc = "CH14"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut W {
        self.variant(SQ8_A::B_0XE)
    }
    #[doc = "No channel selected (End of sequence)"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut W {
        self.variant(SQ8_A::B_0XF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - 1st conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn sq1(&self) -> SQ1_R {
        SQ1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 2nd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn sq2(&self) -> SQ2_R {
        SQ2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 3rd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn sq3(&self) -> SQ3_R {
        SQ3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 4th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn sq4(&self) -> SQ4_R {
        SQ4_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 5th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn sq5(&self) -> SQ5_R {
        SQ5_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 6th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn sq6(&self) -> SQ6_R {
        SQ6_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 7th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn sq7(&self) -> SQ7_R {
        SQ7_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 8th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates the end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. ... Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn sq8(&self) -> SQ8_R {
        SQ8_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1st conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn sq1(&mut self) -> SQ1_W {
        SQ1_W { w: self }
    }
    #[doc = "Bits 4:7 - 2nd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn sq2(&mut self) -> SQ2_W {
        SQ2_W { w: self }
    }
    #[doc = "Bits 8:11 - 3rd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn sq3(&mut self) -> SQ3_W {
        SQ3_W { w: self }
    }
    #[doc = "Bits 12:15 - 4th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn sq4(&mut self) -> SQ4_W {
        SQ4_W { w: self }
    }
    #[doc = "Bits 16:19 - 5th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn sq5(&mut self) -> SQ5_W {
        SQ5_W { w: self }
    }
    #[doc = "Bits 20:23 - 6th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn sq6(&mut self) -> SQ6_W {
        SQ6_W { w: self }
    }
    #[doc = "Bits 24:27 - 7th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn sq7(&mut self) -> SQ7_W {
        SQ7_W { w: self }
    }
    #[doc = "Bits 28:31 - 8th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates the end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. ... Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn sq8(&mut self) -> SQ8_W {
        SQ8_W { w: self }
    }
}
