#[doc = "Register `BKP3R` reader"]
pub type R = crate::R<Bkp3rSpec>;
#[doc = "Register `BKP3R` writer"]
pub type W = crate::W<Bkp3rSpec>;
#[doc = "Field `BKP` reader - BKP"]
pub type BkpR = crate::FieldReader<u32>;
#[doc = "Field `BKP` writer - BKP"]
pub type BkpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - BKP"]
    #[inline(always)]
    pub fn bkp(&self) -> BkpR {
        BkpR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - BKP"]
    #[inline(always)]
    pub fn bkp(&mut self) -> BkpW<'_, Bkp3rSpec> {
        BkpW::new(self, 0)
    }
}
#[doc = "backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp3r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp3r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bkp3rSpec;
impl crate::RegisterSpec for Bkp3rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkp3r::R`](R) reader structure"]
impl crate::Readable for Bkp3rSpec {}
#[doc = "`write(|w| ..)` method takes [`bkp3r::W`](W) writer structure"]
impl crate::Writable for Bkp3rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BKP3R to value 0"]
impl crate::Resettable for Bkp3rSpec {}
