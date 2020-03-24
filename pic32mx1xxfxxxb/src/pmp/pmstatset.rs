#[doc = "Reader of register PMSTATSET"]
pub type R = crate::R<u32, super::PMSTATSET>;
#[doc = "Writer for register PMSTATSET"]
pub type W = crate::W<u32, super::PMSTATSET>;
#[doc = "Register PMSTATSET `reset()`'s with value 0"]
impl crate::ResetValue for super::PMSTATSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OB0E`"]
pub type OB0E_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OB0E`"]
pub struct OB0E_W<'a> {
    w: &'a mut W,
}
impl<'a> OB0E_W<'a> {
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
#[doc = "Reader of field `OB1E`"]
pub type OB1E_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OB1E`"]
pub struct OB1E_W<'a> {
    w: &'a mut W,
}
impl<'a> OB1E_W<'a> {
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
#[doc = "Reader of field `OB2E`"]
pub type OB2E_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OB2E`"]
pub struct OB2E_W<'a> {
    w: &'a mut W,
}
impl<'a> OB2E_W<'a> {
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
#[doc = "Reader of field `OB3E`"]
pub type OB3E_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OB3E`"]
pub struct OB3E_W<'a> {
    w: &'a mut W,
}
impl<'a> OB3E_W<'a> {
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
#[doc = "Reader of field `OBUF`"]
pub type OBUF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OBUF`"]
pub struct OBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> OBUF_W<'a> {
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
#[doc = "Reader of field `OBE`"]
pub type OBE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OBE`"]
pub struct OBE_W<'a> {
    w: &'a mut W,
}
impl<'a> OBE_W<'a> {
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
#[doc = "Reader of field `IB0F`"]
pub type IB0F_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IB0F`"]
pub struct IB0F_W<'a> {
    w: &'a mut W,
}
impl<'a> IB0F_W<'a> {
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
#[doc = "Reader of field `IB1F`"]
pub type IB1F_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IB1F`"]
pub struct IB1F_W<'a> {
    w: &'a mut W,
}
impl<'a> IB1F_W<'a> {
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
#[doc = "Reader of field `IB2F`"]
pub type IB2F_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IB2F`"]
pub struct IB2F_W<'a> {
    w: &'a mut W,
}
impl<'a> IB2F_W<'a> {
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
#[doc = "Reader of field `IB3F`"]
pub type IB3F_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IB3F`"]
pub struct IB3F_W<'a> {
    w: &'a mut W,
}
impl<'a> IB3F_W<'a> {
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
#[doc = "Reader of field `IBOV`"]
pub type IBOV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IBOV`"]
pub struct IBOV_W<'a> {
    w: &'a mut W,
}
impl<'a> IBOV_W<'a> {
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
#[doc = "Reader of field `IBF`"]
pub type IBF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IBF`"]
pub struct IBF_W<'a> {
    w: &'a mut W,
}
impl<'a> IBF_W<'a> {
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
    pub fn ob0e(&self) -> OB0E_R {
        OB0E_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ob1e(&self) -> OB1E_R {
        OB1E_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ob2e(&self) -> OB2E_R {
        OB2E_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ob3e(&self) -> OB3E_R {
        OB3E_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn obuf(&self) -> OBUF_R {
        OBUF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn obe(&self) -> OBE_R {
        OBE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ib0f(&self) -> IB0F_R {
        IB0F_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ib1f(&self) -> IB1F_R {
        IB1F_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ib2f(&self) -> IB2F_R {
        IB2F_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ib3f(&self) -> IB3F_R {
        IB3F_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ibov(&self) -> IBOV_R {
        IBOV_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ibf(&self) -> IBF_R {
        IBF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ob0e(&mut self) -> OB0E_W {
        OB0E_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ob1e(&mut self) -> OB1E_W {
        OB1E_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ob2e(&mut self) -> OB2E_W {
        OB2E_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ob3e(&mut self) -> OB3E_W {
        OB3E_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn obuf(&mut self) -> OBUF_W {
        OBUF_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn obe(&mut self) -> OBE_W {
        OBE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ib0f(&mut self) -> IB0F_W {
        IB0F_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ib1f(&mut self) -> IB1F_W {
        IB1F_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ib2f(&mut self) -> IB2F_W {
        IB2F_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ib3f(&mut self) -> IB3F_W {
        IB3F_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ibov(&mut self) -> IBOV_W {
        IBOV_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ibf(&mut self) -> IBF_W {
        IBF_W { w: self }
    }
}
