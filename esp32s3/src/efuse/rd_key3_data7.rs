#[doc = "Register `RD_KEY3_DATA7` reader"]
pub struct R(crate::R<RD_KEY3_DATA7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_KEY3_DATA7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_KEY3_DATA7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_KEY3_DATA7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `KEY3_DATA7` reader - Stores the seventh 32 bits of KEY3."]
pub type KEY3_DATA7_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the seventh 32 bits of KEY3."]
    #[inline(always)]
    pub fn key3_data7(&self) -> KEY3_DATA7_R {
        KEY3_DATA7_R::new(self.bits)
    }
}
#[doc = "Register 7 of BLOCK7 (KEY3).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key3_data7](index.html) module"]
pub struct RD_KEY3_DATA7_SPEC;
impl crate::RegisterSpec for RD_KEY3_DATA7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_key3_data7::R](R) reader structure"]
impl crate::Readable for RD_KEY3_DATA7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_KEY3_DATA7 to value 0"]
impl crate::Resettable for RD_KEY3_DATA7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}