#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `ADDM7` reader - 7-bit Address Detection/4-bit Address Detection"]
pub type Addm7R = crate::BitReader;
#[doc = "Field `ADDM7` writer - 7-bit Address Detection/4-bit Address Detection"]
pub type Addm7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBDL` reader - LIN break detection length"]
pub type LbdlR = crate::BitReader;
#[doc = "Field `LBDL` writer - LIN break detection length"]
pub type LbdlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBDIE` reader - LIN break detection interrupt enable"]
pub type LbdieR = crate::BitReader;
#[doc = "Field `LBDIE` writer - LIN break detection interrupt enable"]
pub type LbdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBCL` reader - Last bit clock pulse"]
pub type LbclR = crate::BitReader;
#[doc = "Field `LBCL` writer - Last bit clock pulse"]
pub type LbclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPHA` reader - Clock phase"]
pub type CphaR = crate::BitReader;
#[doc = "Field `CPHA` writer - Clock phase"]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - Clock polarity"]
pub type CpolR = crate::BitReader;
#[doc = "Field `CPOL` writer - Clock polarity"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKEN` reader - Clock enable"]
pub type ClkenR = crate::BitReader;
#[doc = "Field `CLKEN` writer - Clock enable"]
pub type ClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - STOP bits"]
pub type StopR = crate::FieldReader;
#[doc = "Field `STOP` writer - STOP bits"]
pub type StopW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LINEN` reader - LIN mode enable"]
pub type LinenR = crate::BitReader;
#[doc = "Field `LINEN` writer - LIN mode enable"]
pub type LinenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAP` reader - Swap TX/RX pins"]
pub type SwapR = crate::BitReader;
#[doc = "Field `SWAP` writer - Swap TX/RX pins"]
pub type SwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXINV` reader - RX pin active level inversion"]
pub type RxinvR = crate::BitReader;
#[doc = "Field `RXINV` writer - RX pin active level inversion"]
pub type RxinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXINV` reader - TX pin active level inversion"]
pub type TxinvR = crate::BitReader;
#[doc = "Field `TXINV` writer - TX pin active level inversion"]
pub type TxinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAINV` reader - Binary data inversion"]
pub type DatainvR = crate::BitReader;
#[doc = "Field `DATAINV` writer - Binary data inversion"]
pub type DatainvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSBFIRST` reader - Most significant bit first"]
pub type MsbfirstR = crate::BitReader;
#[doc = "Field `MSBFIRST` writer - Most significant bit first"]
pub type MsbfirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABREN` reader - Auto baud rate enable"]
pub type AbrenR = crate::BitReader;
#[doc = "Field `ABREN` writer - Auto baud rate enable"]
pub type AbrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABRMOD` reader - Auto baud rate mode"]
pub type AbrmodR = crate::FieldReader;
#[doc = "Field `ABRMOD` writer - Auto baud rate mode"]
pub type AbrmodW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTOEN` reader - Receiver timeout enable"]
pub type RtoenR = crate::BitReader;
#[doc = "Field `RTOEN` writer - Receiver timeout enable"]
pub type RtoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD0` reader - Address of the USART node"]
pub type Add0R = crate::FieldReader;
#[doc = "Field `ADD0` writer - Address of the USART node"]
pub type Add0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADD4` reader - Address of the USART node"]
pub type Add4R = crate::FieldReader;
#[doc = "Field `ADD4` writer - Address of the USART node"]
pub type Add4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection"]
    #[inline(always)]
    pub fn addm7(&self) -> Addm7R {
        Addm7R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LIN break detection length"]
    #[inline(always)]
    pub fn lbdl(&self) -> LbdlR {
        LbdlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    pub fn lbdie(&self) -> LbdieR {
        LbdieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline(always)]
    pub fn lbcl(&self) -> LbclR {
        LbclR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline(always)]
    pub fn clken(&self) -> ClkenR {
        ClkenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    pub fn linen(&self) -> LinenR {
        LinenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Swap TX/RX pins"]
    #[inline(always)]
    pub fn swap(&self) -> SwapR {
        SwapR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RX pin active level inversion"]
    #[inline(always)]
    pub fn rxinv(&self) -> RxinvR {
        RxinvR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TX pin active level inversion"]
    #[inline(always)]
    pub fn txinv(&self) -> TxinvR {
        TxinvR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Binary data inversion"]
    #[inline(always)]
    pub fn datainv(&self) -> DatainvR {
        DatainvR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Most significant bit first"]
    #[inline(always)]
    pub fn msbfirst(&self) -> MsbfirstR {
        MsbfirstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Auto baud rate enable"]
    #[inline(always)]
    pub fn abren(&self) -> AbrenR {
        AbrenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Auto baud rate mode"]
    #[inline(always)]
    pub fn abrmod(&self) -> AbrmodR {
        AbrmodR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Receiver timeout enable"]
    #[inline(always)]
    pub fn rtoen(&self) -> RtoenR {
        RtoenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Address of the USART node"]
    #[inline(always)]
    pub fn add0(&self) -> Add0R {
        Add0R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Address of the USART node"]
    #[inline(always)]
    pub fn add4(&self) -> Add4R {
        Add4R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection"]
    #[inline(always)]
    pub fn addm7(&mut self) -> Addm7W<'_, Cr2Spec> {
        Addm7W::new(self, 4)
    }
    #[doc = "Bit 5 - LIN break detection length"]
    #[inline(always)]
    pub fn lbdl(&mut self) -> LbdlW<'_, Cr2Spec> {
        LbdlW::new(self, 5)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    pub fn lbdie(&mut self) -> LbdieW<'_, Cr2Spec> {
        LbdieW::new(self, 6)
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline(always)]
    pub fn lbcl(&mut self) -> LbclW<'_, Cr2Spec> {
        LbclW::new(self, 8)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CphaW<'_, Cr2Spec> {
        CphaW::new(self, 9)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<'_, Cr2Spec> {
        CpolW::new(self, 10)
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline(always)]
    pub fn clken(&mut self) -> ClkenW<'_, Cr2Spec> {
        ClkenW::new(self, 11)
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, Cr2Spec> {
        StopW::new(self, 12)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    pub fn linen(&mut self) -> LinenW<'_, Cr2Spec> {
        LinenW::new(self, 14)
    }
    #[doc = "Bit 15 - Swap TX/RX pins"]
    #[inline(always)]
    pub fn swap(&mut self) -> SwapW<'_, Cr2Spec> {
        SwapW::new(self, 15)
    }
    #[doc = "Bit 16 - RX pin active level inversion"]
    #[inline(always)]
    pub fn rxinv(&mut self) -> RxinvW<'_, Cr2Spec> {
        RxinvW::new(self, 16)
    }
    #[doc = "Bit 17 - TX pin active level inversion"]
    #[inline(always)]
    pub fn txinv(&mut self) -> TxinvW<'_, Cr2Spec> {
        TxinvW::new(self, 17)
    }
    #[doc = "Bit 18 - Binary data inversion"]
    #[inline(always)]
    pub fn datainv(&mut self) -> DatainvW<'_, Cr2Spec> {
        DatainvW::new(self, 18)
    }
    #[doc = "Bit 19 - Most significant bit first"]
    #[inline(always)]
    pub fn msbfirst(&mut self) -> MsbfirstW<'_, Cr2Spec> {
        MsbfirstW::new(self, 19)
    }
    #[doc = "Bit 20 - Auto baud rate enable"]
    #[inline(always)]
    pub fn abren(&mut self) -> AbrenW<'_, Cr2Spec> {
        AbrenW::new(self, 20)
    }
    #[doc = "Bits 21:22 - Auto baud rate mode"]
    #[inline(always)]
    pub fn abrmod(&mut self) -> AbrmodW<'_, Cr2Spec> {
        AbrmodW::new(self, 21)
    }
    #[doc = "Bit 23 - Receiver timeout enable"]
    #[inline(always)]
    pub fn rtoen(&mut self) -> RtoenW<'_, Cr2Spec> {
        RtoenW::new(self, 23)
    }
    #[doc = "Bits 24:27 - Address of the USART node"]
    #[inline(always)]
    pub fn add0(&mut self) -> Add0W<'_, Cr2Spec> {
        Add0W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Address of the USART node"]
    #[inline(always)]
    pub fn add4(&mut self) -> Add4W<'_, Cr2Spec> {
        Add4W::new(self, 28)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {}
