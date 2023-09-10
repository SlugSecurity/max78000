#[doc = "Register `INTMODE` reader"]
pub type R = crate::R<INTMODE_SPEC>;
#[doc = "Register `INTMODE` writer"]
pub type W = crate::W<INTMODE_SPEC>;
#[doc = "Field `GPIO_INTMODE` reader - Mask of all of the pins on the port."]
pub type GPIO_INTMODE_R = crate::FieldReader<GPIO_INTMODE_A>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GPIO_INTMODE_A {
    #[doc = "0: Interrupts for this pin are level triggered."]
    LEVEL = 0,
    #[doc = "1: Interrupts for this pin are edge triggered."]
    EDGE = 1,
}
impl From<GPIO_INTMODE_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_INTMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GPIO_INTMODE_A {
    type Ux = u32;
}
impl GPIO_INTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_INTMODE_A> {
        match self.bits {
            0 => Some(GPIO_INTMODE_A::LEVEL),
            1 => Some(GPIO_INTMODE_A::EDGE),
            _ => None,
        }
    }
    #[doc = "Interrupts for this pin are level triggered."]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == GPIO_INTMODE_A::LEVEL
    }
    #[doc = "Interrupts for this pin are edge triggered."]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == GPIO_INTMODE_A::EDGE
    }
}
#[doc = "Field `GPIO_INTMODE` writer - Mask of all of the pins on the port."]
pub type GPIO_INTMODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, GPIO_INTMODE_A>;
impl<'a, REG, const O: u8> GPIO_INTMODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Interrupts for this pin are level triggered."]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_INTMODE_A::LEVEL)
    }
    #[doc = "Interrupts for this pin are edge triggered."]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_INTMODE_A::EDGE)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_intmode(&self) -> GPIO_INTMODE_R {
        GPIO_INTMODE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_intmode(&mut self) -> GPIO_INTMODE_W<INTMODE_SPEC, 0> {
        GPIO_INTMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPIO Interrupt Mode Register. Each bit in this register controls the interrupt mode setting for the associated GPIO pin on this port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intmode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intmode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTMODE_SPEC;
impl crate::RegisterSpec for INTMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intmode::R`](R) reader structure"]
impl crate::Readable for INTMODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intmode::W`](W) writer structure"]
impl crate::Writable for INTMODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTMODE to value 0"]
impl crate::Resettable for INTMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
