#[doc = "Register `OR` reader"]
pub type R = crate::R<OrSpec>;
#[doc = "Register `OR` writer"]
pub type W = crate::W<OrSpec>;
#[doc = "Field `TIM8_ETR_ADC2_RMP` reader - TIM8_ETR_ADC2 remapping capability"]
pub type Tim8EtrAdc2RmpR = crate::FieldReader;
#[doc = "Field `TIM8_ETR_ADC2_RMP` writer - TIM8_ETR_ADC2 remapping capability"]
pub type Tim8EtrAdc2RmpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIM8_ETR_ADC3_RMP` reader - TIM8_ETR_ADC3 remapping capability"]
pub type Tim8EtrAdc3RmpR = crate::FieldReader;
#[doc = "Field `TIM8_ETR_ADC3_RMP` writer - TIM8_ETR_ADC3 remapping capability"]
pub type Tim8EtrAdc3RmpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - TIM8_ETR_ADC2 remapping capability"]
    #[inline(always)]
    pub fn tim8_etr_adc2_rmp(&self) -> Tim8EtrAdc2RmpR {
        Tim8EtrAdc2RmpR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - TIM8_ETR_ADC3 remapping capability"]
    #[inline(always)]
    pub fn tim8_etr_adc3_rmp(&self) -> Tim8EtrAdc3RmpR {
        Tim8EtrAdc3RmpR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - TIM8_ETR_ADC2 remapping capability"]
    #[inline(always)]
    pub fn tim8_etr_adc2_rmp(&mut self) -> Tim8EtrAdc2RmpW<'_, OrSpec> {
        Tim8EtrAdc2RmpW::new(self, 0)
    }
    #[doc = "Bits 2:3 - TIM8_ETR_ADC3 remapping capability"]
    #[inline(always)]
    pub fn tim8_etr_adc3_rmp(&mut self) -> Tim8EtrAdc3RmpW<'_, OrSpec> {
        Tim8EtrAdc3RmpW::new(self, 2)
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
