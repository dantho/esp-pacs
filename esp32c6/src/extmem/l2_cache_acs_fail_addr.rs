#[doc = "Register `L2_CACHE_ACS_FAIL_ADDR` reader"]
pub struct R(crate::R<L2_CACHE_ACS_FAIL_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2_CACHE_ACS_FAIL_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2_CACHE_ACS_FAIL_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2_CACHE_ACS_FAIL_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L2_CACHE_FAIL_ADDR` reader - The register records the address of fail-access when L1-Cache accesses L2-Cache."]
pub type L2_CACHE_FAIL_ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the address of fail-access when L1-Cache accesses L2-Cache."]
    #[inline(always)]
    pub fn l2_cache_fail_addr(&self) -> L2_CACHE_FAIL_ADDR_R {
        L2_CACHE_FAIL_ADDR_R::new(self.bits)
    }
}
#[doc = "L2-Cache Access Fail Address information register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2_cache_acs_fail_addr](index.html) module"]
pub struct L2_CACHE_ACS_FAIL_ADDR_SPEC;
impl crate::RegisterSpec for L2_CACHE_ACS_FAIL_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2_cache_acs_fail_addr::R](R) reader structure"]
impl crate::Readable for L2_CACHE_ACS_FAIL_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L2_CACHE_ACS_FAIL_ADDR to value 0"]
impl crate::Resettable for L2_CACHE_ACS_FAIL_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
