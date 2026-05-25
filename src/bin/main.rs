#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;

use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    main,
};

#[main]
fn main() -> ! {

    let config = esp_hal::Config::default()
        .with_cpu_clock(CpuClock::max());

    let peripherals = esp_hal::init(config);
    let mut delay = Delay::new();

    let mut trig = Output::new(
        peripherals.GPIO1,
        Level::Low,
        OutputConfig::default(),
    );

    let echo = Input::new(
        peripherals.GPIO2,
        InputConfig::default().with_pull(Pull::None),
    );

    // GPIO4=Hijau, GPIO5=Merah, GPIO19=Kuning
    let mut led_hijau = Output::new(
        peripherals.GPIO4,
        Level::Low,
        OutputConfig::default(),
    );

    let mut led_merah = Output::new(
        peripherals.GPIO5,
        Level::Low,
        OutputConfig::default(),
    );

    let mut led_kuning = Output::new(
        peripherals.GPIO19,
        Level::Low,
        OutputConfig::default(),
    );

    println!("====================================");
    println!(" ADAPTIVE TRAFFIC LIGHT SYSTEM     ");
    println!(" ESP32-S3 + HC-SR04 + RUST          ");
    println!("====================================");

    loop {

        // =====================================
        // DEFAULT: MERAH ON
        // =====================================

        led_merah.set_high();
        led_hijau.set_low();
        led_kuning.set_low();

        // =====================================
        // TRIGGER HC-SR04
        // =====================================

        trig.set_low();
        delay.delay_micros(2);
        trig.set_high();
        delay.delay_micros(10);
        trig.set_low();

        // =====================================
        // TUNGGU ECHO HIGH (timeout 30ms)
        // =====================================

        let mut timeout = 0u32;
        while echo.is_low() {
            delay.delay_micros(1);
            timeout += 1;
            if timeout > 30_000 {
                break;
            }
        }

        if timeout >= 30_000 {
            println!("Timeout - tidak ada objek");
            delay.delay_millis(200);
            continue;
        }

        // =====================================
        // HITUNG DURASI ECHO
        // =====================================

        let mut duration_us = 0u32;
        while echo.is_high() {
            delay.delay_micros(1);
            duration_us += 1;
            if duration_us > 25_000 {
                break;
            }
        }

        // =====================================
        // HITUNG JARAK
        // distance(cm) = duration_us * 34 / 2000
        // =====================================

        let distance = (duration_us * 34 * 20) / 2000;

        // =====================================
        // FILTER RANGE VALID: 2 - 400 cm
        // =====================================

        if distance < 2 || distance > 400 {
            println!("Out of range: {} cm", distance);
            delay.delay_millis(200);
            continue;
        }

        println!("====================================");
        println!("Distance : {} cm", distance);

        // =====================================
        // FASE 1: HIJAU
        // 2-150cm   = PADAT  → Hijau 15 detik
        // 151-400cm = LANCAR → Hijau 5 detik
        // =====================================

        led_merah.set_low();
        led_hijau.set_high();
        led_kuning.set_low();

        if distance <= 150 {
            println!("Status : PADAT");
            println!("Mode   : High Traffic");
            println!("Green  : 15 Seconds");
            delay.delay_millis(15_000);

        } else {
            println!("Status : LANCAR");
            println!("Mode   : Smooth Traffic");
            println!("Green  : 5 Seconds");
            delay.delay_millis(5_000);
        }

        // =====================================
        // FASE 2: KUNING
        // =====================================

        led_hijau.set_low();
        led_kuning.set_high();
        led_merah.set_low();

        println!("Yellow : 2 Seconds");
        delay.delay_millis(2_000);

        // =====================================
        // FASE 3: MERAH
        // =====================================

        led_hijau.set_low();
        led_kuning.set_low();
        led_merah.set_high();

        println!("Red    : 5 Seconds");
        delay.delay_millis(5_000);

        println!("Repeating Monitoring...");
        println!("====================================");
    }
}