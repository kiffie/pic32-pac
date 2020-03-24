#[doc = "Reader of register DCH1CONSET"]
pub type R = crate::R<u32, super::DCH1CONSET>;
#[doc = "Writer for register DCH1CONSET"]
pub type W = crate::W<u32, super::DCH1CONSET>;
#[doc = "Register DCH1CONSET `reset()`'s with value 0"]
impl crate::ResetValue for super::DCH1CONSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHPRI`"]
pub type CHPRI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHPRI`"]
pub struct CHPRI_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPRI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `CHEDET`"]
pub type CHEDET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHEDET`"]
pub struct CHEDET_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEDET_W<'a> {
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
#[doc = "Reader of field `CHAEN`"]
pub type CHAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHAEN`"]
pub struct CHAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHAEN_W<'a> {
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
#[doc = "Reader of field `CHCHN`"]
pub type CHCHN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHCHN`"]
pub struct CHCHN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHCHN_W<'a> {
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
#[doc = "Reader of field `CHAED`"]
pub type CHAED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHAED`"]
pub struct CHAED_W<'a> {
    w: &'a mut W,
}
impl<'a> CHAED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `CHEN`"]
pub type CHEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHEN`"]
pub struct CHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `CHCHNS`"]
pub type CHCHNS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHCHNS`"]
pub struct CHCHNS_W<'a> {
    w: &'a mut W,
}
impl<'a> CHCHNS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `CHBUSY`"]
pub type CHBUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHBUSY`"]
pub struct CHBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> CHBUSY_W<'a> {
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
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn chpri(&self) -> CHPRI_R {
        CHPRI_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn chedet(&self) -> CHEDET_R {
        CHEDET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn chaen(&self) -> CHAEN_R {
        CHAEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn chchn(&self) -> CHCHN_R {
        CHCHN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn chaed(&self) -> CHAED_R {
        CHAED_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn chchns(&self) -> CHCHNS_R {
        CHCHNS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn chbusy(&self) -> CHBUSY_R {
        CHBUSY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn chpri(&mut self) -> CHPRI_W {
        CHPRI_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn chedet(&mut self) -> CHEDET_W {
        CHEDET_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn chaen(&mut self) -> CHAEN_W {
        CHAEN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn chchn(&mut self) -> CHCHN_W {
        CHCHN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn chaed(&mut self) -> CHAED_W {
        CHAED_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn chen(&mut self) -> CHEN_W {
        CHEN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn chchns(&mut self) -> CHCHNS_W {
        CHCHNS_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn chbusy(&mut self) -> CHBUSY_W {
        CHBUSY_W { w: self }
    }
}
