#[doc = "Reader of register INTCONSET"]
pub type R = crate::R<u32, super::INTCONSET>;
#[doc = "Writer for register INTCONSET"]
pub type W = crate::W<u32, super::INTCONSET>;
#[doc = "Register INTCONSET `reset()`'s with value 0"]
impl crate::ResetValue for super::INTCONSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INT0EP`"]
pub type INT0EP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INT0EP`"]
pub struct INT0EP_W<'a> {
    w: &'a mut W,
}
impl<'a> INT0EP_W<'a> {
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
#[doc = "Reader of field `INT1EP`"]
pub type INT1EP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INT1EP`"]
pub struct INT1EP_W<'a> {
    w: &'a mut W,
}
impl<'a> INT1EP_W<'a> {
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
#[doc = "Reader of field `INT2EP`"]
pub type INT2EP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INT2EP`"]
pub struct INT2EP_W<'a> {
    w: &'a mut W,
}
impl<'a> INT2EP_W<'a> {
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
#[doc = "Reader of field `INT3EP`"]
pub type INT3EP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INT3EP`"]
pub struct INT3EP_W<'a> {
    w: &'a mut W,
}
impl<'a> INT3EP_W<'a> {
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
#[doc = "Reader of field `INT4EP`"]
pub type INT4EP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INT4EP`"]
pub struct INT4EP_W<'a> {
    w: &'a mut W,
}
impl<'a> INT4EP_W<'a> {
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
#[doc = "Reader of field `TPC`"]
pub type TPC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TPC`"]
pub struct TPC_W<'a> {
    w: &'a mut W,
}
impl<'a> TPC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `MVEC`"]
pub type MVEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MVEC`"]
pub struct MVEC_W<'a> {
    w: &'a mut W,
}
impl<'a> MVEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `FRZ`"]
pub type FRZ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRZ`"]
pub struct FRZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FRZ_W<'a> {
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
#[doc = "Reader of field `SS0`"]
pub type SS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SS0`"]
pub struct SS0_W<'a> {
    w: &'a mut W,
}
impl<'a> SS0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn int0ep(&self) -> INT0EP_R {
        INT0EP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn int1ep(&self) -> INT1EP_R {
        INT1EP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn int2ep(&self) -> INT2EP_R {
        INT2EP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn int3ep(&self) -> INT3EP_R {
        INT3EP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn int4ep(&self) -> INT4EP_R {
        INT4EP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn tpc(&self) -> TPC_R {
        TPC_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn mvec(&self) -> MVEC_R {
        MVEC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn frz(&self) -> FRZ_R {
        FRZ_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ss0(&self) -> SS0_R {
        SS0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn int0ep(&mut self) -> INT0EP_W {
        INT0EP_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn int1ep(&mut self) -> INT1EP_W {
        INT1EP_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn int2ep(&mut self) -> INT2EP_W {
        INT2EP_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn int3ep(&mut self) -> INT3EP_W {
        INT3EP_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn int4ep(&mut self) -> INT4EP_W {
        INT4EP_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn tpc(&mut self) -> TPC_W {
        TPC_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn mvec(&mut self) -> MVEC_W {
        MVEC_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn frz(&mut self) -> FRZ_W {
        FRZ_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ss0(&mut self) -> SS0_W {
        SS0_W { w: self }
    }
}
