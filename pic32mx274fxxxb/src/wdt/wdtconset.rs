#[doc = "Reader of register WDTCONSET"]
pub type R = crate::R<u32, super::WDTCONSET>;
#[doc = "Writer for register WDTCONSET"]
pub type W = crate::W<u32, super::WDTCONSET>;
#[doc = "Register WDTCONSET `reset()`'s with value 0"]
impl crate::ResetValue for super::WDTCONSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDTWINEN`"]
pub type WDTWINEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTWINEN`"]
pub struct WDTWINEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTWINEN_W<'a> {
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
#[doc = "Reader of field `RUNDIV`"]
pub type RUNDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RUNDIV`"]
pub struct RUNDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> RUNDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
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
#[doc = "Reader of field `WDTCLRKEY`"]
pub type WDTCLRKEY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WDTCLRKEY`"]
pub struct WDTCLRKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTCLRKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wdtwinen(&self) -> WDTWINEN_R {
        WDTWINEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn rundiv(&self) -> RUNDIV_R {
        RUNDIV_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&self) -> ON_R {
        ON_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn wdtclrkey(&self) -> WDTCLRKEY_R {
        WDTCLRKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wdtwinen(&mut self) -> WDTWINEN_W {
        WDTWINEN_W { w: self }
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn rundiv(&mut self) -> RUNDIV_W {
        RUNDIV_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&mut self) -> ON_W {
        ON_W { w: self }
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn wdtclrkey(&mut self) -> WDTCLRKEY_W {
        WDTCLRKEY_W { w: self }
    }
}
