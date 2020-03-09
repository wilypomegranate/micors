#[doc = "Reader of register DR27"]
pub type R = crate::R<u32, super::DR27>;
#[doc = "Writer for register DR27"]
pub type W = crate::W<u32, super::DR27>;
#[doc = "Register DR27 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR27 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D27`"]
pub type D27_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D27`"]
pub struct D27_W<'a> {
    w: &'a mut W,
}
impl<'a> D27_W<'a> {
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
    pub fn d27(&self) -> D27_R {
        D27_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d27(&mut self) -> D27_W {
        D27_W { w: self }
    }
}
