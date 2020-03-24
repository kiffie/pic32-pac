#[doc = "Reader of register T1CONCLR"]
pub type R = crate::R<u32, super::T1CONCLR>;
#[doc = "Writer for register T1CONCLR"]
pub type W = crate::W<u32, super::T1CONCLR>;
#[doc = "Register T1CONCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::T1CONCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TCS`"]
pub type TCS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCS`"]
pub struct TCS_W<'a> {
    w: &'a mut W,
}
impl<'a> TCS_W<'a> {
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
#[doc = "Reader of field `TSYNC`"]
pub type TSYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSYNC`"]
pub struct TSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> TSYNC_W<'a> {
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
#[doc = "Reader of field `TCKPS`"]
pub type TCKPS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TCKPS`"]
pub struct TCKPS_W<'a> {
    w: &'a mut W,
}
impl<'a> TCKPS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `TGATE`"]
pub type TGATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TGATE`"]
pub struct TGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> TGATE_W<'a> {
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
#[doc = "Reader of field `TECS`"]
pub type TECS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TECS`"]
pub struct TECS_W<'a> {
    w: &'a mut W,
}
impl<'a> TECS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `TWIP`"]
pub type TWIP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TWIP`"]
pub struct TWIP_W<'a> {
    w: &'a mut W,
}
impl<'a> TWIP_W<'a> {
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
#[doc = "Reader of field `TWDIS`"]
pub type TWDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TWDIS`"]
pub struct TWDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TWDIS_W<'a> {
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
#[doc = "Reader of field `SIDL`"]
pub type SIDL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIDL`"]
pub struct SIDL_W<'a> {
    w: &'a mut W,
}
impl<'a> SIDL_W<'a> {
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
#[doc = "Reader of field `ON`"]
pub type ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ON`"]
pub struct ON_W<'a> {
    w: &'a mut W,
}
impl<'a> ON_W<'a> {
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
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tcs(&self) -> TCS_R {
        TCS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tsync(&self) -> TSYNC_R {
        TSYNC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn tckps(&self) -> TCKPS_R {
        TCKPS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tgate(&self) -> TGATE_R {
        TGATE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn tecs(&self) -> TECS_R {
        TECS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn twip(&self) -> TWIP_R {
        TWIP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn twdis(&self) -> TWDIS_R {
        TWDIS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn sidl(&self) -> SIDL_R {
        SIDL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn frz(&self) -> FRZ_R {
        FRZ_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&self) -> ON_R {
        ON_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tcs(&mut self) -> TCS_W {
        TCS_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tsync(&mut self) -> TSYNC_W {
        TSYNC_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn tckps(&mut self) -> TCKPS_W {
        TCKPS_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tgate(&mut self) -> TGATE_W {
        TGATE_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn tecs(&mut self) -> TECS_W {
        TECS_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn twip(&mut self) -> TWIP_W {
        TWIP_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn twdis(&mut self) -> TWDIS_W {
        TWDIS_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn sidl(&mut self) -> SIDL_W {
        SIDL_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn frz(&mut self) -> FRZ_W {
        FRZ_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&mut self) -> ON_W {
        ON_W { w: self }
    }
}
