#[doc = "Register `EXTSETUP` reader"]
pub type R = crate::R<EXTSETUP_SPEC>;
#[doc = "Register `EXTSETUP` writer"]
pub type W = crate::W<EXTSETUP_SPEC>;
#[doc = "Field `EXT_BITS_WORD` reader - Word Length for ch_mode."]
pub type EXT_BITS_WORD_R = crate::FieldReader;
#[doc = "Field `EXT_BITS_WORD` writer - Word Length for ch_mode."]
pub type EXT_BITS_WORD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Word Length for ch_mode."]
    #[inline(always)]
    pub fn ext_bits_word(&self) -> EXT_BITS_WORD_R {
        EXT_BITS_WORD_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Word Length for ch_mode."]
    #[inline(always)]
    #[must_use]
    pub fn ext_bits_word(&mut self) -> EXT_BITS_WORD_W<EXTSETUP_SPEC, 0> {
        EXT_BITS_WORD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ext Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extsetup::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extsetup::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTSETUP_SPEC;
impl crate::RegisterSpec for EXTSETUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extsetup::R`](R) reader structure"]
impl crate::Readable for EXTSETUP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extsetup::W`](W) writer structure"]
impl crate::Writable for EXTSETUP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTSETUP to value 0"]
impl crate::Resettable for EXTSETUP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
