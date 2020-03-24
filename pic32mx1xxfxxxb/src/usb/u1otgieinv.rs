#[doc = "Reader of register U1OTGIEINV"]
pub type R = crate::R<u32, super::U1OTGIEINV>;
#[doc = "Writer for register U1OTGIEINV"]
pub type W = crate::W<u32, super::U1OTGIEINV>;
#[doc = "Register U1OTGIEINV `reset()`'s with value 0"]
impl crate::ResetValue for super::U1OTGIEINV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VBUSVDIE`"]
pub type VBUSVDIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBUSVDIE`"]
pub struct VBUSVDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSVDIE_W<'a> {
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
#[doc = "Reader of field `SESENDIE`"]
pub type SESENDIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SESENDIE`"]
pub struct SESENDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SESENDIE_W<'a> {
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
#[doc = "Reader of field `SESVDIE`"]
pub type SESVDIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SESVDIE`"]
pub struct SESVDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SESVDIE_W<'a> {
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
#[doc = "Reader of field `ACTVIE`"]
pub type ACTVIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACTVIE`"]
pub struct ACTVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTVIE_W<'a> {
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
#[doc = "Reader of field `LSTATEIE`"]
pub type LSTATEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSTATEIE`"]
pub struct LSTATEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTATEIE_W<'a> {
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
#[doc = "Reader of field `T1MSECIE`"]
pub type T1MSECIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T1MSECIE`"]
pub struct T1MSECIE_W<'a> {
    w: &'a mut W,
}
impl<'a> T1MSECIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `IDIE`"]
pub type IDIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDIE`"]
pub struct IDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDIE_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vbusvdie(&self) -> VBUSVDIE_R {
        VBUSVDIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sesendie(&self) -> SESENDIE_R {
        SESENDIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sesvdie(&self) -> SESVDIE_R {
        SESVDIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn actvie(&self) -> ACTVIE_R {
        ACTVIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lstateie(&self) -> LSTATEIE_R {
        LSTATEIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn t1msecie(&self) -> T1MSECIE_R {
        T1MSECIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn idie(&self) -> IDIE_R {
        IDIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vbusvdie(&mut self) -> VBUSVDIE_W {
        VBUSVDIE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sesendie(&mut self) -> SESENDIE_W {
        SESENDIE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sesvdie(&mut self) -> SESVDIE_W {
        SESVDIE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn actvie(&mut self) -> ACTVIE_W {
        ACTVIE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lstateie(&mut self) -> LSTATEIE_W {
        LSTATEIE_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn t1msecie(&mut self) -> T1MSECIE_W {
        T1MSECIE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn idie(&mut self) -> IDIE_W {
        IDIE_W { w: self }
    }
}
