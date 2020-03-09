#[doc = "Reader of register RESPI1"]
pub type R = crate::R<u32, super::RESPI1>;
#[doc = "Reader of field `CARDSTATUS1`"]
pub type CARDSTATUS1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CARDSTATUS1"]
    #[inline(always)]
    pub fn cardstatus1(&self) -> CARDSTATUS1_R {
        CARDSTATUS1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
