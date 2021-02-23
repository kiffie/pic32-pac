#[doc = "Register `IC2BUF` reader"]
pub struct R(crate::R<IC2BUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC2BUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IC2BUF_SPEC>> for R {
    fn from(reader: crate::R<IC2BUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IC2BUF` writer"]
pub struct W(crate::W<IC2BUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IC2BUF_SPEC>;
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
impl core::convert::From<crate::W<IC2BUF_SPEC>> for W {
    fn from(writer: crate::W<IC2BUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IC2BUF` reader - "]
pub struct IC2BUF_R(crate::FieldReader<u32, u32>);
impl IC2BUF_R {
    pub(crate) fn new(bits: u32) -> Self {
        IC2BUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC2BUF_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC2BUF` writer - "]
pub struct IC2BUF_W<'a> {
    w: &'a mut W,
}
impl<'a> IC2BUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ic2buf(&self) -> IC2BUF_R {
        IC2BUF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ic2buf(&mut self) -> IC2BUF_W {
        IC2BUF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IC2BUF register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic2buf](index.html) module"]
pub struct IC2BUF_SPEC;
impl crate::RegisterSpec for IC2BUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic2buf::R](R) reader structure"]
impl crate::Readable for IC2BUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ic2buf::W](W) writer structure"]
impl crate::Writable for IC2BUF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IC2BUF to value 0"]
impl crate::Resettable for IC2BUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
