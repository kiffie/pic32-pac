#[doc = "Reader of register CMSTATINV"]
pub type R = crate::R<u32, super::CMSTATINV>;
#[doc = "Writer for register CMSTATINV"]
pub type W = crate::W<u32, super::CMSTATINV>;
#[doc = "Register CMSTATINV `reset()`'s with value 0"]
impl crate::ResetValue for super::CMSTATINV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `C1OUT`"]
pub type C1OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C1OUT`"]
pub struct C1OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> C1OUT_W<'a> {
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
#[doc = "Reader of field `C2OUT`"]
pub type C2OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C2OUT`"]
pub struct C2OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> C2OUT_W<'a> {
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
#[doc = "Reader of field `C3OUT`"]
pub type C3OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C3OUT`"]
pub struct C3OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> C3OUT_W<'a> {
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
#[doc = "Reader of field `SIDL`"]
pub type SIDL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIDL`"]
pub struct SIDL_W<'a> {
    w: &'a mut W,
}
impl<'a> SIDL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `FRZ`"]
pub type FRZ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRZ`"]
pub struct FRZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FRZ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn c1out(&self) -> C1OUT_R {
        C1OUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn c2out(&self) -> C2OUT_R {
        C2OUT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn c3out(&self) -> C3OUT_R {
        C3OUT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn sidl(&self) -> SIDL_R {
        SIDL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn frz(&self) -> FRZ_R {
        FRZ_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn c1out(&mut self) -> C1OUT_W {
        C1OUT_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn c2out(&mut self) -> C2OUT_W {
        C2OUT_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn c3out(&mut self) -> C3OUT_W {
        C3OUT_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn sidl(&mut self) -> SIDL_W {
        SIDL_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn frz(&mut self) -> FRZ_W {
        FRZ_W { w: self }
    }
}
