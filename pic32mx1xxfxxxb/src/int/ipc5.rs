#[doc = "Reader of register IPC5"]
pub type R = crate::R<u32, super::IPC5>;
#[doc = "Writer for register IPC5"]
pub type W = crate::W<u32, super::IPC5>;
#[doc = "Register IPC5 `reset()`'s with value 0"]
impl crate::ResetValue for super::IPC5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `T5IS`"]
pub type T5IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T5IS`"]
pub struct T5IS_W<'a> {
    w: &'a mut W,
}
impl<'a> T5IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `T5IP`"]
pub type T5IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T5IP`"]
pub struct T5IP_W<'a> {
    w: &'a mut W,
}
impl<'a> T5IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `IC5IS`"]
pub type IC5IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IC5IS`"]
pub struct IC5IS_W<'a> {
    w: &'a mut W,
}
impl<'a> IC5IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `IC5IP`"]
pub type IC5IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IC5IP`"]
pub struct IC5IP_W<'a> {
    w: &'a mut W,
}
impl<'a> IC5IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "Reader of field `OC5IS`"]
pub type OC5IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OC5IS`"]
pub struct OC5IS_W<'a> {
    w: &'a mut W,
}
impl<'a> OC5IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `OC5IP`"]
pub type OC5IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OC5IP`"]
pub struct OC5IP_W<'a> {
    w: &'a mut W,
}
impl<'a> OC5IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `AD1IS`"]
pub type AD1IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AD1IS`"]
pub struct AD1IS_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `AD1IP`"]
pub type AD1IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AD1IP`"]
pub struct AD1IP_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1IP_W<'a> {
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
    pub fn t5is(&self) -> T5IS_R {
        T5IS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn t5ip(&self) -> T5IP_R {
        T5IP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ic5is(&self) -> IC5IS_R {
        IC5IS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn ic5ip(&self) -> IC5IP_R {
        IC5IP_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn oc5is(&self) -> OC5IS_R {
        OC5IS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn oc5ip(&self) -> OC5IP_R {
        OC5IP_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ad1is(&self) -> AD1IS_R {
        AD1IS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn ad1ip(&self) -> AD1IP_R {
        AD1IP_R::new(((self.bits >> 26) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn t5is(&mut self) -> T5IS_W {
        T5IS_W { w: self }
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn t5ip(&mut self) -> T5IP_W {
        T5IP_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ic5is(&mut self) -> IC5IS_W {
        IC5IS_W { w: self }
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn ic5ip(&mut self) -> IC5IP_W {
        IC5IP_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn oc5is(&mut self) -> OC5IS_W {
        OC5IS_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn oc5ip(&mut self) -> OC5IP_W {
        OC5IP_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ad1is(&mut self) -> AD1IS_W {
        AD1IS_W { w: self }
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn ad1ip(&mut self) -> AD1IP_W {
        AD1IP_W { w: self }
    }
}
