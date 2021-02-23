#[doc = "Register `U1TOK` reader"]
pub struct R(crate::R<U1TOK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U1TOK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<U1TOK_SPEC>> for R {
    fn from(reader: crate::R<U1TOK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U1TOK` writer"]
pub struct W(crate::W<U1TOK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U1TOK_SPEC>;
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
impl core::convert::From<crate::W<U1TOK_SPEC>> for W {
    fn from(writer: crate::W<U1TOK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EP` reader - "]
pub struct EP_R(crate::FieldReader<u8, u8>);
impl EP_R {
    pub(crate) fn new(bits: u8) -> Self {
        EP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP` writer - "]
pub struct EP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Field `PID` reader - "]
pub struct PID_R(crate::FieldReader<u8, u8>);
impl PID_R {
    pub(crate) fn new(bits: u8) -> Self {
        PID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID` writer - "]
pub struct PID_W<'a> {
    w: &'a mut W,
}
impl<'a> PID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ep(&mut self) -> EP_W {
        EP_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pid(&mut self) -> PID_W {
        PID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "U1TOK register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1tok](index.html) module"]
pub struct U1TOK_SPEC;
impl crate::RegisterSpec for U1TOK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u1tok::R](R) reader structure"]
impl crate::Readable for U1TOK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u1tok::W](W) writer structure"]
impl crate::Writable for U1TOK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets U1TOK to value 0"]
impl crate::Resettable for U1TOK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
