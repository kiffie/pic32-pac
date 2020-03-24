#[doc = "Reader of register CNPU"]
pub type R = crate::R<u32, super::CNPU>;
#[doc = "Writer for register CNPU"]
pub type W = crate::W<u32, super::CNPU>;
#[doc = "Register CNPU `reset()`'s with value 0"]
impl crate::ResetValue for super::CNPU {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNPUA0`"]
pub type CNPUA0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPUA0`"]
pub struct CNPUA0_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUA0_W<'a> {
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
#[doc = "Reader of field `CNPUA1`"]
pub type CNPUA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPUA1`"]
pub struct CNPUA1_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUA1_W<'a> {
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
#[doc = "Reader of field `CNPUA2`"]
pub type CNPUA2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPUA2`"]
pub struct CNPUA2_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUA2_W<'a> {
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
#[doc = "Reader of field `CNPUA3`"]
pub type CNPUA3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPUA3`"]
pub struct CNPUA3_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUA3_W<'a> {
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
#[doc = "Reader of field `CNPUA4`"]
pub type CNPUA4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPUA4`"]
pub struct CNPUA4_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUA4_W<'a> {
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
    pub fn cnpua0(&self) -> CNPUA0_R {
        CNPUA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cnpua1(&self) -> CNPUA1_R {
        CNPUA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cnpua2(&self) -> CNPUA2_R {
        CNPUA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cnpua3(&self) -> CNPUA3_R {
        CNPUA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cnpua4(&self) -> CNPUA4_R {
        CNPUA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cnpua0(&mut self) -> CNPUA0_W {
        CNPUA0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cnpua1(&mut self) -> CNPUA1_W {
        CNPUA1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cnpua2(&mut self) -> CNPUA2_W {
        CNPUA2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cnpua3(&mut self) -> CNPUA3_W {
        CNPUA3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cnpua4(&mut self) -> CNPUA4_W {
        CNPUA4_W { w: self }
    }
}
