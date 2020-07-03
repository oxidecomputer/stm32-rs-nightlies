#[doc = "Reader of register CFGR2"]
pub type R = crate::R<u32, super::CFGR2>;
#[doc = "Writer for register CFGR2"]
pub type W = crate::W<u32, super::CFGR2>;
#[doc = "Register CFGR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PVD_LOCK`"]
pub type PVD_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PVD_LOCK`"]
pub struct PVD_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> PVD_LOCK_W<'a> {
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
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    pub fn pvd_lock(&self) -> PVD_LOCK_R {
        PVD_LOCK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    pub fn pvd_lock(&mut self) -> PVD_LOCK_W {
        PVD_LOCK_W { w: self }
    }
}
