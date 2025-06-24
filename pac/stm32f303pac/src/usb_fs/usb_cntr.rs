#[doc = "Register `USB_CNTR` reader"]
pub type R = crate::R<UsbCntrSpec>;
#[doc = "Register `USB_CNTR` writer"]
pub type W = crate::W<UsbCntrSpec>;
#[doc = "Field `FRES` reader - Force USB Reset"]
pub type FresR = crate::BitReader;
#[doc = "Field `FRES` writer - Force USB Reset"]
pub type FresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDWN` reader - Power down"]
pub type PdwnR = crate::BitReader;
#[doc = "Field `PDWN` writer - Power down"]
pub type PdwnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMODE` reader - Low-power mode"]
pub type LpmodeR = crate::BitReader;
#[doc = "Field `LPMODE` writer - Low-power mode"]
pub type LpmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSUSP` reader - Force suspend"]
pub type FsuspR = crate::BitReader;
#[doc = "Field `FSUSP` writer - Force suspend"]
pub type FsuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESUME` reader - Resume request"]
pub type ResumeR = crate::BitReader;
#[doc = "Field `RESUME` writer - Resume request"]
pub type ResumeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESOFM` reader - Expected start of frame interrupt mask"]
pub type EsofmR = crate::BitReader;
#[doc = "Field `ESOFM` writer - Expected start of frame interrupt mask"]
pub type EsofmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFM` reader - Start of frame interrupt mask"]
pub type SofmR = crate::BitReader;
#[doc = "Field `SOFM` writer - Start of frame interrupt mask"]
pub type SofmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETM` reader - USB reset interrupt mask"]
pub type ResetmR = crate::BitReader;
#[doc = "Field `RESETM` writer - USB reset interrupt mask"]
pub type ResetmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSPM` reader - Suspend mode interrupt mask"]
pub type SuspmR = crate::BitReader;
#[doc = "Field `SUSPM` writer - Suspend mode interrupt mask"]
pub type SuspmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPM` reader - Wakeup interrupt mask"]
pub type WkupmR = crate::BitReader;
#[doc = "Field `WKUPM` writer - Wakeup interrupt mask"]
pub type WkupmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRM` reader - Error interrupt mask"]
pub type ErrmR = crate::BitReader;
#[doc = "Field `ERRM` writer - Error interrupt mask"]
pub type ErrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMAOVRM` reader - Packet memory area over / underrun interrupt mask"]
pub type PmaovrmR = crate::BitReader;
#[doc = "Field `PMAOVRM` writer - Packet memory area over / underrun interrupt mask"]
pub type PmaovrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRM` reader - Correct transfer interrupt mask"]
pub type CtrmR = crate::BitReader;
#[doc = "Field `CTRM` writer - Correct transfer interrupt mask"]
pub type CtrmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Force USB Reset"]
    #[inline(always)]
    pub fn fres(&self) -> FresR {
        FresR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power down"]
    #[inline(always)]
    pub fn pdwn(&self) -> PdwnR {
        PdwnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low-power mode"]
    #[inline(always)]
    pub fn lpmode(&self) -> LpmodeR {
        LpmodeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force suspend"]
    #[inline(always)]
    pub fn fsusp(&self) -> FsuspR {
        FsuspR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Resume request"]
    #[inline(always)]
    pub fn resume(&self) -> ResumeR {
        ResumeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Expected start of frame interrupt mask"]
    #[inline(always)]
    pub fn esofm(&self) -> EsofmR {
        EsofmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start of frame interrupt mask"]
    #[inline(always)]
    pub fn sofm(&self) -> SofmR {
        SofmR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USB reset interrupt mask"]
    #[inline(always)]
    pub fn resetm(&self) -> ResetmR {
        ResetmR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Suspend mode interrupt mask"]
    #[inline(always)]
    pub fn suspm(&self) -> SuspmR {
        SuspmR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wakeup interrupt mask"]
    #[inline(always)]
    pub fn wkupm(&self) -> WkupmR {
        WkupmR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Error interrupt mask"]
    #[inline(always)]
    pub fn errm(&self) -> ErrmR {
        ErrmR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun interrupt mask"]
    #[inline(always)]
    pub fn pmaovrm(&self) -> PmaovrmR {
        PmaovrmR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Correct transfer interrupt mask"]
    #[inline(always)]
    pub fn ctrm(&self) -> CtrmR {
        CtrmR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force USB Reset"]
    #[inline(always)]
    pub fn fres(&mut self) -> FresW<'_, UsbCntrSpec> {
        FresW::new(self, 0)
    }
    #[doc = "Bit 1 - Power down"]
    #[inline(always)]
    pub fn pdwn(&mut self) -> PdwnW<'_, UsbCntrSpec> {
        PdwnW::new(self, 1)
    }
    #[doc = "Bit 2 - Low-power mode"]
    #[inline(always)]
    pub fn lpmode(&mut self) -> LpmodeW<'_, UsbCntrSpec> {
        LpmodeW::new(self, 2)
    }
    #[doc = "Bit 3 - Force suspend"]
    #[inline(always)]
    pub fn fsusp(&mut self) -> FsuspW<'_, UsbCntrSpec> {
        FsuspW::new(self, 3)
    }
    #[doc = "Bit 4 - Resume request"]
    #[inline(always)]
    pub fn resume(&mut self) -> ResumeW<'_, UsbCntrSpec> {
        ResumeW::new(self, 4)
    }
    #[doc = "Bit 8 - Expected start of frame interrupt mask"]
    #[inline(always)]
    pub fn esofm(&mut self) -> EsofmW<'_, UsbCntrSpec> {
        EsofmW::new(self, 8)
    }
    #[doc = "Bit 9 - Start of frame interrupt mask"]
    #[inline(always)]
    pub fn sofm(&mut self) -> SofmW<'_, UsbCntrSpec> {
        SofmW::new(self, 9)
    }
    #[doc = "Bit 10 - USB reset interrupt mask"]
    #[inline(always)]
    pub fn resetm(&mut self) -> ResetmW<'_, UsbCntrSpec> {
        ResetmW::new(self, 10)
    }
    #[doc = "Bit 11 - Suspend mode interrupt mask"]
    #[inline(always)]
    pub fn suspm(&mut self) -> SuspmW<'_, UsbCntrSpec> {
        SuspmW::new(self, 11)
    }
    #[doc = "Bit 12 - Wakeup interrupt mask"]
    #[inline(always)]
    pub fn wkupm(&mut self) -> WkupmW<'_, UsbCntrSpec> {
        WkupmW::new(self, 12)
    }
    #[doc = "Bit 13 - Error interrupt mask"]
    #[inline(always)]
    pub fn errm(&mut self) -> ErrmW<'_, UsbCntrSpec> {
        ErrmW::new(self, 13)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun interrupt mask"]
    #[inline(always)]
    pub fn pmaovrm(&mut self) -> PmaovrmW<'_, UsbCntrSpec> {
        PmaovrmW::new(self, 14)
    }
    #[doc = "Bit 15 - Correct transfer interrupt mask"]
    #[inline(always)]
    pub fn ctrm(&mut self) -> CtrmW<'_, UsbCntrSpec> {
        CtrmW::new(self, 15)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_cntr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_cntr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbCntrSpec;
impl crate::RegisterSpec for UsbCntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_cntr::R`](R) reader structure"]
impl crate::Readable for UsbCntrSpec {}
#[doc = "`write(|w| ..)` method takes [`usb_cntr::W`](W) writer structure"]
impl crate::Writable for UsbCntrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USB_CNTR to value 0x03"]
impl crate::Resettable for UsbCntrSpec {
    const RESET_VALUE: u32 = 0x03;
}
