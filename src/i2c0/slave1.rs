#[doc = "Register `SLAVE1` reader"]
pub type R = crate::R<SLAVE1_SPEC>;
#[doc = "Register `SLAVE1` writer"]
pub type W = crate::W<SLAVE1_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SLAVE1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Slave Address Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slave1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLAVE1_SPEC;
impl crate::RegisterSpec for SLAVE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slave1::R`](R) reader structure"]
impl crate::Readable for SLAVE1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slave1::W`](W) writer structure"]
impl crate::Writable for SLAVE1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLAVE1 to value 0"]
impl crate::Resettable for SLAVE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
