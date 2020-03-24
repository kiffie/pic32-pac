#[doc = "Reader of register DSCON"]
pub type R = crate::R<u32, super::DSCON>;
#[doc = "Writer for register DSCON"]
pub type W = crate::W<u32, super::DSCON>;
#[doc = "Register DSCON `reset()`'s with value 0"]
impl crate::ResetValue for super::DSCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RELEASE`"]
pub type RELEASE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RELEASE`"]
pub struct RELEASE_W<'a> {
    w: &'a mut W,
}
impl<'a> RELEASE_W<'a> {
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
#[doc = "Reader of field `DSBOR`"]
pub type DSBOR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSBOR`"]
pub struct DSBOR_W<'a> {
    w: &'a mut W,
}
impl<'a> DSBOR_W<'a> {
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
#[doc = "Reader of field `WAKEDIS`"]
pub type WAKEDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKEDIS`"]
pub struct WAKEDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEDIS_W<'a> {
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
#[doc = "Reader of field `RTCCWDIS`"]
pub type RTCCWDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCCWDIS`"]
pub struct RTCCWDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCWDIS_W<'a> {
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
#[doc = "Reader of field `RTCDIS`"]
pub type RTCDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCDIS`"]
pub struct RTCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `DSGPREN`"]
pub type DSGPREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSGPREN`"]
pub struct DSGPREN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSGPREN_W<'a> {
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
#[doc = "Reader of field `DSEN`"]
pub type DSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSEN`"]
pub struct DSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSEN_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn release(&self) -> RELEASE_R {
        RELEASE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dsbor(&self) -> DSBOR_R {
        DSBOR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wakedis(&self) -> WAKEDIS_R {
        WAKEDIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rtccwdis(&self) -> RTCCWDIS_R {
        RTCCWDIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rtcdis(&self) -> RTCDIS_R {
        RTCDIS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dsgpren(&self) -> DSGPREN_R {
        DSGPREN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dsen(&self) -> DSEN_R {
        DSEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn release(&mut self) -> RELEASE_W {
        RELEASE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dsbor(&mut self) -> DSBOR_W {
        DSBOR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wakedis(&mut self) -> WAKEDIS_W {
        WAKEDIS_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rtccwdis(&mut self) -> RTCCWDIS_W {
        RTCCWDIS_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rtcdis(&mut self) -> RTCDIS_W {
        RTCDIS_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dsgpren(&mut self) -> DSGPREN_W {
        DSGPREN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dsen(&mut self) -> DSEN_W {
        DSEN_W { w: self }
    }
}
