#[doc = "Reader of register CNSTATINV"]
pub type R = crate::R<u32, super::CNSTATINV>;
#[doc = "Writer for register CNSTATINV"]
pub type W = crate::W<u32, super::CNSTATINV>;
#[doc = "Register CNSTATINV `reset()`'s with value 0"]
impl crate::ResetValue for super::CNSTATINV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNSTATB0`"]
pub type CNSTATB0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNSTATB0`"]
pub struct CNSTATB0_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATB0_W<'a> {
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
#[doc = "Reader of field `CNSTATB1`"]
pub type CNSTATB1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNSTATB1`"]
pub struct CNSTATB1_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATB1_W<'a> {
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
#[doc = "Reader of field `CNSTATB2`"]
pub type CNSTATB2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNSTATB2`"]
pub struct CNSTATB2_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATB2_W<'a> {
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
#[doc = "Reader of field `CNSTATB3`"]
pub type CNSTATB3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNSTATB3`"]
pub struct CNSTATB3_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATB3_W<'a> {
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
#[doc = "Reader of field `CNSTATB4`"]
pub type CNSTATB4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNSTATB4`"]
pub struct CNSTATB4_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATB4_W<'a> {
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
#[doc = "Reader of field `CNSTATB5`"]
pub type CNSTATB5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNSTATB5`"]
pub struct CNSTATB5_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATB5_W<'a> {
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
#[doc = "Reader of field `CNSTATB7`"]
pub type CNSTATB7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNSTATB7`"]
pub struct CNSTATB7_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATB7_W<'a> {
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
#[doc = "Reader of field `CNSTATB8`"]
pub type CNSTATB8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNSTATB8`"]
pub struct CNSTATB8_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATB8_W<'a> {
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
#[doc = "Reader of field `CNSTATB9`"]
pub type CNSTATB9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNSTATB9`"]
pub struct CNSTATB9_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATB9_W<'a> {
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
#[doc = "Reader of field `CNSTATB13`"]
pub type CNSTATB13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNSTATB13`"]
pub struct CNSTATB13_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATB13_W<'a> {
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
#[doc = "Reader of field `CNSTATB14`"]
pub type CNSTATB14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNSTATB14`"]
pub struct CNSTATB14_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATB14_W<'a> {
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
#[doc = "Reader of field `CNSTATB15`"]
pub type CNSTATB15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNSTATB15`"]
pub struct CNSTATB15_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATB15_W<'a> {
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
    pub fn cnstatb0(&self) -> CNSTATB0_R {
        CNSTATB0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cnstatb1(&self) -> CNSTATB1_R {
        CNSTATB1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cnstatb2(&self) -> CNSTATB2_R {
        CNSTATB2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cnstatb3(&self) -> CNSTATB3_R {
        CNSTATB3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cnstatb4(&self) -> CNSTATB4_R {
        CNSTATB4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cnstatb5(&self) -> CNSTATB5_R {
        CNSTATB5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cnstatb7(&self) -> CNSTATB7_R {
        CNSTATB7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cnstatb8(&self) -> CNSTATB8_R {
        CNSTATB8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cnstatb9(&self) -> CNSTATB9_R {
        CNSTATB9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cnstatb13(&self) -> CNSTATB13_R {
        CNSTATB13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cnstatb14(&self) -> CNSTATB14_R {
        CNSTATB14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cnstatb15(&self) -> CNSTATB15_R {
        CNSTATB15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cnstatb0(&mut self) -> CNSTATB0_W {
        CNSTATB0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cnstatb1(&mut self) -> CNSTATB1_W {
        CNSTATB1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cnstatb2(&mut self) -> CNSTATB2_W {
        CNSTATB2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cnstatb3(&mut self) -> CNSTATB3_W {
        CNSTATB3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cnstatb4(&mut self) -> CNSTATB4_W {
        CNSTATB4_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cnstatb5(&mut self) -> CNSTATB5_W {
        CNSTATB5_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cnstatb7(&mut self) -> CNSTATB7_W {
        CNSTATB7_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cnstatb8(&mut self) -> CNSTATB8_W {
        CNSTATB8_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cnstatb9(&mut self) -> CNSTATB9_W {
        CNSTATB9_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cnstatb13(&mut self) -> CNSTATB13_W {
        CNSTATB13_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cnstatb14(&mut self) -> CNSTATB14_W {
        CNSTATB14_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cnstatb15(&mut self) -> CNSTATB15_W {
        CNSTATB15_W { w: self }
    }
}
