#[doc = "Reader of register DCH3INTCLR"]
pub type R = crate::R<u32, super::DCH3INTCLR>;
#[doc = "Writer for register DCH3INTCLR"]
pub type W = crate::W<u32, super::DCH3INTCLR>;
#[doc = "Register DCH3INTCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::DCH3INTCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHERIF`"]
pub type CHERIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHERIF`"]
pub struct CHERIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CHERIF_W<'a> {
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
#[doc = "Reader of field `CHTAIF`"]
pub type CHTAIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHTAIF`"]
pub struct CHTAIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTAIF_W<'a> {
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
#[doc = "Reader of field `CHCCIF`"]
pub type CHCCIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHCCIF`"]
pub struct CHCCIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CHCCIF_W<'a> {
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
#[doc = "Reader of field `CHBCIF`"]
pub type CHBCIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHBCIF`"]
pub struct CHBCIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CHBCIF_W<'a> {
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
#[doc = "Reader of field `CHDHIF`"]
pub type CHDHIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHDHIF`"]
pub struct CHDHIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CHDHIF_W<'a> {
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
#[doc = "Reader of field `CHDDIF`"]
pub type CHDDIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHDDIF`"]
pub struct CHDDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CHDDIF_W<'a> {
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
#[doc = "Reader of field `CHSHIF`"]
pub type CHSHIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHSHIF`"]
pub struct CHSHIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSHIF_W<'a> {
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
#[doc = "Reader of field `CHSDIF`"]
pub type CHSDIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHSDIF`"]
pub struct CHSDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSDIF_W<'a> {
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
#[doc = "Reader of field `CHERIE`"]
pub type CHERIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHERIE`"]
pub struct CHERIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHERIE_W<'a> {
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
#[doc = "Reader of field `CHTAIE`"]
pub type CHTAIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHTAIE`"]
pub struct CHTAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTAIE_W<'a> {
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
#[doc = "Reader of field `CHCCIE`"]
pub type CHCCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHCCIE`"]
pub struct CHCCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHCCIE_W<'a> {
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
#[doc = "Reader of field `CHBCIE`"]
pub type CHBCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHBCIE`"]
pub struct CHBCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHBCIE_W<'a> {
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
#[doc = "Reader of field `CHDHIE`"]
pub type CHDHIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHDHIE`"]
pub struct CHDHIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHDHIE_W<'a> {
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
#[doc = "Reader of field `CHDDIE`"]
pub type CHDDIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHDDIE`"]
pub struct CHDDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHDDIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `CHSHIE`"]
pub type CHSHIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHSHIE`"]
pub struct CHSHIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSHIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `CHSDIE`"]
pub type CHSDIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHSDIE`"]
pub struct CHSDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSDIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cherif(&self) -> CHERIF_R {
        CHERIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn chtaif(&self) -> CHTAIF_R {
        CHTAIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn chccif(&self) -> CHCCIF_R {
        CHCCIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn chbcif(&self) -> CHBCIF_R {
        CHBCIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn chdhif(&self) -> CHDHIF_R {
        CHDHIF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn chddif(&self) -> CHDDIF_R {
        CHDDIF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn chshif(&self) -> CHSHIF_R {
        CHSHIF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn chsdif(&self) -> CHSDIF_R {
        CHSDIF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cherie(&self) -> CHERIE_R {
        CHERIE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn chtaie(&self) -> CHTAIE_R {
        CHTAIE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn chccie(&self) -> CHCCIE_R {
        CHCCIE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn chbcie(&self) -> CHBCIE_R {
        CHBCIE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn chdhie(&self) -> CHDHIE_R {
        CHDHIE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn chddie(&self) -> CHDDIE_R {
        CHDDIE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn chshie(&self) -> CHSHIE_R {
        CHSHIE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn chsdie(&self) -> CHSDIE_R {
        CHSDIE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cherif(&mut self) -> CHERIF_W {
        CHERIF_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn chtaif(&mut self) -> CHTAIF_W {
        CHTAIF_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn chccif(&mut self) -> CHCCIF_W {
        CHCCIF_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn chbcif(&mut self) -> CHBCIF_W {
        CHBCIF_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn chdhif(&mut self) -> CHDHIF_W {
        CHDHIF_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn chddif(&mut self) -> CHDDIF_W {
        CHDDIF_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn chshif(&mut self) -> CHSHIF_W {
        CHSHIF_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn chsdif(&mut self) -> CHSDIF_W {
        CHSDIF_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cherie(&mut self) -> CHERIE_W {
        CHERIE_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn chtaie(&mut self) -> CHTAIE_W {
        CHTAIE_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn chccie(&mut self) -> CHCCIE_W {
        CHCCIE_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn chbcie(&mut self) -> CHBCIE_W {
        CHBCIE_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn chdhie(&mut self) -> CHDHIE_W {
        CHDHIE_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn chddie(&mut self) -> CHDDIE_W {
        CHDDIE_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn chshie(&mut self) -> CHSHIE_W {
        CHSHIE_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn chsdie(&mut self) -> CHSDIE_W {
        CHSDIE_W { w: self }
    }
}
