#[doc = "Reader of register PMADDRSET"]
pub type R = crate::R<u32, super::PMADDRSET>;
#[doc = "Writer for register PMADDRSET"]
pub type W = crate::W<u32, super::PMADDRSET>;
#[doc = "Register PMADDRSET `reset()`'s with value 0"]
impl crate::ResetValue for super::PMADDRSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADDR`"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
#[doc = "Reader of field `ADDR14`"]
pub type ADDR14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADDR14`"]
pub struct ADDR14_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR14_W<'a> {
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
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn addr14(&self) -> ADDR14_R {
        ADDR14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn addr14(&mut self) -> ADDR14_W {
        ADDR14_W { w: self }
    }
}
