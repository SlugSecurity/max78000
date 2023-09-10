#[doc = "Register `AUTOCAL1` reader"]
pub type R = crate::R<AUTOCAL1_SPEC>;
#[doc = "Register `AUTOCAL1` writer"]
pub type W = crate::W<AUTOCAL1_SPEC>;
#[doc = "Field `INITTRM` reader - Initial Trim Setting."]
pub type INITTRM_R = crate::FieldReader<u16>;
#[doc = "Field `INITTRM` writer - Initial Trim Setting."]
pub type INITTRM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
impl R {
    #[doc = "Bits 0:8 - Initial Trim Setting."]
    #[inline(always)]
    pub fn inittrm(&self) -> INITTRM_R {
        INITTRM_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Initial Trim Setting."]
    #[inline(always)]
    #[must_use]
    pub fn inittrm(&mut self) -> INITTRM_W<AUTOCAL1_SPEC, 0> {
        INITTRM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Automatic Calibration 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`autocal1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`autocal1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AUTOCAL1_SPEC;
impl crate::RegisterSpec for AUTOCAL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`autocal1::R`](R) reader structure"]
impl crate::Readable for AUTOCAL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`autocal1::W`](W) writer structure"]
impl crate::Writable for AUTOCAL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AUTOCAL1 to value 0"]
impl crate::Resettable for AUTOCAL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
