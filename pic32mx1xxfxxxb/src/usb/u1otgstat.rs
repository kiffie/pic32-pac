#[doc = "Reader of register U1OTGSTAT"]
pub type R = crate::R<u32, super::U1OTGSTAT>;
#[doc = "Writer for register U1OTGSTAT"]
pub type W = crate::W<u32, super::U1OTGSTAT>;
#[doc = "Register U1OTGSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::U1OTGSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VBUSVD`"]
pub type VBUSVD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBUSVD`"]
pub struct VBUSVD_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSVD_W<'a> {
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
#[doc = "Reader of field `SESEND`"]
pub type SESEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SESEND`"]
pub struct SESEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SESEND_W<'a> {
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
#[doc = "Reader of field `SESVD`"]
pub type SESVD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SESVD`"]
pub struct SESVD_W<'a> {
    w: &'a mut W,
}
impl<'a> SESVD_W<'a> {
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
#[doc = "Reader of field `LSTATE`"]
pub type LSTATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSTATE`"]
pub struct LSTATE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTATE_W<'a> {
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
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ID`"]
pub struct ID_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_W<'a> {
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
    pub fn vbusvd(&self) -> VBUSVD_R {
        VBUSVD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sesend(&self) -> SESEND_R {
        SESEND_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sesvd(&self) -> SESVD_R {
        SESVD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lstate(&self) -> LSTATE_R {
        LSTATE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vbusvd(&mut self) -> VBUSVD_W {
        VBUSVD_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sesend(&mut self) -> SESEND_W {
        SESEND_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sesvd(&mut self) -> SESVD_W {
        SESVD_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lstate(&mut self) -> LSTATE_W {
        LSTATE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W {
        ID_W { w: self }
    }
}
