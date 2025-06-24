#[doc = "Register `SYSCFG_RCR` reader"]
pub type R = crate::R<SyscfgRcrSpec>;
#[doc = "Register `SYSCFG_RCR` writer"]
pub type W = crate::W<SyscfgRcrSpec>;
#[doc = "Field `PAGE0_WP` reader - CCM SRAM page write protection bit"]
pub type Page0WpR = crate::BitReader;
#[doc = "Field `PAGE0_WP` writer - CCM SRAM page write protection bit"]
pub type Page0WpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAGE1_WP` reader - CCM SRAM page write protection bit"]
pub type Page1WpR = crate::BitReader;
#[doc = "Field `PAGE1_WP` writer - CCM SRAM page write protection bit"]
pub type Page1WpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAGE2_WP` reader - CCM SRAM page write protection bit"]
pub type Page2WpR = crate::BitReader;
#[doc = "Field `PAGE2_WP` writer - CCM SRAM page write protection bit"]
pub type Page2WpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAGE3_WP` reader - CCM SRAM page write protection bit"]
pub type Page3WpR = crate::BitReader;
#[doc = "Field `PAGE3_WP` writer - CCM SRAM page write protection bit"]
pub type Page3WpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAGE4_WP` reader - CCM SRAM page write protection bit"]
pub type Page4WpR = crate::BitReader;
#[doc = "Field `PAGE4_WP` writer - CCM SRAM page write protection bit"]
pub type Page4WpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAGE5_WP` reader - CCM SRAM page write protection bit"]
pub type Page5WpR = crate::BitReader;
#[doc = "Field `PAGE5_WP` writer - CCM SRAM page write protection bit"]
pub type Page5WpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAGE6_WP` reader - CCM SRAM page write protection bit"]
pub type Page6WpR = crate::BitReader;
#[doc = "Field `PAGE6_WP` writer - CCM SRAM page write protection bit"]
pub type Page6WpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAGE7_WP` reader - CCM SRAM page write protection bit"]
pub type Page7WpR = crate::BitReader;
#[doc = "Field `PAGE7_WP` writer - CCM SRAM page write protection bit"]
pub type Page7WpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page0_wp(&self) -> Page0WpR {
        Page0WpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page1_wp(&self) -> Page1WpR {
        Page1WpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page2_wp(&self) -> Page2WpR {
        Page2WpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page3_wp(&self) -> Page3WpR {
        Page3WpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page4_wp(&self) -> Page4WpR {
        Page4WpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page5_wp(&self) -> Page5WpR {
        Page5WpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page6_wp(&self) -> Page6WpR {
        Page6WpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page7_wp(&self) -> Page7WpR {
        Page7WpR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page0_wp(&mut self) -> Page0WpW<'_, SyscfgRcrSpec> {
        Page0WpW::new(self, 0)
    }
    #[doc = "Bit 1 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page1_wp(&mut self) -> Page1WpW<'_, SyscfgRcrSpec> {
        Page1WpW::new(self, 1)
    }
    #[doc = "Bit 2 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page2_wp(&mut self) -> Page2WpW<'_, SyscfgRcrSpec> {
        Page2WpW::new(self, 2)
    }
    #[doc = "Bit 3 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page3_wp(&mut self) -> Page3WpW<'_, SyscfgRcrSpec> {
        Page3WpW::new(self, 3)
    }
    #[doc = "Bit 4 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page4_wp(&mut self) -> Page4WpW<'_, SyscfgRcrSpec> {
        Page4WpW::new(self, 4)
    }
    #[doc = "Bit 5 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page5_wp(&mut self) -> Page5WpW<'_, SyscfgRcrSpec> {
        Page5WpW::new(self, 5)
    }
    #[doc = "Bit 6 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page6_wp(&mut self) -> Page6WpW<'_, SyscfgRcrSpec> {
        Page6WpW::new(self, 6)
    }
    #[doc = "Bit 7 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page7_wp(&mut self) -> Page7WpW<'_, SyscfgRcrSpec> {
        Page7WpW::new(self, 7)
    }
}
#[doc = "CCM SRAM protection register\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg_rcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_rcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgRcrSpec;
impl crate::RegisterSpec for SyscfgRcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_rcr::R`](R) reader structure"]
impl crate::Readable for SyscfgRcrSpec {}
#[doc = "`write(|w| ..)` method takes [`syscfg_rcr::W`](W) writer structure"]
impl crate::Writable for SyscfgRcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCFG_RCR to value 0"]
impl crate::Resettable for SyscfgRcrSpec {}
