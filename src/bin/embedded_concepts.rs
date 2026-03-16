// Rust Tutorial #27: Embedded Rust — no_std and Embassy
// Conceptual examples demonstrating embedded patterns using std equivalents.
// These patterns show how embedded Rust works without requiring actual hardware.

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;

// ========================================================
// Concept 1: No Allocator — Fixed-Size Buffers
// In no_std, you can't use Vec or String. You use fixed arrays.
// ========================================================

struct FixedBuffer<const N: usize> {
    data: [u8; N],
    len: usize,
}

impl<const N: usize> FixedBuffer<N> {
    fn new() -> Self {
        Self {
            data: [0u8; N],
            len: 0,
        }
    }

    fn push(&mut self, byte: u8) -> Result<(), BufferError> {
        if self.len >= N {
            return Err(BufferError::Full);
        }
        self.data[self.len] = byte;
        self.len += 1;
        Ok(())
    }

    fn as_slice(&self) -> &[u8] {
        &self.data[..self.len]
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn capacity(&self) -> usize {
        N
    }

    fn clear(&mut self) {
        self.len = 0;
    }

    fn is_full(&self) -> bool {
        self.len >= N
    }
}

#[derive(Debug, PartialEq)]
enum BufferError {
    Full,
}

// ========================================================
// Concept 2: Ring Buffer — Common in Embedded UART/SPI
// ========================================================

struct RingBuffer<const N: usize> {
    data: [u8; N],
    read_pos: usize,
    write_pos: usize,
    count: usize,
}

impl<const N: usize> RingBuffer<N> {
    fn new() -> Self {
        Self {
            data: [0u8; N],
            read_pos: 0,
            write_pos: 0,
            count: 0,
        }
    }

    fn write(&mut self, byte: u8) -> Result<(), BufferError> {
        if self.count >= N {
            return Err(BufferError::Full);
        }
        self.data[self.write_pos] = byte;
        self.write_pos = (self.write_pos + 1) % N;
        self.count += 1;
        Ok(())
    }

    fn read(&mut self) -> Option<u8> {
        if self.count == 0 {
            return None;
        }
        let byte = self.data[self.read_pos];
        self.read_pos = (self.read_pos + 1) % N;
        self.count -= 1;
        Some(byte)
    }

    fn len(&self) -> usize {
        self.count
    }

    fn is_empty(&self) -> bool {
        self.count == 0
    }

    fn is_full(&self) -> bool {
        self.count >= N
    }
}

// ========================================================
// Concept 3: Hardware Abstraction — Simulated GPIO
// In real embedded, you use embedded-hal traits.
// ========================================================

trait OutputPin {
    fn set_high(&mut self);
    fn set_low(&mut self);
    fn is_high(&self) -> bool;
    fn toggle(&mut self);
}

trait InputPin {
    fn is_high(&self) -> bool;
    fn is_low(&self) -> bool;
}

// Simulated LED connected to a pin
struct SimulatedPin {
    state: bool,
    name: String,
}

impl SimulatedPin {
    fn new(name: &str) -> Self {
        Self {
            state: false,
            name: name.to_string(),
        }
    }
}

impl OutputPin for SimulatedPin {
    fn set_high(&mut self) {
        self.state = true;
    }

    fn set_low(&mut self) {
        self.state = false;
    }

    fn is_high(&self) -> bool {
        self.state
    }

    fn toggle(&mut self) {
        self.state = !self.state;
    }
}

impl InputPin for SimulatedPin {
    fn is_high(&self) -> bool {
        self.state
    }

    fn is_low(&self) -> bool {
        !self.state
    }
}

// LED driver that works with any OutputPin
struct Led<P: OutputPin> {
    pin: P,
}

impl<P: OutputPin> Led<P> {
    fn new(pin: P) -> Self {
        Self { pin }
    }

    fn on(&mut self) {
        self.pin.set_high();
    }

    fn off(&mut self) {
        self.pin.set_low();
    }

    fn toggle(&mut self) {
        self.pin.toggle();
    }

    fn is_on(&self) -> bool {
        self.pin.is_high()
    }
}

// ========================================================
// Concept 4: State Machine — Common Embedded Pattern
// ========================================================

#[derive(Debug, Clone, PartialEq)]
enum DeviceState {
    Idle,
    Initializing,
    Running,
    Error(String),
    Shutdown,
}

struct StateMachine {
    state: DeviceState,
    uptime_seconds: u32,
    error_count: u32,
}

impl StateMachine {
    fn new() -> Self {
        Self {
            state: DeviceState::Idle,
            uptime_seconds: 0,
            error_count: 0,
        }
    }

    fn transition(&mut self, event: Event) -> Result<(), String> {
        self.state = match (&self.state, event) {
            (DeviceState::Idle, Event::Start) => DeviceState::Initializing,
            (DeviceState::Initializing, Event::Ready) => DeviceState::Running,
            (DeviceState::Initializing, Event::Fault(msg)) => {
                self.error_count += 1;
                DeviceState::Error(msg)
            }
            (DeviceState::Running, Event::Tick) => {
                self.uptime_seconds += 1;
                DeviceState::Running
            }
            (DeviceState::Running, Event::Fault(msg)) => {
                self.error_count += 1;
                DeviceState::Error(msg)
            }
            (DeviceState::Running, Event::Stop) => DeviceState::Shutdown,
            (DeviceState::Error(_), Event::Reset) => DeviceState::Idle,
            (DeviceState::Shutdown, Event::Start) => DeviceState::Initializing,
            (state, event) => {
                return Err(format!(
                    "Invalid transition: {:?} with {:?}",
                    state, event
                ));
            }
        };
        Ok(())
    }

    fn state(&self) -> &DeviceState {
        &self.state
    }

    fn uptime(&self) -> u32 {
        self.uptime_seconds
    }

    fn error_count(&self) -> u32 {
        self.error_count
    }
}

#[derive(Debug)]
enum Event {
    Start,
    Ready,
    Tick,
    Stop,
    Fault(String),
    Reset,
}

// ========================================================
// Concept 5: Interrupt-like Pattern with Atomics
// In embedded, ISRs use atomic operations to share data.
// ========================================================

struct InterruptCounter {
    count: AtomicU32,
    enabled: AtomicBool,
}

impl InterruptCounter {
    fn new() -> Self {
        Self {
            count: AtomicU32::new(0),
            enabled: AtomicBool::new(false),
        }
    }

    // Called from "interrupt context"
    fn increment(&self) {
        if self.enabled.load(Ordering::Relaxed) {
            self.count.fetch_add(1, Ordering::Relaxed);
        }
    }

    // Called from main loop
    fn read_and_reset(&self) -> u32 {
        self.count.swap(0, Ordering::Relaxed)
    }

    fn enable(&self) {
        self.enabled.store(true, Ordering::Relaxed);
    }

    fn disable(&self) {
        self.enabled.store(false, Ordering::Relaxed);
    }

    fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }
}

// ========================================================
// Concept 6: Sensor Reading with Error Handling
// ========================================================

#[derive(Debug, Clone, PartialEq)]
struct SensorReading {
    temperature: f32,
    humidity: f32,
    timestamp: u64,
}

#[derive(Debug, PartialEq)]
enum SensorError {
    ReadTimeout,
    InvalidData,
    OutOfRange,
    NotCalibrated,
}

trait Sensor {
    fn read(&self) -> Result<SensorReading, SensorError>;
    fn calibrate(&mut self) -> Result<(), SensorError>;
    fn name(&self) -> &str;
}

struct SimulatedSensor {
    name: String,
    calibrated: bool,
    readings: Vec<SensorReading>,
    index: std::cell::Cell<usize>,
}

impl SimulatedSensor {
    fn new(name: &str, readings: Vec<SensorReading>) -> Self {
        Self {
            name: name.to_string(),
            calibrated: false,
            readings,
            index: std::cell::Cell::new(0),
        }
    }
}

impl Sensor for SimulatedSensor {
    fn read(&self) -> Result<SensorReading, SensorError> {
        if !self.calibrated {
            return Err(SensorError::NotCalibrated);
        }
        let idx = self.index.get();
        if idx >= self.readings.len() {
            return Err(SensorError::ReadTimeout);
        }
        let reading = self.readings[idx].clone();
        self.index.set(idx + 1);

        if reading.temperature < -40.0 || reading.temperature > 85.0 {
            return Err(SensorError::OutOfRange);
        }
        Ok(reading)
    }

    fn calibrate(&mut self) -> Result<(), SensorError> {
        self.calibrated = true;
        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }
}

// ========================================================
// Concept 7: Command Parser — Serial Protocol
// ========================================================

#[derive(Debug, PartialEq)]
enum Command {
    SetLed(bool),
    ReadSensor(u8),
    SetPwm(u8, u16),
    Reset,
    Status,
    Unknown(String),
}

fn parse_command(input: &str) -> Command {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    match parts.as_slice() {
        ["LED", "ON"] => Command::SetLed(true),
        ["LED", "OFF"] => Command::SetLed(false),
        ["READ", id] => {
            if let Ok(sensor_id) = id.parse::<u8>() {
                Command::ReadSensor(sensor_id)
            } else {
                Command::Unknown(input.to_string())
            }
        }
        ["PWM", channel, duty] => {
            if let (Ok(ch), Ok(d)) = (channel.parse::<u8>(), duty.parse::<u16>()) {
                Command::SetPwm(ch, d)
            } else {
                Command::Unknown(input.to_string())
            }
        }
        ["RESET"] => Command::Reset,
        ["STATUS"] => Command::Status,
        _ => Command::Unknown(input.to_string()),
    }
}

// ========================================================
// Concept 8: Watchdog Timer Pattern
// ========================================================

struct WatchdogTimer {
    timeout_ms: u32,
    last_feed_ms: u32,
    current_ms: u32,
    triggered: bool,
}

impl WatchdogTimer {
    fn new(timeout_ms: u32) -> Self {
        Self {
            timeout_ms,
            last_feed_ms: 0,
            current_ms: 0,
            triggered: false,
        }
    }

    fn feed(&mut self) {
        self.last_feed_ms = self.current_ms;
        self.triggered = false;
    }

    fn tick(&mut self, elapsed_ms: u32) {
        self.current_ms += elapsed_ms;
        if self.current_ms - self.last_feed_ms >= self.timeout_ms {
            self.triggered = true;
        }
    }

    fn is_triggered(&self) -> bool {
        self.triggered
    }

    fn time_remaining(&self) -> u32 {
        let elapsed = self.current_ms - self.last_feed_ms;
        if elapsed >= self.timeout_ms {
            0
        } else {
            self.timeout_ms - elapsed
        }
    }
}

fn main() {
    println!("=== Embedded Rust Concepts ===\n");

    // Demo 1: Fixed-size buffer
    println!("--- Fixed Buffer ---");
    let mut buf = FixedBuffer::<8>::new();
    for byte in b"Hello" {
        buf.push(*byte).unwrap();
    }
    println!("Buffer: {:?} (len: {}, cap: {})", buf.as_slice(), buf.len(), buf.capacity());

    println!();

    // Demo 2: Ring buffer
    println!("--- Ring Buffer ---");
    let mut ring = RingBuffer::<4>::new();
    ring.write(1).unwrap();
    ring.write(2).unwrap();
    ring.write(3).unwrap();
    ring.write(4).unwrap();
    println!("Full: {}", ring.is_full());
    println!("Read: {:?}", ring.read());
    ring.write(5).unwrap();
    while let Some(byte) = ring.read() {
        print!("{} ", byte);
    }
    println!();

    println!();

    // Demo 3: GPIO simulation
    println!("--- GPIO / LED ---");
    let pin = SimulatedPin::new("GPIO_13");
    let mut led = Led::new(pin);
    println!("LED is on: {}", led.is_on());
    led.on();
    println!("LED is on: {}", led.is_on());
    led.toggle();
    println!("LED is on: {}", led.is_on());

    println!();

    // Demo 4: State machine
    println!("--- State Machine ---");
    let mut sm = StateMachine::new();
    println!("State: {:?}", sm.state());
    sm.transition(Event::Start).unwrap();
    println!("State: {:?}", sm.state());
    sm.transition(Event::Ready).unwrap();
    println!("State: {:?}", sm.state());
    sm.transition(Event::Tick).unwrap();
    sm.transition(Event::Tick).unwrap();
    println!("Uptime: {}s", sm.uptime());
    sm.transition(Event::Fault("Overheated".to_string())).unwrap();
    println!("State: {:?}", sm.state());
    println!("Errors: {}", sm.error_count());
    sm.transition(Event::Reset).unwrap();
    println!("State: {:?}", sm.state());

    println!();

    // Demo 5: Atomic interrupt counter
    println!("--- Interrupt Counter ---");
    let counter = Arc::new(InterruptCounter::new());
    counter.enable();
    for _ in 0..10 {
        counter.increment();
    }
    let count = counter.read_and_reset();
    println!("Interrupt count: {}", count);
    println!("After reset: {}", counter.read_and_reset());

    println!();

    // Demo 6: Sensor reading
    println!("--- Sensor ---");
    let readings = vec![
        SensorReading { temperature: 22.5, humidity: 45.0, timestamp: 1000 },
        SensorReading { temperature: 23.0, humidity: 44.0, timestamp: 2000 },
    ];
    let mut sensor = SimulatedSensor::new("DHT22", readings);

    match sensor.read() {
        Err(SensorError::NotCalibrated) => println!("Sensor not calibrated (expected)"),
        _ => println!("Unexpected result"),
    }

    sensor.calibrate().unwrap();
    match sensor.read() {
        Ok(r) => println!("Reading: {:.1}C, {:.1}%", r.temperature, r.humidity),
        Err(e) => println!("Error: {:?}", e),
    }

    println!();

    // Demo 7: Command parser
    println!("--- Command Parser ---");
    let commands = vec!["LED ON", "LED OFF", "READ 3", "PWM 1 1024", "RESET", "STATUS", "UNKNOWN CMD"];
    for cmd in &commands {
        println!("  {:?} -> {:?}", cmd, parse_command(cmd));
    }

    println!();

    // Demo 8: Watchdog
    println!("--- Watchdog Timer ---");
    let mut wdt = WatchdogTimer::new(1000);
    wdt.tick(300);
    println!("Remaining: {}ms", wdt.time_remaining());
    wdt.tick(300);
    wdt.feed();
    println!("After feed, remaining: {}ms", wdt.time_remaining());
    wdt.tick(1100);
    println!("Triggered: {}", wdt.is_triggered());

    println!("\n=== All demos completed! ===");
}

// ========================================================
// Tests
// ========================================================

#[cfg(test)]
mod tests {
    use super::*;

    // --- Fixed Buffer ---

    #[test]
    fn test_fixed_buffer_push_and_read() {
        let mut buf = FixedBuffer::<4>::new();
        buf.push(1).unwrap();
        buf.push(2).unwrap();
        assert_eq!(buf.as_slice(), &[1, 2]);
        assert_eq!(buf.len(), 2);
    }

    #[test]
    fn test_fixed_buffer_full() {
        let mut buf = FixedBuffer::<2>::new();
        buf.push(1).unwrap();
        buf.push(2).unwrap();
        assert_eq!(buf.push(3), Err(BufferError::Full));
        assert!(buf.is_full());
    }

    #[test]
    fn test_fixed_buffer_clear() {
        let mut buf = FixedBuffer::<4>::new();
        buf.push(1).unwrap();
        buf.clear();
        assert!(buf.is_empty());
        assert_eq!(buf.len(), 0);
    }

    #[test]
    fn test_fixed_buffer_capacity() {
        let buf = FixedBuffer::<16>::new();
        assert_eq!(buf.capacity(), 16);
    }

    // --- Ring Buffer ---

    #[test]
    fn test_ring_buffer_basic() {
        let mut ring = RingBuffer::<4>::new();
        ring.write(1).unwrap();
        ring.write(2).unwrap();
        assert_eq!(ring.read(), Some(1));
        assert_eq!(ring.read(), Some(2));
        assert_eq!(ring.read(), None);
    }

    #[test]
    fn test_ring_buffer_wraparound() {
        let mut ring = RingBuffer::<3>::new();
        ring.write(1).unwrap();
        ring.write(2).unwrap();
        ring.write(3).unwrap();
        assert!(ring.is_full());
        assert_eq!(ring.read(), Some(1));
        ring.write(4).unwrap();
        assert_eq!(ring.read(), Some(2));
        assert_eq!(ring.read(), Some(3));
        assert_eq!(ring.read(), Some(4));
        assert!(ring.is_empty());
    }

    #[test]
    fn test_ring_buffer_full() {
        let mut ring = RingBuffer::<2>::new();
        ring.write(1).unwrap();
        ring.write(2).unwrap();
        assert_eq!(ring.write(3), Err(BufferError::Full));
    }

    // --- GPIO / LED ---

    #[test]
    fn test_led_on_off() {
        let pin = SimulatedPin::new("test");
        let mut led = Led::new(pin);
        assert!(!led.is_on());
        led.on();
        assert!(led.is_on());
        led.off();
        assert!(!led.is_on());
    }

    #[test]
    fn test_led_toggle() {
        let pin = SimulatedPin::new("test");
        let mut led = Led::new(pin);
        led.toggle();
        assert!(led.is_on());
        led.toggle();
        assert!(!led.is_on());
    }

    #[test]
    fn test_simulated_pin_input() {
        let mut pin = SimulatedPin::new("test");
        assert!(InputPin::is_low(&pin));
        pin.set_high();
        assert!(InputPin::is_high(&pin));
        assert!(!InputPin::is_low(&pin));
    }

    // --- State Machine ---

    #[test]
    fn test_state_machine_normal_flow() {
        let mut sm = StateMachine::new();
        assert_eq!(*sm.state(), DeviceState::Idle);
        sm.transition(Event::Start).unwrap();
        assert_eq!(*sm.state(), DeviceState::Initializing);
        sm.transition(Event::Ready).unwrap();
        assert_eq!(*sm.state(), DeviceState::Running);
        sm.transition(Event::Stop).unwrap();
        assert_eq!(*sm.state(), DeviceState::Shutdown);
    }

    #[test]
    fn test_state_machine_tick() {
        let mut sm = StateMachine::new();
        sm.transition(Event::Start).unwrap();
        sm.transition(Event::Ready).unwrap();
        sm.transition(Event::Tick).unwrap();
        sm.transition(Event::Tick).unwrap();
        assert_eq!(sm.uptime(), 2);
    }

    #[test]
    fn test_state_machine_error_flow() {
        let mut sm = StateMachine::new();
        sm.transition(Event::Start).unwrap();
        sm.transition(Event::Fault("test".to_string())).unwrap();
        assert!(matches!(sm.state(), DeviceState::Error(_)));
        assert_eq!(sm.error_count(), 1);
        sm.transition(Event::Reset).unwrap();
        assert_eq!(*sm.state(), DeviceState::Idle);
    }

    #[test]
    fn test_state_machine_invalid_transition() {
        let mut sm = StateMachine::new();
        let result = sm.transition(Event::Tick);
        assert!(result.is_err());
    }

    // --- Interrupt Counter ---

    #[test]
    fn test_interrupt_counter() {
        let counter = InterruptCounter::new();
        counter.enable();
        counter.increment();
        counter.increment();
        counter.increment();
        assert_eq!(counter.read_and_reset(), 3);
        assert_eq!(counter.read_and_reset(), 0);
    }

    #[test]
    fn test_interrupt_counter_disabled() {
        let counter = InterruptCounter::new();
        counter.increment();
        counter.increment();
        assert_eq!(counter.read_and_reset(), 0);
    }

    #[test]
    fn test_interrupt_counter_enable_disable() {
        let counter = InterruptCounter::new();
        assert!(!counter.is_enabled());
        counter.enable();
        assert!(counter.is_enabled());
        counter.disable();
        assert!(!counter.is_enabled());
    }

    // --- Sensor ---

    #[test]
    fn test_sensor_not_calibrated() {
        let sensor = SimulatedSensor::new("test", vec![]);
        assert_eq!(sensor.read(), Err(SensorError::NotCalibrated));
    }

    #[test]
    fn test_sensor_read() {
        let readings = vec![SensorReading {
            temperature: 22.5,
            humidity: 45.0,
            timestamp: 1000,
        }];
        let mut sensor = SimulatedSensor::new("test", readings);
        sensor.calibrate().unwrap();
        let reading = sensor.read().unwrap();
        assert!((reading.temperature - 22.5).abs() < 0.01);
    }

    #[test]
    fn test_sensor_timeout() {
        let mut sensor = SimulatedSensor::new("test", vec![]);
        sensor.calibrate().unwrap();
        assert_eq!(sensor.read(), Err(SensorError::ReadTimeout));
    }

    #[test]
    fn test_sensor_out_of_range() {
        let readings = vec![SensorReading {
            temperature: 100.0,
            humidity: 45.0,
            timestamp: 1000,
        }];
        let mut sensor = SimulatedSensor::new("test", readings);
        sensor.calibrate().unwrap();
        assert_eq!(sensor.read(), Err(SensorError::OutOfRange));
    }

    // --- Command Parser ---

    #[test]
    fn test_parse_led_on() {
        assert_eq!(parse_command("LED ON"), Command::SetLed(true));
    }

    #[test]
    fn test_parse_led_off() {
        assert_eq!(parse_command("LED OFF"), Command::SetLed(false));
    }

    #[test]
    fn test_parse_read_sensor() {
        assert_eq!(parse_command("READ 3"), Command::ReadSensor(3));
    }

    #[test]
    fn test_parse_pwm() {
        assert_eq!(parse_command("PWM 1 1024"), Command::SetPwm(1, 1024));
    }

    #[test]
    fn test_parse_reset() {
        assert_eq!(parse_command("RESET"), Command::Reset);
    }

    #[test]
    fn test_parse_status() {
        assert_eq!(parse_command("STATUS"), Command::Status);
    }

    #[test]
    fn test_parse_unknown() {
        assert!(matches!(parse_command("FOO BAR"), Command::Unknown(_)));
    }

    // --- Watchdog ---

    #[test]
    fn test_watchdog_not_triggered() {
        let mut wdt = WatchdogTimer::new(1000);
        wdt.tick(500);
        assert!(!wdt.is_triggered());
        assert_eq!(wdt.time_remaining(), 500);
    }

    #[test]
    fn test_watchdog_triggered() {
        let mut wdt = WatchdogTimer::new(1000);
        wdt.tick(1000);
        assert!(wdt.is_triggered());
        assert_eq!(wdt.time_remaining(), 0);
    }

    #[test]
    fn test_watchdog_feed_resets() {
        let mut wdt = WatchdogTimer::new(1000);
        wdt.tick(800);
        wdt.feed();
        assert!(!wdt.is_triggered());
        assert_eq!(wdt.time_remaining(), 1000);
    }

    #[test]
    fn test_watchdog_feed_then_timeout() {
        let mut wdt = WatchdogTimer::new(1000);
        wdt.tick(500);
        wdt.feed();
        wdt.tick(500);
        assert!(!wdt.is_triggered());
        wdt.tick(600);
        assert!(wdt.is_triggered());
    }
}
