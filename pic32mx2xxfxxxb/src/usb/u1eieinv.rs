#[doc = "Register `U1EIEINV` reader"]
pub struct R(crate::R<U1EIEINV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U1EIEINV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<U1EIEINV_SPEC>> for R {
    fn from(reader: crate::R<U1EIEINV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U1EIEINV` writer"]
pub struct W(crate::W<U1EIEINV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U1EIEINV_SPEC>;
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
impl core::convert::From<crate::W<U1EIEINV_SPEC>> for W {
    fn from(writer: crate::W<U1EIEINV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIDEE` reader - "]
pub struct PIDEE_R(crate::FieldReader<bool, bool>);
impl PIDEE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIDEE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIDEE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIDEE` writer - "]
pub struct PIDEE_W<'a> {
    w: &'a mut W,
}
impl<'a> PIDEE_W<'a> {
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
#[doc = "Field `CRC5EE_EOFEE` reader - "]
pub struct CRC5EE_EOFEE_R(crate::FieldReader<bool, bool>);
impl CRC5EE_EOFEE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRC5EE_EOFEE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC5EE_EOFEE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC5EE_EOFEE` writer - "]
pub struct CRC5EE_EOFEE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC5EE_EOFEE_W<'a> {
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
#[doc = "Field `CRC16EE` reader - "]
pub struct CRC16EE_R(crate::FieldReader<bool, bool>);
impl CRC16EE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRC16EE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC16EE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC16EE` writer - "]
pub struct CRC16EE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC16EE_W<'a> {
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
#[doc = "Field `DFN8EE` reader - "]
pub struct DFN8EE_R(crate::FieldReader<bool, bool>);
impl DFN8EE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFN8EE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFN8EE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFN8EE` writer - "]
pub struct DFN8EE_W<'a> {
    w: &'a mut W,
}
impl<'a> DFN8EE_W<'a> {
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
#[doc = "Field `BTOEE` reader - "]
pub struct BTOEE_R(crate::FieldReader<bool, bool>);
impl BTOEE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BTOEE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BTOEE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BTOEE` writer - "]
pub struct BTOEE_W<'a> {
    w: &'a mut W,
}
impl<'a> BTOEE_W<'a> {
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
#[doc = "Field `DMAEE` reader - "]
pub struct DMAEE_R(crate::FieldReader<bool, bool>);
impl DMAEE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAEE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAEE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAEE` writer - "]
pub struct DMAEE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `BMXEE` reader - "]
pub struct BMXEE_R(crate::FieldReader<bool, bool>);
impl BMXEE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BMXEE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMXEE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BMXEE` writer - "]
pub struct BMXEE_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXEE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `BTSEE` reader - "]
pub struct BTSEE_R(crate::FieldReader<bool, bool>);
impl BTSEE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BTSEE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BTSEE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BTSEE` writer - "]
pub struct BTSEE_W<'a> {
    w: &'a mut W,
}
impl<'a> BTSEE_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pidee(&self) -> PIDEE_R {
        PIDEE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn crc5ee_eofee(&self) -> CRC5EE_EOFEE_R {
        CRC5EE_EOFEE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn crc16ee(&self) -> CRC16EE_R {
        CRC16EE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dfn8ee(&self) -> DFN8EE_R {
        DFN8EE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn btoee(&self) -> BTOEE_R {
        BTOEE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dmaee(&self) -> DMAEE_R {
        DMAEE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bmxee(&self) -> BMXEE_R {
        BMXEE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn btsee(&self) -> BTSEE_R {
        BTSEE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pidee(&mut self) -> PIDEE_W {
        PIDEE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn crc5ee_eofee(&mut self) -> CRC5EE_EOFEE_W {
        CRC5EE_EOFEE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn crc16ee(&mut self) -> CRC16EE_W {
        CRC16EE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dfn8ee(&mut self) -> DFN8EE_W {
        DFN8EE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn btoee(&mut self) -> BTOEE_W {
        BTOEE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dmaee(&mut self) -> DMAEE_W {
        DMAEE_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bmxee(&mut self) -> BMXEE_W {
        BMXEE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn btsee(&mut self) -> BTSEE_W {
        BTSEE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "U1EIEINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1eieinv](index.html) module"]
pub struct U1EIEINV_SPEC;
impl crate::RegisterSpec for U1EIEINV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u1eieinv::R](R) reader structure"]
impl crate::Readable for U1EIEINV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u1eieinv::W](W) writer structure"]
impl crate::Writable for U1EIEINV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets U1EIEINV to value 0"]
impl crate::Resettable for U1EIEINV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
