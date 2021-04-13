#[doc = "Reader of register VREFBUF_CSR"]
pub type R = crate::R<u32, super::VREFBUF_CSR>;
#[doc = "Writer for register VREFBUF_CSR"]
pub type W = crate::W<u32, super::VREFBUF_CSR>;
#[doc = "Register VREFBUF_CSR `reset()`'s with value 0x02"]
impl crate::ResetValue for super::VREFBUF_CSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Voltage reference buffer mode enable This bit is used to enable the voltage reference buffer mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENVR_A {
    #[doc = "0: Internal voltage reference mode disable (external voltage reference mode)."]
    B_0X0 = 0,
    #[doc = "1: Internal voltage reference mode (reference buffer enable or hold mode) enable."]
    B_0X1 = 1,
}
impl From<ENVR_A> for bool {
    #[inline(always)]
    fn from(variant: ENVR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENVR`"]
pub type ENVR_R = crate::R<bool, ENVR_A>;
impl ENVR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENVR_A {
        match self.bits {
            false => ENVR_A::B_0X0,
            true => ENVR_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ENVR_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ENVR_A::B_0X1
    }
}
#[doc = "Write proxy for field `ENVR`"]
pub struct ENVR_W<'a> {
    w: &'a mut W,
}
impl<'a> ENVR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENVR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Internal voltage reference mode disable (external voltage reference mode)."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ENVR_A::B_0X0)
    }
    #[doc = "Internal voltage reference mode (reference buffer enable or hold mode) enable."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ENVR_A::B_0X1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "High impedance mode This bit controls the analog switch to connect or not the VREF+ pin. Refer to for the mode descriptions depending on ENVR bit configuration.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIZ_A {
    #[doc = "0: VREF+ pin is internally connected to the voltage reference buffer output."]
    B_0X0 = 0,
    #[doc = "1: VREF+ pin is high impedance."]
    B_0X1 = 1,
}
impl From<HIZ_A> for bool {
    #[inline(always)]
    fn from(variant: HIZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HIZ`"]
pub type HIZ_R = crate::R<bool, HIZ_A>;
impl HIZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIZ_A {
        match self.bits {
            false => HIZ_A::B_0X0,
            true => HIZ_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HIZ_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HIZ_A::B_0X1
    }
}
#[doc = "Write proxy for field `HIZ`"]
pub struct HIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> HIZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "VREF+ pin is internally connected to the voltage reference buffer output."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HIZ_A::B_0X0)
    }
    #[doc = "VREF+ pin is high impedance."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HIZ_A::B_0X1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Voltage reference scale This bit selects the value generated by the voltage reference buffer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VRS_A {
    #[doc = "0: Voltage reference set to VREF_OUT1 (around 2.048 V). "]
    B_0X0 = 0,
    #[doc = "1: Voltage reference set to VREF_OUT2 (around 2.5 V). "]
    B_0X1 = 1,
}
impl From<VRS_A> for bool {
    #[inline(always)]
    fn from(variant: VRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VRS`"]
pub type VRS_R = crate::R<bool, VRS_A>;
impl VRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VRS_A {
        match self.bits {
            false => VRS_A::B_0X0,
            true => VRS_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == VRS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == VRS_A::B_0X1
    }
}
#[doc = "Write proxy for field `VRS`"]
pub struct VRS_W<'a> {
    w: &'a mut W,
}
impl<'a> VRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VRS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Voltage reference set to VREF_OUT1 (around 2.048 V)."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(VRS_A::B_0X0)
    }
    #[doc = "Voltage reference set to VREF_OUT2 (around 2.5 V)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(VRS_A::B_0X1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Voltage reference buffer ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VRR_A {
    #[doc = "0: the voltage reference buffer output is not ready."]
    B_0X0 = 0,
    #[doc = "1: the voltage reference buffer output reached the requested level."]
    B_0X1 = 1,
}
impl From<VRR_A> for bool {
    #[inline(always)]
    fn from(variant: VRR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VRR`"]
pub type VRR_R = crate::R<bool, VRR_A>;
impl VRR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VRR_A {
        match self.bits {
            false => VRR_A::B_0X0,
            true => VRR_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == VRR_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == VRR_A::B_0X1
    }
}
impl R {
    #[doc = "Bit 0 - Voltage reference buffer mode enable This bit is used to enable the voltage reference buffer mode."]
    #[inline(always)]
    pub fn envr(&self) -> ENVR_R {
        ENVR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - High impedance mode This bit controls the analog switch to connect or not the VREF+ pin. Refer to for the mode descriptions depending on ENVR bit configuration."]
    #[inline(always)]
    pub fn hiz(&self) -> HIZ_R {
        HIZ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Voltage reference scale This bit selects the value generated by the voltage reference buffer."]
    #[inline(always)]
    pub fn vrs(&self) -> VRS_R {
        VRS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Voltage reference buffer ready"]
    #[inline(always)]
    pub fn vrr(&self) -> VRR_R {
        VRR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage reference buffer mode enable This bit is used to enable the voltage reference buffer mode."]
    #[inline(always)]
    pub fn envr(&mut self) -> ENVR_W {
        ENVR_W { w: self }
    }
    #[doc = "Bit 1 - High impedance mode This bit controls the analog switch to connect or not the VREF+ pin. Refer to for the mode descriptions depending on ENVR bit configuration."]
    #[inline(always)]
    pub fn hiz(&mut self) -> HIZ_W {
        HIZ_W { w: self }
    }
    #[doc = "Bit 2 - Voltage reference scale This bit selects the value generated by the voltage reference buffer."]
    #[inline(always)]
    pub fn vrs(&mut self) -> VRS_W {
        VRS_W { w: self }
    }
}
