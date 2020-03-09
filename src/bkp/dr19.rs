#[doc = "Reader of register DR19"]
pub type R = crate::R<u32, super::DR19>;
#[doc = "Writer for register DR19"]
pub type W = crate::W<u32, super::DR19>;
#[doc = "Register DR19 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR19 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D19`"]
pub type D19_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D19`"]
pub struct D19_W<'a> {
    w: &'a mut W,
}
impl<'a> D19_W<'a> {
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
    pub fn d19(&self) -> D19_R {
        D19_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d19(&mut self) -> D19_W {
        D19_W { w: self }
    }
}
