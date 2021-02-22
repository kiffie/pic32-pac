#[doc = "Register `OSCTUN` reader"]
pub struct R(crate::R<OSCTUN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCTUN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<OSCTUN_SPEC>> for R {
    fn from(reader: crate::R<OSCTUN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSCTUN` writer"]
pub struct W(crate::W<OSCTUN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSCTUN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<OSCTUN_SPEC>> for W {
    fn from(writer: crate::W<OSCTUN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TUN` reader - "]
pub struct TUN_R(crate::FieldReader<u8, u8>);
impl TUN_R {
    pub(crate) fn new(bits: u8) -> Self {
        TUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TUN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TUN` writer - "]
pub struct TUN_W<'a> {
    w: &'a mut W,
}
impl<'a> TUN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn tun(&self) -> TUN_R {
        TUN_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn tun(&mut self) -> TUN_W {
        TUN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OSCTUN register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osctun](index.html) module"]
pub struct OSCTUN_SPEC;
impl crate::RegisterSpec for OSCTUN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osctun::R](R) reader structure"]
impl crate::Readable for OSCTUN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osctun::W](W) writer structure"]
impl crate::Writable for OSCTUN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSCTUN to value 0"]
impl crate::Resettable for OSCTUN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
