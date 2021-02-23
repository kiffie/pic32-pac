#[doc = "Register `CNPUINV` reader"]
pub struct R(crate::R<CNPUINV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNPUINV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CNPUINV_SPEC>> for R {
    fn from(reader: crate::R<CNPUINV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNPUINV` writer"]
pub struct W(crate::W<CNPUINV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNPUINV_SPEC>;
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
impl core::convert::From<crate::W<CNPUINV_SPEC>> for W {
    fn from(writer: crate::W<CNPUINV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNPUA0` reader - "]
pub struct CNPUA0_R(crate::FieldReader<bool, bool>);
impl CNPUA0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPUA0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPUA0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPUA0` writer - "]
pub struct CNPUA0_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUA0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Field `CNPUA1` reader - "]
pub struct CNPUA1_R(crate::FieldReader<bool, bool>);
impl CNPUA1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPUA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPUA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPUA1` writer - "]
pub struct CNPUA1_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUA1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CNPUA2` reader - "]
pub struct CNPUA2_R(crate::FieldReader<bool, bool>);
impl CNPUA2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPUA2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPUA2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPUA2` writer - "]
pub struct CNPUA2_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUA2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `CNPUA3` reader - "]
pub struct CNPUA3_R(crate::FieldReader<bool, bool>);
impl CNPUA3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPUA3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPUA3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPUA3` writer - "]
pub struct CNPUA3_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUA3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CNPUA4` reader - "]
pub struct CNPUA4_R(crate::FieldReader<bool, bool>);
impl CNPUA4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPUA4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPUA4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPUA4` writer - "]
pub struct CNPUA4_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUA4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cnpua0(&self) -> CNPUA0_R {
        CNPUA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cnpua1(&self) -> CNPUA1_R {
        CNPUA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cnpua2(&self) -> CNPUA2_R {
        CNPUA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cnpua3(&self) -> CNPUA3_R {
        CNPUA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cnpua4(&self) -> CNPUA4_R {
        CNPUA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cnpua0(&mut self) -> CNPUA0_W {
        CNPUA0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cnpua1(&mut self) -> CNPUA1_W {
        CNPUA1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cnpua2(&mut self) -> CNPUA2_W {
        CNPUA2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cnpua3(&mut self) -> CNPUA3_W {
        CNPUA3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cnpua4(&mut self) -> CNPUA4_W {
        CNPUA4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CNPUAINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnpuinv](index.html) module"]
pub struct CNPUINV_SPEC;
impl crate::RegisterSpec for CNPUINV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnpuinv::R](R) reader structure"]
impl crate::Readable for CNPUINV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnpuinv::W](W) writer structure"]
impl crate::Writable for CNPUINV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNPUINV to value 0"]
impl crate::Resettable for CNPUINV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
