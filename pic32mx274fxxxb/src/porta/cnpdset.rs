#[doc = "Reader of register CNPDSET"]
pub type R = crate::R<u32, super::CNPDSET>;
#[doc = "Writer for register CNPDSET"]
pub type W = crate::W<u32, super::CNPDSET>;
#[doc = "Register CNPDSET `reset()`'s with value 0"]
impl crate::ResetValue for super::CNPDSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNPDA0`"]
pub type CNPDA0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPDA0`"]
pub struct CNPDA0_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDA0_W<'a> {
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
#[doc = "Reader of field `CNPDA1`"]
pub type CNPDA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPDA1`"]
pub struct CNPDA1_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDA1_W<'a> {
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
#[doc = "Reader of field `CNPDA2`"]
pub type CNPDA2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPDA2`"]
pub struct CNPDA2_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDA2_W<'a> {
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
#[doc = "Reader of field `CNPDA3`"]
pub type CNPDA3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPDA3`"]
pub struct CNPDA3_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDA3_W<'a> {
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
#[doc = "Reader of field `CNPDA4`"]
pub type CNPDA4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPDA4`"]
pub struct CNPDA4_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDA4_W<'a> {
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
    pub fn cnpda0(&self) -> CNPDA0_R {
        CNPDA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cnpda1(&self) -> CNPDA1_R {
        CNPDA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cnpda2(&self) -> CNPDA2_R {
        CNPDA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cnpda3(&self) -> CNPDA3_R {
        CNPDA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cnpda4(&self) -> CNPDA4_R {
        CNPDA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cnpda0(&mut self) -> CNPDA0_W {
        CNPDA0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cnpda1(&mut self) -> CNPDA1_W {
        CNPDA1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cnpda2(&mut self) -> CNPDA2_W {
        CNPDA2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cnpda3(&mut self) -> CNPDA3_W {
        CNPDA3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cnpda4(&mut self) -> CNPDA4_W {
        CNPDA4_W { w: self }
    }
}
