#[doc = "Reader of register U1CNFG1CLR"]
pub type R = crate::R<u32, super::U1CNFG1CLR>;
#[doc = "Writer for register U1CNFG1CLR"]
pub type W = crate::W<u32, super::U1CNFG1CLR>;
#[doc = "Register U1CNFG1CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::U1CNFG1CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UASUSPND`"]
pub type UASUSPND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UASUSPND`"]
pub struct UASUSPND_W<'a> {
    w: &'a mut W,
}
impl<'a> UASUSPND_W<'a> {
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
#[doc = "Reader of field `USBSIDL`"]
pub type USBSIDL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBSIDL`"]
pub struct USBSIDL_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSIDL_W<'a> {
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
#[doc = "Reader of field `USBFRZ`"]
pub type USBFRZ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBFRZ`"]
pub struct USBFRZ_W<'a> {
    w: &'a mut W,
}
impl<'a> USBFRZ_W<'a> {
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
#[doc = "Reader of field `UOEMON`"]
pub type UOEMON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UOEMON`"]
pub struct UOEMON_W<'a> {
    w: &'a mut W,
}
impl<'a> UOEMON_W<'a> {
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
#[doc = "Reader of field `UTEYE`"]
pub type UTEYE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UTEYE`"]
pub struct UTEYE_W<'a> {
    w: &'a mut W,
}
impl<'a> UTEYE_W<'a> {
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
    pub fn uasuspnd(&self) -> UASUSPND_R {
        UASUSPND_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn usbsidl(&self) -> USBSIDL_R {
        USBSIDL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn usbfrz(&self) -> USBFRZ_R {
        USBFRZ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn uoemon(&self) -> UOEMON_R {
        UOEMON_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uteye(&self) -> UTEYE_R {
        UTEYE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uasuspnd(&mut self) -> UASUSPND_W {
        UASUSPND_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn usbsidl(&mut self) -> USBSIDL_W {
        USBSIDL_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn usbfrz(&mut self) -> USBFRZ_W {
        USBFRZ_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn uoemon(&mut self) -> UOEMON_W {
        UOEMON_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uteye(&mut self) -> UTEYE_W {
        UTEYE_W { w: self }
    }
}
