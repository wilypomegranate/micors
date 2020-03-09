#[doc = "Reader of register DR37"]
pub type R = crate::R<u32, super::DR37>;
#[doc = "Writer for register DR37"]
pub type W = crate::W<u32, super::DR37>;
#[doc = "Register DR37 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR37 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D37`"]
pub type D37_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D37`"]
pub struct D37_W<'a> {
    w: &'a mut W,
}
impl<'a> D37_W<'a> {
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
    pub fn d37(&self) -> D37_R {
        D37_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d37(&mut self) -> D37_W {
        D37_W { w: self }
    }
}
