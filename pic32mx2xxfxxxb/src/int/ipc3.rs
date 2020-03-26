#[doc = "Reader of register IPC3"]
pub type R = crate::R<u32, super::IPC3>;
#[doc = "Writer for register IPC3"]
pub type W = crate::W<u32, super::IPC3>;
#[doc = "Register IPC3 `reset()`'s with value 0"]
impl crate::ResetValue for super::IPC3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `T3IS`"]
pub type T3IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T3IS`"]
pub struct T3IS_W<'a> {
    w: &'a mut W,
}
impl<'a> T3IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `T3IP`"]
pub type T3IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T3IP`"]
pub struct T3IP_W<'a> {
    w: &'a mut W,
}
impl<'a> T3IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `IC3IS`"]
pub type IC3IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IC3IS`"]
pub struct IC3IS_W<'a> {
    w: &'a mut W,
}
impl<'a> IC3IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `IC3IP`"]
pub type IC3IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IC3IP`"]
pub struct IC3IP_W<'a> {
    w: &'a mut W,
}
impl<'a> IC3IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "Reader of field `OC3IS`"]
pub type OC3IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OC3IS`"]
pub struct OC3IS_W<'a> {
    w: &'a mut W,
}
impl<'a> OC3IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `OC3IP`"]
pub type OC3IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OC3IP`"]
pub struct OC3IP_W<'a> {
    w: &'a mut W,
}
impl<'a> OC3IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `INT3IS`"]
pub type INT3IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT3IS`"]
pub struct INT3IS_W<'a> {
    w: &'a mut W,
}
impl<'a> INT3IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `INT3IP`"]
pub type INT3IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT3IP`"]
pub struct INT3IP_W<'a> {
    w: &'a mut W,
}
impl<'a> INT3IP_W<'a> {
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
    pub fn t3is(&self) -> T3IS_R {
        T3IS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn t3ip(&self) -> T3IP_R {
        T3IP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ic3is(&self) -> IC3IS_R {
        IC3IS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn ic3ip(&self) -> IC3IP_R {
        IC3IP_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn oc3is(&self) -> OC3IS_R {
        OC3IS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn oc3ip(&self) -> OC3IP_R {
        OC3IP_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn int3is(&self) -> INT3IS_R {
        INT3IS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn int3ip(&self) -> INT3IP_R {
        INT3IP_R::new(((self.bits >> 26) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn t3is(&mut self) -> T3IS_W {
        T3IS_W { w: self }
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn t3ip(&mut self) -> T3IP_W {
        T3IP_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ic3is(&mut self) -> IC3IS_W {
        IC3IS_W { w: self }
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn ic3ip(&mut self) -> IC3IP_W {
        IC3IP_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn oc3is(&mut self) -> OC3IS_W {
        OC3IS_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn oc3ip(&mut self) -> OC3IP_W {
        OC3IP_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn int3is(&mut self) -> INT3IS_W {
        INT3IS_W { w: self }
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn int3ip(&mut self) -> INT3IP_W {
        INT3IP_W { w: self }
    }
}
