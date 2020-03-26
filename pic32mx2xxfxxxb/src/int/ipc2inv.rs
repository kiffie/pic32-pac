#[doc = "Reader of register IPC2INV"]
pub type R = crate::R<u32, super::IPC2INV>;
#[doc = "Writer for register IPC2INV"]
pub type W = crate::W<u32, super::IPC2INV>;
#[doc = "Register IPC2INV `reset()`'s with value 0"]
impl crate::ResetValue for super::IPC2INV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `T2IS`"]
pub type T2IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T2IS`"]
pub struct T2IS_W<'a> {
    w: &'a mut W,
}
impl<'a> T2IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `T2IP`"]
pub type T2IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T2IP`"]
pub struct T2IP_W<'a> {
    w: &'a mut W,
}
impl<'a> T2IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `IC2IS`"]
pub type IC2IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IC2IS`"]
pub struct IC2IS_W<'a> {
    w: &'a mut W,
}
impl<'a> IC2IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `IC2IP`"]
pub type IC2IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IC2IP`"]
pub struct IC2IP_W<'a> {
    w: &'a mut W,
}
impl<'a> IC2IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "Reader of field `OC2IS`"]
pub type OC2IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OC2IS`"]
pub struct OC2IS_W<'a> {
    w: &'a mut W,
}
impl<'a> OC2IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `OC2IP`"]
pub type OC2IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OC2IP`"]
pub struct OC2IP_W<'a> {
    w: &'a mut W,
}
impl<'a> OC2IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `INT2IS`"]
pub type INT2IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT2IS`"]
pub struct INT2IS_W<'a> {
    w: &'a mut W,
}
impl<'a> INT2IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `INT2IP`"]
pub type INT2IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT2IP`"]
pub struct INT2IP_W<'a> {
    w: &'a mut W,
}
impl<'a> INT2IP_W<'a> {
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
    pub fn t2is(&self) -> T2IS_R {
        T2IS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn t2ip(&self) -> T2IP_R {
        T2IP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ic2is(&self) -> IC2IS_R {
        IC2IS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn ic2ip(&self) -> IC2IP_R {
        IC2IP_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn oc2is(&self) -> OC2IS_R {
        OC2IS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn oc2ip(&self) -> OC2IP_R {
        OC2IP_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn int2is(&self) -> INT2IS_R {
        INT2IS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn int2ip(&self) -> INT2IP_R {
        INT2IP_R::new(((self.bits >> 26) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn t2is(&mut self) -> T2IS_W {
        T2IS_W { w: self }
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn t2ip(&mut self) -> T2IP_W {
        T2IP_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ic2is(&mut self) -> IC2IS_W {
        IC2IS_W { w: self }
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn ic2ip(&mut self) -> IC2IP_W {
        IC2IP_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn oc2is(&mut self) -> OC2IS_W {
        OC2IS_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn oc2ip(&mut self) -> OC2IP_W {
        OC2IP_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn int2is(&mut self) -> INT2IS_W {
        INT2IS_W { w: self }
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn int2ip(&mut self) -> INT2IP_W {
        INT2IP_W { w: self }
    }
}
