#[doc = "Reader of register C1EMR2"]
pub type R = crate::R<u32, super::C1EMR2>;
#[doc = "Writer for register C1EMR2"]
pub type W = crate::W<u32, super::C1EMR2>;
#[doc = "Register C1EMR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::C1EMR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MR32`"]
pub type MR32_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR32`"]
pub struct MR32_W<'a> {
    w: &'a mut W,
}
impl<'a> MR32_W<'a> {
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
#[doc = "Reader of field `MR33`"]
pub type MR33_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR33`"]
pub struct MR33_W<'a> {
    w: &'a mut W,
}
impl<'a> MR33_W<'a> {
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
#[doc = "Reader of field `MR34`"]
pub type MR34_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR34`"]
pub struct MR34_W<'a> {
    w: &'a mut W,
}
impl<'a> MR34_W<'a> {
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
#[doc = "Reader of field `MR35`"]
pub type MR35_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR35`"]
pub struct MR35_W<'a> {
    w: &'a mut W,
}
impl<'a> MR35_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `MR36`"]
pub type MR36_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR36`"]
pub struct MR36_W<'a> {
    w: &'a mut W,
}
impl<'a> MR36_W<'a> {
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
#[doc = "Reader of field `MR37`"]
pub type MR37_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR37`"]
pub struct MR37_W<'a> {
    w: &'a mut W,
}
impl<'a> MR37_W<'a> {
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
#[doc = "Reader of field `MR38`"]
pub type MR38_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR38`"]
pub struct MR38_W<'a> {
    w: &'a mut W,
}
impl<'a> MR38_W<'a> {
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
#[doc = "Reader of field `MR39`"]
pub type MR39_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR39`"]
pub struct MR39_W<'a> {
    w: &'a mut W,
}
impl<'a> MR39_W<'a> {
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
#[doc = "Reader of field `MR40`"]
pub type MR40_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR40`"]
pub struct MR40_W<'a> {
    w: &'a mut W,
}
impl<'a> MR40_W<'a> {
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
#[doc = "Reader of field `MR41`"]
pub type MR41_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR41`"]
pub struct MR41_W<'a> {
    w: &'a mut W,
}
impl<'a> MR41_W<'a> {
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
#[doc = "Reader of field `MR42`"]
pub type MR42_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR42`"]
pub struct MR42_W<'a> {
    w: &'a mut W,
}
impl<'a> MR42_W<'a> {
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
#[doc = "Reader of field `MR43`"]
pub type MR43_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR43`"]
pub struct MR43_W<'a> {
    w: &'a mut W,
}
impl<'a> MR43_W<'a> {
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
#[doc = "Reader of field `MR44`"]
pub type MR44_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR44`"]
pub struct MR44_W<'a> {
    w: &'a mut W,
}
impl<'a> MR44_W<'a> {
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
#[doc = "Reader of field `MR46`"]
pub type MR46_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR46`"]
pub struct MR46_W<'a> {
    w: &'a mut W,
}
impl<'a> MR46_W<'a> {
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
#[doc = "Reader of field `MR47`"]
pub type MR47_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR47`"]
pub struct MR47_W<'a> {
    w: &'a mut W,
}
impl<'a> MR47_W<'a> {
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
#[doc = "Reader of field `MR48`"]
pub type MR48_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR48`"]
pub struct MR48_W<'a> {
    w: &'a mut W,
}
impl<'a> MR48_W<'a> {
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
#[doc = "Reader of field `MR49`"]
pub type MR49_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR49`"]
pub struct MR49_W<'a> {
    w: &'a mut W,
}
impl<'a> MR49_W<'a> {
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
#[doc = "Reader of field `MR50`"]
pub type MR50_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR50`"]
pub struct MR50_W<'a> {
    w: &'a mut W,
}
impl<'a> MR50_W<'a> {
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
#[doc = "Reader of field `MR51`"]
pub type MR51_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR51`"]
pub struct MR51_W<'a> {
    w: &'a mut W,
}
impl<'a> MR51_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `MR52`"]
pub type MR52_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR52`"]
pub struct MR52_W<'a> {
    w: &'a mut W,
}
impl<'a> MR52_W<'a> {
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
#[doc = "Reader of field `MR53`"]
pub type MR53_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR53`"]
pub struct MR53_W<'a> {
    w: &'a mut W,
}
impl<'a> MR53_W<'a> {
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
#[doc = "Reader of field `MR54`"]
pub type MR54_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR54`"]
pub struct MR54_W<'a> {
    w: &'a mut W,
}
impl<'a> MR54_W<'a> {
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
#[doc = "Reader of field `MR55`"]
pub type MR55_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR55`"]
pub struct MR55_W<'a> {
    w: &'a mut W,
}
impl<'a> MR55_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `MR56`"]
pub type MR56_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR56`"]
pub struct MR56_W<'a> {
    w: &'a mut W,
}
impl<'a> MR56_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `MR57`"]
pub type MR57_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR57`"]
pub struct MR57_W<'a> {
    w: &'a mut W,
}
impl<'a> MR57_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `MR58`"]
pub type MR58_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR58`"]
pub struct MR58_W<'a> {
    w: &'a mut W,
}
impl<'a> MR58_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `MR59`"]
pub type MR59_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR59`"]
pub struct MR59_W<'a> {
    w: &'a mut W,
}
impl<'a> MR59_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `MR60`"]
pub type MR60_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR60`"]
pub struct MR60_W<'a> {
    w: &'a mut W,
}
impl<'a> MR60_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `MR61`"]
pub type MR61_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR61`"]
pub struct MR61_W<'a> {
    w: &'a mut W,
}
impl<'a> MR61_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `MR62`"]
pub type MR62_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR62`"]
pub struct MR62_W<'a> {
    w: &'a mut W,
}
impl<'a> MR62_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `MR63`"]
pub type MR63_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR63`"]
pub struct MR63_W<'a> {
    w: &'a mut W,
}
impl<'a> MR63_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr32(&self) -> MR32_R {
        MR32_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr33(&self) -> MR33_R {
        MR33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr34(&self) -> MR34_R {
        MR34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr35(&self) -> MR35_R {
        MR35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr36(&self) -> MR36_R {
        MR36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr37(&self) -> MR37_R {
        MR37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr38(&self) -> MR38_R {
        MR38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr39(&self) -> MR39_R {
        MR39_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr40(&self) -> MR40_R {
        MR40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr41(&self) -> MR41_R {
        MR41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr42(&self) -> MR42_R {
        MR42_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr43(&self) -> MR43_R {
        MR43_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr44(&self) -> MR44_R {
        MR44_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr46(&self) -> MR46_R {
        MR46_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr47(&self) -> MR47_R {
        MR47_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr48(&self) -> MR48_R {
        MR48_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr49(&self) -> MR49_R {
        MR49_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr50(&self) -> MR50_R {
        MR50_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr51(&self) -> MR51_R {
        MR51_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr52(&self) -> MR52_R {
        MR52_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr53(&self) -> MR53_R {
        MR53_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr54(&self) -> MR54_R {
        MR54_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr55(&self) -> MR55_R {
        MR55_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr56(&self) -> MR56_R {
        MR56_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr57(&self) -> MR57_R {
        MR57_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr58(&self) -> MR58_R {
        MR58_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr59(&self) -> MR59_R {
        MR59_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr60(&self) -> MR60_R {
        MR60_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr61(&self) -> MR61_R {
        MR61_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr62(&self) -> MR62_R {
        MR62_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr63(&self) -> MR63_R {
        MR63_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr32(&mut self) -> MR32_W {
        MR32_W { w: self }
    }
    #[doc = "Bit 1 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr33(&mut self) -> MR33_W {
        MR33_W { w: self }
    }
    #[doc = "Bit 2 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr34(&mut self) -> MR34_W {
        MR34_W { w: self }
    }
    #[doc = "Bit 3 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr35(&mut self) -> MR35_W {
        MR35_W { w: self }
    }
    #[doc = "Bit 4 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr36(&mut self) -> MR36_W {
        MR36_W { w: self }
    }
    #[doc = "Bit 5 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr37(&mut self) -> MR37_W {
        MR37_W { w: self }
    }
    #[doc = "Bit 6 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr38(&mut self) -> MR38_W {
        MR38_W { w: self }
    }
    #[doc = "Bit 7 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr39(&mut self) -> MR39_W {
        MR39_W { w: self }
    }
    #[doc = "Bit 8 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr40(&mut self) -> MR40_W {
        MR40_W { w: self }
    }
    #[doc = "Bit 9 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr41(&mut self) -> MR41_W {
        MR41_W { w: self }
    }
    #[doc = "Bit 10 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr42(&mut self) -> MR42_W {
        MR42_W { w: self }
    }
    #[doc = "Bit 11 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr43(&mut self) -> MR43_W {
        MR43_W { w: self }
    }
    #[doc = "Bit 12 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr44(&mut self) -> MR44_W {
        MR44_W { w: self }
    }
    #[doc = "Bit 14 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr46(&mut self) -> MR46_W {
        MR46_W { w: self }
    }
    #[doc = "Bit 15 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr47(&mut self) -> MR47_W {
        MR47_W { w: self }
    }
    #[doc = "Bit 16 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr48(&mut self) -> MR48_W {
        MR48_W { w: self }
    }
    #[doc = "Bit 17 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr49(&mut self) -> MR49_W {
        MR49_W { w: self }
    }
    #[doc = "Bit 18 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr50(&mut self) -> MR50_W {
        MR50_W { w: self }
    }
    #[doc = "Bit 19 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr51(&mut self) -> MR51_W {
        MR51_W { w: self }
    }
    #[doc = "Bit 20 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr52(&mut self) -> MR52_W {
        MR52_W { w: self }
    }
    #[doc = "Bit 21 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr53(&mut self) -> MR53_W {
        MR53_W { w: self }
    }
    #[doc = "Bit 22 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr54(&mut self) -> MR54_W {
        MR54_W { w: self }
    }
    #[doc = "Bit 23 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr55(&mut self) -> MR55_W {
        MR55_W { w: self }
    }
    #[doc = "Bit 24 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr56(&mut self) -> MR56_W {
        MR56_W { w: self }
    }
    #[doc = "Bit 25 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr57(&mut self) -> MR57_W {
        MR57_W { w: self }
    }
    #[doc = "Bit 26 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr58(&mut self) -> MR58_W {
        MR58_W { w: self }
    }
    #[doc = "Bit 27 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr59(&mut self) -> MR59_W {
        MR59_W { w: self }
    }
    #[doc = "Bit 28 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr60(&mut self) -> MR60_W {
        MR60_W { w: self }
    }
    #[doc = "Bit 29 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr61(&mut self) -> MR61_W {
        MR61_W { w: self }
    }
    #[doc = "Bit 30 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr62(&mut self) -> MR62_W {
        MR62_W { w: self }
    }
    #[doc = "Bit 31 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr63(&mut self) -> MR63_W {
        MR63_W { w: self }
    }
}
