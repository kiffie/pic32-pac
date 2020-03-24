#[doc = "Reader of register NVMCONINV"]
pub type R = crate::R<u32, super::NVMCONINV>;
#[doc = "Writer for register NVMCONINV"]
pub type W = crate::W<u32, super::NVMCONINV>;
#[doc = "Register NVMCONINV `reset()`'s with value 0"]
impl crate::ResetValue for super::NVMCONINV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NVMOP`"]
pub type NVMOP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NVMOP`"]
pub struct NVMOP_W<'a> {
    w: &'a mut W,
}
impl<'a> NVMOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `LVDSTAT`"]
pub type LVDSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LVDSTAT`"]
pub struct LVDSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDSTAT_W<'a> {
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
#[doc = "Reader of field `LVDERR`"]
pub type LVDERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LVDERR`"]
pub struct LVDERR_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDERR_W<'a> {
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
#[doc = "Reader of field `WRERR`"]
pub type WRERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRERR`"]
pub struct WRERR_W<'a> {
    w: &'a mut W,
}
impl<'a> WRERR_W<'a> {
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
#[doc = "Reader of field `WREN`"]
pub type WREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WREN`"]
pub struct WREN_W<'a> {
    w: &'a mut W,
}
impl<'a> WREN_W<'a> {
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
#[doc = "Reader of field `WR`"]
pub type WR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WR`"]
pub struct WR_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_W<'a> {
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
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn nvmop(&self) -> NVMOP_R {
        NVMOP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn lvdstat(&self) -> LVDSTAT_R {
        LVDSTAT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lvderr(&self) -> LVDERR_R {
        LVDERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn wrerr(&self) -> WRERR_R {
        WRERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn wr(&self) -> WR_R {
        WR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn nvmop(&mut self) -> NVMOP_W {
        NVMOP_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn lvdstat(&mut self) -> LVDSTAT_W {
        LVDSTAT_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lvderr(&mut self) -> LVDERR_W {
        LVDERR_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn wrerr(&mut self) -> WRERR_W {
        WRERR_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn wren(&mut self) -> WREN_W {
        WREN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn wr(&mut self) -> WR_W {
        WR_W { w: self }
    }
}
