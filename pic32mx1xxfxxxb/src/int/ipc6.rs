#[doc = "Reader of register IPC6"]
pub type R = crate::R<u32, super::IPC6>;
#[doc = "Writer for register IPC6"]
pub type W = crate::W<u32, super::IPC6>;
#[doc = "Register IPC6 `reset()`'s with value 0"]
impl crate::ResetValue for super::IPC6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FSCMIS`"]
pub type FSCMIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FSCMIS`"]
pub struct FSCMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FSCMIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `FSCMIP`"]
pub type FSCMIP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FSCMIP`"]
pub struct FSCMIP_W<'a> {
    w: &'a mut W,
}
impl<'a> FSCMIP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `RTCCIS`"]
pub type RTCCIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTCCIS`"]
pub struct RTCCIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `RTCCIP`"]
pub type RTCCIP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTCCIP`"]
pub struct RTCCIP_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCIP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "Reader of field `FCEIS`"]
pub type FCEIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FCEIS`"]
pub struct FCEIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FCEIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `FCEIP`"]
pub type FCEIP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FCEIP`"]
pub struct FCEIP_W<'a> {
    w: &'a mut W,
}
impl<'a> FCEIP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `CMP1IS`"]
pub type CMP1IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMP1IS`"]
pub struct CMP1IS_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `CMP1IP`"]
pub type CMP1IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMP1IP`"]
pub struct CMP1IP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1IP_W<'a> {
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
    pub fn fscmis(&self) -> FSCMIS_R {
        FSCMIS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn fscmip(&self) -> FSCMIP_R {
        FSCMIP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn rtccis(&self) -> RTCCIS_R {
        RTCCIS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn rtccip(&self) -> RTCCIP_R {
        RTCCIP_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn fceis(&self) -> FCEIS_R {
        FCEIS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn fceip(&self) -> FCEIP_R {
        FCEIP_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn cmp1is(&self) -> CMP1IS_R {
        CMP1IS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn cmp1ip(&self) -> CMP1IP_R {
        CMP1IP_R::new(((self.bits >> 26) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn fscmis(&mut self) -> FSCMIS_W {
        FSCMIS_W { w: self }
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn fscmip(&mut self) -> FSCMIP_W {
        FSCMIP_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn rtccis(&mut self) -> RTCCIS_W {
        RTCCIS_W { w: self }
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn rtccip(&mut self) -> RTCCIP_W {
        RTCCIP_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn fceis(&mut self) -> FCEIS_W {
        FCEIS_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn fceip(&mut self) -> FCEIP_W {
        FCEIP_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn cmp1is(&mut self) -> CMP1IS_W {
        CMP1IS_W { w: self }
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn cmp1ip(&mut self) -> CMP1IP_W {
        CMP1IP_W { w: self }
    }
}
