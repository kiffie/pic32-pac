#[doc = "Reader of register TRISINV"]
pub type R = crate::R<u32, super::TRISINV>;
#[doc = "Writer for register TRISINV"]
pub type W = crate::W<u32, super::TRISINV>;
#[doc = "Register TRISINV `reset()`'s with value 0"]
impl crate::ResetValue for super::TRISINV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRISB0`"]
pub type TRISB0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRISB0`"]
pub struct TRISB0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISB0_W<'a> {
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
#[doc = "Reader of field `TRISB1`"]
pub type TRISB1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRISB1`"]
pub struct TRISB1_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISB1_W<'a> {
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
#[doc = "Reader of field `TRISB2`"]
pub type TRISB2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRISB2`"]
pub struct TRISB2_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISB2_W<'a> {
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
#[doc = "Reader of field `TRISB3`"]
pub type TRISB3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRISB3`"]
pub struct TRISB3_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISB3_W<'a> {
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
#[doc = "Reader of field `TRISB4`"]
pub type TRISB4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRISB4`"]
pub struct TRISB4_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISB4_W<'a> {
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
#[doc = "Reader of field `TRISB5`"]
pub type TRISB5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRISB5`"]
pub struct TRISB5_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISB5_W<'a> {
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
#[doc = "Reader of field `TRISB7`"]
pub type TRISB7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRISB7`"]
pub struct TRISB7_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISB7_W<'a> {
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
#[doc = "Reader of field `TRISB8`"]
pub type TRISB8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRISB8`"]
pub struct TRISB8_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISB8_W<'a> {
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
#[doc = "Reader of field `TRISB9`"]
pub type TRISB9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRISB9`"]
pub struct TRISB9_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISB9_W<'a> {
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
#[doc = "Reader of field `TRISB13`"]
pub type TRISB13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRISB13`"]
pub struct TRISB13_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISB13_W<'a> {
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
#[doc = "Reader of field `TRISB14`"]
pub type TRISB14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRISB14`"]
pub struct TRISB14_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISB14_W<'a> {
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
#[doc = "Reader of field `TRISB15`"]
pub type TRISB15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRISB15`"]
pub struct TRISB15_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISB15_W<'a> {
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
    pub fn trisb0(&self) -> TRISB0_R {
        TRISB0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn trisb1(&self) -> TRISB1_R {
        TRISB1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn trisb2(&self) -> TRISB2_R {
        TRISB2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn trisb3(&self) -> TRISB3_R {
        TRISB3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn trisb4(&self) -> TRISB4_R {
        TRISB4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn trisb5(&self) -> TRISB5_R {
        TRISB5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn trisb7(&self) -> TRISB7_R {
        TRISB7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn trisb8(&self) -> TRISB8_R {
        TRISB8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn trisb9(&self) -> TRISB9_R {
        TRISB9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn trisb13(&self) -> TRISB13_R {
        TRISB13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn trisb14(&self) -> TRISB14_R {
        TRISB14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn trisb15(&self) -> TRISB15_R {
        TRISB15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn trisb0(&mut self) -> TRISB0_W {
        TRISB0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn trisb1(&mut self) -> TRISB1_W {
        TRISB1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn trisb2(&mut self) -> TRISB2_W {
        TRISB2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn trisb3(&mut self) -> TRISB3_W {
        TRISB3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn trisb4(&mut self) -> TRISB4_W {
        TRISB4_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn trisb5(&mut self) -> TRISB5_W {
        TRISB5_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn trisb7(&mut self) -> TRISB7_W {
        TRISB7_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn trisb8(&mut self) -> TRISB8_W {
        TRISB8_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn trisb9(&mut self) -> TRISB9_W {
        TRISB9_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn trisb13(&mut self) -> TRISB13_W {
        TRISB13_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn trisb14(&mut self) -> TRISB14_W {
        TRISB14_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn trisb15(&mut self) -> TRISB15_W {
        TRISB15_W { w: self }
    }
}
