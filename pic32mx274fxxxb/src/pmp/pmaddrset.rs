#[doc = "Register `PMADDRSET` reader"]
pub struct R(crate::R<PMADDRSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMADDRSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PMADDRSET_SPEC>> for R {
    fn from(reader: crate::R<PMADDRSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMADDRSET` writer"]
pub struct W(crate::W<PMADDRSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMADDRSET_SPEC>;
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
impl core::convert::From<crate::W<PMADDRSET_SPEC>> for W {
    fn from(writer: crate::W<PMADDRSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - "]
pub struct ADDR_R(crate::FieldReader<u16, u16>);
impl ADDR_R {
    pub(crate) fn new(bits: u16) -> Self {
        ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR` writer - "]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
#[doc = "Field `ADDR14` reader - "]
pub struct ADDR14_R(crate::FieldReader<bool, bool>);
impl ADDR14_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADDR14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR14` writer - "]
pub struct ADDR14_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn addr14(&self) -> ADDR14_R {
        ADDR14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn addr14(&mut self) -> ADDR14_W {
        ADDR14_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMADDRSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmaddrset](index.html) module"]
pub struct PMADDRSET_SPEC;
impl crate::RegisterSpec for PMADDRSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmaddrset::R](R) reader structure"]
impl crate::Readable for PMADDRSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmaddrset::W](W) writer structure"]
impl crate::Writable for PMADDRSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMADDRSET to value 0"]
impl crate::Resettable for PMADDRSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
