#[doc = "Register `INTCONSET` reader"]
pub struct R(crate::R<INTCONSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTCONSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<INTCONSET_SPEC>> for R {
    fn from(reader: crate::R<INTCONSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTCONSET` writer"]
pub struct W(crate::W<INTCONSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTCONSET_SPEC>;
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
impl core::convert::From<crate::W<INTCONSET_SPEC>> for W {
    fn from(writer: crate::W<INTCONSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT0EP` reader - "]
pub struct INT0EP_R(crate::FieldReader<bool, bool>);
impl INT0EP_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT0EP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT0EP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT0EP` writer - "]
pub struct INT0EP_W<'a> {
    w: &'a mut W,
}
impl<'a> INT0EP_W<'a> {
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
#[doc = "Field `INT1EP` reader - "]
pub struct INT1EP_R(crate::FieldReader<bool, bool>);
impl INT1EP_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT1EP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT1EP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT1EP` writer - "]
pub struct INT1EP_W<'a> {
    w: &'a mut W,
}
impl<'a> INT1EP_W<'a> {
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
#[doc = "Field `INT2EP` reader - "]
pub struct INT2EP_R(crate::FieldReader<bool, bool>);
impl INT2EP_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT2EP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT2EP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT2EP` writer - "]
pub struct INT2EP_W<'a> {
    w: &'a mut W,
}
impl<'a> INT2EP_W<'a> {
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
#[doc = "Field `INT3EP` reader - "]
pub struct INT3EP_R(crate::FieldReader<bool, bool>);
impl INT3EP_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT3EP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT3EP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT3EP` writer - "]
pub struct INT3EP_W<'a> {
    w: &'a mut W,
}
impl<'a> INT3EP_W<'a> {
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
#[doc = "Field `INT4EP` reader - "]
pub struct INT4EP_R(crate::FieldReader<bool, bool>);
impl INT4EP_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT4EP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT4EP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT4EP` writer - "]
pub struct INT4EP_W<'a> {
    w: &'a mut W,
}
impl<'a> INT4EP_W<'a> {
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
#[doc = "Field `TPC` reader - "]
pub struct TPC_R(crate::FieldReader<u8, u8>);
impl TPC_R {
    pub(crate) fn new(bits: u8) -> Self {
        TPC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TPC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPC` writer - "]
pub struct TPC_W<'a> {
    w: &'a mut W,
}
impl<'a> TPC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `MVEC` reader - "]
pub struct MVEC_R(crate::FieldReader<bool, bool>);
impl MVEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        MVEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MVEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MVEC` writer - "]
pub struct MVEC_W<'a> {
    w: &'a mut W,
}
impl<'a> MVEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `FRZ` reader - "]
pub struct FRZ_R(crate::FieldReader<bool, bool>);
impl FRZ_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRZ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRZ` writer - "]
pub struct FRZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FRZ_W<'a> {
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
#[doc = "Field `SS0` reader - "]
pub struct SS0_R(crate::FieldReader<bool, bool>);
impl SS0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SS0` writer - "]
pub struct SS0_W<'a> {
    w: &'a mut W,
}
impl<'a> SS0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn int0ep(&self) -> INT0EP_R {
        INT0EP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn int1ep(&self) -> INT1EP_R {
        INT1EP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn int2ep(&self) -> INT2EP_R {
        INT2EP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn int3ep(&self) -> INT3EP_R {
        INT3EP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn int4ep(&self) -> INT4EP_R {
        INT4EP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn tpc(&self) -> TPC_R {
        TPC_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn mvec(&self) -> MVEC_R {
        MVEC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn frz(&self) -> FRZ_R {
        FRZ_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ss0(&self) -> SS0_R {
        SS0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn int0ep(&mut self) -> INT0EP_W {
        INT0EP_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn int1ep(&mut self) -> INT1EP_W {
        INT1EP_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn int2ep(&mut self) -> INT2EP_W {
        INT2EP_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn int3ep(&mut self) -> INT3EP_W {
        INT3EP_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn int4ep(&mut self) -> INT4EP_W {
        INT4EP_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn tpc(&mut self) -> TPC_W {
        TPC_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn mvec(&mut self) -> MVEC_W {
        MVEC_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn frz(&mut self) -> FRZ_W {
        FRZ_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ss0(&mut self) -> SS0_W {
        SS0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "INTCONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intconset](index.html) module"]
pub struct INTCONSET_SPEC;
impl crate::RegisterSpec for INTCONSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intconset::R](R) reader structure"]
impl crate::Readable for INTCONSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intconset::W](W) writer structure"]
impl crate::Writable for INTCONSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTCONSET to value 0"]
impl crate::Resettable for INTCONSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
