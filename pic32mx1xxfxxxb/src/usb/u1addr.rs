#[doc = "Register `U1ADDR` reader"]
pub struct R(crate::R<U1ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U1ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<U1ADDR_SPEC>> for R {
    fn from(reader: crate::R<U1ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U1ADDR` writer"]
pub struct W(crate::W<U1ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U1ADDR_SPEC>;
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
impl core::convert::From<crate::W<U1ADDR_SPEC>> for W {
    fn from(writer: crate::W<U1ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEVADDR` reader - "]
pub struct DEVADDR_R(crate::FieldReader<u8, u8>);
impl DEVADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        DEVADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEVADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEVADDR` writer - "]
pub struct DEVADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Field `LSPDEN` reader - "]
pub struct LSPDEN_R(crate::FieldReader<bool, bool>);
impl LSPDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSPDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSPDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSPDEN` writer - "]
pub struct LSPDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LSPDEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn devaddr(&self) -> DEVADDR_R {
        DEVADDR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn lspden(&self) -> LSPDEN_R {
        LSPDEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn devaddr(&mut self) -> DEVADDR_W {
        DEVADDR_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn lspden(&mut self) -> LSPDEN_W {
        LSPDEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "U1ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1addr](index.html) module"]
pub struct U1ADDR_SPEC;
impl crate::RegisterSpec for U1ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u1addr::R](R) reader structure"]
impl crate::Readable for U1ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u1addr::W](W) writer structure"]
impl crate::Writable for U1ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets U1ADDR to value 0"]
impl crate::Resettable for U1ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
