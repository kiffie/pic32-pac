#[doc = "Reader of register PB1DIVINV"]
pub type R = crate::R<u32, super::PB1DIVINV>;
#[doc = "Writer for register PB1DIVINV"]
pub type W = crate::W<u32, super::PB1DIVINV>;
#[doc = "Register PB1DIVINV `reset()`'s with value 0"]
impl crate::ResetValue for super::PB1DIVINV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PBDIV`"]
pub type PBDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PBDIV`"]
pub struct PBDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PBDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `PBDIVRDY`"]
pub type PBDIVRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBDIVRDY`"]
pub struct PBDIVRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> PBDIVRDY_W<'a> {
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
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn pbdiv(&self) -> PBDIV_R {
        PBDIV_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pbdivrdy(&self) -> PBDIVRDY_R {
        PBDIVRDY_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&self) -> ON_R {
        ON_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn pbdiv(&mut self) -> PBDIV_W {
        PBDIV_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pbdivrdy(&mut self) -> PBDIVRDY_W {
        PBDIVRDY_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&mut self) -> ON_W {
        ON_W { w: self }
    }
}
