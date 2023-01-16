#[doc = "Register `BLK2_W10` reader"]
pub struct R(crate::R<BLK2_W10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK2_W10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK2_W10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK2_W10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLOCK2_W10` reader - Otp block2 word10 data."]
pub type BLOCK2_W10_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block2 word10 data."]
    #[inline(always)]
    pub fn block2_w10(&self) -> BLOCK2_W10_R {
        BLOCK2_W10_R::new(self.bits)
    }
}
#[doc = "Otp debuger block2 data register10.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk2_w10](index.html) module"]
pub struct BLK2_W10_SPEC;
impl crate::RegisterSpec for BLK2_W10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk2_w10::R](R) reader structure"]
impl crate::Readable for BLK2_W10_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK2_W10 to value 0"]
impl crate::Resettable for BLK2_W10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}