#[doc = "Reader of register OC2CONSET"]
pub type R = crate::R<u32, super::OC2CONSET>;
#[doc = "Writer for register OC2CONSET"]
pub type W = crate::W<u32, super::OC2CONSET>;
#[doc = "Register OC2CONSET `reset()`'s with value 0"]
impl crate::ResetValue for super::OC2CONSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OCM`"]
pub type OCM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OCM`"]
pub struct OCM_W<'a> {
    w: &'a mut W,
}
impl<'a> OCM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `OCTSEL`"]
pub type OCTSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCTSEL`"]
pub struct OCTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `OCFLT`"]
pub type OCFLT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCFLT`"]
pub struct OCFLT_W<'a> {
    w: &'a mut W,
}
impl<'a> OCFLT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `OC32`"]
pub type OC32_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC32`"]
pub struct OC32_W<'a> {
    w: &'a mut W,
}
impl<'a> OC32_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
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
#[doc = "Reader of field `ON`"]
pub type ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ON`"]
pub struct ON_W<'a> {
    w: &'a mut W,
}
impl<'a> ON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn ocm(&self) -> OCM_R {
        OCM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn octsel(&self) -> OCTSEL_R {
        OCTSEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ocflt(&self) -> OCFLT_R {
        OCFLT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn oc32(&self) -> OC32_R {
        OC32_R::new(((self.bits >> 5) & 0x01) != 0)
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
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&self) -> ON_R {
        ON_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn ocm(&mut self) -> OCM_W {
        OCM_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn octsel(&mut self) -> OCTSEL_W {
        OCTSEL_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ocflt(&mut self) -> OCFLT_W {
        OCFLT_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn oc32(&mut self) -> OC32_W {
        OC32_W { w: self }
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
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&mut self) -> ON_W {
        ON_W { w: self }
    }
}
