#[doc = "Reader of register CLKSTATSET"]
pub type R = crate::R<u32, super::CLKSTATSET>;
#[doc = "Writer for register CLKSTATSET"]
pub type W = crate::W<u32, super::CLKSTATSET>;
#[doc = "Register CLKSTATSET `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKSTATSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FRCRDY`"]
pub type FRCRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRCRDY`"]
pub struct FRCRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> FRCRDY_W<'a> {
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
#[doc = "Reader of field `SPDIVRDY`"]
pub type SPDIVRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPDIVRDY`"]
pub struct SPDIVRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> SPDIVRDY_W<'a> {
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
#[doc = "Reader of field `POSCRDY`"]
pub type POSCRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POSCRDY`"]
pub struct POSCRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> POSCRDY_W<'a> {
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
#[doc = "Reader of field `SOSCRDY`"]
pub type SOSCRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOSCRDY`"]
pub struct SOSCRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> SOSCRDY_W<'a> {
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
#[doc = "Reader of field `LPRCRDY`"]
pub type LPRCRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPRCRDY`"]
pub struct LPRCRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> LPRCRDY_W<'a> {
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
#[doc = "Reader of field `SPLLRDY`"]
pub type SPLLRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPLLRDY`"]
pub struct SPLLRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLLRDY_W<'a> {
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
#[doc = "Reader of field `UPLLRDY`"]
pub type UPLLRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UPLLRDY`"]
pub struct UPLLRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> UPLLRDY_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn frcrdy(&self) -> FRCRDY_R {
        FRCRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spdivrdy(&self) -> SPDIVRDY_R {
        SPDIVRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn poscrdy(&self) -> POSCRDY_R {
        POSCRDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn soscrdy(&self) -> SOSCRDY_R {
        SOSCRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lprcrdy(&self) -> LPRCRDY_R {
        LPRCRDY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn spllrdy(&self) -> SPLLRDY_R {
        SPLLRDY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn upllrdy(&self) -> UPLLRDY_R {
        UPLLRDY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn frcrdy(&mut self) -> FRCRDY_W {
        FRCRDY_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spdivrdy(&mut self) -> SPDIVRDY_W {
        SPDIVRDY_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn poscrdy(&mut self) -> POSCRDY_W {
        POSCRDY_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn soscrdy(&mut self) -> SOSCRDY_W {
        SOSCRDY_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lprcrdy(&mut self) -> LPRCRDY_W {
        LPRCRDY_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn spllrdy(&mut self) -> SPLLRDY_W {
        SPLLRDY_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn upllrdy(&mut self) -> UPLLRDY_W {
        UPLLRDY_W { w: self }
    }
}
