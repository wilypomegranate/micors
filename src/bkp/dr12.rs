#[doc = "Reader of register DR12"]
pub type R = crate::R<u32, super::DR12>;
#[doc = "Writer for register DR12"]
pub type W = crate::W<u32, super::DR12>;
#[doc = "Register DR12 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR12 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DR12`"]
pub type DR12_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DR12`"]
pub struct DR12_W<'a> {
    w: &'a mut W,
}
impl<'a> DR12_W<'a> {
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
    pub fn dr12(&self) -> DR12_R {
        DR12_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn dr12(&mut self) -> DR12_W {
        DR12_W { w: self }
    }
}
