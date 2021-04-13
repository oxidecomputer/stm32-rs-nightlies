#[doc = "Reader of register LPTIM_ISR"]
pub type R = crate::R<u32, super::LPTIM_ISR>;
#[doc = "Reader of field `CMPM`"]
pub type CMPM_R = crate::R<bool, bool>;
#[doc = "Reader of field `ARRM`"]
pub type ARRM_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTTRIG`"]
pub type EXTTRIG_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMPOK`"]
pub type CMPOK_R = crate::R<bool, bool>;
#[doc = "Reader of field `ARROK`"]
pub type ARROK_R = crate::R<bool, bool>;
#[doc = "Reader of field `UP`"]
pub type UP_R = crate::R<bool, bool>;
#[doc = "Reader of field `DOWN`"]
pub type DOWN_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Compare match The CMPM bit is set by hardware to inform application that LPTIM_CNT register value reached the LPTIM_CMP registerâ\u{80}\u{99}s value."]
    #[inline(always)]
    pub fn cmpm(&self) -> CMPM_R {
        CMPM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Autoreload match ARRM is set by hardware to inform application that LPTIM_CNT registerâ\u{80}\u{99}s value reached the LPTIM_ARR registerâ\u{80}\u{99}s value. ARRM flag can be cleared by writing 1 to the ARRMCF bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn arrm(&self) -> ARRM_R {
        ARRM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External trigger edge event EXTTRIG is set by hardware to inform application that a valid edge on the selected external trigger input has occurred. If the trigger is ignored because the timer has already started, then this flag is not set. EXTTRIG flag can be cleared by writing 1 to the EXTTRIGCF bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn exttrig(&self) -> EXTTRIG_R {
        EXTTRIG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Compare register update OK CMPOK is set by hardware to inform application that the APB bus write operation to the LPTIM_CMP register has been successfully completed."]
    #[inline(always)]
    pub fn cmpok(&self) -> CMPOK_R {
        CMPOK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Autoreload register update OK ARROK is set by hardware to inform application that the APB bus write operation to the LPTIM_ARR register has been successfully completed. ARROK flag can be cleared by writing 1 to the ARROKCF bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn arrok(&self) -> ARROK_R {
        ARROK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Counter direction change down to up In Encoder mode, UP bit is set by hardware to inform application that the counter direction has changed from down to up. UP flag can be cleared by writing 1 to the UPCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Counter direction change up to down In Encoder mode, DOWN bit is set by hardware to inform application that the counter direction has changed from up to down. DOWN flag can be cleared by writing 1 to the DOWNCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn down(&self) -> DOWN_R {
        DOWN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
