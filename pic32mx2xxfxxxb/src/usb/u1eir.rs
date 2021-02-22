#[doc = "Register `U1EIR` reader"]
pub struct R(crate::R<U1EIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U1EIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<U1EIR_SPEC>> for R {
    fn from(reader: crate::R<U1EIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U1EIR` writer"]
pub struct W(crate::W<U1EIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U1EIR_SPEC>;
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
impl core::convert::From<crate::W<U1EIR_SPEC>> for W {
    fn from(writer: crate::W<U1EIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIDEF` reader - "]
pub struct PIDEF_R(crate::FieldReader<bool, bool>);
impl PIDEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIDEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIDEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIDEF` writer - "]
pub struct PIDEF_W<'a> {
    w: &'a mut W,
}
impl<'a> PIDEF_W<'a> {
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
#[doc = "Field `CRC5EF_EOFEF` reader - "]
pub struct CRC5EF_EOFEF_R(crate::FieldReader<bool, bool>);
impl CRC5EF_EOFEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRC5EF_EOFEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC5EF_EOFEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC5EF_EOFEF` writer - "]
pub struct CRC5EF_EOFEF_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC5EF_EOFEF_W<'a> {
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
#[doc = "Field `CRC16EF` reader - "]
pub struct CRC16EF_R(crate::FieldReader<bool, bool>);
impl CRC16EF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRC16EF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC16EF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC16EF` writer - "]
pub struct CRC16EF_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC16EF_W<'a> {
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
#[doc = "Field `DFN8EF` reader - "]
pub struct DFN8EF_R(crate::FieldReader<bool, bool>);
impl DFN8EF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFN8EF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFN8EF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFN8EF` writer - "]
pub struct DFN8EF_W<'a> {
    w: &'a mut W,
}
impl<'a> DFN8EF_W<'a> {
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
#[doc = "Field `BTOEF` reader - "]
pub struct BTOEF_R(crate::FieldReader<bool, bool>);
impl BTOEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BTOEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BTOEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BTOEF` writer - "]
pub struct BTOEF_W<'a> {
    w: &'a mut W,
}
impl<'a> BTOEF_W<'a> {
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
#[doc = "Field `DMAEF` reader - "]
pub struct DMAEF_R(crate::FieldReader<bool, bool>);
impl DMAEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAEF` writer - "]
pub struct DMAEF_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEF_W<'a> {
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
#[doc = "Field `BMXEF` reader - "]
pub struct BMXEF_R(crate::FieldReader<bool, bool>);
impl BMXEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BMXEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMXEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BMXEF` writer - "]
pub struct BMXEF_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXEF_W<'a> {
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
#[doc = "Field `BTSEF` reader - "]
pub struct BTSEF_R(crate::FieldReader<bool, bool>);
impl BTSEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BTSEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BTSEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BTSEF` writer - "]
pub struct BTSEF_W<'a> {
    w: &'a mut W,
}
impl<'a> BTSEF_W<'a> {
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
    pub fn pidef(&self) -> PIDEF_R {
        PIDEF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn crc5ef_eofef(&self) -> CRC5EF_EOFEF_R {
        CRC5EF_EOFEF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn crc16ef(&self) -> CRC16EF_R {
        CRC16EF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dfn8ef(&self) -> DFN8EF_R {
        DFN8EF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn btoef(&self) -> BTOEF_R {
        BTOEF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dmaef(&self) -> DMAEF_R {
        DMAEF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bmxef(&self) -> BMXEF_R {
        BMXEF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn btsef(&self) -> BTSEF_R {
        BTSEF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pidef(&mut self) -> PIDEF_W {
        PIDEF_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn crc5ef_eofef(&mut self) -> CRC5EF_EOFEF_W {
        CRC5EF_EOFEF_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn crc16ef(&mut self) -> CRC16EF_W {
        CRC16EF_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dfn8ef(&mut self) -> DFN8EF_W {
        DFN8EF_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn btoef(&mut self) -> BTOEF_W {
        BTOEF_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dmaef(&mut self) -> DMAEF_W {
        DMAEF_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bmxef(&mut self) -> BMXEF_W {
        BMXEF_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn btsef(&mut self) -> BTSEF_W {
        BTSEF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "U1EIR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1eir](index.html) module"]
pub struct U1EIR_SPEC;
impl crate::RegisterSpec for U1EIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u1eir::R](R) reader structure"]
impl crate::Readable for U1EIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u1eir::W](W) writer structure"]
impl crate::Writable for U1EIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets U1EIR to value 0"]
impl crate::Resettable for U1EIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
