#[doc = "Reader of register CHECONINV"]
pub type R = crate::R<u32, super::CHECONINV>;
#[doc = "Writer for register CHECONINV"]
pub type W = crate::W<u32, super::CHECONINV>;
#[doc = "Register CHECONINV `reset()`'s with value 0"]
impl crate::ResetValue for super::CHECONINV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PFMWS`"]
pub type PFMWS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PFMWS`"]
pub struct PFMWS_W<'a> {
    w: &'a mut W,
}
impl<'a> PFMWS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `PREFEN`"]
pub type PREFEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PREFEN`"]
pub struct PREFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PREFEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `DCSZ`"]
pub type DCSZ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DCSZ`"]
pub struct DCSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> DCSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `CHECOH`"]
pub type CHECOH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHECOH`"]
pub struct CHECOH_W<'a> {
    w: &'a mut W,
}
impl<'a> CHECOH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn pfmws(&self) -> PFMWS_R {
        PFMWS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn prefen(&self) -> PREFEN_R {
        PREFEN_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn dcsz(&self) -> DCSZ_R {
        DCSZ_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn checoh(&self) -> CHECOH_R {
        CHECOH_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn pfmws(&mut self) -> PFMWS_W {
        PFMWS_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn prefen(&mut self) -> PREFEN_W {
        PREFEN_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn dcsz(&mut self) -> DCSZ_W {
        DCSZ_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn checoh(&mut self) -> CHECOH_W {
        CHECOH_W { w: self }
    }
}
