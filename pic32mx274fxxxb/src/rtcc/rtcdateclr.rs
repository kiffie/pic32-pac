#[doc = "Reader of register RTCDATECLR"]
pub type R = crate::R<u32, super::RTCDATECLR>;
#[doc = "Writer for register RTCDATECLR"]
pub type W = crate::W<u32, super::RTCDATECLR>;
#[doc = "Register RTCDATECLR `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCDATECLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDAY01`"]
pub type WDAY01_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WDAY01`"]
pub struct WDAY01_W<'a> {
    w: &'a mut W,
}
impl<'a> WDAY01_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `DAY01`"]
pub type DAY01_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAY01`"]
pub struct DAY01_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY01_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `DAY10`"]
pub type DAY10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAY10`"]
pub struct DAY10_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `MONTH01`"]
pub type MONTH01_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MONTH01`"]
pub struct MONTH01_W<'a> {
    w: &'a mut W,
}
impl<'a> MONTH01_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `MONTH10`"]
pub type MONTH10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MONTH10`"]
pub struct MONTH10_W<'a> {
    w: &'a mut W,
}
impl<'a> MONTH10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `YEAR01`"]
pub type YEAR01_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `YEAR01`"]
pub struct YEAR01_W<'a> {
    w: &'a mut W,
}
impl<'a> YEAR01_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `YEAR10`"]
pub type YEAR10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `YEAR10`"]
pub struct YEAR10_W<'a> {
    w: &'a mut W,
}
impl<'a> YEAR10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn wday01(&self) -> WDAY01_R {
        WDAY01_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn day01(&self) -> DAY01_R {
        DAY01_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn day10(&self) -> DAY10_R {
        DAY10_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn month01(&self) -> MONTH01_R {
        MONTH01_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn month10(&self) -> MONTH10_R {
        MONTH10_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn year01(&self) -> YEAR01_R {
        YEAR01_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn year10(&self) -> YEAR10_R {
        YEAR10_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn wday01(&mut self) -> WDAY01_W {
        WDAY01_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn day01(&mut self) -> DAY01_W {
        DAY01_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn day10(&mut self) -> DAY10_W {
        DAY10_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn month01(&mut self) -> MONTH01_W {
        MONTH01_W { w: self }
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn month10(&mut self) -> MONTH10_W {
        MONTH10_W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn year01(&mut self) -> YEAR01_W {
        YEAR01_W { w: self }
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn year10(&mut self) -> YEAR10_W {
        YEAR10_W { w: self }
    }
}
