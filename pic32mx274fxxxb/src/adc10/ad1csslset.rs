#[doc = "Reader of register AD1CSSLSET"]
pub type R = crate::R<u32, super::AD1CSSLSET>;
#[doc = "Writer for register AD1CSSLSET"]
pub type W = crate::W<u32, super::AD1CSSLSET>;
#[doc = "Register AD1CSSLSET `reset()`'s with value 0"]
impl crate::ResetValue for super::AD1CSSLSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSSL0`"]
pub type CSSL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSSL0`"]
pub struct CSSL0_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSL0_W<'a> {
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
#[doc = "Reader of field `CSSL1`"]
pub type CSSL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSSL1`"]
pub struct CSSL1_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSL1_W<'a> {
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
#[doc = "Reader of field `CSSL2`"]
pub type CSSL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSSL2`"]
pub struct CSSL2_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSL2_W<'a> {
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
#[doc = "Reader of field `CSSL3`"]
pub type CSSL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSSL3`"]
pub struct CSSL3_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSL3_W<'a> {
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
#[doc = "Reader of field `CSSL4`"]
pub type CSSL4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSSL4`"]
pub struct CSSL4_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSL4_W<'a> {
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
#[doc = "Reader of field `CSSL5`"]
pub type CSSL5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSSL5`"]
pub struct CSSL5_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSL5_W<'a> {
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
#[doc = "Reader of field `CSSL9`"]
pub type CSSL9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSSL9`"]
pub struct CSSL9_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSL9_W<'a> {
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
#[doc = "Reader of field `CSSL10`"]
pub type CSSL10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSSL10`"]
pub struct CSSL10_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSL10_W<'a> {
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
#[doc = "Reader of field `CSSL11`"]
pub type CSSL11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSSL11`"]
pub struct CSSL11_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSL11_W<'a> {
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
#[doc = "Reader of field `CSSL13`"]
pub type CSSL13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSSL13`"]
pub struct CSSL13_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSL13_W<'a> {
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
#[doc = "Reader of field `CSSL14`"]
pub type CSSL14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSSL14`"]
pub struct CSSL14_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSL14_W<'a> {
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
#[doc = "Reader of field `CSSL15`"]
pub type CSSL15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSSL15`"]
pub struct CSSL15_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSL15_W<'a> {
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
#[doc = "Reader of field `CSSL17`"]
pub type CSSL17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSSL17`"]
pub struct CSSL17_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSL17_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cssl0(&self) -> CSSL0_R {
        CSSL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cssl1(&self) -> CSSL1_R {
        CSSL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cssl2(&self) -> CSSL2_R {
        CSSL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cssl3(&self) -> CSSL3_R {
        CSSL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cssl4(&self) -> CSSL4_R {
        CSSL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cssl5(&self) -> CSSL5_R {
        CSSL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cssl9(&self) -> CSSL9_R {
        CSSL9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cssl10(&self) -> CSSL10_R {
        CSSL10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cssl11(&self) -> CSSL11_R {
        CSSL11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cssl13(&self) -> CSSL13_R {
        CSSL13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cssl14(&self) -> CSSL14_R {
        CSSL14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cssl15(&self) -> CSSL15_R {
        CSSL15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cssl17(&self) -> CSSL17_R {
        CSSL17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cssl0(&mut self) -> CSSL0_W {
        CSSL0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cssl1(&mut self) -> CSSL1_W {
        CSSL1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cssl2(&mut self) -> CSSL2_W {
        CSSL2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cssl3(&mut self) -> CSSL3_W {
        CSSL3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cssl4(&mut self) -> CSSL4_W {
        CSSL4_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cssl5(&mut self) -> CSSL5_W {
        CSSL5_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cssl9(&mut self) -> CSSL9_W {
        CSSL9_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cssl10(&mut self) -> CSSL10_W {
        CSSL10_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cssl11(&mut self) -> CSSL11_W {
        CSSL11_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cssl13(&mut self) -> CSSL13_W {
        CSSL13_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cssl14(&mut self) -> CSSL14_W {
        CSSL14_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cssl15(&mut self) -> CSSL15_W {
        CSSL15_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cssl17(&mut self) -> CSSL17_W {
        CSSL17_W { w: self }
    }
}
