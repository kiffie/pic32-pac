#[doc = "Reader of register IPC4INV"]
pub type R = crate::R<u32, super::IPC4INV>;
#[doc = "Writer for register IPC4INV"]
pub type W = crate::W<u32, super::IPC4INV>;
#[doc = "Register IPC4INV `reset()`'s with value 0"]
impl crate::ResetValue for super::IPC4INV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `T4IS`"]
pub type T4IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T4IS`"]
pub struct T4IS_W<'a> {
    w: &'a mut W,
}
impl<'a> T4IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `T4IP`"]
pub type T4IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T4IP`"]
pub struct T4IP_W<'a> {
    w: &'a mut W,
}
impl<'a> T4IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `IC4IS`"]
pub type IC4IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IC4IS`"]
pub struct IC4IS_W<'a> {
    w: &'a mut W,
}
impl<'a> IC4IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `IC4IP`"]
pub type IC4IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IC4IP`"]
pub struct IC4IP_W<'a> {
    w: &'a mut W,
}
impl<'a> IC4IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "Reader of field `OC4IS`"]
pub type OC4IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OC4IS`"]
pub struct OC4IS_W<'a> {
    w: &'a mut W,
}
impl<'a> OC4IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `OC4IP`"]
pub type OC4IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OC4IP`"]
pub struct OC4IP_W<'a> {
    w: &'a mut W,
}
impl<'a> OC4IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `INT4IS`"]
pub type INT4IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT4IS`"]
pub struct INT4IS_W<'a> {
    w: &'a mut W,
}
impl<'a> INT4IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `INT4IP`"]
pub type INT4IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT4IP`"]
pub struct INT4IP_W<'a> {
    w: &'a mut W,
}
impl<'a> INT4IP_W<'a> {
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
    pub fn t4is(&self) -> T4IS_R {
        T4IS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn t4ip(&self) -> T4IP_R {
        T4IP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ic4is(&self) -> IC4IS_R {
        IC4IS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn ic4ip(&self) -> IC4IP_R {
        IC4IP_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn oc4is(&self) -> OC4IS_R {
        OC4IS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn oc4ip(&self) -> OC4IP_R {
        OC4IP_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn int4is(&self) -> INT4IS_R {
        INT4IS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn int4ip(&self) -> INT4IP_R {
        INT4IP_R::new(((self.bits >> 26) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn t4is(&mut self) -> T4IS_W {
        T4IS_W { w: self }
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn t4ip(&mut self) -> T4IP_W {
        T4IP_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ic4is(&mut self) -> IC4IS_W {
        IC4IS_W { w: self }
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn ic4ip(&mut self) -> IC4IP_W {
        IC4IP_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn oc4is(&mut self) -> OC4IS_W {
        OC4IS_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn oc4ip(&mut self) -> OC4IP_W {
        OC4IP_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn int4is(&mut self) -> INT4IS_W {
        INT4IS_W { w: self }
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn int4ip(&mut self) -> INT4IP_W {
        INT4IP_W { w: self }
    }
}
