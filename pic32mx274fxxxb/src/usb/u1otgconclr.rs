#[doc = "Reader of register U1OTGCONCLR"]
pub type R = crate::R<u32, super::U1OTGCONCLR>;
#[doc = "Writer for register U1OTGCONCLR"]
pub type W = crate::W<u32, super::U1OTGCONCLR>;
#[doc = "Register U1OTGCONCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::U1OTGCONCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VBUSDIS`"]
pub type VBUSDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBUSDIS`"]
pub struct VBUSDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSDIS_W<'a> {
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
#[doc = "Reader of field `VBUSCHG`"]
pub type VBUSCHG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBUSCHG`"]
pub struct VBUSCHG_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSCHG_W<'a> {
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
#[doc = "Reader of field `OTGEN`"]
pub type OTGEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OTGEN`"]
pub struct OTGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGEN_W<'a> {
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
#[doc = "Reader of field `VBUSON`"]
pub type VBUSON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBUSON`"]
pub struct VBUSON_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSON_W<'a> {
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
#[doc = "Reader of field `DMPULDWN`"]
pub type DMPULDWN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMPULDWN`"]
pub struct DMPULDWN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMPULDWN_W<'a> {
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
#[doc = "Reader of field `DPPULDWN`"]
pub type DPPULDWN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPPULDWN`"]
pub struct DPPULDWN_W<'a> {
    w: &'a mut W,
}
impl<'a> DPPULDWN_W<'a> {
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
#[doc = "Reader of field `DMPULUP`"]
pub type DMPULUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMPULUP`"]
pub struct DMPULUP_W<'a> {
    w: &'a mut W,
}
impl<'a> DMPULUP_W<'a> {
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
#[doc = "Reader of field `DPPULUP`"]
pub type DPPULUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPPULUP`"]
pub struct DPPULUP_W<'a> {
    w: &'a mut W,
}
impl<'a> DPPULUP_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vbusdis(&self) -> VBUSDIS_R {
        VBUSDIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn vbuschg(&self) -> VBUSCHG_R {
        VBUSCHG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn otgen(&self) -> OTGEN_R {
        OTGEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn vbuson(&self) -> VBUSON_R {
        VBUSON_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dmpuldwn(&self) -> DMPULDWN_R {
        DMPULDWN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dppuldwn(&self) -> DPPULDWN_R {
        DPPULDWN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dmpulup(&self) -> DMPULUP_R {
        DMPULUP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dppulup(&self) -> DPPULUP_R {
        DPPULUP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vbusdis(&mut self) -> VBUSDIS_W {
        VBUSDIS_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn vbuschg(&mut self) -> VBUSCHG_W {
        VBUSCHG_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn otgen(&mut self) -> OTGEN_W {
        OTGEN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn vbuson(&mut self) -> VBUSON_W {
        VBUSON_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dmpuldwn(&mut self) -> DMPULDWN_W {
        DMPULDWN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dppuldwn(&mut self) -> DPPULDWN_W {
        DPPULDWN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dmpulup(&mut self) -> DMPULUP_W {
        DMPULUP_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dppulup(&mut self) -> DPPULUP_W {
        DPPULUP_W { w: self }
    }
}
