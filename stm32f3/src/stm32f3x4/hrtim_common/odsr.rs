#[doc = "Reader of register ODSR"]
pub type R = crate::R<u32, super::ODSR>;
#[doc = "Timer E Output 2 disable status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TE2ODS_A {
    #[doc = "0: Output disabled in idle state"]
    IDLE = 0,
    #[doc = "1: Output disabled in fault state"]
    FAULT = 1,
}
impl From<TE2ODS_A> for bool {
    #[inline(always)]
    fn from(variant: TE2ODS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TE2ODS`"]
pub type TE2ODS_R = crate::R<bool, TE2ODS_A>;
impl TE2ODS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TE2ODS_A {
        match self.bits {
            false => TE2ODS_A::IDLE,
            true => TE2ODS_A::FAULT,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == TE2ODS_A::IDLE
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == TE2ODS_A::FAULT
    }
}
#[doc = "Timer E Output 1 disable status"]
pub type TE1ODS_A = TE2ODS_A;
#[doc = "Reader of field `TE1ODS`"]
pub type TE1ODS_R = crate::R<bool, TE2ODS_A>;
#[doc = "Timer D Output 2 disable status"]
pub type TD2ODS_A = TE2ODS_A;
#[doc = "Reader of field `TD2ODS`"]
pub type TD2ODS_R = crate::R<bool, TE2ODS_A>;
#[doc = "Timer D Output 1 disable status"]
pub type TD1ODS_A = TE2ODS_A;
#[doc = "Reader of field `TD1ODS`"]
pub type TD1ODS_R = crate::R<bool, TE2ODS_A>;
#[doc = "Timer C Output 2 disable status"]
pub type TC2ODS_A = TE2ODS_A;
#[doc = "Reader of field `TC2ODS`"]
pub type TC2ODS_R = crate::R<bool, TE2ODS_A>;
#[doc = "Timer C Output 1 disable status"]
pub type TC1ODS_A = TE2ODS_A;
#[doc = "Reader of field `TC1ODS`"]
pub type TC1ODS_R = crate::R<bool, TE2ODS_A>;
#[doc = "Timer B Output 2 disable status"]
pub type TB2ODS_A = TE2ODS_A;
#[doc = "Reader of field `TB2ODS`"]
pub type TB2ODS_R = crate::R<bool, TE2ODS_A>;
#[doc = "Timer B Output 1 disable status"]
pub type TB1ODS_A = TE2ODS_A;
#[doc = "Reader of field `TB1ODS`"]
pub type TB1ODS_R = crate::R<bool, TE2ODS_A>;
#[doc = "Timer A Output 2 disable status"]
pub type TA2ODS_A = TE2ODS_A;
#[doc = "Reader of field `TA2ODS`"]
pub type TA2ODS_R = crate::R<bool, TE2ODS_A>;
#[doc = "Timer A Output 1 disable status"]
pub type TA1ODS_A = TE2ODS_A;
#[doc = "Reader of field `TA1ODS`"]
pub type TA1ODS_R = crate::R<bool, TE2ODS_A>;
impl R {
    #[doc = "Bit 9 - Timer E Output 2 disable status"]
    #[inline(always)]
    pub fn te2ods(&self) -> TE2ODS_R {
        TE2ODS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Timer E Output 1 disable status"]
    #[inline(always)]
    pub fn te1ods(&self) -> TE1ODS_R {
        TE1ODS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Timer D Output 2 disable status"]
    #[inline(always)]
    pub fn td2ods(&self) -> TD2ODS_R {
        TD2ODS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Timer D Output 1 disable status"]
    #[inline(always)]
    pub fn td1ods(&self) -> TD1ODS_R {
        TD1ODS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Timer C Output 2 disable status"]
    #[inline(always)]
    pub fn tc2ods(&self) -> TC2ODS_R {
        TC2ODS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Timer C Output 1 disable status"]
    #[inline(always)]
    pub fn tc1ods(&self) -> TC1ODS_R {
        TC1ODS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timer B Output 2 disable status"]
    #[inline(always)]
    pub fn tb2ods(&self) -> TB2ODS_R {
        TB2ODS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timer B Output 1 disable status"]
    #[inline(always)]
    pub fn tb1ods(&self) -> TB1ODS_R {
        TB1ODS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timer A Output 2 disable status"]
    #[inline(always)]
    pub fn ta2ods(&self) -> TA2ODS_R {
        TA2ODS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Timer A Output 1 disable status"]
    #[inline(always)]
    pub fn ta1ods(&self) -> TA1ODS_R {
        TA1ODS_R::new((self.bits & 0x01) != 0)
    }
}
