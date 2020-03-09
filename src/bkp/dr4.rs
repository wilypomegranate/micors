#[doc = "Reader of register DR4"]
pub type R = crate::R<u32, super::DR4>;
#[doc = "Writer for register DR4"]
pub type W = crate::W<u32, super::DR4>;
#[doc = "Register DR4 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D4`"]
pub type D4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D4`"]
pub struct D4_W<'a> {
    w: &'a mut W,
}
impl<'a> D4_W<'a> {
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
    pub fn d4(&self) -> D4_R {
        D4_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d4(&mut self) -> D4_W {
        D4_W { w: self }
    }
}
