#[doc = "Reader of register IPC1CLR"]
pub type R = crate::R<u32, super::IPC1CLR>;
#[doc = "Writer for register IPC1CLR"]
pub type W = crate::W<u32, super::IPC1CLR>;
#[doc = "Register IPC1CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::IPC1CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `T1IS`"]
pub type T1IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T1IS`"]
pub struct T1IS_W<'a> {
    w: &'a mut W,
}
impl<'a> T1IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `T1IP`"]
pub type T1IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T1IP`"]
pub struct T1IP_W<'a> {
    w: &'a mut W,
}
impl<'a> T1IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `IC1IS`"]
pub type IC1IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IC1IS`"]
pub struct IC1IS_W<'a> {
    w: &'a mut W,
}
impl<'a> IC1IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `IC1IP`"]
pub type IC1IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IC1IP`"]
pub struct IC1IP_W<'a> {
    w: &'a mut W,
}
impl<'a> IC1IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "Reader of field `OC1IS`"]
pub type OC1IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OC1IS`"]
pub struct OC1IS_W<'a> {
    w: &'a mut W,
}
impl<'a> OC1IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `OC1IP`"]
pub type OC1IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OC1IP`"]
pub struct OC1IP_W<'a> {
    w: &'a mut W,
}
impl<'a> OC1IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `INT1IS`"]
pub type INT1IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT1IS`"]
pub struct INT1IS_W<'a> {
    w: &'a mut W,
}
impl<'a> INT1IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `INT1IP`"]
pub type INT1IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT1IP`"]
pub struct INT1IP_W<'a> {
    w: &'a mut W,
}
impl<'a> INT1IP_W<'a> {
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
    pub fn t1is(&self) -> T1IS_R {
        T1IS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn t1ip(&self) -> T1IP_R {
        T1IP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ic1is(&self) -> IC1IS_R {
        IC1IS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn ic1ip(&self) -> IC1IP_R {
        IC1IP_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn oc1is(&self) -> OC1IS_R {
        OC1IS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn oc1ip(&self) -> OC1IP_R {
        OC1IP_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn int1is(&self) -> INT1IS_R {
        INT1IS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn int1ip(&self) -> INT1IP_R {
        INT1IP_R::new(((self.bits >> 26) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn t1is(&mut self) -> T1IS_W {
        T1IS_W { w: self }
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn t1ip(&mut self) -> T1IP_W {
        T1IP_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ic1is(&mut self) -> IC1IS_W {
        IC1IS_W { w: self }
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn ic1ip(&mut self) -> IC1IP_W {
        IC1IP_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn oc1is(&mut self) -> OC1IS_W {
        OC1IS_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn oc1ip(&mut self) -> OC1IP_W {
        OC1IP_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn int1is(&mut self) -> INT1IS_W {
        INT1IS_W { w: self }
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn int1ip(&mut self) -> INT1IP_W {
        INT1IP_W { w: self }
    }
}
