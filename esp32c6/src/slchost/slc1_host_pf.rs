#[doc = "Register `SLC1_HOST_PF` reader"]
pub struct R(crate::R<SLC1_HOST_PF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLC1_HOST_PF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLC1_HOST_PF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLC1_HOST_PF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLC1_PF_DATA` reader - *******Description***********"]
pub type SLC1_PF_DATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - *******Description***********"]
    #[inline(always)]
    pub fn slc1_pf_data(&self) -> SLC1_PF_DATA_R {
        SLC1_PF_DATA_R::new(self.bits)
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slc1_host_pf](index.html) module"]
pub struct SLC1_HOST_PF_SPEC;
impl crate::RegisterSpec for SLC1_HOST_PF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slc1_host_pf::R](R) reader structure"]
impl crate::Readable for SLC1_HOST_PF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SLC1_HOST_PF to value 0"]
impl crate::Resettable for SLC1_HOST_PF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
