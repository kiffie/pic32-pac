#[doc = "Reader of register CNENINV"]
pub type R = crate::R<u32, super::CNENINV>;
#[doc = "Writer for register CNENINV"]
pub type W = crate::W<u32, super::CNENINV>;
#[doc = "Register CNENINV `reset()`'s with value 0"]
impl crate::ResetValue for super::CNENINV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNIEB0`"]
pub type CNIEB0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNIEB0`"]
pub struct CNIEB0_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEB0_W<'a> {
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
#[doc = "Reader of field `CNIEB1`"]
pub type CNIEB1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNIEB1`"]
pub struct CNIEB1_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEB1_W<'a> {
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
#[doc = "Reader of field `CNIEB2`"]
pub type CNIEB2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNIEB2`"]
pub struct CNIEB2_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEB2_W<'a> {
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
#[doc = "Reader of field `CNIEB3`"]
pub type CNIEB3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNIEB3`"]
pub struct CNIEB3_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEB3_W<'a> {
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
#[doc = "Reader of field `CNIEB4`"]
pub type CNIEB4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNIEB4`"]
pub struct CNIEB4_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEB4_W<'a> {
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
#[doc = "Reader of field `CNIEB5`"]
pub type CNIEB5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNIEB5`"]
pub struct CNIEB5_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEB5_W<'a> {
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
#[doc = "Reader of field `CNIEB7`"]
pub type CNIEB7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNIEB7`"]
pub struct CNIEB7_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEB7_W<'a> {
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
#[doc = "Reader of field `CNIEB8`"]
pub type CNIEB8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNIEB8`"]
pub struct CNIEB8_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEB8_W<'a> {
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
#[doc = "Reader of field `CNIEB9`"]
pub type CNIEB9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNIEB9`"]
pub struct CNIEB9_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEB9_W<'a> {
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
#[doc = "Reader of field `CNIEB13`"]
pub type CNIEB13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNIEB13`"]
pub struct CNIEB13_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEB13_W<'a> {
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
#[doc = "Reader of field `CNIEB14`"]
pub type CNIEB14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNIEB14`"]
pub struct CNIEB14_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEB14_W<'a> {
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
#[doc = "Reader of field `CNIEB15`"]
pub type CNIEB15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNIEB15`"]
pub struct CNIEB15_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEB15_W<'a> {
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
    pub fn cnieb0(&self) -> CNIEB0_R {
        CNIEB0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cnieb1(&self) -> CNIEB1_R {
        CNIEB1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cnieb2(&self) -> CNIEB2_R {
        CNIEB2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cnieb3(&self) -> CNIEB3_R {
        CNIEB3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cnieb4(&self) -> CNIEB4_R {
        CNIEB4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cnieb5(&self) -> CNIEB5_R {
        CNIEB5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cnieb7(&self) -> CNIEB7_R {
        CNIEB7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cnieb8(&self) -> CNIEB8_R {
        CNIEB8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cnieb9(&self) -> CNIEB9_R {
        CNIEB9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cnieb13(&self) -> CNIEB13_R {
        CNIEB13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cnieb14(&self) -> CNIEB14_R {
        CNIEB14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cnieb15(&self) -> CNIEB15_R {
        CNIEB15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cnieb0(&mut self) -> CNIEB0_W {
        CNIEB0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cnieb1(&mut self) -> CNIEB1_W {
        CNIEB1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cnieb2(&mut self) -> CNIEB2_W {
        CNIEB2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cnieb3(&mut self) -> CNIEB3_W {
        CNIEB3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cnieb4(&mut self) -> CNIEB4_W {
        CNIEB4_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cnieb5(&mut self) -> CNIEB5_W {
        CNIEB5_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cnieb7(&mut self) -> CNIEB7_W {
        CNIEB7_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cnieb8(&mut self) -> CNIEB8_W {
        CNIEB8_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cnieb9(&mut self) -> CNIEB9_W {
        CNIEB9_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cnieb13(&mut self) -> CNIEB13_W {
        CNIEB13_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cnieb14(&mut self) -> CNIEB14_W {
        CNIEB14_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cnieb15(&mut self) -> CNIEB15_W {
        CNIEB15_W { w: self }
    }
}
