#[doc = "Reader of register ILS"]
pub type R = crate::R<u32, super::ILS>;
#[doc = "Reader of field `RF0NL`"]
pub type RF0NL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RF0WL`"]
pub type RF0WL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RF0FL`"]
pub type RF0FL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RF0LL`"]
pub type RF0LL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RF1NL`"]
pub type RF1NL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RF1WL`"]
pub type RF1WL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RF1FL`"]
pub type RF1FL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RF1LL`"]
pub type RF1LL_R = crate::R<bool, bool>;
#[doc = "Reader of field `HPML`"]
pub type HPML_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCL`"]
pub type TCL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCFL`"]
pub type TCFL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEFL`"]
pub type TEFL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEFNL`"]
pub type TEFNL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEFWL`"]
pub type TEFWL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEFFL`"]
pub type TEFFL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEFLL`"]
pub type TEFLL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSWL`"]
pub type TSWL_R = crate::R<bool, bool>;
#[doc = "Reader of field `MRAFL`"]
pub type MRAFL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TOOL`"]
pub type TOOL_R = crate::R<bool, bool>;
#[doc = "Reader of field `DRXL`"]
pub type DRXL_R = crate::R<bool, bool>;
#[doc = "Reader of field `BECL`"]
pub type BECL_R = crate::R<bool, bool>;
#[doc = "Reader of field `BEUL`"]
pub type BEUL_R = crate::R<bool, bool>;
#[doc = "Reader of field `ELOL`"]
pub type ELOL_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPL`"]
pub type EPL_R = crate::R<bool, bool>;
#[doc = "Reader of field `EWL`"]
pub type EWL_R = crate::R<bool, bool>;
#[doc = "Reader of field `BOL`"]
pub type BOL_R = crate::R<bool, bool>;
#[doc = "Reader of field `WDIL`"]
pub type WDIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `PEAL`"]
pub type PEAL_R = crate::R<bool, bool>;
#[doc = "Reader of field `PEDL`"]
pub type PEDL_R = crate::R<bool, bool>;
#[doc = "Reader of field `ARAL`"]
pub type ARAL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 New Message Interrupt Line"]
    #[inline(always)]
    pub fn rf0nl(&self) -> RF0NL_R {
        RF0NL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Watermark Reached Interrupt Line"]
    #[inline(always)]
    pub fn rf0wl(&self) -> RF0WL_R {
        RF0WL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full Interrupt Line"]
    #[inline(always)]
    pub fn rf0fl(&self) -> RF0FL_R {
        RF0FL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost Interrupt Line"]
    #[inline(always)]
    pub fn rf0ll(&self) -> RF0LL_R {
        RF0LL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message Interrupt Line"]
    #[inline(always)]
    pub fn rf1nl(&self) -> RF1NL_R {
        RF1NL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached Interrupt Line"]
    #[inline(always)]
    pub fn rf1wl(&self) -> RF1WL_R {
        RF1WL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO 1 Full Interrupt Line"]
    #[inline(always)]
    pub fn rf1fl(&self) -> RF1FL_R {
        RF1FL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost Interrupt Line"]
    #[inline(always)]
    pub fn rf1ll(&self) -> RF1LL_R {
        RF1LL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - High Priority Message Interrupt Line"]
    #[inline(always)]
    pub fn hpml(&self) -> HPML_R {
        HPML_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmission Completed Interrupt Line"]
    #[inline(always)]
    pub fn tcl(&self) -> TCL_R {
        TCL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished Interrupt Line"]
    #[inline(always)]
    pub fn tcfl(&self) -> TCFL_R {
        TCFL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Tx FIFO Empty Interrupt Line"]
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry Interrupt Line"]
    #[inline(always)]
    pub fn tefnl(&self) -> TEFNL_R {
        TEFNL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached Interrupt Line"]
    #[inline(always)]
    pub fn tefwl(&self) -> TEFWL_R {
        TEFWL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full Interrupt Line"]
    #[inline(always)]
    pub fn teffl(&self) -> TEFFL_R {
        TEFFL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost Interrupt Line"]
    #[inline(always)]
    pub fn tefll(&self) -> TEFLL_R {
        TEFLL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Timestamp Wraparound Interrupt Line"]
    #[inline(always)]
    pub fn tswl(&self) -> TSWL_R {
        TSWL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Message RAM Access Failure Interrupt Line"]
    #[inline(always)]
    pub fn mrafl(&self) -> MRAFL_R {
        MRAFL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Timeout Occurred Interrupt Line"]
    #[inline(always)]
    pub fn tool(&self) -> TOOL_R {
        TOOL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Message stored to Dedicated Rx Buffer Interrupt Line"]
    #[inline(always)]
    pub fn drxl(&self) -> DRXL_R {
        DRXL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Bit Error Corrected Interrupt Line"]
    #[inline(always)]
    pub fn becl(&self) -> BECL_R {
        BECL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Bit Error Uncorrected Interrupt Line"]
    #[inline(always)]
    pub fn beul(&self) -> BEUL_R {
        BEUL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Error Logging Overflow Interrupt Line"]
    #[inline(always)]
    pub fn elol(&self) -> ELOL_R {
        ELOL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Error Passive Interrupt Line"]
    #[inline(always)]
    pub fn epl(&self) -> EPL_R {
        EPL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Warning Status Interrupt Line"]
    #[inline(always)]
    pub fn ewl(&self) -> EWL_R {
        EWL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Bus_Off Status"]
    #[inline(always)]
    pub fn bol(&self) -> BOL_R {
        BOL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Watchdog Interrupt Line"]
    #[inline(always)]
    pub fn wdil(&self) -> WDIL_R {
        WDIL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase Line"]
    #[inline(always)]
    pub fn peal(&self) -> PEAL_R {
        PEAL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase Line"]
    #[inline(always)]
    pub fn pedl(&self) -> PEDL_R {
        PEDL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Access to Reserved Address Line"]
    #[inline(always)]
    pub fn aral(&self) -> ARAL_R {
        ARAL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
