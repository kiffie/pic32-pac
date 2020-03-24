#[doc = "Reader of register PMD3"]
pub type R = crate::R<u32, super::PMD3>;
#[doc = "Writer for register PMD3"]
pub type W = crate::W<u32, super::PMD3>;
#[doc = "Register PMD3 `reset()`'s with value 0"]
impl crate::ResetValue for super::PMD3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IC1MD`"]
pub type IC1MD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IC1MD`"]
pub struct IC1MD_W<'a> {
    w: &'a mut W,
}
impl<'a> IC1MD_W<'a> {
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
#[doc = "Reader of field `IC2MD`"]
pub type IC2MD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IC2MD`"]
pub struct IC2MD_W<'a> {
    w: &'a mut W,
}
impl<'a> IC2MD_W<'a> {
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
#[doc = "Reader of field `IC3MD`"]
pub type IC3MD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IC3MD`"]
pub struct IC3MD_W<'a> {
    w: &'a mut W,
}
impl<'a> IC3MD_W<'a> {
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
#[doc = "Reader of field `IC4MD`"]
pub type IC4MD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IC4MD`"]
pub struct IC4MD_W<'a> {
    w: &'a mut W,
}
impl<'a> IC4MD_W<'a> {
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
#[doc = "Reader of field `IC5MD`"]
pub type IC5MD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IC5MD`"]
pub struct IC5MD_W<'a> {
    w: &'a mut W,
}
impl<'a> IC5MD_W<'a> {
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
#[doc = "Reader of field `OC1MD`"]
pub type OC1MD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC1MD`"]
pub struct OC1MD_W<'a> {
    w: &'a mut W,
}
impl<'a> OC1MD_W<'a> {
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
#[doc = "Reader of field `OC2MD`"]
pub type OC2MD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC2MD`"]
pub struct OC2MD_W<'a> {
    w: &'a mut W,
}
impl<'a> OC2MD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `OC3MD`"]
pub type OC3MD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC3MD`"]
pub struct OC3MD_W<'a> {
    w: &'a mut W,
}
impl<'a> OC3MD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `OC4MD`"]
pub type OC4MD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC4MD`"]
pub struct OC4MD_W<'a> {
    w: &'a mut W,
}
impl<'a> OC4MD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `OC5MD`"]
pub type OC5MD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC5MD`"]
pub struct OC5MD_W<'a> {
    w: &'a mut W,
}
impl<'a> OC5MD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ic1md(&self) -> IC1MD_R {
        IC1MD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ic2md(&self) -> IC2MD_R {
        IC2MD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ic3md(&self) -> IC3MD_R {
        IC3MD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ic4md(&self) -> IC4MD_R {
        IC4MD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ic5md(&self) -> IC5MD_R {
        IC5MD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn oc1md(&self) -> OC1MD_R {
        OC1MD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn oc2md(&self) -> OC2MD_R {
        OC2MD_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn oc3md(&self) -> OC3MD_R {
        OC3MD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn oc4md(&self) -> OC4MD_R {
        OC4MD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn oc5md(&self) -> OC5MD_R {
        OC5MD_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ic1md(&mut self) -> IC1MD_W {
        IC1MD_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ic2md(&mut self) -> IC2MD_W {
        IC2MD_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ic3md(&mut self) -> IC3MD_W {
        IC3MD_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ic4md(&mut self) -> IC4MD_W {
        IC4MD_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ic5md(&mut self) -> IC5MD_W {
        IC5MD_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn oc1md(&mut self) -> OC1MD_W {
        OC1MD_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn oc2md(&mut self) -> OC2MD_W {
        OC2MD_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn oc3md(&mut self) -> OC3MD_W {
        OC3MD_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn oc4md(&mut self) -> OC4MD_W {
        OC4MD_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn oc5md(&mut self) -> OC5MD_W {
        OC5MD_W { w: self }
    }
}
