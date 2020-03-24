#[doc = "Reader of register CNSTAT"]
pub type R = crate::R<u32, super::CNSTAT>;
#[doc = "Writer for register CNSTAT"]
pub type W = crate::W<u32, super::CNSTAT>;
#[doc = "Register CNSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::CNSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNSTATA0`"]
pub type CNSTATA0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNSTATA0`"]
pub struct CNSTATA0_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATA0_W<'a> {
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
#[doc = "Reader of field `CNSTATA1`"]
pub type CNSTATA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNSTATA1`"]
pub struct CNSTATA1_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATA1_W<'a> {
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
#[doc = "Reader of field `CNSTATA2`"]
pub type CNSTATA2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNSTATA2`"]
pub struct CNSTATA2_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATA2_W<'a> {
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
#[doc = "Reader of field `CNSTATA3`"]
pub type CNSTATA3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNSTATA3`"]
pub struct CNSTATA3_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATA3_W<'a> {
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
#[doc = "Reader of field `CNSTATA4`"]
pub type CNSTATA4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNSTATA4`"]
pub struct CNSTATA4_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATA4_W<'a> {
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
    pub fn cnstata0(&self) -> CNSTATA0_R {
        CNSTATA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cnstata1(&self) -> CNSTATA1_R {
        CNSTATA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cnstata2(&self) -> CNSTATA2_R {
        CNSTATA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cnstata3(&self) -> CNSTATA3_R {
        CNSTATA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cnstata4(&self) -> CNSTATA4_R {
        CNSTATA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cnstata0(&mut self) -> CNSTATA0_W {
        CNSTATA0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cnstata1(&mut self) -> CNSTATA1_W {
        CNSTATA1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cnstata2(&mut self) -> CNSTATA2_W {
        CNSTATA2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cnstata3(&mut self) -> CNSTATA3_W {
        CNSTATA3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cnstata4(&mut self) -> CNSTATA4_W {
        CNSTATA4_W { w: self }
    }
}
