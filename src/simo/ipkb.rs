#[doc = "Register `IPKB` reader"]
pub type R = crate::R<IPKB_SPEC>;
#[doc = "Register `IPKB` writer"]
pub type W = crate::W<IPKB_SPEC>;
#[doc = "Field `IPKSETC` reader - Voltage Regulator Peak Current Setting"]
pub type IPKSETC_R = crate::FieldReader;
#[doc = "Field `IPKSETC` writer - Voltage Regulator Peak Current Setting"]
pub type IPKSETC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `IPKSETD` reader - Voltage Regulator Peak Current Setting"]
pub type IPKSETD_R = crate::FieldReader;
#[doc = "Field `IPKSETD` writer - Voltage Regulator Peak Current Setting"]
pub type IPKSETD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Voltage Regulator Peak Current Setting"]
    #[inline(always)]
    pub fn ipksetc(&self) -> IPKSETC_R {
        IPKSETC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Voltage Regulator Peak Current Setting"]
    #[inline(always)]
    pub fn ipksetd(&self) -> IPKSETD_R {
        IPKSETD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Voltage Regulator Peak Current Setting"]
    #[inline(always)]
    #[must_use]
    pub fn ipksetc(&mut self) -> IPKSETC_W<IPKB_SPEC, 0> {
        IPKSETC_W::new(self)
    }
    #[doc = "Bits 4:7 - Voltage Regulator Peak Current Setting"]
    #[inline(always)]
    #[must_use]
    pub fn ipksetd(&mut self) -> IPKSETD_W<IPKB_SPEC, 4> {
        IPKSETD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "High Side FET Peak Current VREGO_C/VREGO_D Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipkb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipkb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IPKB_SPEC;
impl crate::RegisterSpec for IPKB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipkb::R`](R) reader structure"]
impl crate::Readable for IPKB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ipkb::W`](W) writer structure"]
impl crate::Writable for IPKB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPKB to value 0"]
impl crate::Resettable for IPKB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
