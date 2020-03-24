#[doc = "Reader of register U1OTGIRCLR"]
pub type R = crate::R<u32, super::U1OTGIRCLR>;
#[doc = "Writer for register U1OTGIRCLR"]
pub type W = crate::W<u32, super::U1OTGIRCLR>;
#[doc = "Register U1OTGIRCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::U1OTGIRCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VBUSVDIF`"]
pub type VBUSVDIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBUSVDIF`"]
pub struct VBUSVDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSVDIF_W<'a> {
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
#[doc = "Reader of field `SESENDIF`"]
pub type SESENDIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SESENDIF`"]
pub struct SESENDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SESENDIF_W<'a> {
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
#[doc = "Reader of field `SESVDIF`"]
pub type SESVDIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SESVDIF`"]
pub struct SESVDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SESVDIF_W<'a> {
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
#[doc = "Reader of field `ACTVIF`"]
pub type ACTVIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACTVIF`"]
pub struct ACTVIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTVIF_W<'a> {
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
#[doc = "Reader of field `LSTATEIF`"]
pub type LSTATEIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSTATEIF`"]
pub struct LSTATEIF_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTATEIF_W<'a> {
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
#[doc = "Reader of field `T1MSECIF`"]
pub type T1MSECIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T1MSECIF`"]
pub struct T1MSECIF_W<'a> {
    w: &'a mut W,
}
impl<'a> T1MSECIF_W<'a> {
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
#[doc = "Reader of field `IDIF`"]
pub type IDIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDIF`"]
pub struct IDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> IDIF_W<'a> {
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
    pub fn vbusvdif(&self) -> VBUSVDIF_R {
        VBUSVDIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sesendif(&self) -> SESENDIF_R {
        SESENDIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sesvdif(&self) -> SESVDIF_R {
        SESVDIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn actvif(&self) -> ACTVIF_R {
        ACTVIF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lstateif(&self) -> LSTATEIF_R {
        LSTATEIF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn t1msecif(&self) -> T1MSECIF_R {
        T1MSECIF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn idif(&self) -> IDIF_R {
        IDIF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vbusvdif(&mut self) -> VBUSVDIF_W {
        VBUSVDIF_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sesendif(&mut self) -> SESENDIF_W {
        SESENDIF_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sesvdif(&mut self) -> SESVDIF_W {
        SESVDIF_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn actvif(&mut self) -> ACTVIF_W {
        ACTVIF_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lstateif(&mut self) -> LSTATEIF_W {
        LSTATEIF_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn t1msecif(&mut self) -> T1MSECIF_W {
        T1MSECIF_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn idif(&mut self) -> IDIF_W {
        IDIF_W { w: self }
    }
}
