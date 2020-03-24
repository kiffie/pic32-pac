#[doc = "Reader of register CNPD"]
pub type R = crate::R<u32, super::CNPD>;
#[doc = "Writer for register CNPD"]
pub type W = crate::W<u32, super::CNPD>;
#[doc = "Register CNPD `reset()`'s with value 0"]
impl crate::ResetValue for super::CNPD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNPDB0`"]
pub type CNPDB0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPDB0`"]
pub struct CNPDB0_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDB0_W<'a> {
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
#[doc = "Reader of field `CNPDB1`"]
pub type CNPDB1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPDB1`"]
pub struct CNPDB1_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDB1_W<'a> {
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
#[doc = "Reader of field `CNPDB2`"]
pub type CNPDB2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPDB2`"]
pub struct CNPDB2_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDB2_W<'a> {
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
#[doc = "Reader of field `CNPDB3`"]
pub type CNPDB3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPDB3`"]
pub struct CNPDB3_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDB3_W<'a> {
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
#[doc = "Reader of field `CNPDB4`"]
pub type CNPDB4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPDB4`"]
pub struct CNPDB4_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDB4_W<'a> {
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
#[doc = "Reader of field `CNPDB5`"]
pub type CNPDB5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPDB5`"]
pub struct CNPDB5_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDB5_W<'a> {
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
#[doc = "Reader of field `CNPDB7`"]
pub type CNPDB7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPDB7`"]
pub struct CNPDB7_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDB7_W<'a> {
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
#[doc = "Reader of field `CNPDB8`"]
pub type CNPDB8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPDB8`"]
pub struct CNPDB8_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDB8_W<'a> {
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
#[doc = "Reader of field `CNPDB9`"]
pub type CNPDB9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPDB9`"]
pub struct CNPDB9_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDB9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `CNPDB13`"]
pub type CNPDB13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPDB13`"]
pub struct CNPDB13_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDB13_W<'a> {
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
#[doc = "Reader of field `CNPDB14`"]
pub type CNPDB14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPDB14`"]
pub struct CNPDB14_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDB14_W<'a> {
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
#[doc = "Reader of field `CNPDB15`"]
pub type CNPDB15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPDB15`"]
pub struct CNPDB15_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDB15_W<'a> {
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
    pub fn cnpdb0(&self) -> CNPDB0_R {
        CNPDB0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cnpdb1(&self) -> CNPDB1_R {
        CNPDB1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cnpdb2(&self) -> CNPDB2_R {
        CNPDB2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cnpdb3(&self) -> CNPDB3_R {
        CNPDB3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cnpdb4(&self) -> CNPDB4_R {
        CNPDB4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cnpdb5(&self) -> CNPDB5_R {
        CNPDB5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cnpdb7(&self) -> CNPDB7_R {
        CNPDB7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cnpdb8(&self) -> CNPDB8_R {
        CNPDB8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cnpdb9(&self) -> CNPDB9_R {
        CNPDB9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cnpdb13(&self) -> CNPDB13_R {
        CNPDB13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cnpdb14(&self) -> CNPDB14_R {
        CNPDB14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cnpdb15(&self) -> CNPDB15_R {
        CNPDB15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cnpdb0(&mut self) -> CNPDB0_W {
        CNPDB0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cnpdb1(&mut self) -> CNPDB1_W {
        CNPDB1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cnpdb2(&mut self) -> CNPDB2_W {
        CNPDB2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cnpdb3(&mut self) -> CNPDB3_W {
        CNPDB3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cnpdb4(&mut self) -> CNPDB4_W {
        CNPDB4_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cnpdb5(&mut self) -> CNPDB5_W {
        CNPDB5_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cnpdb7(&mut self) -> CNPDB7_W {
        CNPDB7_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cnpdb8(&mut self) -> CNPDB8_W {
        CNPDB8_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cnpdb9(&mut self) -> CNPDB9_W {
        CNPDB9_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cnpdb13(&mut self) -> CNPDB13_W {
        CNPDB13_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cnpdb14(&mut self) -> CNPDB14_W {
        CNPDB14_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cnpdb15(&mut self) -> CNPDB15_W {
        CNPDB15_W { w: self }
    }
}
