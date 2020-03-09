#[doc = "Reader of register DR13"]
pub type R = crate::R<u32, super::DR13>;
#[doc = "Writer for register DR13"]
pub type W = crate::W<u32, super::DR13>;
#[doc = "Register DR13 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR13 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DR13`"]
pub type DR13_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DR13`"]
pub struct DR13_W<'a> {
    w: &'a mut W,
}
impl<'a> DR13_W<'a> {
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
    pub fn dr13(&self) -> DR13_R {
        DR13_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn dr13(&mut self) -> DR13_W {
        DR13_W { w: self }
    }
}
