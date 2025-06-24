#[doc = "Register `APB2ENR` reader"]
pub type R = crate::R<Apb2enrSpec>;
#[doc = "Register `APB2ENR` writer"]
pub type W = crate::W<Apb2enrSpec>;
#[doc = "Field `SYSCFGEN` reader - SYSCFG clock enable"]
pub type SyscfgenR = crate::BitReader;
#[doc = "Field `SYSCFGEN` writer - SYSCFG clock enable"]
pub type SyscfgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM1EN` reader - TIM1 Timer clock enable"]
pub type Tim1enR = crate::BitReader;
#[doc = "Field `TIM1EN` writer - TIM1 Timer clock enable"]
pub type Tim1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1EN` reader - SPI 1 clock enable"]
pub type Spi1enR = crate::BitReader;
#[doc = "Field `SPI1EN` writer - SPI 1 clock enable"]
pub type Spi1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM8EN` reader - TIM8 Timer clock enable"]
pub type Tim8enR = crate::BitReader;
#[doc = "Field `TIM8EN` writer - TIM8 Timer clock enable"]
pub type Tim8enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1EN` reader - USART1 clock enable"]
pub type Usart1enR = crate::BitReader;
#[doc = "Field `USART1EN` writer - USART1 clock enable"]
pub type Usart1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM15EN` reader - TIM15 timer clock enable"]
pub type Tim15enR = crate::BitReader;
#[doc = "Field `TIM15EN` writer - TIM15 timer clock enable"]
pub type Tim15enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16EN` reader - TIM16 timer clock enable"]
pub type Tim16enR = crate::BitReader;
#[doc = "Field `TIM16EN` writer - TIM16 timer clock enable"]
pub type Tim16enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17EN` reader - TIM17 timer clock enable"]
pub type Tim17enR = crate::BitReader;
#[doc = "Field `TIM17EN` writer - TIM17 timer clock enable"]
pub type Tim17enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SYSCFG clock enable"]
    #[inline(always)]
    pub fn syscfgen(&self) -> SyscfgenR {
        SyscfgenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 Timer clock enable"]
    #[inline(always)]
    pub fn tim1en(&self) -> Tim1enR {
        Tim1enR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI 1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> Spi1enR {
        Spi1enR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 Timer clock enable"]
    #[inline(always)]
    pub fn tim8en(&self) -> Tim8enR {
        Tim8enR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> Usart1enR {
        Usart1enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 timer clock enable"]
    #[inline(always)]
    pub fn tim15en(&self) -> Tim15enR {
        Tim15enR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable"]
    #[inline(always)]
    pub fn tim16en(&self) -> Tim16enR {
        Tim16enR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 timer clock enable"]
    #[inline(always)]
    pub fn tim17en(&self) -> Tim17enR {
        Tim17enR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG clock enable"]
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SyscfgenW<Apb2enrSpec> {
        SyscfgenW::new(self, 0)
    }
    #[doc = "Bit 11 - TIM1 Timer clock enable"]
    #[inline(always)]
    pub fn tim1en(&mut self) -> Tim1enW<Apb2enrSpec> {
        Tim1enW::new(self, 11)
    }
    #[doc = "Bit 12 - SPI 1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&mut self) -> Spi1enW<Apb2enrSpec> {
        Spi1enW::new(self, 12)
    }
    #[doc = "Bit 13 - TIM8 Timer clock enable"]
    #[inline(always)]
    pub fn tim8en(&mut self) -> Tim8enW<Apb2enrSpec> {
        Tim8enW::new(self, 13)
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&mut self) -> Usart1enW<Apb2enrSpec> {
        Usart1enW::new(self, 14)
    }
    #[doc = "Bit 16 - TIM15 timer clock enable"]
    #[inline(always)]
    pub fn tim15en(&mut self) -> Tim15enW<Apb2enrSpec> {
        Tim15enW::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable"]
    #[inline(always)]
    pub fn tim16en(&mut self) -> Tim16enW<Apb2enrSpec> {
        Tim16enW::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 timer clock enable"]
    #[inline(always)]
    pub fn tim17en(&mut self) -> Tim17enW<Apb2enrSpec> {
        Tim17enW::new(self, 18)
    }
}
#[doc = "APB2 peripheral clock enable register (RCC_APB2ENR)\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2enrSpec;
impl crate::RegisterSpec for Apb2enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2enr::R`](R) reader structure"]
impl crate::Readable for Apb2enrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb2enr::W`](W) writer structure"]
impl crate::Writable for Apb2enrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB2ENR to value 0"]
impl crate::Resettable for Apb2enrSpec {}
