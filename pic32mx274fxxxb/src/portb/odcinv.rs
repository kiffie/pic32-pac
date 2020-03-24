#[doc = "Reader of register ODCINV"]
pub type R = crate::R<u32, super::ODCINV>;
#[doc = "Writer for register ODCINV"]
pub type W = crate::W<u32, super::ODCINV>;
#[doc = "Register ODCINV `reset()`'s with value 0"]
impl crate::ResetValue for super::ODCINV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ODCB0`"]
pub type ODCB0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ODCB0`"]
pub struct ODCB0_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCB0_W<'a> {
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
#[doc = "Reader of field `ODCB1`"]
pub type ODCB1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ODCB1`"]
pub struct ODCB1_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCB1_W<'a> {
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
#[doc = "Reader of field `ODCB2`"]
pub type ODCB2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ODCB2`"]
pub struct ODCB2_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCB2_W<'a> {
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
#[doc = "Reader of field `ODCB3`"]
pub type ODCB3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ODCB3`"]
pub struct ODCB3_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCB3_W<'a> {
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
#[doc = "Reader of field `ODCB4`"]
pub type ODCB4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ODCB4`"]
pub struct ODCB4_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCB4_W<'a> {
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
#[doc = "Reader of field `ODCB5`"]
pub type ODCB5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ODCB5`"]
pub struct ODCB5_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCB5_W<'a> {
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
#[doc = "Reader of field `ODCB7`"]
pub type ODCB7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ODCB7`"]
pub struct ODCB7_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCB7_W<'a> {
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
#[doc = "Reader of field `ODCB8`"]
pub type ODCB8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ODCB8`"]
pub struct ODCB8_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCB8_W<'a> {
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
#[doc = "Reader of field `ODCB9`"]
pub type ODCB9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ODCB9`"]
pub struct ODCB9_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCB9_W<'a> {
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
#[doc = "Reader of field `ODCB13`"]
pub type ODCB13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ODCB13`"]
pub struct ODCB13_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCB13_W<'a> {
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
#[doc = "Reader of field `ODCB14`"]
pub type ODCB14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ODCB14`"]
pub struct ODCB14_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCB14_W<'a> {
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
#[doc = "Reader of field `ODCB15`"]
pub type ODCB15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ODCB15`"]
pub struct ODCB15_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCB15_W<'a> {
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
    pub fn odcb0(&self) -> ODCB0_R {
        ODCB0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn odcb1(&self) -> ODCB1_R {
        ODCB1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn odcb2(&self) -> ODCB2_R {
        ODCB2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn odcb3(&self) -> ODCB3_R {
        ODCB3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn odcb4(&self) -> ODCB4_R {
        ODCB4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn odcb5(&self) -> ODCB5_R {
        ODCB5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn odcb7(&self) -> ODCB7_R {
        ODCB7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn odcb8(&self) -> ODCB8_R {
        ODCB8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn odcb9(&self) -> ODCB9_R {
        ODCB9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn odcb13(&self) -> ODCB13_R {
        ODCB13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn odcb14(&self) -> ODCB14_R {
        ODCB14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn odcb15(&self) -> ODCB15_R {
        ODCB15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn odcb0(&mut self) -> ODCB0_W {
        ODCB0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn odcb1(&mut self) -> ODCB1_W {
        ODCB1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn odcb2(&mut self) -> ODCB2_W {
        ODCB2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn odcb3(&mut self) -> ODCB3_W {
        ODCB3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn odcb4(&mut self) -> ODCB4_W {
        ODCB4_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn odcb5(&mut self) -> ODCB5_W {
        ODCB5_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn odcb7(&mut self) -> ODCB7_W {
        ODCB7_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn odcb8(&mut self) -> ODCB8_W {
        ODCB8_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn odcb9(&mut self) -> ODCB9_W {
        ODCB9_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn odcb13(&mut self) -> ODCB13_W {
        ODCB13_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn odcb14(&mut self) -> ODCB14_W {
        ODCB14_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn odcb15(&mut self) -> ODCB15_W {
        ODCB15_W { w: self }
    }
}
