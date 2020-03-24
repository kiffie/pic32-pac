#[doc = "Reader of register DCRCCONINV"]
pub type R = crate::R<u32, super::DCRCCONINV>;
#[doc = "Writer for register DCRCCONINV"]
pub type W = crate::W<u32, super::DCRCCONINV>;
#[doc = "Register DCRCCONINV `reset()`'s with value 0"]
impl crate::ResetValue for super::DCRCCONINV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRCCH`"]
pub type CRCCH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CRCCH`"]
pub struct CRCCH_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `CRCTYP`"]
pub type CRCTYP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRCTYP`"]
pub struct CRCTYP_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCTYP_W<'a> {
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
#[doc = "Reader of field `CRCAPP`"]
pub type CRCAPP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRCAPP`"]
pub struct CRCAPP_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCAPP_W<'a> {
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
#[doc = "Reader of field `CRCEN`"]
pub type CRCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRCEN`"]
pub struct CRCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCEN_W<'a> {
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
#[doc = "Reader of field `PLEN`"]
pub type PLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLEN`"]
pub struct PLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `BITO`"]
pub type BITO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BITO`"]
pub struct BITO_W<'a> {
    w: &'a mut W,
}
impl<'a> BITO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `WBO`"]
pub type WBO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WBO`"]
pub struct WBO_W<'a> {
    w: &'a mut W,
}
impl<'a> WBO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `BYTO`"]
pub type BYTO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BYTO`"]
pub struct BYTO_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn crcch(&self) -> CRCCH_R {
        CRCCH_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn crctyp(&self) -> CRCTYP_R {
        CRCTYP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn crcapp(&self) -> CRCAPP_R {
        CRCAPP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn plen(&self) -> PLEN_R {
        PLEN_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn bito(&self) -> BITO_R {
        BITO_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn wbo(&self) -> WBO_R {
        WBO_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn byto(&self) -> BYTO_R {
        BYTO_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn crcch(&mut self) -> CRCCH_W {
        CRCCH_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn crctyp(&mut self) -> CRCTYP_W {
        CRCTYP_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn crcapp(&mut self) -> CRCAPP_W {
        CRCAPP_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W {
        CRCEN_W { w: self }
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn plen(&mut self) -> PLEN_W {
        PLEN_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn bito(&mut self) -> BITO_W {
        BITO_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn wbo(&mut self) -> WBO_W {
        WBO_W { w: self }
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn byto(&mut self) -> BYTO_W {
        BYTO_W { w: self }
    }
}
