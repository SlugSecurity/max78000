#[doc = "Register `AUTOCAL2` reader"]
pub type R = crate::R<AUTOCAL2_SPEC>;
#[doc = "Register `AUTOCAL2` writer"]
pub type W = crate::W<AUTOCAL2_SPEC>;
#[doc = "Field `DONECNT` reader - Auto-callibration Done Counter Setting."]
pub type DONECNT_R = crate::FieldReader;
#[doc = "Field `DONECNT` writer - Auto-callibration Done Counter Setting."]
pub type DONECNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `ACDIV` reader - Auto-callibration Div Setting."]
pub type ACDIV_R = crate::FieldReader<u16>;
#[doc = "Field `ACDIV` writer - Auto-callibration Div Setting."]
pub type ACDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 13, O, u16>;
impl R {
    #[doc = "Bits 0:7 - Auto-callibration Done Counter Setting."]
    #[inline(always)]
    pub fn donecnt(&self) -> DONECNT_R {
        DONECNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:20 - Auto-callibration Div Setting."]
    #[inline(always)]
    pub fn acdiv(&self) -> ACDIV_R {
        ACDIV_R::new(((self.bits >> 8) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Auto-callibration Done Counter Setting."]
    #[inline(always)]
    #[must_use]
    pub fn donecnt(&mut self) -> DONECNT_W<AUTOCAL2_SPEC, 0> {
        DONECNT_W::new(self)
    }
    #[doc = "Bits 8:20 - Auto-callibration Div Setting."]
    #[inline(always)]
    #[must_use]
    pub fn acdiv(&mut self) -> ACDIV_W<AUTOCAL2_SPEC, 8> {
        ACDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Automatic Calibration 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`autocal2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`autocal2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AUTOCAL2_SPEC;
impl crate::RegisterSpec for AUTOCAL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`autocal2::R`](R) reader structure"]
impl crate::Readable for AUTOCAL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`autocal2::W`](W) writer structure"]
impl crate::Writable for AUTOCAL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AUTOCAL2 to value 0"]
impl crate::Resettable for AUTOCAL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
