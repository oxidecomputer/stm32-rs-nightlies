#[doc = "Reader of register FDCAN_NDAT2"]
pub type R = crate::R<u32, super::FDCAN_NDAT2>;
#[doc = "Reader of field `ND32`"]
pub type ND32_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND33`"]
pub type ND33_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND34`"]
pub type ND34_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND35`"]
pub type ND35_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND36`"]
pub type ND36_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND37`"]
pub type ND37_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND38`"]
pub type ND38_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND39`"]
pub type ND39_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND40`"]
pub type ND40_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND41`"]
pub type ND41_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND42`"]
pub type ND42_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND43`"]
pub type ND43_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND44`"]
pub type ND44_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND45`"]
pub type ND45_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND46`"]
pub type ND46_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND47`"]
pub type ND47_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND48`"]
pub type ND48_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND49`"]
pub type ND49_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND50`"]
pub type ND50_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND51`"]
pub type ND51_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND52`"]
pub type ND52_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND53`"]
pub type ND53_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND54`"]
pub type ND54_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND55`"]
pub type ND55_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND56`"]
pub type ND56_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND57`"]
pub type ND57_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND58`"]
pub type ND58_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND59`"]
pub type ND59_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND60`"]
pub type ND60_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND61`"]
pub type ND61_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND62`"]
pub type ND62_R = crate::R<bool, bool>;
#[doc = "Reader of field `ND63`"]
pub type ND63_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - New data"]
    #[inline(always)]
    pub fn nd32(&self) -> ND32_R {
        ND32_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - New data"]
    #[inline(always)]
    pub fn nd33(&self) -> ND33_R {
        ND33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - New data"]
    #[inline(always)]
    pub fn nd34(&self) -> ND34_R {
        ND34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - New data"]
    #[inline(always)]
    pub fn nd35(&self) -> ND35_R {
        ND35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - New data"]
    #[inline(always)]
    pub fn nd36(&self) -> ND36_R {
        ND36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - New data"]
    #[inline(always)]
    pub fn nd37(&self) -> ND37_R {
        ND37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - New data"]
    #[inline(always)]
    pub fn nd38(&self) -> ND38_R {
        ND38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - New data"]
    #[inline(always)]
    pub fn nd39(&self) -> ND39_R {
        ND39_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - New data"]
    #[inline(always)]
    pub fn nd40(&self) -> ND40_R {
        ND40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - New data"]
    #[inline(always)]
    pub fn nd41(&self) -> ND41_R {
        ND41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - New data"]
    #[inline(always)]
    pub fn nd42(&self) -> ND42_R {
        ND42_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - New data"]
    #[inline(always)]
    pub fn nd43(&self) -> ND43_R {
        ND43_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - New data"]
    #[inline(always)]
    pub fn nd44(&self) -> ND44_R {
        ND44_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - New data"]
    #[inline(always)]
    pub fn nd45(&self) -> ND45_R {
        ND45_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - New data"]
    #[inline(always)]
    pub fn nd46(&self) -> ND46_R {
        ND46_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - New data"]
    #[inline(always)]
    pub fn nd47(&self) -> ND47_R {
        ND47_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - New data"]
    #[inline(always)]
    pub fn nd48(&self) -> ND48_R {
        ND48_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - New data"]
    #[inline(always)]
    pub fn nd49(&self) -> ND49_R {
        ND49_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - New data"]
    #[inline(always)]
    pub fn nd50(&self) -> ND50_R {
        ND50_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - New data"]
    #[inline(always)]
    pub fn nd51(&self) -> ND51_R {
        ND51_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - New data"]
    #[inline(always)]
    pub fn nd52(&self) -> ND52_R {
        ND52_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - New data"]
    #[inline(always)]
    pub fn nd53(&self) -> ND53_R {
        ND53_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - New data"]
    #[inline(always)]
    pub fn nd54(&self) -> ND54_R {
        ND54_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - New data"]
    #[inline(always)]
    pub fn nd55(&self) -> ND55_R {
        ND55_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - New data"]
    #[inline(always)]
    pub fn nd56(&self) -> ND56_R {
        ND56_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - New data"]
    #[inline(always)]
    pub fn nd57(&self) -> ND57_R {
        ND57_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - New data"]
    #[inline(always)]
    pub fn nd58(&self) -> ND58_R {
        ND58_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - New data"]
    #[inline(always)]
    pub fn nd59(&self) -> ND59_R {
        ND59_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - New data"]
    #[inline(always)]
    pub fn nd60(&self) -> ND60_R {
        ND60_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - New data"]
    #[inline(always)]
    pub fn nd61(&self) -> ND61_R {
        ND61_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - New data"]
    #[inline(always)]
    pub fn nd62(&self) -> ND62_R {
        ND62_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - New data"]
    #[inline(always)]
    pub fn nd63(&self) -> ND63_R {
        ND63_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
