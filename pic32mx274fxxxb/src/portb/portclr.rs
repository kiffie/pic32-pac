#[doc = "Reader of register PORTCLR"]
pub type R = crate::R<u32, super::PORTCLR>;
#[doc = "Writer for register PORTCLR"]
pub type W = crate::W<u32, super::PORTCLR>;
#[doc = "Register PORTCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::PORTCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RB0`"]
pub type RB0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RB0`"]
pub struct RB0_W<'a> {
    w: &'a mut W,
}
impl<'a> RB0_W<'a> {
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
#[doc = "Reader of field `RB1`"]
pub type RB1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RB1`"]
pub struct RB1_W<'a> {
    w: &'a mut W,
}
impl<'a> RB1_W<'a> {
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
#[doc = "Reader of field `RB2`"]
pub type RB2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RB2`"]
pub struct RB2_W<'a> {
    w: &'a mut W,
}
impl<'a> RB2_W<'a> {
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
#[doc = "Reader of field `RB3`"]
pub type RB3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RB3`"]
pub struct RB3_W<'a> {
    w: &'a mut W,
}
impl<'a> RB3_W<'a> {
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
#[doc = "Reader of field `RB4`"]
pub type RB4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RB4`"]
pub struct RB4_W<'a> {
    w: &'a mut W,
}
impl<'a> RB4_W<'a> {
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
#[doc = "Reader of field `RB5`"]
pub type RB5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RB5`"]
pub struct RB5_W<'a> {
    w: &'a mut W,
}
impl<'a> RB5_W<'a> {
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
#[doc = "Reader of field `RB7`"]
pub type RB7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RB7`"]
pub struct RB7_W<'a> {
    w: &'a mut W,
}
impl<'a> RB7_W<'a> {
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
#[doc = "Reader of field `RB8`"]
pub type RB8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RB8`"]
pub struct RB8_W<'a> {
    w: &'a mut W,
}
impl<'a> RB8_W<'a> {
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
#[doc = "Reader of field `RB9`"]
pub type RB9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RB9`"]
pub struct RB9_W<'a> {
    w: &'a mut W,
}
impl<'a> RB9_W<'a> {
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
#[doc = "Reader of field `RB13`"]
pub type RB13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RB13`"]
pub struct RB13_W<'a> {
    w: &'a mut W,
}
impl<'a> RB13_W<'a> {
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
#[doc = "Reader of field `RB14`"]
pub type RB14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RB14`"]
pub struct RB14_W<'a> {
    w: &'a mut W,
}
impl<'a> RB14_W<'a> {
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
#[doc = "Reader of field `RB15`"]
pub type RB15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RB15`"]
pub struct RB15_W<'a> {
    w: &'a mut W,
}
impl<'a> RB15_W<'a> {
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
    pub fn rb0(&self) -> RB0_R {
        RB0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rb1(&self) -> RB1_R {
        RB1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rb2(&self) -> RB2_R {
        RB2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rb3(&self) -> RB3_R {
        RB3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rb4(&self) -> RB4_R {
        RB4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rb5(&self) -> RB5_R {
        RB5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rb7(&self) -> RB7_R {
        RB7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rb8(&self) -> RB8_R {
        RB8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rb9(&self) -> RB9_R {
        RB9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rb13(&self) -> RB13_R {
        RB13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rb14(&self) -> RB14_R {
        RB14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rb15(&self) -> RB15_R {
        RB15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rb0(&mut self) -> RB0_W {
        RB0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rb1(&mut self) -> RB1_W {
        RB1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rb2(&mut self) -> RB2_W {
        RB2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rb3(&mut self) -> RB3_W {
        RB3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rb4(&mut self) -> RB4_W {
        RB4_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rb5(&mut self) -> RB5_W {
        RB5_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rb7(&mut self) -> RB7_W {
        RB7_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rb8(&mut self) -> RB8_W {
        RB8_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rb9(&mut self) -> RB9_W {
        RB9_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rb13(&mut self) -> RB13_W {
        RB13_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rb14(&mut self) -> RB14_W {
        RB14_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rb15(&mut self) -> RB15_W {
        RB15_W { w: self }
    }
}
