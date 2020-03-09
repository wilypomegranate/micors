#[doc = "Reader of register DR9"]
pub type R = crate::R<u32, super::DR9>;
#[doc = "Writer for register DR9"]
pub type W = crate::W<u32, super::DR9>;
#[doc = "Register DR9 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR9 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D9`"]
pub type D9_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D9`"]
pub struct D9_W<'a> {
    w: &'a mut W,
}
impl<'a> D9_W<'a> {
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
    pub fn d9(&self) -> D9_R {
        D9_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d9(&mut self) -> D9_W {
        D9_W { w: self }
    }
}
