#[doc = "Register `PMEM4` reader"]
pub type R = crate::R<Pmem4Spec>;
#[doc = "Register `PMEM4` writer"]
pub type W = crate::W<Pmem4Spec>;
#[doc = "Field `MEMSETx` reader - MEMSETx"]
pub type MemsetxR = crate::FieldReader;
#[doc = "Field `MEMSETx` writer - MEMSETx"]
pub type MemsetxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEMWAITx` reader - MEMWAITx"]
pub type MemwaitxR = crate::FieldReader;
#[doc = "Field `MEMWAITx` writer - MEMWAITx"]
pub type MemwaitxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEMHOLDx` reader - MEMHOLDx"]
pub type MemholdxR = crate::FieldReader;
#[doc = "Field `MEMHOLDx` writer - MEMHOLDx"]
pub type MemholdxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEMHIZx` reader - MEMHIZx"]
pub type MemhizxR = crate::FieldReader;
#[doc = "Field `MEMHIZx` writer - MEMHIZx"]
pub type MemhizxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - MEMSETx"]
    #[inline(always)]
    pub fn memsetx(&self) -> MemsetxR {
        MemsetxR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - MEMWAITx"]
    #[inline(always)]
    pub fn memwaitx(&self) -> MemwaitxR {
        MemwaitxR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - MEMHOLDx"]
    #[inline(always)]
    pub fn memholdx(&self) -> MemholdxR {
        MemholdxR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - MEMHIZx"]
    #[inline(always)]
    pub fn memhizx(&self) -> MemhizxR {
        MemhizxR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - MEMSETx"]
    #[inline(always)]
    pub fn memsetx(&mut self) -> MemsetxW<'_, Pmem4Spec> {
        MemsetxW::new(self, 0)
    }
    #[doc = "Bits 8:15 - MEMWAITx"]
    #[inline(always)]
    pub fn memwaitx(&mut self) -> MemwaitxW<'_, Pmem4Spec> {
        MemwaitxW::new(self, 8)
    }
    #[doc = "Bits 16:23 - MEMHOLDx"]
    #[inline(always)]
    pub fn memholdx(&mut self) -> MemholdxW<'_, Pmem4Spec> {
        MemholdxW::new(self, 16)
    }
    #[doc = "Bits 24:31 - MEMHIZx"]
    #[inline(always)]
    pub fn memhizx(&mut self) -> MemhizxW<'_, Pmem4Spec> {
        MemhizxW::new(self, 24)
    }
}
#[doc = "Common memory space timing register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`pmem4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmem4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pmem4Spec;
impl crate::RegisterSpec for Pmem4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmem4::R`](R) reader structure"]
impl crate::Readable for Pmem4Spec {}
#[doc = "`write(|w| ..)` method takes [`pmem4::W`](W) writer structure"]
impl crate::Writable for Pmem4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PMEM4 to value 0xfcfc_fcfc"]
impl crate::Resettable for Pmem4Spec {
    const RESET_VALUE: u32 = 0xfcfc_fcfc;
}
