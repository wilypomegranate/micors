#[doc = "Reader of register CR2"]
pub type R = crate::R<u32, super::CR2>;
#[doc = "Writer for register CR2"]
pub type W = crate::W<u32, super::CR2>;
#[doc = "Register CR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSVREFE`"]
pub type TSVREFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSVREFE`"]
pub struct TSVREFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSVREFE_W<'a> {
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
#[doc = "Reader of field `SWSTART`"]
pub type SWSTART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWSTART`"]
pub struct SWSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> SWSTART_W<'a> {
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
#[doc = "Reader of field `JSWSTART`"]
pub type JSWSTART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JSWSTART`"]
pub struct JSWSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> JSWSTART_W<'a> {
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
#[doc = "Reader of field `EXTTRIG`"]
pub type EXTTRIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTTRIG`"]
pub struct EXTTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTTRIG_W<'a> {
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
#[doc = "Reader of field `EXTSEL`"]
pub type EXTSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTSEL`"]
pub struct EXTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | (((value as u32) & 0x07) << 17);
        self.w
    }
}
#[doc = "Reader of field `JEXTTRIG`"]
pub type JEXTTRIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JEXTTRIG`"]
pub struct JEXTTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> JEXTTRIG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `JEXTSEL`"]
pub type JEXTSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `JEXTSEL`"]
pub struct JEXTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> JEXTSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `ALIGN`"]
pub type ALIGN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALIGN`"]
pub struct ALIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALIGN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `DMA`"]
pub type DMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA`"]
pub struct DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `RSTCAL`"]
pub type RSTCAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTCAL`"]
pub struct RSTCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTCAL_W<'a> {
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
#[doc = "Reader of field `CAL`"]
pub type CAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAL`"]
pub struct CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL_W<'a> {
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
#[doc = "Reader of field `CONT`"]
pub type CONT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONT`"]
pub struct CONT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONT_W<'a> {
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
#[doc = "Reader of field `ADON`"]
pub type ADON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADON`"]
pub struct ADON_W<'a> {
    w: &'a mut W,
}
impl<'a> ADON_W<'a> {
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
impl R {
    #[doc = "Bit 23 - Temperature sensor and VREFINT enable"]
    #[inline(always)]
    pub fn tsvrefe(&self) -> TSVREFE_R {
        TSVREFE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Start conversion of regular channels"]
    #[inline(always)]
    pub fn swstart(&self) -> SWSTART_R {
        SWSTART_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Start conversion of injected channels"]
    #[inline(always)]
    pub fn jswstart(&self) -> JSWSTART_R {
        JSWSTART_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - External trigger conversion mode for regular channels"]
    #[inline(always)]
    pub fn exttrig(&self) -> EXTTRIG_R {
        EXTTRIG_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 17:19 - External event select for regular group"]
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bit 15 - External trigger conversion mode for injected channels"]
    #[inline(always)]
    pub fn jexttrig(&self) -> JEXTTRIG_R {
        JEXTTRIG_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - External event select for injected group"]
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Direct memory access mode"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Reset calibration"]
    #[inline(always)]
    pub fn rstcal(&self) -> RSTCAL_R {
        RSTCAL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - A/D calibration"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Continuous conversion"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - A/D converter ON / OFF"]
    #[inline(always)]
    pub fn adon(&self) -> ADON_R {
        ADON_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - Temperature sensor and VREFINT enable"]
    #[inline(always)]
    pub fn tsvrefe(&mut self) -> TSVREFE_W {
        TSVREFE_W { w: self }
    }
    #[doc = "Bit 22 - Start conversion of regular channels"]
    #[inline(always)]
    pub fn swstart(&mut self) -> SWSTART_W {
        SWSTART_W { w: self }
    }
    #[doc = "Bit 21 - Start conversion of injected channels"]
    #[inline(always)]
    pub fn jswstart(&mut self) -> JSWSTART_W {
        JSWSTART_W { w: self }
    }
    #[doc = "Bit 20 - External trigger conversion mode for regular channels"]
    #[inline(always)]
    pub fn exttrig(&mut self) -> EXTTRIG_W {
        EXTTRIG_W { w: self }
    }
    #[doc = "Bits 17:19 - External event select for regular group"]
    #[inline(always)]
    pub fn extsel(&mut self) -> EXTSEL_W {
        EXTSEL_W { w: self }
    }
    #[doc = "Bit 15 - External trigger conversion mode for injected channels"]
    #[inline(always)]
    pub fn jexttrig(&mut self) -> JEXTTRIG_W {
        JEXTTRIG_W { w: self }
    }
    #[doc = "Bits 12:14 - External event select for injected group"]
    #[inline(always)]
    pub fn jextsel(&mut self) -> JEXTSEL_W {
        JEXTSEL_W { w: self }
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn align(&mut self) -> ALIGN_W {
        ALIGN_W { w: self }
    }
    #[doc = "Bit 8 - Direct memory access mode"]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
    }
    #[doc = "Bit 3 - Reset calibration"]
    #[inline(always)]
    pub fn rstcal(&mut self) -> RSTCAL_W {
        RSTCAL_W { w: self }
    }
    #[doc = "Bit 2 - A/D calibration"]
    #[inline(always)]
    pub fn cal(&mut self) -> CAL_W {
        CAL_W { w: self }
    }
    #[doc = "Bit 1 - Continuous conversion"]
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W {
        CONT_W { w: self }
    }
    #[doc = "Bit 0 - A/D converter ON / OFF"]
    #[inline(always)]
    pub fn adon(&mut self) -> ADON_W {
        ADON_W { w: self }
    }
}
