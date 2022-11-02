#[doc = "Register `TIMER1` reader"]
pub struct R(crate::R<TIMER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER1` writer"]
pub struct W(crate::W<TIMER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TIMER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPU_STALL_EN` reader - Enables CPU stalling."]
pub type CPU_STALL_EN_R = crate::BitReader<bool>;
#[doc = "Field `CPU_STALL_EN` writer - Enables CPU stalling."]
pub type CPU_STALL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMER1_SPEC, bool, O>;
#[doc = "Field `CPU_STALL_WAIT` reader - Sets the CPU stall waiting cycle (using the RTC fast clock)."]
pub type CPU_STALL_WAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPU_STALL_WAIT` writer - Sets the CPU stall waiting cycle (using the RTC fast clock)."]
pub type CPU_STALL_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMER1_SPEC, u8, u8, 5, O>;
#[doc = "Field `CK8M_WAIT` reader - Sets the 8 MHz clock waiting (using the RTC slow clock)."]
pub type CK8M_WAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CK8M_WAIT` writer - Sets the 8 MHz clock waiting (using the RTC slow clock)."]
pub type CK8M_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMER1_SPEC, u8, u8, 8, O>;
#[doc = "Field `XTL_BUF_WAIT` reader - Sets the XTAL waiting cycle (using the RTC slow clock)."]
pub type XTL_BUF_WAIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `XTL_BUF_WAIT` writer - Sets the XTAL waiting cycle (using the RTC slow clock)."]
pub type XTL_BUF_WAIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER1_SPEC, u16, u16, 10, O>;
#[doc = "Field `PLL_BUF_WAIT` reader - Sets the PLL waiting cycle (using the RTC slow clock)."]
pub type PLL_BUF_WAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLL_BUF_WAIT` writer - Sets the PLL waiting cycle (using the RTC slow clock)."]
pub type PLL_BUF_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMER1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Enables CPU stalling."]
    #[inline(always)]
    pub fn cpu_stall_en(&self) -> CPU_STALL_EN_R {
        CPU_STALL_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Sets the CPU stall waiting cycle (using the RTC fast clock)."]
    #[inline(always)]
    pub fn cpu_stall_wait(&self) -> CPU_STALL_WAIT_R {
        CPU_STALL_WAIT_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 6:13 - Sets the 8 MHz clock waiting (using the RTC slow clock)."]
    #[inline(always)]
    pub fn ck8m_wait(&self) -> CK8M_WAIT_R {
        CK8M_WAIT_R::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bits 14:23 - Sets the XTAL waiting cycle (using the RTC slow clock)."]
    #[inline(always)]
    pub fn xtl_buf_wait(&self) -> XTL_BUF_WAIT_R {
        XTL_BUF_WAIT_R::new(((self.bits >> 14) & 0x03ff) as u16)
    }
    #[doc = "Bits 24:31 - Sets the PLL waiting cycle (using the RTC slow clock)."]
    #[inline(always)]
    pub fn pll_buf_wait(&self) -> PLL_BUF_WAIT_R {
        PLL_BUF_WAIT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables CPU stalling."]
    #[inline(always)]
    pub fn cpu_stall_en(&mut self) -> CPU_STALL_EN_W<0> {
        CPU_STALL_EN_W::new(self)
    }
    #[doc = "Bits 1:5 - Sets the CPU stall waiting cycle (using the RTC fast clock)."]
    #[inline(always)]
    pub fn cpu_stall_wait(&mut self) -> CPU_STALL_WAIT_W<1> {
        CPU_STALL_WAIT_W::new(self)
    }
    #[doc = "Bits 6:13 - Sets the 8 MHz clock waiting (using the RTC slow clock)."]
    #[inline(always)]
    pub fn ck8m_wait(&mut self) -> CK8M_WAIT_W<6> {
        CK8M_WAIT_W::new(self)
    }
    #[doc = "Bits 14:23 - Sets the XTAL waiting cycle (using the RTC slow clock)."]
    #[inline(always)]
    pub fn xtl_buf_wait(&mut self) -> XTL_BUF_WAIT_W<14> {
        XTL_BUF_WAIT_W::new(self)
    }
    #[doc = "Bits 24:31 - Sets the PLL waiting cycle (using the RTC slow clock)."]
    #[inline(always)]
    pub fn pll_buf_wait(&mut self) -> PLL_BUF_WAIT_W<24> {
        PLL_BUF_WAIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configures CPU stall options\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer1](index.html) module"]
pub struct TIMER1_SPEC;
impl crate::RegisterSpec for TIMER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer1::R](R) reader structure"]
impl crate::Readable for TIMER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer1::W](W) writer structure"]
impl crate::Writable for TIMER1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER1 to value 0x2814_0403"]
impl crate::Resettable for TIMER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2814_0403
    }
}