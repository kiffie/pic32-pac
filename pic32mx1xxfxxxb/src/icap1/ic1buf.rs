#[doc = "Register `IC1BUF` reader"]
pub struct R(crate::R<IC1BUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC1BUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IC1BUF_SPEC>> for R {
    fn from(reader: crate::R<IC1BUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IC1BUF` writer"]
pub struct W(crate::W<IC1BUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IC1BUF_SPEC>;
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
impl core::convert::From<crate::W<IC1BUF_SPEC>> for W {
    fn from(writer: crate::W<IC1BUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IC1BUF` reader - "]
pub struct IC1BUF_R(crate::FieldReader<u32, u32>);
impl IC1BUF_R {
    pub(crate) fn new(bits: u32) -> Self {
        IC1BUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC1BUF_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC1BUF` writer - "]
pub struct IC1BUF_W<'a> {
    w: &'a mut W,
}
impl<'a> IC1BUF_W<'a> {
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
    pub fn ic1buf(&self) -> IC1BUF_R {
        IC1BUF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ic1buf(&mut self) -> IC1BUF_W {
        IC1BUF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IC1BUF register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic1buf](index.html) module"]
pub struct IC1BUF_SPEC;
impl crate::RegisterSpec for IC1BUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic1buf::R](R) reader structure"]
impl crate::Readable for IC1BUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ic1buf::W](W) writer structure"]
impl crate::Writable for IC1BUF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IC1BUF to value 0"]
impl crate::Resettable for IC1BUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
