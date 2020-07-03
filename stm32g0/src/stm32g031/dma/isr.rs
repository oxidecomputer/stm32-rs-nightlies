#[doc = "Reader of register ISR"]
pub type R = crate::R<u32, super::ISR>;
#[doc = "Reader of field `GIF0`"]
pub type GIF0_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF1`"]
pub type TCIF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTIF2`"]
pub type HTIF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEIF3`"]
pub type TEIF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF4`"]
pub type GIF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF5`"]
pub type TCIF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTIF6`"]
pub type HTIF6_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEIF7`"]
pub type TEIF7_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF8`"]
pub type GIF8_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF9`"]
pub type TCIF9_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTIF10`"]
pub type HTIF10_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEIF11`"]
pub type TEIF11_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF12`"]
pub type GIF12_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF13`"]
pub type TCIF13_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTIF14`"]
pub type HTIF14_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEIF15`"]
pub type TEIF15_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF16`"]
pub type GIF16_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF17`"]
pub type TCIF17_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTIF18`"]
pub type HTIF18_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEIF19`"]
pub type TEIF19_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF20`"]
pub type GIF20_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF21`"]
pub type TCIF21_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTIF22`"]
pub type HTIF22_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEIF23`"]
pub type TEIF23_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF24`"]
pub type GIF24_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF25`"]
pub type TCIF25_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTIF26`"]
pub type HTIF26_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEIF27`"]
pub type TEIF27_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Channel global interrupt flag"]
    #[inline(always)]
    pub fn gif0(&self) -> GIF0_R {
        GIF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel transfer complete flag"]
    #[inline(always)]
    pub fn tcif1(&self) -> TCIF1_R {
        TCIF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel half transfer flag"]
    #[inline(always)]
    pub fn htif2(&self) -> HTIF2_R {
        HTIF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel transfer error flag"]
    #[inline(always)]
    pub fn teif3(&self) -> TEIF3_R {
        TEIF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel global interrupt flag"]
    #[inline(always)]
    pub fn gif4(&self) -> GIF4_R {
        GIF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel transfer complete flag"]
    #[inline(always)]
    pub fn tcif5(&self) -> TCIF5_R {
        TCIF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel half transfer flag"]
    #[inline(always)]
    pub fn htif6(&self) -> HTIF6_R {
        HTIF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel transfer error flag"]
    #[inline(always)]
    pub fn teif7(&self) -> TEIF7_R {
        TEIF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel global interrupt flag"]
    #[inline(always)]
    pub fn gif8(&self) -> GIF8_R {
        GIF8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel transfer complete flag"]
    #[inline(always)]
    pub fn tcif9(&self) -> TCIF9_R {
        TCIF9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Channel half transfer flag"]
    #[inline(always)]
    pub fn htif10(&self) -> HTIF10_R {
        HTIF10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Channel transfer error flag"]
    #[inline(always)]
    pub fn teif11(&self) -> TEIF11_R {
        TEIF11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Channel global interrupt flag"]
    #[inline(always)]
    pub fn gif12(&self) -> GIF12_R {
        GIF12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Channel transfer complete flag"]
    #[inline(always)]
    pub fn tcif13(&self) -> TCIF13_R {
        TCIF13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Channel half transfer flag"]
    #[inline(always)]
    pub fn htif14(&self) -> HTIF14_R {
        HTIF14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Channel transfer error flag"]
    #[inline(always)]
    pub fn teif15(&self) -> TEIF15_R {
        TEIF15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Channel global interrupt flag"]
    #[inline(always)]
    pub fn gif16(&self) -> GIF16_R {
        GIF16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Channel transfer complete flag"]
    #[inline(always)]
    pub fn tcif17(&self) -> TCIF17_R {
        TCIF17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Channel half transfer flag"]
    #[inline(always)]
    pub fn htif18(&self) -> HTIF18_R {
        HTIF18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Channel transfer error flag"]
    #[inline(always)]
    pub fn teif19(&self) -> TEIF19_R {
        TEIF19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Channel global interrupt flag"]
    #[inline(always)]
    pub fn gif20(&self) -> GIF20_R {
        GIF20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Channel transfer complete flag"]
    #[inline(always)]
    pub fn tcif21(&self) -> TCIF21_R {
        TCIF21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Channel half transfer flag"]
    #[inline(always)]
    pub fn htif22(&self) -> HTIF22_R {
        HTIF22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Channel transfer error flag"]
    #[inline(always)]
    pub fn teif23(&self) -> TEIF23_R {
        TEIF23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Channel global interrupt flag"]
    #[inline(always)]
    pub fn gif24(&self) -> GIF24_R {
        GIF24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Channel transfer complete flag"]
    #[inline(always)]
    pub fn tcif25(&self) -> TCIF25_R {
        TCIF25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Channel half transfer flag"]
    #[inline(always)]
    pub fn htif26(&self) -> HTIF26_R {
        HTIF26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Channel transfer error flag"]
    #[inline(always)]
    pub fn teif27(&self) -> TEIF27_R {
        TEIF27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
