#[doc = "Reader of register IR"]
pub type R = crate::R<u32, super::IR>;
#[doc = "Reader of field `RF0N`"]
pub type RF0N_R = crate::R<bool, bool>;
#[doc = "Reader of field `RF0W`"]
pub type RF0W_R = crate::R<bool, bool>;
#[doc = "Reader of field `RF0F`"]
pub type RF0F_R = crate::R<bool, bool>;
#[doc = "Reader of field `RF0L`"]
pub type RF0L_R = crate::R<bool, bool>;
#[doc = "Reader of field `RF1N`"]
pub type RF1N_R = crate::R<bool, bool>;
#[doc = "Reader of field `RF1W`"]
pub type RF1W_R = crate::R<bool, bool>;
#[doc = "Reader of field `RF1F`"]
pub type RF1F_R = crate::R<bool, bool>;
#[doc = "Reader of field `RF1L`"]
pub type RF1L_R = crate::R<bool, bool>;
#[doc = "Reader of field `HPM`"]
pub type HPM_R = crate::R<bool, bool>;
#[doc = "Reader of field `TC`"]
pub type TC_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCF`"]
pub type TCF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEF`"]
pub type TEF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEFN`"]
pub type TEFN_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEFW`"]
pub type TEFW_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEFF`"]
pub type TEFF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEFL`"]
pub type TEFL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSW`"]
pub type TSW_R = crate::R<bool, bool>;
#[doc = "Reader of field `MRAF`"]
pub type MRAF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TOO`"]
pub type TOO_R = crate::R<bool, bool>;
#[doc = "Reader of field `DRX`"]
pub type DRX_R = crate::R<bool, bool>;
#[doc = "Reader of field `ELO`"]
pub type ELO_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP`"]
pub type EP_R = crate::R<bool, bool>;
#[doc = "Reader of field `EW`"]
pub type EW_R = crate::R<bool, bool>;
#[doc = "Reader of field `BO`"]
pub type BO_R = crate::R<bool, bool>;
#[doc = "Reader of field `WDI`"]
pub type WDI_R = crate::R<bool, bool>;
#[doc = "Reader of field `PEA`"]
pub type PEA_R = crate::R<bool, bool>;
#[doc = "Reader of field `PED`"]
pub type PED_R = crate::R<bool, bool>;
#[doc = "Reader of field `ARA`"]
pub type ARA_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 New Message"]
    #[inline(always)]
    pub fn rf0n(&self) -> RF0N_R {
        RF0N_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Full"]
    #[inline(always)]
    pub fn rf0w(&self) -> RF0W_R {
        RF0W_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full"]
    #[inline(always)]
    pub fn rf0f(&self) -> RF0F_R {
        RF0F_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost"]
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message"]
    #[inline(always)]
    pub fn rf1n(&self) -> RF1N_R {
        RF1N_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached"]
    #[inline(always)]
    pub fn rf1w(&self) -> RF1W_R {
        RF1W_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO 1 Watermark Reached"]
    #[inline(always)]
    pub fn rf1f(&self) -> RF1F_R {
        RF1F_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost"]
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - High Priority Message"]
    #[inline(always)]
    pub fn hpm(&self) -> HPM_R {
        HPM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmission Completed"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Tx FIFO Empty"]
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry"]
    #[inline(always)]
    pub fn tefn(&self) -> TEFN_R {
        TEFN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached"]
    #[inline(always)]
    pub fn tefw(&self) -> TEFW_R {
        TEFW_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full"]
    #[inline(always)]
    pub fn teff(&self) -> TEFF_R {
        TEFF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost"]
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Timestamp Wraparound"]
    #[inline(always)]
    pub fn tsw(&self) -> TSW_R {
        TSW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Message RAM Access Failure"]
    #[inline(always)]
    pub fn mraf(&self) -> MRAF_R {
        MRAF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Timeout Occurred"]
    #[inline(always)]
    pub fn too(&self) -> TOO_R {
        TOO_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Message stored to Dedicated Rx Buffer"]
    #[inline(always)]
    pub fn drx(&self) -> DRX_R {
        DRX_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Error Logging Overflow"]
    #[inline(always)]
    pub fn elo(&self) -> ELO_R {
        ELO_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Error Passive"]
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Warning Status"]
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Bus_Off Status"]
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Watchdog Interrupt"]
    #[inline(always)]
    pub fn wdi(&self) -> WDI_R {
        WDI_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase (Nominal Bit Time is used)"]
    #[inline(always)]
    pub fn pea(&self) -> PEA_R {
        PEA_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase (Data Bit Time is used)"]
    #[inline(always)]
    pub fn ped(&self) -> PED_R {
        PED_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Access to Reserved Address"]
    #[inline(always)]
    pub fn ara(&self) -> ARA_R {
        ARA_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
