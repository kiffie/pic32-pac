#[doc = "Reader of register IPC8SET"]
pub type R = crate::R<u32, super::IPC8SET>;
#[doc = "Writer for register IPC8SET"]
pub type W = crate::W<u32, super::IPC8SET>;
#[doc = "Register IPC8SET `reset()`'s with value 0"]
impl crate::ResetValue for super::IPC8SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `U1IS`"]
pub type U1IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `U1IS`"]
pub struct U1IS_W<'a> {
    w: &'a mut W,
}
impl<'a> U1IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `U1IP`"]
pub type U1IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `U1IP`"]
pub struct U1IP_W<'a> {
    w: &'a mut W,
}
impl<'a> U1IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `I2C1IS`"]
pub type I2C1IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C1IS`"]
pub struct I2C1IS_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `I2C1IP`"]
pub type I2C1IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C1IP`"]
pub struct I2C1IP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "Reader of field `CNIS`"]
pub type CNIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CNIS`"]
pub struct CNIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `CNIP`"]
pub type CNIP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CNIP`"]
pub struct CNIP_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `PMPIS`"]
pub type PMPIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PMPIS`"]
pub struct PMPIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PMPIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `PMPIP`"]
pub type PMPIP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PMPIP`"]
pub struct PMPIP_W<'a> {
    w: &'a mut W,
}
impl<'a> PMPIP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 26)) | (((value as u32) & 0x07) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn u1is(&self) -> U1IS_R {
        U1IS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn u1ip(&self) -> U1IP_R {
        U1IP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn i2c1is(&self) -> I2C1IS_R {
        I2C1IS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn i2c1ip(&self) -> I2C1IP_R {
        I2C1IP_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn cnis(&self) -> CNIS_R {
        CNIS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn cnip(&self) -> CNIP_R {
        CNIP_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn pmpis(&self) -> PMPIS_R {
        PMPIS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn pmpip(&self) -> PMPIP_R {
        PMPIP_R::new(((self.bits >> 26) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn u1is(&mut self) -> U1IS_W {
        U1IS_W { w: self }
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn u1ip(&mut self) -> U1IP_W {
        U1IP_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn i2c1is(&mut self) -> I2C1IS_W {
        I2C1IS_W { w: self }
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn i2c1ip(&mut self) -> I2C1IP_W {
        I2C1IP_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn cnis(&mut self) -> CNIS_W {
        CNIS_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn cnip(&mut self) -> CNIP_W {
        CNIP_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn pmpis(&mut self) -> PMPIS_W {
        PMPIS_W { w: self }
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn pmpip(&mut self) -> PMPIP_W {
        PMPIP_W { w: self }
    }
}
