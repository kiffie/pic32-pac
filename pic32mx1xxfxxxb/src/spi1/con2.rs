#[doc = "Reader of register CON2"]
pub type R = crate::R<u32, super::CON2>;
#[doc = "Writer for register CON2"]
pub type W = crate::W<u32, super::CON2>;
#[doc = "Register CON2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CON2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AUDMOD`"]
pub type AUDMOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AUDMOD`"]
pub struct AUDMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDMOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `AUDMONO`"]
pub type AUDMONO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUDMONO`"]
pub struct AUDMONO_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDMONO_W<'a> {
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
#[doc = "Reader of field `AUDEN`"]
pub type AUDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUDEN`"]
pub struct AUDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDEN_W<'a> {
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
#[doc = "Reader of field `IGNTUR`"]
pub type IGNTUR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IGNTUR`"]
pub struct IGNTUR_W<'a> {
    w: &'a mut W,
}
impl<'a> IGNTUR_W<'a> {
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
#[doc = "Reader of field `IGNROV`"]
pub type IGNROV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IGNROV`"]
pub struct IGNROV_W<'a> {
    w: &'a mut W,
}
impl<'a> IGNROV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `SPITUREN`"]
pub type SPITUREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPITUREN`"]
pub struct SPITUREN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPITUREN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `SPIROVEN`"]
pub type SPIROVEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPIROVEN`"]
pub struct SPIROVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIROVEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `FRMERREN`"]
pub type FRMERREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRMERREN`"]
pub struct FRMERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRMERREN_W<'a> {
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
#[doc = "Reader of field `SPISGNEXT`"]
pub type SPISGNEXT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPISGNEXT`"]
pub struct SPISGNEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> SPISGNEXT_W<'a> {
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
    pub fn audmod(&self) -> AUDMOD_R {
        AUDMOD_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn audmono(&self) -> AUDMONO_R {
        AUDMONO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn auden(&self) -> AUDEN_R {
        AUDEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn igntur(&self) -> IGNTUR_R {
        IGNTUR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ignrov(&self) -> IGNROV_R {
        IGNROV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn spituren(&self) -> SPITUREN_R {
        SPITUREN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn spiroven(&self) -> SPIROVEN_R {
        SPIROVEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn frmerren(&self) -> FRMERREN_R {
        FRMERREN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn spisgnext(&self) -> SPISGNEXT_R {
        SPISGNEXT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn audmod(&mut self) -> AUDMOD_W {
        AUDMOD_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn audmono(&mut self) -> AUDMONO_W {
        AUDMONO_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn auden(&mut self) -> AUDEN_W {
        AUDEN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn igntur(&mut self) -> IGNTUR_W {
        IGNTUR_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ignrov(&mut self) -> IGNROV_W {
        IGNROV_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn spituren(&mut self) -> SPITUREN_W {
        SPITUREN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn spiroven(&mut self) -> SPIROVEN_W {
        SPIROVEN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn frmerren(&mut self) -> FRMERREN_W {
        FRMERREN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn spisgnext(&mut self) -> SPISGNEXT_W {
        SPISGNEXT_W { w: self }
    }
}
