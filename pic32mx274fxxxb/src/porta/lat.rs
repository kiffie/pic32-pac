#[doc = "Reader of register LAT"]
pub type R = crate::R<u32, super::LAT>;
#[doc = "Writer for register LAT"]
pub type W = crate::W<u32, super::LAT>;
#[doc = "Register LAT `reset()`'s with value 0"]
impl crate::ResetValue for super::LAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LATA0`"]
pub type LATA0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LATA0`"]
pub struct LATA0_W<'a> {
    w: &'a mut W,
}
impl<'a> LATA0_W<'a> {
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
#[doc = "Reader of field `LATA1`"]
pub type LATA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LATA1`"]
pub struct LATA1_W<'a> {
    w: &'a mut W,
}
impl<'a> LATA1_W<'a> {
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
#[doc = "Reader of field `LATA2`"]
pub type LATA2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LATA2`"]
pub struct LATA2_W<'a> {
    w: &'a mut W,
}
impl<'a> LATA2_W<'a> {
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
#[doc = "Reader of field `LATA3`"]
pub type LATA3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LATA3`"]
pub struct LATA3_W<'a> {
    w: &'a mut W,
}
impl<'a> LATA3_W<'a> {
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
#[doc = "Reader of field `LATA4`"]
pub type LATA4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LATA4`"]
pub struct LATA4_W<'a> {
    w: &'a mut W,
}
impl<'a> LATA4_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lata0(&self) -> LATA0_R {
        LATA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lata1(&self) -> LATA1_R {
        LATA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn lata2(&self) -> LATA2_R {
        LATA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lata3(&self) -> LATA3_R {
        LATA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lata4(&self) -> LATA4_R {
        LATA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lata0(&mut self) -> LATA0_W {
        LATA0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lata1(&mut self) -> LATA1_W {
        LATA1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn lata2(&mut self) -> LATA2_W {
        LATA2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lata3(&mut self) -> LATA3_W {
        LATA3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lata4(&mut self) -> LATA4_W {
        LATA4_W { w: self }
    }
}
