#[doc = "Reader of register CNPUINV"]
pub type R = crate::R<u32, super::CNPUINV>;
#[doc = "Writer for register CNPUINV"]
pub type W = crate::W<u32, super::CNPUINV>;
#[doc = "Register CNPUINV `reset()`'s with value 0"]
impl crate::ResetValue for super::CNPUINV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNPUB0`"]
pub type CNPUB0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPUB0`"]
pub struct CNPUB0_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUB0_W<'a> {
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
#[doc = "Reader of field `CNPUB1`"]
pub type CNPUB1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPUB1`"]
pub struct CNPUB1_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUB1_W<'a> {
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
#[doc = "Reader of field `CNPUB2`"]
pub type CNPUB2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPUB2`"]
pub struct CNPUB2_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUB2_W<'a> {
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
#[doc = "Reader of field `CNPUB3`"]
pub type CNPUB3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPUB3`"]
pub struct CNPUB3_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUB3_W<'a> {
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
#[doc = "Reader of field `CNPUB4`"]
pub type CNPUB4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPUB4`"]
pub struct CNPUB4_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUB4_W<'a> {
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
#[doc = "Reader of field `CNPUB5`"]
pub type CNPUB5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPUB5`"]
pub struct CNPUB5_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUB5_W<'a> {
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
#[doc = "Reader of field `CNPUB7`"]
pub type CNPUB7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPUB7`"]
pub struct CNPUB7_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUB7_W<'a> {
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
#[doc = "Reader of field `CNPUB8`"]
pub type CNPUB8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPUB8`"]
pub struct CNPUB8_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUB8_W<'a> {
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
#[doc = "Reader of field `CNPUB9`"]
pub type CNPUB9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPUB9`"]
pub struct CNPUB9_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUB9_W<'a> {
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
#[doc = "Reader of field `CNPUB13`"]
pub type CNPUB13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPUB13`"]
pub struct CNPUB13_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUB13_W<'a> {
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
#[doc = "Reader of field `CNPUB14`"]
pub type CNPUB14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPUB14`"]
pub struct CNPUB14_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUB14_W<'a> {
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
#[doc = "Reader of field `CNPUB15`"]
pub type CNPUB15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNPUB15`"]
pub struct CNPUB15_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUB15_W<'a> {
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
    pub fn cnpub0(&self) -> CNPUB0_R {
        CNPUB0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cnpub1(&self) -> CNPUB1_R {
        CNPUB1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cnpub2(&self) -> CNPUB2_R {
        CNPUB2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cnpub3(&self) -> CNPUB3_R {
        CNPUB3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cnpub4(&self) -> CNPUB4_R {
        CNPUB4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cnpub5(&self) -> CNPUB5_R {
        CNPUB5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cnpub7(&self) -> CNPUB7_R {
        CNPUB7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cnpub8(&self) -> CNPUB8_R {
        CNPUB8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cnpub9(&self) -> CNPUB9_R {
        CNPUB9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cnpub13(&self) -> CNPUB13_R {
        CNPUB13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cnpub14(&self) -> CNPUB14_R {
        CNPUB14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cnpub15(&self) -> CNPUB15_R {
        CNPUB15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cnpub0(&mut self) -> CNPUB0_W {
        CNPUB0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cnpub1(&mut self) -> CNPUB1_W {
        CNPUB1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cnpub2(&mut self) -> CNPUB2_W {
        CNPUB2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cnpub3(&mut self) -> CNPUB3_W {
        CNPUB3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cnpub4(&mut self) -> CNPUB4_W {
        CNPUB4_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cnpub5(&mut self) -> CNPUB5_W {
        CNPUB5_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cnpub7(&mut self) -> CNPUB7_W {
        CNPUB7_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cnpub8(&mut self) -> CNPUB8_W {
        CNPUB8_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cnpub9(&mut self) -> CNPUB9_W {
        CNPUB9_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cnpub13(&mut self) -> CNPUB13_W {
        CNPUB13_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cnpub14(&mut self) -> CNPUB14_W {
        CNPUB14_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cnpub15(&mut self) -> CNPUB15_W {
        CNPUB15_W { w: self }
    }
}
