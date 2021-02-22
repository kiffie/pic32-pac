#[doc = "Register `IC5BUF` reader"]
pub struct R(crate::R<IC5BUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC5BUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IC5BUF_SPEC>> for R {
    fn from(reader: crate::R<IC5BUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IC5BUF` writer"]
pub struct W(crate::W<IC5BUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IC5BUF_SPEC>;
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
impl core::convert::From<crate::W<IC5BUF_SPEC>> for W {
    fn from(writer: crate::W<IC5BUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IC5BUF` reader - "]
pub struct IC5BUF_R(crate::FieldReader<u32, u32>);
impl IC5BUF_R {
    pub(crate) fn new(bits: u32) -> Self {
        IC5BUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC5BUF_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC5BUF` writer - "]
pub struct IC5BUF_W<'a> {
    w: &'a mut W,
}
impl<'a> IC5BUF_W<'a> {
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
    pub fn ic5buf(&self) -> IC5BUF_R {
        IC5BUF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ic5buf(&mut self) -> IC5BUF_W {
        IC5BUF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IC5BUF register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic5buf](index.html) module"]
pub struct IC5BUF_SPEC;
impl crate::RegisterSpec for IC5BUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic5buf::R](R) reader structure"]
impl crate::Readable for IC5BUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ic5buf::W](W) writer structure"]
impl crate::Writable for IC5BUF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IC5BUF to value 0"]
impl crate::Resettable for IC5BUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
