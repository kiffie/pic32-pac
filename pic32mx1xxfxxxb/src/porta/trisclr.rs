#[doc = "Reader of register TRISCLR"]
pub type R = crate::R<u32, super::TRISCLR>;
#[doc = "Writer for register TRISCLR"]
pub type W = crate::W<u32, super::TRISCLR>;
#[doc = "Register TRISCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::TRISCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRISA0`"]
pub type TRISA0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRISA0`"]
pub struct TRISA0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISA0_W<'a> {
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
#[doc = "Reader of field `TRISA1`"]
pub type TRISA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRISA1`"]
pub struct TRISA1_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISA1_W<'a> {
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
#[doc = "Reader of field `TRISA2`"]
pub type TRISA2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRISA2`"]
pub struct TRISA2_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISA2_W<'a> {
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
#[doc = "Reader of field `TRISA3`"]
pub type TRISA3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRISA3`"]
pub struct TRISA3_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISA3_W<'a> {
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
#[doc = "Reader of field `TRISA4`"]
pub type TRISA4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRISA4`"]
pub struct TRISA4_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISA4_W<'a> {
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
    pub fn trisa0(&self) -> TRISA0_R {
        TRISA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn trisa1(&self) -> TRISA1_R {
        TRISA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn trisa2(&self) -> TRISA2_R {
        TRISA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn trisa3(&self) -> TRISA3_R {
        TRISA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn trisa4(&self) -> TRISA4_R {
        TRISA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn trisa0(&mut self) -> TRISA0_W {
        TRISA0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn trisa1(&mut self) -> TRISA1_W {
        TRISA1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn trisa2(&mut self) -> TRISA2_W {
        TRISA2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn trisa3(&mut self) -> TRISA3_W {
        TRISA3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn trisa4(&mut self) -> TRISA4_W {
        TRISA4_W { w: self }
    }
}
