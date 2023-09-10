#[doc = "Register `ECCADDR` reader"]
pub type R = crate::R<ECCADDR_SPEC>;
#[doc = "Register `ECCADDR` writer"]
pub type W = crate::W<ECCADDR_SPEC>;
#[doc = "Field `ECCERRAD` reader - ECC Error Address."]
pub type ECCERRAD_R = crate::FieldReader<u32>;
#[doc = "Field `ECCERRAD` writer - ECC Error Address."]
pub type ECCERRAD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - ECC Error Address."]
    #[inline(always)]
    pub fn eccerrad(&self) -> ECCERRAD_R {
        ECCERRAD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ECC Error Address."]
    #[inline(always)]
    #[must_use]
    pub fn eccerrad(&mut self) -> ECCERRAD_W<ECCADDR_SPEC, 0> {
        ECCERRAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ECC Error Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECCADDR_SPEC;
impl crate::RegisterSpec for ECCADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccaddr::R`](R) reader structure"]
impl crate::Readable for ECCADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eccaddr::W`](W) writer structure"]
impl crate::Writable for ECCADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECCADDR to value 0"]
impl crate::Resettable for ECCADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
