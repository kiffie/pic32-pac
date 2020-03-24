#[doc = "Reader of register ANSELSET"]
pub type R = crate::R<u32, super::ANSELSET>;
#[doc = "Writer for register ANSELSET"]
pub type W = crate::W<u32, super::ANSELSET>;
#[doc = "Register ANSELSET `reset()`'s with value 0"]
impl crate::ResetValue for super::ANSELSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ANSA0`"]
pub type ANSA0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ANSA0`"]
pub struct ANSA0_W<'a> {
    w: &'a mut W,
}
impl<'a> ANSA0_W<'a> {
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
#[doc = "Reader of field `ANSA1`"]
pub type ANSA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ANSA1`"]
pub struct ANSA1_W<'a> {
    w: &'a mut W,
}
impl<'a> ANSA1_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ansa0(&self) -> ANSA0_R {
        ANSA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ansa1(&self) -> ANSA1_R {
        ANSA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ansa0(&mut self) -> ANSA0_W {
        ANSA0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ansa1(&mut self) -> ANSA1_W {
        ANSA1_W { w: self }
    }
}
