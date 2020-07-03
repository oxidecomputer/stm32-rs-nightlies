#[doc = "Reader of register IE"]
pub type R = crate::R<u32, super::IE>;
#[doc = "Reader of field `RF0NE`"]
pub type RF0NE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RF0WE`"]
pub type RF0WE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RF0FE`"]
pub type RF0FE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RF0LE`"]
pub type RF0LE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RF1NE`"]
pub type RF1NE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RF1WE`"]
pub type RF1WE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RF1FE`"]
pub type RF1FE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RF1LE`"]
pub type RF1LE_R = crate::R<bool, bool>;
#[doc = "Reader of field `HPME`"]
pub type HPME_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCE`"]
pub type TCE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCFE`"]
pub type TCFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEFE`"]
pub type TEFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEFNE`"]
pub type TEFNE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEFWE`"]
pub type TEFWE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEFFE`"]
pub type TEFFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEFLE`"]
pub type TEFLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSWE`"]
pub type TSWE_R = crate::R<bool, bool>;
#[doc = "Reader of field `MRAFE`"]
pub type MRAFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TOOE`"]
pub type TOOE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DRXE`"]
pub type DRXE_R = crate::R<bool, bool>;
#[doc = "Reader of field `BECE`"]
pub type BECE_R = crate::R<bool, bool>;
#[doc = "Reader of field `BEUE`"]
pub type BEUE_R = crate::R<bool, bool>;
#[doc = "Reader of field `ELOE`"]
pub type ELOE_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPE`"]
pub type EPE_R = crate::R<bool, bool>;
#[doc = "Reader of field `EWE`"]
pub type EWE_R = crate::R<bool, bool>;
#[doc = "Reader of field `BOE`"]
pub type BOE_R = crate::R<bool, bool>;
#[doc = "Reader of field `WDIE`"]
pub type WDIE_R = crate::R<bool, bool>;
#[doc = "Reader of field `PEAE`"]
pub type PEAE_R = crate::R<bool, bool>;
#[doc = "Reader of field `PEDE`"]
pub type PEDE_R = crate::R<bool, bool>;
#[doc = "Reader of field `ARAE`"]
pub type ARAE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 New Message Enable"]
    #[inline(always)]
    pub fn rf0ne(&self) -> RF0NE_R {
        RF0NE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Full Enable"]
    #[inline(always)]
    pub fn rf0we(&self) -> RF0WE_R {
        RF0WE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full Enable"]
    #[inline(always)]
    pub fn rf0fe(&self) -> RF0FE_R {
        RF0FE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost Enable"]
    #[inline(always)]
    pub fn rf0le(&self) -> RF0LE_R {
        RF0LE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message Enable"]
    #[inline(always)]
    pub fn rf1ne(&self) -> RF1NE_R {
        RF1NE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached Enable"]
    #[inline(always)]
    pub fn rf1we(&self) -> RF1WE_R {
        RF1WE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO 1 Watermark Reached Enable"]
    #[inline(always)]
    pub fn rf1fe(&self) -> RF1FE_R {
        RF1FE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost Enable"]
    #[inline(always)]
    pub fn rf1le(&self) -> RF1LE_R {
        RF1LE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - High Priority Message Enable"]
    #[inline(always)]
    pub fn hpme(&self) -> HPME_R {
        HPME_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmission Completed Enable"]
    #[inline(always)]
    pub fn tce(&self) -> TCE_R {
        TCE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished Enable"]
    #[inline(always)]
    pub fn tcfe(&self) -> TCFE_R {
        TCFE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Tx FIFO Empty Enable"]
    #[inline(always)]
    pub fn tefe(&self) -> TEFE_R {
        TEFE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry Enable"]
    #[inline(always)]
    pub fn tefne(&self) -> TEFNE_R {
        TEFNE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached Enable"]
    #[inline(always)]
    pub fn tefwe(&self) -> TEFWE_R {
        TEFWE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full Enable"]
    #[inline(always)]
    pub fn teffe(&self) -> TEFFE_R {
        TEFFE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost Enable"]
    #[inline(always)]
    pub fn tefle(&self) -> TEFLE_R {
        TEFLE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Timestamp Wraparound Enable"]
    #[inline(always)]
    pub fn tswe(&self) -> TSWE_R {
        TSWE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Message RAM Access Failure Enable"]
    #[inline(always)]
    pub fn mrafe(&self) -> MRAFE_R {
        MRAFE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Timeout Occurred Enable"]
    #[inline(always)]
    pub fn tooe(&self) -> TOOE_R {
        TOOE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Message stored to Dedicated Rx Buffer Enable"]
    #[inline(always)]
    pub fn drxe(&self) -> DRXE_R {
        DRXE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Bit Error Corrected Interrupt Enable"]
    #[inline(always)]
    pub fn bece(&self) -> BECE_R {
        BECE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Bit Error Uncorrected Interrupt Enable"]
    #[inline(always)]
    pub fn beue(&self) -> BEUE_R {
        BEUE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Error Logging Overflow Enable"]
    #[inline(always)]
    pub fn eloe(&self) -> ELOE_R {
        ELOE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Error Passive Enable"]
    #[inline(always)]
    pub fn epe(&self) -> EPE_R {
        EPE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Warning Status Enable"]
    #[inline(always)]
    pub fn ewe(&self) -> EWE_R {
        EWE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Bus_Off Status Enable"]
    #[inline(always)]
    pub fn boe(&self) -> BOE_R {
        BOE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn wdie(&self) -> WDIE_R {
        WDIE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase Enable"]
    #[inline(always)]
    pub fn peae(&self) -> PEAE_R {
        PEAE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase Enable"]
    #[inline(always)]
    pub fn pede(&self) -> PEDE_R {
        PEDE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Access to Reserved Address Enable"]
    #[inline(always)]
    pub fn arae(&self) -> ARAE_R {
        ARAE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
