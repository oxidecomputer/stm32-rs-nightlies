#[doc = "Reader of register NDAT1"]
pub type R = crate::R<u32, super::NDAT1>;
#[doc = "Reader of field `ND0`"]
pub type ND0_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND1`"]
pub type ND1_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND2`"]
pub type ND2_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND3`"]
pub type ND3_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND4`"]
pub type ND4_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND5`"]
pub type ND5_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND6`"]
pub type ND6_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND7`"]
pub type ND7_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND8`"]
pub type ND8_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND9`"]
pub type ND9_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND10`"]
pub type ND10_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND11`"]
pub type ND11_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND12`"]
pub type ND12_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND13`"]
pub type ND13_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND14`"]
pub type ND14_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND15`"]
pub type ND15_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND16`"]
pub type ND16_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND17`"]
pub type ND17_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND18`"]
pub type ND18_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND19`"]
pub type ND19_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND20`"]
pub type ND20_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND21`"]
pub type ND21_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND22`"]
pub type ND22_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND23`"]
pub type ND23_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND24`"]
pub type ND24_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND25`"]
pub type ND25_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND26`"]
pub type ND26_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND27`"]
pub type ND27_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND28`"]
pub type ND28_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND29`"]
pub type ND29_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND30`"]
pub type ND30_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND31`"]
pub type ND31_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - New data"]
    #[inline(always)]
    pub fn nd0(&self) -> ND0_R {
        ND0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - New data"]
    #[inline(always)]
    pub fn nd1(&self) -> ND1_R {
        ND1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - New data"]
    #[inline(always)]
    pub fn nd2(&self) -> ND2_R {
        ND2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - New data"]
    #[inline(always)]
    pub fn nd3(&self) -> ND3_R {
        ND3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - New data"]
    #[inline(always)]
    pub fn nd4(&self) -> ND4_R {
        ND4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - New data"]
    #[inline(always)]
    pub fn nd5(&self) -> ND5_R {
        ND5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - New data"]
    #[inline(always)]
    pub fn nd6(&self) -> ND6_R {
        ND6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - New data"]
    #[inline(always)]
    pub fn nd7(&self) -> ND7_R {
        ND7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - New data"]
    #[inline(always)]
    pub fn nd8(&self) -> ND8_R {
        ND8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - New data"]
    #[inline(always)]
    pub fn nd9(&self) -> ND9_R {
        ND9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - New data"]
    #[inline(always)]
    pub fn nd10(&self) -> ND10_R {
        ND10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - New data"]
    #[inline(always)]
    pub fn nd11(&self) -> ND11_R {
        ND11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - New data"]
    #[inline(always)]
    pub fn nd12(&self) -> ND12_R {
        ND12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - New data"]
    #[inline(always)]
    pub fn nd13(&self) -> ND13_R {
        ND13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - New data"]
    #[inline(always)]
    pub fn nd14(&self) -> ND14_R {
        ND14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - New data"]
    #[inline(always)]
    pub fn nd15(&self) -> ND15_R {
        ND15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - New data"]
    #[inline(always)]
    pub fn nd16(&self) -> ND16_R {
        ND16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - New data"]
    #[inline(always)]
    pub fn nd17(&self) -> ND17_R {
        ND17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - New data"]
    #[inline(always)]
    pub fn nd18(&self) -> ND18_R {
        ND18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - New data"]
    #[inline(always)]
    pub fn nd19(&self) -> ND19_R {
        ND19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - New data"]
    #[inline(always)]
    pub fn nd20(&self) -> ND20_R {
        ND20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - New data"]
    #[inline(always)]
    pub fn nd21(&self) -> ND21_R {
        ND21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - New data"]
    #[inline(always)]
    pub fn nd22(&self) -> ND22_R {
        ND22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - New data"]
    #[inline(always)]
    pub fn nd23(&self) -> ND23_R {
        ND23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - New data"]
    #[inline(always)]
    pub fn nd24(&self) -> ND24_R {
        ND24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - New data"]
    #[inline(always)]
    pub fn nd25(&self) -> ND25_R {
        ND25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - New data"]
    #[inline(always)]
    pub fn nd26(&self) -> ND26_R {
        ND26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - New data"]
    #[inline(always)]
    pub fn nd27(&self) -> ND27_R {
        ND27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - New data"]
    #[inline(always)]
    pub fn nd28(&self) -> ND28_R {
        ND28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - New data"]
    #[inline(always)]
    pub fn nd29(&self) -> ND29_R {
        ND29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - New data"]
    #[inline(always)]
    pub fn nd30(&self) -> ND30_R {
        ND30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - New data"]
    #[inline(always)]
    pub fn nd31(&self) -> ND31_R {
        ND31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
