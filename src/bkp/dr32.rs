#[doc = "Reader of register DR32"]
pub type R = crate::R<u32, super::DR32>;
#[doc = "Writer for register DR32"]
pub type W = crate::W<u32, super::DR32>;
#[doc = "Register DR32 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR32 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D32`"]
pub type D32_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D32`"]
pub struct D32_W<'a> {
    w: &'a mut W,
}
impl<'a> D32_W<'a> {
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
    pub fn d32(&self) -> D32_R {
        D32_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d32(&mut self) -> D32_W {
        D32_W { w: self }
    }
}
