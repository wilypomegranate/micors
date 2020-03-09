#[doc = "Reader of register DR29"]
pub type R = crate::R<u32, super::DR29>;
#[doc = "Writer for register DR29"]
pub type W = crate::W<u32, super::DR29>;
#[doc = "Register DR29 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR29 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D29`"]
pub type D29_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D29`"]
pub struct D29_W<'a> {
    w: &'a mut W,
}
impl<'a> D29_W<'a> {
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
    pub fn d29(&self) -> D29_R {
        D29_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d29(&mut self) -> D29_W {
        D29_W { w: self }
    }
}
