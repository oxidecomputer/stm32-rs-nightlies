#[doc = "Reader of register ETH_MACRxTxSR"]
pub type R = crate::R<u32, super::ETH_MACRXTXSR>;
#[doc = "Reader of field `TJT`"]
pub type TJT_R = crate::R<bool, bool>;
#[doc = "Reader of field `NCARR`"]
pub type NCARR_R = crate::R<bool, bool>;
#[doc = "Reader of field `LCARR`"]
pub type LCARR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXDEF`"]
pub type EXDEF_R = crate::R<bool, bool>;
#[doc = "Reader of field `LCOL`"]
pub type LCOL_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXCOL`"]
pub type EXCOL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RWT`"]
pub type RWT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - TJT"]
    #[inline(always)]
    pub fn tjt(&self) -> TJT_R {
        TJT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - NCARR"]
    #[inline(always)]
    pub fn ncarr(&self) -> NCARR_R {
        NCARR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LCARR"]
    #[inline(always)]
    pub fn lcarr(&self) -> LCARR_R {
        LCARR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - EXDEF"]
    #[inline(always)]
    pub fn exdef(&self) -> EXDEF_R {
        EXDEF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LCOL"]
    #[inline(always)]
    pub fn lcol(&self) -> LCOL_R {
        LCOL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - EXCOL"]
    #[inline(always)]
    pub fn excol(&self) -> EXCOL_R {
        EXCOL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RWT"]
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
