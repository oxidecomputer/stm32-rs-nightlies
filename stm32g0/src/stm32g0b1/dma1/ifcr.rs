#[doc = "Reader of register IFCR"]
pub type R = crate::R<u32, super::IFCR>;
#[doc = "Reader of field `CGIF1`"]
pub type CGIF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTCIF1`"]
pub type CTCIF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHTIF1`"]
pub type CHTIF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTEIF1`"]
pub type CTEIF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CGIF2`"]
pub type CGIF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTCIF2`"]
pub type CTCIF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHTIF2`"]
pub type CHTIF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTEIF2`"]
pub type CTEIF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CGIF3`"]
pub type CGIF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTCIF3`"]
pub type CTCIF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHTIF3`"]
pub type CHTIF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTEIF3`"]
pub type CTEIF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `CGIF4`"]
pub type CGIF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTCIF4`"]
pub type CTCIF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHTIF4`"]
pub type CHTIF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTEIF15`"]
pub type CTEIF15_R = crate::R<bool, bool>;
#[doc = "Reader of field `CGIF5`"]
pub type CGIF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTCIF5`"]
pub type CTCIF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHTIF5`"]
pub type CHTIF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTEIF5`"]
pub type CTEIF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `CGIF6`"]
pub type CGIF6_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTCIF6`"]
pub type CTCIF6_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHTIF6`"]
pub type CHTIF6_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTEIF6`"]
pub type CTEIF6_R = crate::R<bool, bool>;
#[doc = "Reader of field `CGIF7`"]
pub type CGIF7_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTCIF7`"]
pub type CTCIF7_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHTIF7`"]
pub type CHTIF7_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTEIF7`"]
pub type CTEIF7_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Clear channel 1 global interrupt flag"]
    #[inline(always)]
    pub fn cgif1(&self) -> CGIF1_R {
        CGIF1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clear channel 1 transfer complete flag"]
    #[inline(always)]
    pub fn ctcif1(&self) -> CTCIF1_R {
        CTCIF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clear channel 1 half transfer flag"]
    #[inline(always)]
    pub fn chtif1(&self) -> CHTIF1_R {
        CHTIF1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clear channel 1 transfer error flag"]
    #[inline(always)]
    pub fn cteif1(&self) -> CTEIF1_R {
        CTEIF1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Clear channel 2 global interrupt flag"]
    #[inline(always)]
    pub fn cgif2(&self) -> CGIF2_R {
        CGIF2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clear channel 2 transfer complete flag"]
    #[inline(always)]
    pub fn ctcif2(&self) -> CTCIF2_R {
        CTCIF2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Clear channel 2 half transfer flag"]
    #[inline(always)]
    pub fn chtif2(&self) -> CHTIF2_R {
        CHTIF2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Clear channel 2 transfer error flag"]
    #[inline(always)]
    pub fn cteif2(&self) -> CTEIF2_R {
        CTEIF2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Clear channel 3 global interrupt flag"]
    #[inline(always)]
    pub fn cgif3(&self) -> CGIF3_R {
        CGIF3_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Clear channel 3 transfer complete flag"]
    #[inline(always)]
    pub fn ctcif3(&self) -> CTCIF3_R {
        CTCIF3_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Clear channel 3 half transfer flag"]
    #[inline(always)]
    pub fn chtif3(&self) -> CHTIF3_R {
        CHTIF3_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Clear channel 3 transfer error flag"]
    #[inline(always)]
    pub fn cteif3(&self) -> CTEIF3_R {
        CTEIF3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Clear channel 4 global interrupt flag"]
    #[inline(always)]
    pub fn cgif4(&self) -> CGIF4_R {
        CGIF4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Clear channel 4 transfer complete flag"]
    #[inline(always)]
    pub fn ctcif4(&self) -> CTCIF4_R {
        CTCIF4_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Clear channel 4 half transfer flag"]
    #[inline(always)]
    pub fn chtif4(&self) -> CHTIF4_R {
        CHTIF4_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Clear channel 4 transfer error flag"]
    #[inline(always)]
    pub fn cteif15(&self) -> CTEIF15_R {
        CTEIF15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Clear channel 5 global interrupt flag"]
    #[inline(always)]
    pub fn cgif5(&self) -> CGIF5_R {
        CGIF5_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Clear channel 5 transfer complete flag"]
    #[inline(always)]
    pub fn ctcif5(&self) -> CTCIF5_R {
        CTCIF5_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Clear channel 5 half transfer flag"]
    #[inline(always)]
    pub fn chtif5(&self) -> CHTIF5_R {
        CHTIF5_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Clear channel 5 transfer error flag"]
    #[inline(always)]
    pub fn cteif5(&self) -> CTEIF5_R {
        CTEIF5_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Clear channel 6 global interrupt flag"]
    #[inline(always)]
    pub fn cgif6(&self) -> CGIF6_R {
        CGIF6_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Clear channel 6 transfer complete flag"]
    #[inline(always)]
    pub fn ctcif6(&self) -> CTCIF6_R {
        CTCIF6_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Clear channel 6 half transfer flag"]
    #[inline(always)]
    pub fn chtif6(&self) -> CHTIF6_R {
        CHTIF6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Clear channel 6 transfer error flag"]
    #[inline(always)]
    pub fn cteif6(&self) -> CTEIF6_R {
        CTEIF6_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Clear channel 7 global interrupt flag"]
    #[inline(always)]
    pub fn cgif7(&self) -> CGIF7_R {
        CGIF7_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Clear channel 7 transfer complete flag"]
    #[inline(always)]
    pub fn ctcif7(&self) -> CTCIF7_R {
        CTCIF7_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Clear channel 7 half transfer flag"]
    #[inline(always)]
    pub fn chtif7(&self) -> CHTIF7_R {
        CHTIF7_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Clear channel 7 transfer error flag"]
    #[inline(always)]
    pub fn cteif7(&self) -> CTEIF7_R {
        CTEIF7_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
