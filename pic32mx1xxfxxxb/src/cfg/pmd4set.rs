#[doc = "Reader of register PMD4SET"]
pub type R = crate::R<u32, super::PMD4SET>;
#[doc = "Writer for register PMD4SET"]
pub type W = crate::W<u32, super::PMD4SET>;
#[doc = "Register PMD4SET `reset()`'s with value 0"]
impl crate::ResetValue for super::PMD4SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `T1MD`"]
pub type T1MD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T1MD`"]
pub struct T1MD_W<'a> {
    w: &'a mut W,
}
impl<'a> T1MD_W<'a> {
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
#[doc = "Reader of field `T2MD`"]
pub type T2MD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T2MD`"]
pub struct T2MD_W<'a> {
    w: &'a mut W,
}
impl<'a> T2MD_W<'a> {
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
#[doc = "Reader of field `T3MD`"]
pub type T3MD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T3MD`"]
pub struct T3MD_W<'a> {
    w: &'a mut W,
}
impl<'a> T3MD_W<'a> {
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
#[doc = "Reader of field `T4MD`"]
pub type T4MD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T4MD`"]
pub struct T4MD_W<'a> {
    w: &'a mut W,
}
impl<'a> T4MD_W<'a> {
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
#[doc = "Reader of field `T5MD`"]
pub type T5MD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T5MD`"]
pub struct T5MD_W<'a> {
    w: &'a mut W,
}
impl<'a> T5MD_W<'a> {
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
    pub fn t1md(&self) -> T1MD_R {
        T1MD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn t2md(&self) -> T2MD_R {
        T2MD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn t3md(&self) -> T3MD_R {
        T3MD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn t4md(&self) -> T4MD_R {
        T4MD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn t5md(&self) -> T5MD_R {
        T5MD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn t1md(&mut self) -> T1MD_W {
        T1MD_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn t2md(&mut self) -> T2MD_W {
        T2MD_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn t3md(&mut self) -> T3MD_W {
        T3MD_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn t4md(&mut self) -> T4MD_W {
        T4MD_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn t5md(&mut self) -> T5MD_W {
        T5MD_W { w: self }
    }
}
