#[doc = "Register `CFGR3` reader"]
pub type R = crate::R<Cfgr3Spec>;
#[doc = "Register `CFGR3` writer"]
pub type W = crate::W<Cfgr3Spec>;
#[doc = "Field `USART1SW` reader - USART1 clock source selection"]
pub type Usart1swR = crate::FieldReader;
#[doc = "Field `USART1SW` writer - USART1 clock source selection"]
pub type Usart1swW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2C1SW` reader - I2C1 clock source selection"]
pub type I2c1swR = crate::BitReader;
#[doc = "Field `I2C1SW` writer - I2C1 clock source selection"]
pub type I2c1swW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2SW` reader - I2C2 clock source selection"]
pub type I2c2swR = crate::BitReader;
#[doc = "Field `I2C2SW` writer - I2C2 clock source selection"]
pub type I2c2swW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3SW` reader - I2C3 clock source selection"]
pub type I2c3swR = crate::BitReader;
#[doc = "Field `I2C3SW` writer - I2C3 clock source selection"]
pub type I2c3swW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM1SW` reader - Timer1 clock source selection"]
pub type Tim1swR = crate::BitReader;
#[doc = "Field `TIM1SW` writer - Timer1 clock source selection"]
pub type Tim1swW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM8SW` reader - Timer8 clock source selection"]
pub type Tim8swR = crate::BitReader;
#[doc = "Field `TIM8SW` writer - Timer8 clock source selection"]
pub type Tim8swW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2SW` reader - USART2 clock source selection"]
pub type Usart2swR = crate::FieldReader;
#[doc = "Field `USART2SW` writer - USART2 clock source selection"]
pub type Usart2swW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USART3SW` reader - USART3 clock source selection"]
pub type Usart3swR = crate::FieldReader;
#[doc = "Field `USART3SW` writer - USART3 clock source selection"]
pub type Usart3swW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UART4SW` reader - UART4 clock source selection"]
pub type Uart4swR = crate::FieldReader;
#[doc = "Field `UART4SW` writer - UART4 clock source selection"]
pub type Uart4swW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UART5SW` reader - UART5 clock source selection"]
pub type Uart5swR = crate::FieldReader;
#[doc = "Field `UART5SW` writer - UART5 clock source selection"]
pub type Uart5swW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    pub fn usart1sw(&self) -> Usart1swR {
        Usart1swR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - I2C1 clock source selection"]
    #[inline(always)]
    pub fn i2c1sw(&self) -> I2c1swR {
        I2c1swR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C2 clock source selection"]
    #[inline(always)]
    pub fn i2c2sw(&self) -> I2c2swR {
        I2c2swR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C3 clock source selection"]
    #[inline(always)]
    pub fn i2c3sw(&self) -> I2c3swR {
        I2c3swR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Timer1 clock source selection"]
    #[inline(always)]
    pub fn tim1sw(&self) -> Tim1swR {
        Tim1swR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timer8 clock source selection"]
    #[inline(always)]
    pub fn tim8sw(&self) -> Tim8swR {
        Tim8swR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:17 - USART2 clock source selection"]
    #[inline(always)]
    pub fn usart2sw(&self) -> Usart2swR {
        Usart2swR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - USART3 clock source selection"]
    #[inline(always)]
    pub fn usart3sw(&self) -> Usart3swR {
        Usart3swR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - UART4 clock source selection"]
    #[inline(always)]
    pub fn uart4sw(&self) -> Uart4swR {
        Uart4swR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - UART5 clock source selection"]
    #[inline(always)]
    pub fn uart5sw(&self) -> Uart5swR {
        Uart5swR::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    pub fn usart1sw(&mut self) -> Usart1swW<'_, Cfgr3Spec> {
        Usart1swW::new(self, 0)
    }
    #[doc = "Bit 4 - I2C1 clock source selection"]
    #[inline(always)]
    pub fn i2c1sw(&mut self) -> I2c1swW<'_, Cfgr3Spec> {
        I2c1swW::new(self, 4)
    }
    #[doc = "Bit 5 - I2C2 clock source selection"]
    #[inline(always)]
    pub fn i2c2sw(&mut self) -> I2c2swW<'_, Cfgr3Spec> {
        I2c2swW::new(self, 5)
    }
    #[doc = "Bit 6 - I2C3 clock source selection"]
    #[inline(always)]
    pub fn i2c3sw(&mut self) -> I2c3swW<'_, Cfgr3Spec> {
        I2c3swW::new(self, 6)
    }
    #[doc = "Bit 8 - Timer1 clock source selection"]
    #[inline(always)]
    pub fn tim1sw(&mut self) -> Tim1swW<'_, Cfgr3Spec> {
        Tim1swW::new(self, 8)
    }
    #[doc = "Bit 9 - Timer8 clock source selection"]
    #[inline(always)]
    pub fn tim8sw(&mut self) -> Tim8swW<'_, Cfgr3Spec> {
        Tim8swW::new(self, 9)
    }
    #[doc = "Bits 16:17 - USART2 clock source selection"]
    #[inline(always)]
    pub fn usart2sw(&mut self) -> Usart2swW<'_, Cfgr3Spec> {
        Usart2swW::new(self, 16)
    }
    #[doc = "Bits 18:19 - USART3 clock source selection"]
    #[inline(always)]
    pub fn usart3sw(&mut self) -> Usart3swW<'_, Cfgr3Spec> {
        Usart3swW::new(self, 18)
    }
    #[doc = "Bits 20:21 - UART4 clock source selection"]
    #[inline(always)]
    pub fn uart4sw(&mut self) -> Uart4swW<'_, Cfgr3Spec> {
        Uart4swW::new(self, 20)
    }
    #[doc = "Bits 22:23 - UART5 clock source selection"]
    #[inline(always)]
    pub fn uart5sw(&mut self) -> Uart5swW<'_, Cfgr3Spec> {
        Uart5swW::new(self, 22)
    }
}
#[doc = "Clock configuration register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfgr3Spec;
impl crate::RegisterSpec for Cfgr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr3::R`](R) reader structure"]
impl crate::Readable for Cfgr3Spec {}
#[doc = "`write(|w| ..)` method takes [`cfgr3::W`](W) writer structure"]
impl crate::Writable for Cfgr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR3 to value 0"]
impl crate::Resettable for Cfgr3Spec {}
