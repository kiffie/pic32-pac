#[doc = "Reader of register CNENSET"]
pub type R = crate::R<u32, super::CNENSET>;
#[doc = "Writer for register CNENSET"]
pub type W = crate::W<u32, super::CNENSET>;
#[doc = "Register CNENSET `reset()`'s with value 0"]
impl crate::ResetValue for super::CNENSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNIEA0`"]
pub type CNIEA0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNIEA0`"]
pub struct CNIEA0_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEA0_W<'a> {
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
#[doc = "Reader of field `CNIEA1`"]
pub type CNIEA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNIEA1`"]
pub struct CNIEA1_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEA1_W<'a> {
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
#[doc = "Reader of field `CNIEA2`"]
pub type CNIEA2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNIEA2`"]
pub struct CNIEA2_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEA2_W<'a> {
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
#[doc = "Reader of field `CNIEA3`"]
pub type CNIEA3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNIEA3`"]
pub struct CNIEA3_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEA3_W<'a> {
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
#[doc = "Reader of field `CNIEA4`"]
pub type CNIEA4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNIEA4`"]
pub struct CNIEA4_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEA4_W<'a> {
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
    pub fn cniea0(&self) -> CNIEA0_R {
        CNIEA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cniea1(&self) -> CNIEA1_R {
        CNIEA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cniea2(&self) -> CNIEA2_R {
        CNIEA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cniea3(&self) -> CNIEA3_R {
        CNIEA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cniea4(&self) -> CNIEA4_R {
        CNIEA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cniea0(&mut self) -> CNIEA0_W {
        CNIEA0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cniea1(&mut self) -> CNIEA1_W {
        CNIEA1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cniea2(&mut self) -> CNIEA2_W {
        CNIEA2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cniea3(&mut self) -> CNIEA3_W {
        CNIEA3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cniea4(&mut self) -> CNIEA4_W {
        CNIEA4_W { w: self }
    }
}
