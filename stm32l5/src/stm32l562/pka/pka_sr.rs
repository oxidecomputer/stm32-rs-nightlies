#[doc = "Reader of register PKA_SR"]
pub type R = crate::R<u32, super::PKA_SR>;
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `PROCENDF`"]
pub type PROCENDF_R = crate::R<bool, bool>;
#[doc = "Reader of field `RAMERRF`"]
pub type RAMERRF_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADDRERRF`"]
pub type ADDRERRF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 16 - PKA operation in progress"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PKA end of operation flag"]
    #[inline(always)]
    pub fn procendf(&self) -> PROCENDF_R {
        PROCENDF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PKA ram error flag"]
    #[inline(always)]
    pub fn ramerrf(&self) -> RAMERRF_R {
        RAMERRF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - address er flag"]
    #[inline(always)]
    pub fn addrerrf(&self) -> ADDRERRF_R {
        ADDRERRF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
