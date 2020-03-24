#[doc = "Reader of register ANSELINV"]
pub type R = crate::R<u32, super::ANSELINV>;
#[doc = "Writer for register ANSELINV"]
pub type W = crate::W<u32, super::ANSELINV>;
#[doc = "Register ANSELINV `reset()`'s with value 0"]
impl crate::ResetValue for super::ANSELINV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ANSB0`"]
pub type ANSB0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ANSB0`"]
pub struct ANSB0_W<'a> {
    w: &'a mut W,
}
impl<'a> ANSB0_W<'a> {
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
#[doc = "Reader of field `ANSB1`"]
pub type ANSB1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ANSB1`"]
pub struct ANSB1_W<'a> {
    w: &'a mut W,
}
impl<'a> ANSB1_W<'a> {
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
#[doc = "Reader of field `ANSB2`"]
pub type ANSB2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ANSB2`"]
pub struct ANSB2_W<'a> {
    w: &'a mut W,
}
impl<'a> ANSB2_W<'a> {
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
#[doc = "Reader of field `ANSB3`"]
pub type ANSB3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ANSB3`"]
pub struct ANSB3_W<'a> {
    w: &'a mut W,
}
impl<'a> ANSB3_W<'a> {
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
#[doc = "Reader of field `ANSB13`"]
pub type ANSB13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ANSB13`"]
pub struct ANSB13_W<'a> {
    w: &'a mut W,
}
impl<'a> ANSB13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `ANSB14`"]
pub type ANSB14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ANSB14`"]
pub struct ANSB14_W<'a> {
    w: &'a mut W,
}
impl<'a> ANSB14_W<'a> {
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
#[doc = "Reader of field `ANSB15`"]
pub type ANSB15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ANSB15`"]
pub struct ANSB15_W<'a> {
    w: &'a mut W,
}
impl<'a> ANSB15_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ansb0(&self) -> ANSB0_R {
        ANSB0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ansb1(&self) -> ANSB1_R {
        ANSB1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ansb2(&self) -> ANSB2_R {
        ANSB2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ansb3(&self) -> ANSB3_R {
        ANSB3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ansb13(&self) -> ANSB13_R {
        ANSB13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ansb14(&self) -> ANSB14_R {
        ANSB14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ansb15(&self) -> ANSB15_R {
        ANSB15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ansb0(&mut self) -> ANSB0_W {
        ANSB0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ansb1(&mut self) -> ANSB1_W {
        ANSB1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ansb2(&mut self) -> ANSB2_W {
        ANSB2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ansb3(&mut self) -> ANSB3_W {
        ANSB3_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ansb13(&mut self) -> ANSB13_W {
        ANSB13_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ansb14(&mut self) -> ANSB14_W {
        ANSB14_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ansb15(&mut self) -> ANSB15_W {
        ANSB15_W { w: self }
    }
}
