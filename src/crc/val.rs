#[doc = "Register `VAL` reader"]
pub type R = crate::R<VAL_SPEC>;
#[doc = "Register `VAL` writer"]
pub type W = crate::W<VAL_SPEC>;
#[doc = "Field `VALUE` reader - Current CRC Value"]
pub type VALUE_R = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - Current CRC Value"]
pub type VALUE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Current CRC Value"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Current CRC Value"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<VAL_SPEC, 0> {
        VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Current CRC Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`val::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`val::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VAL_SPEC;
impl crate::RegisterSpec for VAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`val::R`](R) reader structure"]
impl crate::Readable for VAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`val::W`](W) writer structure"]
impl crate::Writable for VAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VAL to value 0"]
impl crate::Resettable for VAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
