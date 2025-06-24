#[doc = "Register `INIT` reader"]
pub type R = crate::R<InitSpec>;
#[doc = "Register `INIT` writer"]
pub type W = crate::W<InitSpec>;
#[doc = "Field `INIT` reader - Programmable initial CRC value"]
pub type InitR = crate::FieldReader<u32>;
#[doc = "Field `INIT` writer - Programmable initial CRC value"]
pub type InitW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Programmable initial CRC value"]
    #[inline(always)]
    pub fn init(&self) -> InitR {
        InitR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Programmable initial CRC value"]
    #[inline(always)]
    pub fn init(&mut self) -> InitW<InitSpec> {
        InitW::new(self, 0)
    }
}
#[doc = "Initial CRC value\n\nYou can [`read`](crate::Reg::read) this register and get [`init::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`init::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InitSpec;
impl crate::RegisterSpec for InitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`init::R`](R) reader structure"]
impl crate::Readable for InitSpec {}
#[doc = "`write(|w| ..)` method takes [`init::W`](W) writer structure"]
impl crate::Writable for InitSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INIT to value 0xffff_ffff"]
impl crate::Resettable for InitSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
