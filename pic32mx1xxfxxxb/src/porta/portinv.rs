#[doc = "Reader of register PORTINV"]
pub type R = crate::R<u32, super::PORTINV>;
#[doc = "Writer for register PORTINV"]
pub type W = crate::W<u32, super::PORTINV>;
#[doc = "Register PORTINV `reset()`'s with value 0"]
impl crate::ResetValue for super::PORTINV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RA0`"]
pub type RA0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RA0`"]
pub struct RA0_W<'a> {
    w: &'a mut W,
}
impl<'a> RA0_W<'a> {
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
#[doc = "Reader of field `RA1`"]
pub type RA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RA1`"]
pub struct RA1_W<'a> {
    w: &'a mut W,
}
impl<'a> RA1_W<'a> {
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
#[doc = "Reader of field `RA2`"]
pub type RA2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RA2`"]
pub struct RA2_W<'a> {
    w: &'a mut W,
}
impl<'a> RA2_W<'a> {
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
#[doc = "Reader of field `RA3`"]
pub type RA3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RA3`"]
pub struct RA3_W<'a> {
    w: &'a mut W,
}
impl<'a> RA3_W<'a> {
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
#[doc = "Reader of field `RA4`"]
pub type RA4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RA4`"]
pub struct RA4_W<'a> {
    w: &'a mut W,
}
impl<'a> RA4_W<'a> {
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
    pub fn ra0(&self) -> RA0_R {
        RA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ra1(&self) -> RA1_R {
        RA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ra2(&self) -> RA2_R {
        RA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ra3(&self) -> RA3_R {
        RA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ra4(&self) -> RA4_R {
        RA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ra0(&mut self) -> RA0_W {
        RA0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ra1(&mut self) -> RA1_W {
        RA1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ra2(&mut self) -> RA2_W {
        RA2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ra3(&mut self) -> RA3_W {
        RA3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ra4(&mut self) -> RA4_W {
        RA4_W { w: self }
    }
}
