#[doc = "Reader of register ODCCLR"]
pub type R = crate::R<u32, super::ODCCLR>;
#[doc = "Writer for register ODCCLR"]
pub type W = crate::W<u32, super::ODCCLR>;
#[doc = "Register ODCCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::ODCCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ODCA0`"]
pub type ODCA0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ODCA0`"]
pub struct ODCA0_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCA0_W<'a> {
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
#[doc = "Reader of field `ODCA1`"]
pub type ODCA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ODCA1`"]
pub struct ODCA1_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCA1_W<'a> {
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
#[doc = "Reader of field `ODCA2`"]
pub type ODCA2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ODCA2`"]
pub struct ODCA2_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCA2_W<'a> {
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
#[doc = "Reader of field `ODCA3`"]
pub type ODCA3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ODCA3`"]
pub struct ODCA3_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCA3_W<'a> {
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
#[doc = "Reader of field `ODCA4`"]
pub type ODCA4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ODCA4`"]
pub struct ODCA4_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCA4_W<'a> {
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
    pub fn odca0(&self) -> ODCA0_R {
        ODCA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn odca1(&self) -> ODCA1_R {
        ODCA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn odca2(&self) -> ODCA2_R {
        ODCA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn odca3(&self) -> ODCA3_R {
        ODCA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn odca4(&self) -> ODCA4_R {
        ODCA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn odca0(&mut self) -> ODCA0_W {
        ODCA0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn odca1(&mut self) -> ODCA1_W {
        ODCA1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn odca2(&mut self) -> ODCA2_W {
        ODCA2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn odca3(&mut self) -> ODCA3_W {
        ODCA3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn odca4(&mut self) -> ODCA4_W {
        ODCA4_W { w: self }
    }
}
