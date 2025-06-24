#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CfgrSpec>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CfgrSpec>;
#[doc = "Field `SW` reader - System clock Switch"]
pub type SwR = crate::FieldReader;
#[doc = "Field `SW` writer - System clock Switch"]
pub type SwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SWS` reader - System Clock Switch Status"]
pub type SwsR = crate::FieldReader;
#[doc = "Field `HPRE` reader - AHB prescaler"]
pub type HpreR = crate::FieldReader;
#[doc = "Field `HPRE` writer - AHB prescaler"]
pub type HpreW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PPRE1` reader - APB Low speed prescaler (APB1)"]
pub type Ppre1R = crate::FieldReader;
#[doc = "Field `PPRE1` writer - APB Low speed prescaler (APB1)"]
pub type Ppre1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PPRE2` reader - APB high speed prescaler (APB2)"]
pub type Ppre2R = crate::FieldReader;
#[doc = "Field `PPRE2` writer - APB high speed prescaler (APB2)"]
pub type Ppre2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PLLSRC` reader - PLL entry clock source"]
pub type PllsrcR = crate::FieldReader;
#[doc = "Field `PLLSRC` writer - PLL entry clock source"]
pub type PllsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLLXTPRE` reader - HSE divider for PLL entry"]
pub type PllxtpreR = crate::BitReader;
#[doc = "Field `PLLXTPRE` writer - HSE divider for PLL entry"]
pub type PllxtpreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLMUL` reader - PLL Multiplication Factor"]
pub type PllmulR = crate::FieldReader;
#[doc = "Field `PLLMUL` writer - PLL Multiplication Factor"]
pub type PllmulW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `USBPRES` reader - USB prescaler"]
pub type UsbpresR = crate::BitReader;
#[doc = "Field `USBPRES` writer - USB prescaler"]
pub type UsbpresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SSRC` reader - I2S external clock source selection"]
pub type I2ssrcR = crate::BitReader;
#[doc = "Field `I2SSRC` writer - I2S external clock source selection"]
pub type I2ssrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCO` reader - Microcontroller clock output"]
pub type McoR = crate::FieldReader;
#[doc = "Field `MCO` writer - Microcontroller clock output"]
pub type McoW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MCOF` reader - Microcontroller Clock Output Flag"]
pub type McofR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - System clock Switch"]
    #[inline(always)]
    pub fn sw(&self) -> SwR {
        SwR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - System Clock Switch Status"]
    #[inline(always)]
    pub fn sws(&self) -> SwsR {
        SwsR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&self) -> HpreR {
        HpreR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - APB Low speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre1(&self) -> Ppre1R {
        Ppre1R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - APB high speed prescaler (APB2)"]
    #[inline(always)]
    pub fn ppre2(&self) -> Ppre2R {
        Ppre2R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 15:16 - PLL entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PllsrcR {
        PllsrcR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - HSE divider for PLL entry"]
    #[inline(always)]
    pub fn pllxtpre(&self) -> PllxtpreR {
        PllxtpreR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - PLL Multiplication Factor"]
    #[inline(always)]
    pub fn pllmul(&self) -> PllmulR {
        PllmulR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - USB prescaler"]
    #[inline(always)]
    pub fn usbpres(&self) -> UsbpresR {
        UsbpresR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2S external clock source selection"]
    #[inline(always)]
    pub fn i2ssrc(&self) -> I2ssrcR {
        I2ssrcR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Microcontroller clock output"]
    #[inline(always)]
    pub fn mco(&self) -> McoR {
        McoR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 28 - Microcontroller Clock Output Flag"]
    #[inline(always)]
    pub fn mcof(&self) -> McofR {
        McofR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock Switch"]
    #[inline(always)]
    pub fn sw(&mut self) -> SwW<'_, CfgrSpec> {
        SwW::new(self, 0)
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&mut self) -> HpreW<'_, CfgrSpec> {
        HpreW::new(self, 4)
    }
    #[doc = "Bits 8:10 - APB Low speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre1(&mut self) -> Ppre1W<'_, CfgrSpec> {
        Ppre1W::new(self, 8)
    }
    #[doc = "Bits 11:13 - APB high speed prescaler (APB2)"]
    #[inline(always)]
    pub fn ppre2(&mut self) -> Ppre2W<'_, CfgrSpec> {
        Ppre2W::new(self, 11)
    }
    #[doc = "Bits 15:16 - PLL entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PllsrcW<'_, CfgrSpec> {
        PllsrcW::new(self, 15)
    }
    #[doc = "Bit 17 - HSE divider for PLL entry"]
    #[inline(always)]
    pub fn pllxtpre(&mut self) -> PllxtpreW<'_, CfgrSpec> {
        PllxtpreW::new(self, 17)
    }
    #[doc = "Bits 18:21 - PLL Multiplication Factor"]
    #[inline(always)]
    pub fn pllmul(&mut self) -> PllmulW<'_, CfgrSpec> {
        PllmulW::new(self, 18)
    }
    #[doc = "Bit 22 - USB prescaler"]
    #[inline(always)]
    pub fn usbpres(&mut self) -> UsbpresW<'_, CfgrSpec> {
        UsbpresW::new(self, 22)
    }
    #[doc = "Bit 23 - I2S external clock source selection"]
    #[inline(always)]
    pub fn i2ssrc(&mut self) -> I2ssrcW<'_, CfgrSpec> {
        I2ssrcW::new(self, 23)
    }
    #[doc = "Bits 24:26 - Microcontroller clock output"]
    #[inline(always)]
    pub fn mco(&mut self) -> McoW<'_, CfgrSpec> {
        McoW::new(self, 24)
    }
}
#[doc = "Clock configuration register (RCC_CFGR)\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgrSpec;
impl crate::RegisterSpec for CfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CfgrSpec {}
