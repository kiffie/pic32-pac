#[doc = "Reader of register LATSET"]
pub type R = crate::R<u32, super::LATSET>;
#[doc = "Writer for register LATSET"]
pub type W = crate::W<u32, super::LATSET>;
#[doc = "Register LATSET `reset()`'s with value 0"]
impl crate::ResetValue for super::LATSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LATB0`"]
pub type LATB0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LATB0`"]
pub struct LATB0_W<'a> {
    w: &'a mut W,
}
impl<'a> LATB0_W<'a> {
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
#[doc = "Reader of field `LATB1`"]
pub type LATB1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LATB1`"]
pub struct LATB1_W<'a> {
    w: &'a mut W,
}
impl<'a> LATB1_W<'a> {
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
#[doc = "Reader of field `LATB2`"]
pub type LATB2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LATB2`"]
pub struct LATB2_W<'a> {
    w: &'a mut W,
}
impl<'a> LATB2_W<'a> {
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
#[doc = "Reader of field `LATB3`"]
pub type LATB3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LATB3`"]
pub struct LATB3_W<'a> {
    w: &'a mut W,
}
impl<'a> LATB3_W<'a> {
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
#[doc = "Reader of field `LATB4`"]
pub type LATB4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LATB4`"]
pub struct LATB4_W<'a> {
    w: &'a mut W,
}
impl<'a> LATB4_W<'a> {
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
#[doc = "Reader of field `LATB5`"]
pub type LATB5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LATB5`"]
pub struct LATB5_W<'a> {
    w: &'a mut W,
}
impl<'a> LATB5_W<'a> {
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
#[doc = "Reader of field `LATB6`"]
pub type LATB6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LATB6`"]
pub struct LATB6_W<'a> {
    w: &'a mut W,
}
impl<'a> LATB6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `LATB7`"]
pub type LATB7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LATB7`"]
pub struct LATB7_W<'a> {
    w: &'a mut W,
}
impl<'a> LATB7_W<'a> {
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
#[doc = "Reader of field `LATB8`"]
pub type LATB8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LATB8`"]
pub struct LATB8_W<'a> {
    w: &'a mut W,
}
impl<'a> LATB8_W<'a> {
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
#[doc = "Reader of field `LATB9`"]
pub type LATB9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LATB9`"]
pub struct LATB9_W<'a> {
    w: &'a mut W,
}
impl<'a> LATB9_W<'a> {
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
#[doc = "Reader of field `LATB10`"]
pub type LATB10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LATB10`"]
pub struct LATB10_W<'a> {
    w: &'a mut W,
}
impl<'a> LATB10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `LATB11`"]
pub type LATB11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LATB11`"]
pub struct LATB11_W<'a> {
    w: &'a mut W,
}
impl<'a> LATB11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `LATB12`"]
pub type LATB12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LATB12`"]
pub struct LATB12_W<'a> {
    w: &'a mut W,
}
impl<'a> LATB12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `LATB13`"]
pub type LATB13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LATB13`"]
pub struct LATB13_W<'a> {
    w: &'a mut W,
}
impl<'a> LATB13_W<'a> {
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
#[doc = "Reader of field `LATB14`"]
pub type LATB14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LATB14`"]
pub struct LATB14_W<'a> {
    w: &'a mut W,
}
impl<'a> LATB14_W<'a> {
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
#[doc = "Reader of field `LATB15`"]
pub type LATB15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LATB15`"]
pub struct LATB15_W<'a> {
    w: &'a mut W,
}
impl<'a> LATB15_W<'a> {
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
    pub fn latb0(&self) -> LATB0_R {
        LATB0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn latb1(&self) -> LATB1_R {
        LATB1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn latb2(&self) -> LATB2_R {
        LATB2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn latb3(&self) -> LATB3_R {
        LATB3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn latb4(&self) -> LATB4_R {
        LATB4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn latb5(&self) -> LATB5_R {
        LATB5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn latb6(&self) -> LATB6_R {
        LATB6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn latb7(&self) -> LATB7_R {
        LATB7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn latb8(&self) -> LATB8_R {
        LATB8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn latb9(&self) -> LATB9_R {
        LATB9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn latb10(&self) -> LATB10_R {
        LATB10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn latb11(&self) -> LATB11_R {
        LATB11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn latb12(&self) -> LATB12_R {
        LATB12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn latb13(&self) -> LATB13_R {
        LATB13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn latb14(&self) -> LATB14_R {
        LATB14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn latb15(&self) -> LATB15_R {
        LATB15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn latb0(&mut self) -> LATB0_W {
        LATB0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn latb1(&mut self) -> LATB1_W {
        LATB1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn latb2(&mut self) -> LATB2_W {
        LATB2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn latb3(&mut self) -> LATB3_W {
        LATB3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn latb4(&mut self) -> LATB4_W {
        LATB4_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn latb5(&mut self) -> LATB5_W {
        LATB5_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn latb6(&mut self) -> LATB6_W {
        LATB6_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn latb7(&mut self) -> LATB7_W {
        LATB7_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn latb8(&mut self) -> LATB8_W {
        LATB8_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn latb9(&mut self) -> LATB9_W {
        LATB9_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn latb10(&mut self) -> LATB10_W {
        LATB10_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn latb11(&mut self) -> LATB11_W {
        LATB11_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn latb12(&mut self) -> LATB12_W {
        LATB12_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn latb13(&mut self) -> LATB13_W {
        LATB13_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn latb14(&mut self) -> LATB14_W {
        LATB14_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn latb15(&mut self) -> LATB15_W {
        LATB15_W { w: self }
    }
}
