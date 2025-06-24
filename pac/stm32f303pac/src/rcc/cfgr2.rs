#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<Cfgr2Spec>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<Cfgr2Spec>;
#[doc = "Field `PREDIV` reader - PREDIV division factor"]
pub type PredivR = crate::FieldReader;
#[doc = "Field `PREDIV` writer - PREDIV division factor"]
pub type PredivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADC12PRES` reader - ADC1 and ADC2 prescaler"]
pub type Adc12presR = crate::FieldReader;
#[doc = "Field `ADC12PRES` writer - ADC1 and ADC2 prescaler"]
pub type Adc12presW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADC34PRES` reader - ADC3 and ADC4 prescaler"]
pub type Adc34presR = crate::FieldReader;
#[doc = "Field `ADC34PRES` writer - ADC3 and ADC4 prescaler"]
pub type Adc34presW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:3 - PREDIV division factor"]
    #[inline(always)]
    pub fn prediv(&self) -> PredivR {
        PredivR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - ADC1 and ADC2 prescaler"]
    #[inline(always)]
    pub fn adc12pres(&self) -> Adc12presR {
        Adc12presR::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:13 - ADC3 and ADC4 prescaler"]
    #[inline(always)]
    pub fn adc34pres(&self) -> Adc34presR {
        Adc34presR::new(((self.bits >> 9) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PREDIV division factor"]
    #[inline(always)]
    pub fn prediv(&mut self) -> PredivW<'_, Cfgr2Spec> {
        PredivW::new(self, 0)
    }
    #[doc = "Bits 4:8 - ADC1 and ADC2 prescaler"]
    #[inline(always)]
    pub fn adc12pres(&mut self) -> Adc12presW<'_, Cfgr2Spec> {
        Adc12presW::new(self, 4)
    }
    #[doc = "Bits 9:13 - ADC3 and ADC4 prescaler"]
    #[inline(always)]
    pub fn adc34pres(&mut self) -> Adc34presW<'_, Cfgr2Spec> {
        Adc34presW::new(self, 9)
    }
}
#[doc = "Clock configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfgr2Spec;
impl crate::RegisterSpec for Cfgr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for Cfgr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for Cfgr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for Cfgr2Spec {}
