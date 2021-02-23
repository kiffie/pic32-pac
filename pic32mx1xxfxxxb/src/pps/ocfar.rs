#[doc = "Register `OCFAR` reader"]
pub struct R(crate::R<OCFAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCFAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<OCFAR_SPEC>> for R {
    fn from(reader: crate::R<OCFAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCFAR` writer"]
pub struct W(crate::W<OCFAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCFAR_SPEC>;
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
impl core::convert::From<crate::W<OCFAR_SPEC>> for W {
    fn from(writer: crate::W<OCFAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OCFAR` reader - "]
pub struct OCFAR_R(crate::FieldReader<u8, u8>);
impl OCFAR_R {
    pub(crate) fn new(bits: u8) -> Self {
        OCFAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCFAR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCFAR` writer - "]
pub struct OCFAR_W<'a> {
    w: &'a mut W,
}
impl<'a> OCFAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ocfar(&self) -> OCFAR_R {
        OCFAR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ocfar(&mut self) -> OCFAR_W {
        OCFAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OCFAR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocfar](index.html) module"]
pub struct OCFAR_SPEC;
impl crate::RegisterSpec for OCFAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ocfar::R](R) reader structure"]
impl crate::Readable for OCFAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ocfar::W](W) writer structure"]
impl crate::Writable for OCFAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OCFAR to value 0"]
impl crate::Resettable for OCFAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
