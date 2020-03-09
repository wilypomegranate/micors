#[doc = "Reader of register DR8"]
pub type R = crate::R<u32, super::DR8>;
#[doc = "Writer for register DR8"]
pub type W = crate::W<u32, super::DR8>;
#[doc = "Register DR8 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR8 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D8`"]
pub type D8_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D8`"]
pub struct D8_W<'a> {
    w: &'a mut W,
}
impl<'a> D8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d8(&self) -> D8_R {
        D8_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d8(&mut self) -> D8_W {
        D8_W { w: self }
    }
}
