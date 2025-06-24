#[doc = "Register `OR` reader"]
pub type R = crate::R<OrSpec>;
#[doc = "Register `OR` writer"]
pub type W = crate::W<OrSpec>;
#[doc = "Field `TIM1_ETR_ADC1_RMP` reader - TIM1_ETR_ADC1 remapping capability"]
pub type Tim1EtrAdc1RmpR = crate::FieldReader;
#[doc = "Field `TIM1_ETR_ADC1_RMP` writer - TIM1_ETR_ADC1 remapping capability"]
pub type Tim1EtrAdc1RmpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIM1_ETR_ADC4_RMP` reader - TIM1_ETR_ADC4 remapping capability"]
pub type Tim1EtrAdc4RmpR = crate::FieldReader;
#[doc = "Field `TIM1_ETR_ADC4_RMP` writer - TIM1_ETR_ADC4 remapping capability"]
pub type Tim1EtrAdc4RmpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - TIM1_ETR_ADC1 remapping capability"]
    #[inline(always)]
    pub fn tim1_etr_adc1_rmp(&self) -> Tim1EtrAdc1RmpR {
        Tim1EtrAdc1RmpR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - TIM1_ETR_ADC4 remapping capability"]
    #[inline(always)]
    pub fn tim1_etr_adc4_rmp(&self) -> Tim1EtrAdc4RmpR {
        Tim1EtrAdc4RmpR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - TIM1_ETR_ADC1 remapping capability"]
    #[inline(always)]
    pub fn tim1_etr_adc1_rmp(&mut self) -> Tim1EtrAdc1RmpW<'_, OrSpec> {
        Tim1EtrAdc1RmpW::new(self, 0)
    }
    #[doc = "Bits 2:3 - TIM1_ETR_ADC4 remapping capability"]
    #[inline(always)]
    pub fn tim1_etr_adc4_rmp(&mut self) -> Tim1EtrAdc4RmpW<'_, OrSpec> {
        Tim1EtrAdc4RmpW::new(self, 2)
    }
}
#[doc = "option registers\n\nYou can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OrSpec;
impl crate::RegisterSpec for OrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or::R`](R) reader structure"]
impl crate::Readable for OrSpec {}
#[doc = "`write(|w| ..)` method takes [`or::W`](W) writer structure"]
impl crate::Writable for OrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OR to value 0"]
impl crate::Resettable for OrSpec {}
