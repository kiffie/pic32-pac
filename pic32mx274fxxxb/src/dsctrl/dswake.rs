#[doc = "Reader of register DSWAKE"]
pub type R = crate::R<u32, super::DSWAKE>;
#[doc = "Writer for register DSWAKE"]
pub type W = crate::W<u32, super::DSWAKE>;
#[doc = "Register DSWAKE `reset()`'s with value 0"]
impl crate::ResetValue for super::DSWAKE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DSMCLR`"]
pub type DSMCLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSMCLR`"]
pub struct DSMCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DSMCLR_W<'a> {
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
#[doc = "Reader of field `DSRTC`"]
pub type DSRTC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSRTC`"]
pub struct DSRTC_W<'a> {
    w: &'a mut W,
}
impl<'a> DSRTC_W<'a> {
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
#[doc = "Reader of field `DSWDT`"]
pub type DSWDT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSWDT`"]
pub struct DSWDT_W<'a> {
    w: &'a mut W,
}
impl<'a> DSWDT_W<'a> {
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
#[doc = "Reader of field `DSFLT`"]
pub type DSFLT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSFLT`"]
pub struct DSFLT_W<'a> {
    w: &'a mut W,
}
impl<'a> DSFLT_W<'a> {
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
#[doc = "Reader of field `DSINT0`"]
pub type DSINT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSINT0`"]
pub struct DSINT0_W<'a> {
    w: &'a mut W,
}
impl<'a> DSINT0_W<'a> {
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
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dsmclr(&self) -> DSMCLR_R {
        DSMCLR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dsrtc(&self) -> DSRTC_R {
        DSRTC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dswdt(&self) -> DSWDT_R {
        DSWDT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dsflt(&self) -> DSFLT_R {
        DSFLT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dsint0(&self) -> DSINT0_R {
        DSINT0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dsmclr(&mut self) -> DSMCLR_W {
        DSMCLR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dsrtc(&mut self) -> DSRTC_W {
        DSRTC_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dswdt(&mut self) -> DSWDT_W {
        DSWDT_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dsflt(&mut self) -> DSFLT_W {
        DSFLT_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dsint0(&mut self) -> DSINT0_W {
        DSINT0_W { w: self }
    }
}
