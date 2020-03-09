#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Backup data register (BKP_DR)"]
    pub dr1: DR1,
    #[doc = "0x04 - Backup data register (BKP_DR)"]
    pub dr2: DR2,
    #[doc = "0x08 - Backup data register (BKP_DR)"]
    pub dr3: DR3,
    #[doc = "0x0c - Backup data register (BKP_DR)"]
    pub dr4: DR4,
    #[doc = "0x10 - Backup data register (BKP_DR)"]
    pub dr5: DR5,
    #[doc = "0x14 - Backup data register (BKP_DR)"]
    pub dr6: DR6,
    #[doc = "0x18 - Backup data register (BKP_DR)"]
    pub dr7: DR7,
    #[doc = "0x1c - Backup data register (BKP_DR)"]
    pub dr8: DR8,
    #[doc = "0x20 - Backup data register (BKP_DR)"]
    pub dr9: DR9,
    #[doc = "0x24 - Backup data register (BKP_DR)"]
    pub dr10: DR10,
    #[doc = "0x28 - RTC clock calibration register (BKP_RTCCR)"]
    pub rtccr: RTCCR,
    #[doc = "0x2c - Backup control register (BKP_CR)"]
    pub cr: CR,
    #[doc = "0x30 - BKP_CSR control/status register (BKP_CSR)"]
    pub csr: CSR,
    _reserved13: [u8; 8usize],
    #[doc = "0x3c - Backup data register (BKP_DR)"]
    pub dr11: DR11,
    #[doc = "0x40 - Backup data register (BKP_DR)"]
    pub dr12: DR12,
    #[doc = "0x44 - Backup data register (BKP_DR)"]
    pub dr13: DR13,
    #[doc = "0x48 - Backup data register (BKP_DR)"]
    pub dr14: DR14,
    #[doc = "0x4c - Backup data register (BKP_DR)"]
    pub dr15: DR15,
    #[doc = "0x50 - Backup data register (BKP_DR)"]
    pub dr16: DR16,
    #[doc = "0x54 - Backup data register (BKP_DR)"]
    pub dr17: DR17,
    #[doc = "0x58 - Backup data register (BKP_DR)"]
    pub dr18: DR18,
    #[doc = "0x5c - Backup data register (BKP_DR)"]
    pub dr19: DR19,
    #[doc = "0x60 - Backup data register (BKP_DR)"]
    pub dr20: DR20,
    #[doc = "0x64 - Backup data register (BKP_DR)"]
    pub dr21: DR21,
    #[doc = "0x68 - Backup data register (BKP_DR)"]
    pub dr22: DR22,
    #[doc = "0x6c - Backup data register (BKP_DR)"]
    pub dr23: DR23,
    #[doc = "0x70 - Backup data register (BKP_DR)"]
    pub dr24: DR24,
    #[doc = "0x74 - Backup data register (BKP_DR)"]
    pub dr25: DR25,
    #[doc = "0x78 - Backup data register (BKP_DR)"]
    pub dr26: DR26,
    #[doc = "0x7c - Backup data register (BKP_DR)"]
    pub dr27: DR27,
    #[doc = "0x80 - Backup data register (BKP_DR)"]
    pub dr28: DR28,
    #[doc = "0x84 - Backup data register (BKP_DR)"]
    pub dr29: DR29,
    #[doc = "0x88 - Backup data register (BKP_DR)"]
    pub dr30: DR30,
    #[doc = "0x8c - Backup data register (BKP_DR)"]
    pub dr31: DR31,
    #[doc = "0x90 - Backup data register (BKP_DR)"]
    pub dr32: DR32,
    #[doc = "0x94 - Backup data register (BKP_DR)"]
    pub dr33: DR33,
    #[doc = "0x98 - Backup data register (BKP_DR)"]
    pub dr34: DR34,
    #[doc = "0x9c - Backup data register (BKP_DR)"]
    pub dr35: DR35,
    #[doc = "0xa0 - Backup data register (BKP_DR)"]
    pub dr36: DR36,
    #[doc = "0xa4 - Backup data register (BKP_DR)"]
    pub dr37: DR37,
    #[doc = "0xa8 - Backup data register (BKP_DR)"]
    pub dr38: DR38,
    #[doc = "0xac - Backup data register (BKP_DR)"]
    pub dr39: DR39,
    #[doc = "0xb0 - Backup data register (BKP_DR)"]
    pub dr40: DR40,
    #[doc = "0xb4 - Backup data register (BKP_DR)"]
    pub dr41: DR41,
    #[doc = "0xb8 - Backup data register (BKP_DR)"]
    pub dr42: DR42,
}
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr1](dr1) module"]
pub type DR1 = crate::Reg<u32, _DR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR1;
#[doc = "`read()` method returns [dr1::R](dr1::R) reader structure"]
impl crate::Readable for DR1 {}
#[doc = "`write(|w| ..)` method takes [dr1::W](dr1::W) writer structure"]
impl crate::Writable for DR1 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr1;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr2](dr2) module"]
pub type DR2 = crate::Reg<u32, _DR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR2;
#[doc = "`read()` method returns [dr2::R](dr2::R) reader structure"]
impl crate::Readable for DR2 {}
#[doc = "`write(|w| ..)` method takes [dr2::W](dr2::W) writer structure"]
impl crate::Writable for DR2 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr2;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr3](dr3) module"]
pub type DR3 = crate::Reg<u32, _DR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR3;
#[doc = "`read()` method returns [dr3::R](dr3::R) reader structure"]
impl crate::Readable for DR3 {}
#[doc = "`write(|w| ..)` method takes [dr3::W](dr3::W) writer structure"]
impl crate::Writable for DR3 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr3;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr4](dr4) module"]
pub type DR4 = crate::Reg<u32, _DR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR4;
#[doc = "`read()` method returns [dr4::R](dr4::R) reader structure"]
impl crate::Readable for DR4 {}
#[doc = "`write(|w| ..)` method takes [dr4::W](dr4::W) writer structure"]
impl crate::Writable for DR4 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr4;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr5](dr5) module"]
pub type DR5 = crate::Reg<u32, _DR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR5;
#[doc = "`read()` method returns [dr5::R](dr5::R) reader structure"]
impl crate::Readable for DR5 {}
#[doc = "`write(|w| ..)` method takes [dr5::W](dr5::W) writer structure"]
impl crate::Writable for DR5 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr5;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr6](dr6) module"]
pub type DR6 = crate::Reg<u32, _DR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR6;
#[doc = "`read()` method returns [dr6::R](dr6::R) reader structure"]
impl crate::Readable for DR6 {}
#[doc = "`write(|w| ..)` method takes [dr6::W](dr6::W) writer structure"]
impl crate::Writable for DR6 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr6;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr7](dr7) module"]
pub type DR7 = crate::Reg<u32, _DR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR7;
#[doc = "`read()` method returns [dr7::R](dr7::R) reader structure"]
impl crate::Readable for DR7 {}
#[doc = "`write(|w| ..)` method takes [dr7::W](dr7::W) writer structure"]
impl crate::Writable for DR7 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr7;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr8](dr8) module"]
pub type DR8 = crate::Reg<u32, _DR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR8;
#[doc = "`read()` method returns [dr8::R](dr8::R) reader structure"]
impl crate::Readable for DR8 {}
#[doc = "`write(|w| ..)` method takes [dr8::W](dr8::W) writer structure"]
impl crate::Writable for DR8 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr8;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr9](dr9) module"]
pub type DR9 = crate::Reg<u32, _DR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR9;
#[doc = "`read()` method returns [dr9::R](dr9::R) reader structure"]
impl crate::Readable for DR9 {}
#[doc = "`write(|w| ..)` method takes [dr9::W](dr9::W) writer structure"]
impl crate::Writable for DR9 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr9;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr10](dr10) module"]
pub type DR10 = crate::Reg<u32, _DR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR10;
#[doc = "`read()` method returns [dr10::R](dr10::R) reader structure"]
impl crate::Readable for DR10 {}
#[doc = "`write(|w| ..)` method takes [dr10::W](dr10::W) writer structure"]
impl crate::Writable for DR10 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr10;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr11](dr11) module"]
pub type DR11 = crate::Reg<u32, _DR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR11;
#[doc = "`read()` method returns [dr11::R](dr11::R) reader structure"]
impl crate::Readable for DR11 {}
#[doc = "`write(|w| ..)` method takes [dr11::W](dr11::W) writer structure"]
impl crate::Writable for DR11 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr11;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr12](dr12) module"]
pub type DR12 = crate::Reg<u32, _DR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR12;
#[doc = "`read()` method returns [dr12::R](dr12::R) reader structure"]
impl crate::Readable for DR12 {}
#[doc = "`write(|w| ..)` method takes [dr12::W](dr12::W) writer structure"]
impl crate::Writable for DR12 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr12;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr13](dr13) module"]
pub type DR13 = crate::Reg<u32, _DR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR13;
#[doc = "`read()` method returns [dr13::R](dr13::R) reader structure"]
impl crate::Readable for DR13 {}
#[doc = "`write(|w| ..)` method takes [dr13::W](dr13::W) writer structure"]
impl crate::Writable for DR13 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr13;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr14](dr14) module"]
pub type DR14 = crate::Reg<u32, _DR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR14;
#[doc = "`read()` method returns [dr14::R](dr14::R) reader structure"]
impl crate::Readable for DR14 {}
#[doc = "`write(|w| ..)` method takes [dr14::W](dr14::W) writer structure"]
impl crate::Writable for DR14 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr14;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr15](dr15) module"]
pub type DR15 = crate::Reg<u32, _DR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR15;
#[doc = "`read()` method returns [dr15::R](dr15::R) reader structure"]
impl crate::Readable for DR15 {}
#[doc = "`write(|w| ..)` method takes [dr15::W](dr15::W) writer structure"]
impl crate::Writable for DR15 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr15;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr16](dr16) module"]
pub type DR16 = crate::Reg<u32, _DR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR16;
#[doc = "`read()` method returns [dr16::R](dr16::R) reader structure"]
impl crate::Readable for DR16 {}
#[doc = "`write(|w| ..)` method takes [dr16::W](dr16::W) writer structure"]
impl crate::Writable for DR16 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr16;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr17](dr17) module"]
pub type DR17 = crate::Reg<u32, _DR17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR17;
#[doc = "`read()` method returns [dr17::R](dr17::R) reader structure"]
impl crate::Readable for DR17 {}
#[doc = "`write(|w| ..)` method takes [dr17::W](dr17::W) writer structure"]
impl crate::Writable for DR17 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr17;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr18](dr18) module"]
pub type DR18 = crate::Reg<u32, _DR18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR18;
#[doc = "`read()` method returns [dr18::R](dr18::R) reader structure"]
impl crate::Readable for DR18 {}
#[doc = "`write(|w| ..)` method takes [dr18::W](dr18::W) writer structure"]
impl crate::Writable for DR18 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr18;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr19](dr19) module"]
pub type DR19 = crate::Reg<u32, _DR19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR19;
#[doc = "`read()` method returns [dr19::R](dr19::R) reader structure"]
impl crate::Readable for DR19 {}
#[doc = "`write(|w| ..)` method takes [dr19::W](dr19::W) writer structure"]
impl crate::Writable for DR19 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr19;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr20](dr20) module"]
pub type DR20 = crate::Reg<u32, _DR20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR20;
#[doc = "`read()` method returns [dr20::R](dr20::R) reader structure"]
impl crate::Readable for DR20 {}
#[doc = "`write(|w| ..)` method takes [dr20::W](dr20::W) writer structure"]
impl crate::Writable for DR20 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr20;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr21](dr21) module"]
pub type DR21 = crate::Reg<u32, _DR21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR21;
#[doc = "`read()` method returns [dr21::R](dr21::R) reader structure"]
impl crate::Readable for DR21 {}
#[doc = "`write(|w| ..)` method takes [dr21::W](dr21::W) writer structure"]
impl crate::Writable for DR21 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr21;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr22](dr22) module"]
pub type DR22 = crate::Reg<u32, _DR22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR22;
#[doc = "`read()` method returns [dr22::R](dr22::R) reader structure"]
impl crate::Readable for DR22 {}
#[doc = "`write(|w| ..)` method takes [dr22::W](dr22::W) writer structure"]
impl crate::Writable for DR22 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr22;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr23](dr23) module"]
pub type DR23 = crate::Reg<u32, _DR23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR23;
#[doc = "`read()` method returns [dr23::R](dr23::R) reader structure"]
impl crate::Readable for DR23 {}
#[doc = "`write(|w| ..)` method takes [dr23::W](dr23::W) writer structure"]
impl crate::Writable for DR23 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr23;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr24](dr24) module"]
pub type DR24 = crate::Reg<u32, _DR24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR24;
#[doc = "`read()` method returns [dr24::R](dr24::R) reader structure"]
impl crate::Readable for DR24 {}
#[doc = "`write(|w| ..)` method takes [dr24::W](dr24::W) writer structure"]
impl crate::Writable for DR24 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr24;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr25](dr25) module"]
pub type DR25 = crate::Reg<u32, _DR25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR25;
#[doc = "`read()` method returns [dr25::R](dr25::R) reader structure"]
impl crate::Readable for DR25 {}
#[doc = "`write(|w| ..)` method takes [dr25::W](dr25::W) writer structure"]
impl crate::Writable for DR25 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr25;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr26](dr26) module"]
pub type DR26 = crate::Reg<u32, _DR26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR26;
#[doc = "`read()` method returns [dr26::R](dr26::R) reader structure"]
impl crate::Readable for DR26 {}
#[doc = "`write(|w| ..)` method takes [dr26::W](dr26::W) writer structure"]
impl crate::Writable for DR26 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr26;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr27](dr27) module"]
pub type DR27 = crate::Reg<u32, _DR27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR27;
#[doc = "`read()` method returns [dr27::R](dr27::R) reader structure"]
impl crate::Readable for DR27 {}
#[doc = "`write(|w| ..)` method takes [dr27::W](dr27::W) writer structure"]
impl crate::Writable for DR27 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr27;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr28](dr28) module"]
pub type DR28 = crate::Reg<u32, _DR28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR28;
#[doc = "`read()` method returns [dr28::R](dr28::R) reader structure"]
impl crate::Readable for DR28 {}
#[doc = "`write(|w| ..)` method takes [dr28::W](dr28::W) writer structure"]
impl crate::Writable for DR28 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr28;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr29](dr29) module"]
pub type DR29 = crate::Reg<u32, _DR29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR29;
#[doc = "`read()` method returns [dr29::R](dr29::R) reader structure"]
impl crate::Readable for DR29 {}
#[doc = "`write(|w| ..)` method takes [dr29::W](dr29::W) writer structure"]
impl crate::Writable for DR29 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr29;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr30](dr30) module"]
pub type DR30 = crate::Reg<u32, _DR30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR30;
#[doc = "`read()` method returns [dr30::R](dr30::R) reader structure"]
impl crate::Readable for DR30 {}
#[doc = "`write(|w| ..)` method takes [dr30::W](dr30::W) writer structure"]
impl crate::Writable for DR30 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr30;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr31](dr31) module"]
pub type DR31 = crate::Reg<u32, _DR31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR31;
#[doc = "`read()` method returns [dr31::R](dr31::R) reader structure"]
impl crate::Readable for DR31 {}
#[doc = "`write(|w| ..)` method takes [dr31::W](dr31::W) writer structure"]
impl crate::Writable for DR31 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr31;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr32](dr32) module"]
pub type DR32 = crate::Reg<u32, _DR32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR32;
#[doc = "`read()` method returns [dr32::R](dr32::R) reader structure"]
impl crate::Readable for DR32 {}
#[doc = "`write(|w| ..)` method takes [dr32::W](dr32::W) writer structure"]
impl crate::Writable for DR32 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr32;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr33](dr33) module"]
pub type DR33 = crate::Reg<u32, _DR33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR33;
#[doc = "`read()` method returns [dr33::R](dr33::R) reader structure"]
impl crate::Readable for DR33 {}
#[doc = "`write(|w| ..)` method takes [dr33::W](dr33::W) writer structure"]
impl crate::Writable for DR33 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr33;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr34](dr34) module"]
pub type DR34 = crate::Reg<u32, _DR34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR34;
#[doc = "`read()` method returns [dr34::R](dr34::R) reader structure"]
impl crate::Readable for DR34 {}
#[doc = "`write(|w| ..)` method takes [dr34::W](dr34::W) writer structure"]
impl crate::Writable for DR34 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr34;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr35](dr35) module"]
pub type DR35 = crate::Reg<u32, _DR35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR35;
#[doc = "`read()` method returns [dr35::R](dr35::R) reader structure"]
impl crate::Readable for DR35 {}
#[doc = "`write(|w| ..)` method takes [dr35::W](dr35::W) writer structure"]
impl crate::Writable for DR35 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr35;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr36](dr36) module"]
pub type DR36 = crate::Reg<u32, _DR36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR36;
#[doc = "`read()` method returns [dr36::R](dr36::R) reader structure"]
impl crate::Readable for DR36 {}
#[doc = "`write(|w| ..)` method takes [dr36::W](dr36::W) writer structure"]
impl crate::Writable for DR36 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr36;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr37](dr37) module"]
pub type DR37 = crate::Reg<u32, _DR37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR37;
#[doc = "`read()` method returns [dr37::R](dr37::R) reader structure"]
impl crate::Readable for DR37 {}
#[doc = "`write(|w| ..)` method takes [dr37::W](dr37::W) writer structure"]
impl crate::Writable for DR37 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr37;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr38](dr38) module"]
pub type DR38 = crate::Reg<u32, _DR38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR38;
#[doc = "`read()` method returns [dr38::R](dr38::R) reader structure"]
impl crate::Readable for DR38 {}
#[doc = "`write(|w| ..)` method takes [dr38::W](dr38::W) writer structure"]
impl crate::Writable for DR38 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr38;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr39](dr39) module"]
pub type DR39 = crate::Reg<u32, _DR39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR39;
#[doc = "`read()` method returns [dr39::R](dr39::R) reader structure"]
impl crate::Readable for DR39 {}
#[doc = "`write(|w| ..)` method takes [dr39::W](dr39::W) writer structure"]
impl crate::Writable for DR39 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr39;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr40](dr40) module"]
pub type DR40 = crate::Reg<u32, _DR40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR40;
#[doc = "`read()` method returns [dr40::R](dr40::R) reader structure"]
impl crate::Readable for DR40 {}
#[doc = "`write(|w| ..)` method takes [dr40::W](dr40::W) writer structure"]
impl crate::Writable for DR40 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr40;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr41](dr41) module"]
pub type DR41 = crate::Reg<u32, _DR41>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR41;
#[doc = "`read()` method returns [dr41::R](dr41::R) reader structure"]
impl crate::Readable for DR41 {}
#[doc = "`write(|w| ..)` method takes [dr41::W](dr41::W) writer structure"]
impl crate::Writable for DR41 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr41;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr42](dr42) module"]
pub type DR42 = crate::Reg<u32, _DR42>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR42;
#[doc = "`read()` method returns [dr42::R](dr42::R) reader structure"]
impl crate::Readable for DR42 {}
#[doc = "`write(|w| ..)` method takes [dr42::W](dr42::W) writer structure"]
impl crate::Writable for DR42 {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr42;
#[doc = "RTC clock calibration register (BKP_RTCCR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtccr](rtccr) module"]
pub type RTCCR = crate::Reg<u32, _RTCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCCR;
#[doc = "`read()` method returns [rtccr::R](rtccr::R) reader structure"]
impl crate::Readable for RTCCR {}
#[doc = "`write(|w| ..)` method takes [rtccr::W](rtccr::W) writer structure"]
impl crate::Writable for RTCCR {}
#[doc = "RTC clock calibration register (BKP_RTCCR)"]
pub mod rtccr;
#[doc = "Backup control register (BKP_CR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Backup control register (BKP_CR)"]
pub mod cr;
#[doc = "BKP_CSR control/status register (BKP_CSR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](csr) module"]
pub type CSR = crate::Reg<u32, _CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR;
#[doc = "`read()` method returns [csr::R](csr::R) reader structure"]
impl crate::Readable for CSR {}
#[doc = "`write(|w| ..)` method takes [csr::W](csr::W) writer structure"]
impl crate::Writable for CSR {}
#[doc = "BKP_CSR control/status register (BKP_CSR)"]
pub mod csr;
