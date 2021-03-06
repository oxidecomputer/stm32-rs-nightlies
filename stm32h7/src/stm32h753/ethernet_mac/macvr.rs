#[doc = "Reader of register MACVR"]
pub type R = crate::R<u32, super::MACVR>;
#[doc = "Reader of field `SNPSVER`"]
pub type SNPSVER_R = crate::R<u8, u8>;
#[doc = "Reader of field `USERVER`"]
pub type USERVER_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - IP version"]
    #[inline(always)]
    pub fn snpsver(&self) -> SNPSVER_R {
        SNPSVER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ST-defined version"]
    #[inline(always)]
    pub fn userver(&self) -> USERVER_R {
        USERVER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
