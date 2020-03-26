#[doc = "Reader of register IPC0CLR"]
pub type R = crate::R<u32, super::IPC0CLR>;
#[doc = "Writer for register IPC0CLR"]
pub type W = crate::W<u32, super::IPC0CLR>;
#[doc = "Register IPC0CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::IPC0CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTIS`"]
pub type CTIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTIS`"]
pub struct CTIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `CTIP`"]
pub type CTIP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTIP`"]
pub struct CTIP_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `CS0IS`"]
pub type CS0IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CS0IS`"]
pub struct CS0IS_W<'a> {
    w: &'a mut W,
}
impl<'a> CS0IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `CS0IP`"]
pub type CS0IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CS0IP`"]
pub struct CS0IP_W<'a> {
    w: &'a mut W,
}
impl<'a> CS0IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "Reader of field `CS1IS`"]
pub type CS1IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CS1IS`"]
pub struct CS1IS_W<'a> {
    w: &'a mut W,
}
impl<'a> CS1IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `CS1IP`"]
pub type CS1IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CS1IP`"]
pub struct CS1IP_W<'a> {
    w: &'a mut W,
}
impl<'a> CS1IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `INT0IS`"]
pub type INT0IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT0IS`"]
pub struct INT0IS_W<'a> {
    w: &'a mut W,
}
impl<'a> INT0IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `INT0IP`"]
pub type INT0IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT0IP`"]
pub struct INT0IP_W<'a> {
    w: &'a mut W,
}
impl<'a> INT0IP_W<'a> {
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
    pub fn ctis(&self) -> CTIS_R {
        CTIS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn ctip(&self) -> CTIP_R {
        CTIP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn cs0is(&self) -> CS0IS_R {
        CS0IS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn cs0ip(&self) -> CS0IP_R {
        CS0IP_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn cs1is(&self) -> CS1IS_R {
        CS1IS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn cs1ip(&self) -> CS1IP_R {
        CS1IP_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn int0is(&self) -> INT0IS_R {
        INT0IS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn int0ip(&self) -> INT0IP_R {
        INT0IP_R::new(((self.bits >> 26) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn ctis(&mut self) -> CTIS_W {
        CTIS_W { w: self }
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn ctip(&mut self) -> CTIP_W {
        CTIP_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn cs0is(&mut self) -> CS0IS_W {
        CS0IS_W { w: self }
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn cs0ip(&mut self) -> CS0IP_W {
        CS0IP_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn cs1is(&mut self) -> CS1IS_W {
        CS1IS_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn cs1ip(&mut self) -> CS1IP_W {
        CS1IP_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn int0is(&mut self) -> INT0IS_W {
        INT0IS_W { w: self }
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn int0ip(&mut self) -> INT0IP_W {
        INT0IP_W { w: self }
    }
}
