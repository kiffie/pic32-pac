#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "0 - CORE_TIMER"]
    CORE_TIMER = 0,
    #[doc = "1 - CORE_SOFTWARE_0"]
    CORE_SOFTWARE_0 = 1,
    #[doc = "2 - CORE_SOFTWARE_1"]
    CORE_SOFTWARE_1 = 2,
    #[doc = "3 - EXTERNAL_0"]
    EXTERNAL_0 = 3,
    #[doc = "4 - TIMER_1"]
    TIMER_1 = 4,
    #[doc = "5 - INPUT_CAPTURE_1"]
    INPUT_CAPTURE_1 = 5,
    #[doc = "6 - OUTPUT_COMPARE_1"]
    OUTPUT_COMPARE_1 = 6,
    #[doc = "7 - EXTERNAL_1"]
    EXTERNAL_1 = 7,
    #[doc = "8 - TIMER_2"]
    TIMER_2 = 8,
    #[doc = "9 - INPUT_CAPTURE_2"]
    INPUT_CAPTURE_2 = 9,
    #[doc = "10 - OUTPUT_COMPARE_2"]
    OUTPUT_COMPARE_2 = 10,
    #[doc = "11 - EXTERNAL_2"]
    EXTERNAL_2 = 11,
    #[doc = "12 - TIMER_3"]
    TIMER_3 = 12,
    #[doc = "13 - INPUT_CAPTURE_3"]
    INPUT_CAPTURE_3 = 13,
    #[doc = "14 - OUTPUT_COMPARE_3"]
    OUTPUT_COMPARE_3 = 14,
    #[doc = "15 - EXTERNAL_3"]
    EXTERNAL_3 = 15,
    #[doc = "16 - TIMER_4"]
    TIMER_4 = 16,
    #[doc = "17 - INPUT_CAPTURE_4"]
    INPUT_CAPTURE_4 = 17,
    #[doc = "18 - OUTPUT_COMPARE_4"]
    OUTPUT_COMPARE_4 = 18,
    #[doc = "19 - EXTERNAL_4"]
    EXTERNAL_4 = 19,
    #[doc = "20 - TIMER_5"]
    TIMER_5 = 20,
    #[doc = "21 - INPUT_CAPTURE_5"]
    INPUT_CAPTURE_5 = 21,
    #[doc = "22 - OUTPUT_COMPARE_5"]
    OUTPUT_COMPARE_5 = 22,
    #[doc = "23 - ADC"]
    ADC = 23,
    #[doc = "24 - FAIL_SAFE_MONITOR"]
    FAIL_SAFE_MONITOR = 24,
    #[doc = "25 - RTCC"]
    RTCC = 25,
    #[doc = "26 - FCE"]
    FCE = 26,
    #[doc = "27 - COMPARATOR_1"]
    COMPARATOR_1 = 27,
    #[doc = "28 - COMPARATOR_2"]
    COMPARATOR_2 = 28,
    #[doc = "29 - COMPARATOR_3"]
    COMPARATOR_3 = 29,
    #[doc = "31 - SPI_1"]
    SPI_1 = 31,
    #[doc = "32 - UART_1"]
    UART_1 = 32,
    #[doc = "33 - I2C_1"]
    I2C_1 = 33,
    #[doc = "34 - CHANGE_NOTICE"]
    CHANGE_NOTICE = 34,
    #[doc = "35 - PMP"]
    PMP = 35,
    #[doc = "36 - SPI_2"]
    SPI_2 = 36,
    #[doc = "37 - UART_2"]
    UART_2 = 37,
    #[doc = "38 - I2C_2"]
    I2C_2 = 38,
    #[doc = "39 - CTMU"]
    CTMU = 39,
    #[doc = "40 - DMA_0"]
    DMA_0 = 40,
    #[doc = "41 - DMA_1"]
    DMA_1 = 41,
    #[doc = "42 - DMA_2"]
    DMA_2 = 42,
    #[doc = "43 - DMA_3"]
    DMA_3 = 43,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            0 => Ok(Interrupt::CORE_TIMER),
            1 => Ok(Interrupt::CORE_SOFTWARE_0),
            2 => Ok(Interrupt::CORE_SOFTWARE_1),
            3 => Ok(Interrupt::EXTERNAL_0),
            4 => Ok(Interrupt::TIMER_1),
            5 => Ok(Interrupt::INPUT_CAPTURE_1),
            6 => Ok(Interrupt::OUTPUT_COMPARE_1),
            7 => Ok(Interrupt::EXTERNAL_1),
            8 => Ok(Interrupt::TIMER_2),
            9 => Ok(Interrupt::INPUT_CAPTURE_2),
            10 => Ok(Interrupt::OUTPUT_COMPARE_2),
            11 => Ok(Interrupt::EXTERNAL_2),
            12 => Ok(Interrupt::TIMER_3),
            13 => Ok(Interrupt::INPUT_CAPTURE_3),
            14 => Ok(Interrupt::OUTPUT_COMPARE_3),
            15 => Ok(Interrupt::EXTERNAL_3),
            16 => Ok(Interrupt::TIMER_4),
            17 => Ok(Interrupt::INPUT_CAPTURE_4),
            18 => Ok(Interrupt::OUTPUT_COMPARE_4),
            19 => Ok(Interrupt::EXTERNAL_4),
            20 => Ok(Interrupt::TIMER_5),
            21 => Ok(Interrupt::INPUT_CAPTURE_5),
            22 => Ok(Interrupt::OUTPUT_COMPARE_5),
            23 => Ok(Interrupt::ADC),
            24 => Ok(Interrupt::FAIL_SAFE_MONITOR),
            25 => Ok(Interrupt::RTCC),
            26 => Ok(Interrupt::FCE),
            27 => Ok(Interrupt::COMPARATOR_1),
            28 => Ok(Interrupt::COMPARATOR_2),
            29 => Ok(Interrupt::COMPARATOR_3),
            31 => Ok(Interrupt::SPI_1),
            32 => Ok(Interrupt::UART_1),
            33 => Ok(Interrupt::I2C_1),
            34 => Ok(Interrupt::CHANGE_NOTICE),
            35 => Ok(Interrupt::PMP),
            36 => Ok(Interrupt::SPI_2),
            37 => Ok(Interrupt::UART_2),
            38 => Ok(Interrupt::I2C_2),
            39 => Ok(Interrupt::CTMU),
            40 => Ok(Interrupt::DMA_0),
            41 => Ok(Interrupt::DMA_1),
            42 => Ok(Interrupt::DMA_2),
            43 => Ok(Interrupt::DMA_3),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
