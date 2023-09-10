#[doc = "Register `DS_TIMING_CODES` reader"]
pub type R = crate::R<DS_TIMING_CODES_SPEC>;
#[doc = "Register `DS_TIMING_CODES` writer"]
pub type W = crate::W<DS_TIMING_CODES_SPEC>;
#[doc = "Field `SAV` reader - Start Active Video Code."]
pub type SAV_R = crate::FieldReader;
#[doc = "Field `SAV` writer - Start Active Video Code."]
pub type SAV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `EAV` reader - End Active Video Code."]
pub type EAV_R = crate::FieldReader;
#[doc = "Field `EAV` writer - End Active Video Code."]
pub type EAV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Start Active Video Code."]
    #[inline(always)]
    pub fn sav(&self) -> SAV_R {
        SAV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - End Active Video Code."]
    #[inline(always)]
    pub fn eav(&self) -> EAV_R {
        EAV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Start Active Video Code."]
    #[inline(always)]
    #[must_use]
    pub fn sav(&mut self) -> SAV_W<DS_TIMING_CODES_SPEC, 0> {
        SAV_W::new(self)
    }
    #[doc = "Bits 8:15 - End Active Video Code."]
    #[inline(always)]
    #[must_use]
    pub fn eav(&mut self) -> EAV_W<DS_TIMING_CODES_SPEC, 8> {
        EAV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DS Timing Code Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ds_timing_codes::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ds_timing_codes::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DS_TIMING_CODES_SPEC;
impl crate::RegisterSpec for DS_TIMING_CODES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ds_timing_codes::R`](R) reader structure"]
impl crate::Readable for DS_TIMING_CODES_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ds_timing_codes::W`](W) writer structure"]
impl crate::Writable for DS_TIMING_CODES_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DS_TIMING_CODES to value 0"]
impl crate::Resettable for DS_TIMING_CODES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
