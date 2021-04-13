#[doc = "Reader of register LPTIM1_HWCFGR"]
pub type R = crate::R<u32, super::LPTIM1_HWCFGR>;
#[doc = "Reader of field `CFG1`"]
pub type CFG1_R = crate::R<u8, u8>;
#[doc = "Reader of field `CFG2`"]
pub type CFG2_R = crate::R<u8, u8>;
#[doc = "Reader of field `CFG3`"]
pub type CFG3_R = crate::R<u8, u8>;
#[doc = "Reader of field `CFG4`"]
pub type CFG4_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - CFG1"]
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CFG2"]
    #[inline(always)]
    pub fn cfg2(&self) -> CFG2_R {
        CFG2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - CFG3"]
    #[inline(always)]
    pub fn cfg3(&self) -> CFG3_R {
        CFG3_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - CFG4"]
    #[inline(always)]
    pub fn cfg4(&self) -> CFG4_R {
        CFG4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}