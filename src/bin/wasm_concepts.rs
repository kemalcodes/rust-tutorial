// Rust Tutorial #29: WebAssembly with Rust
// Conceptual examples demonstrating WASM patterns using standard Rust.
// These patterns show how Rust+WASM works without requiring wasm-pack.

use std::collections::HashMap;

// ========================================================
// Concept 1: Functions Exported to JavaScript
// In real WASM, these would be #[wasm_bindgen] exports.
// ========================================================

/// Exported function: compute fibonacci
fn fibonacci(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

/// Exported function: validate email format
fn validate_email(email: &str) -> bool {
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }
    let local = parts[0];
    let domain = parts[1];

    if local.is_empty() || domain.is_empty() {
        return false;
    }

    if !domain.contains('.') {
        return false;
    }

    let domain_parts: Vec<&str> = domain.split('.').collect();
    if domain_parts.iter().any(|p| p.is_empty()) {
        return false;
    }

    true
}

/// Exported function: format number with commas
fn format_number(n: i64) -> String {
    let negative = n < 0;
    let s = n.unsigned_abs().to_string();
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::new();

    for (i, ch) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i) % 3 == 0 {
            result.push(',');
        }
        result.push(*ch);
    }

    if negative {
        format!("-{}", result)
    } else {
        result
    }
}

// ========================================================
// Concept 2: Shared State (WASM Module State)
// In real WASM, this would be module-level state
// accessed through exported functions.
// ========================================================

struct WasmState {
    counter: i32,
    items: Vec<String>,
    scores: HashMap<String, f64>,
}

impl WasmState {
    fn new() -> Self {
        Self {
            counter: 0,
            items: Vec::new(),
            scores: HashMap::new(),
        }
    }

    fn increment(&mut self) -> i32 {
        self.counter += 1;
        self.counter
    }

    fn decrement(&mut self) -> i32 {
        self.counter -= 1;
        self.counter
    }

    fn get_counter(&self) -> i32 {
        self.counter
    }

    fn add_item(&mut self, item: String) -> usize {
        self.items.push(item);
        self.items.len()
    }

    fn remove_item(&mut self, index: usize) -> Option<String> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    fn get_items(&self) -> &[String] {
        &self.items
    }

    fn set_score(&mut self, name: String, score: f64) {
        self.scores.insert(name, score);
    }

    fn get_score(&self, name: &str) -> Option<f64> {
        self.scores.get(name).copied()
    }

    fn top_scores(&self, n: usize) -> Vec<(String, f64)> {
        let mut sorted: Vec<_> = self.scores.iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        sorted.into_iter().take(n).collect()
    }
}

// ========================================================
// Concept 3: DOM-like Virtual DOM
// Frameworks like Leptos use a virtual DOM in WASM.
// ========================================================

#[derive(Debug, Clone, PartialEq)]
enum VNode {
    Element {
        tag: String,
        attributes: Vec<(String, String)>,
        children: Vec<VNode>,
    },
    Text(String),
}

impl VNode {
    fn element(tag: &str) -> VNodeBuilder {
        VNodeBuilder {
            tag: tag.to_string(),
            attributes: Vec::new(),
            children: Vec::new(),
        }
    }

    fn text(content: &str) -> VNode {
        VNode::Text(content.to_string())
    }

    fn render_html(&self) -> String {
        match self {
            VNode::Text(text) => html_escape(text),
            VNode::Element { tag, attributes, children } => {
                let attrs: String = attributes
                    .iter()
                    .map(|(k, v)| format!(" {}=\"{}\"", k, html_escape(v)))
                    .collect();

                if children.is_empty() {
                    format!("<{}{}/>", tag, attrs)
                } else {
                    let inner: String = children.iter().map(|c| c.render_html()).collect();
                    format!("<{}{}>{}</{}>", tag, attrs, inner, tag)
                }
            }
        }
    }
}

struct VNodeBuilder {
    tag: String,
    attributes: Vec<(String, String)>,
    children: Vec<VNode>,
}

impl VNodeBuilder {
    fn attr(mut self, key: &str, value: &str) -> Self {
        self.attributes.push((key.to_string(), value.to_string()));
        self
    }

    fn child(mut self, node: VNode) -> Self {
        self.children.push(node);
        self
    }

    fn children(mut self, nodes: Vec<VNode>) -> Self {
        self.children.extend(nodes);
        self
    }

    fn build(self) -> VNode {
        VNode::Element {
            tag: self.tag,
            attributes: self.attributes,
            children: self.children,
        }
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

// ========================================================
// Concept 4: Virtual DOM Diffing
// ========================================================

#[derive(Debug, PartialEq)]
enum Patch {
    Replace(VNode),
    UpdateText(String),
    AddAttribute(String, String),
    RemoveAttribute(String),
    AppendChild(VNode),
    RemoveChild(usize),
    NoChange,
}

fn diff(old: &VNode, new: &VNode) -> Patch {
    match (old, new) {
        (VNode::Text(old_text), VNode::Text(new_text)) => {
            if old_text == new_text {
                Patch::NoChange
            } else {
                Patch::UpdateText(new_text.clone())
            }
        }
        (VNode::Element { tag: old_tag, .. }, VNode::Element { tag: new_tag, .. })
            if old_tag != new_tag =>
        {
            Patch::Replace(new.clone())
        }
        (VNode::Text(_), VNode::Element { .. }) | (VNode::Element { .. }, VNode::Text(_)) => {
            Patch::Replace(new.clone())
        }
        _ => Patch::NoChange,
    }
}

// ========================================================
// Concept 5: Reactive Signals (Leptos-like)
// ========================================================

struct Signal<T: Clone> {
    value: T,
    subscribers: Vec<Box<dyn Fn(&T)>>,
}

impl<T: Clone> Signal<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            subscribers: Vec::new(),
        }
    }

    fn get(&self) -> T {
        self.value.clone()
    }

    fn set(&mut self, new_value: T) {
        self.value = new_value;
        for subscriber in &self.subscribers {
            subscriber(&self.value);
        }
    }

    fn subscribe(&mut self, callback: impl Fn(&T) + 'static) {
        self.subscribers.push(Box::new(callback));
    }
}

// ========================================================
// Concept 6: JSON Bridge (JS <-> WASM Data Exchange)
// ========================================================

#[derive(Debug, Clone, PartialEq)]
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

impl JsonValue {
    fn to_json_string(&self) -> String {
        match self {
            JsonValue::Null => "null".to_string(),
            JsonValue::Bool(b) => b.to_string(),
            JsonValue::Number(n) => {
                if *n == (*n as i64) as f64 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            JsonValue::Str(s) => format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\"")),
            JsonValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_json_string()).collect();
                format!("[{}]", items.join(","))
            }
            JsonValue::Object(entries) => {
                let items: Vec<String> = entries
                    .iter()
                    .map(|(k, v)| format!("\"{}\":{}", k, v.to_json_string()))
                    .collect();
                format!("{{{}}}", items.join(","))
            }
        }
    }

    fn is_null(&self) -> bool {
        matches!(self, JsonValue::Null)
    }

    fn as_str(&self) -> Option<&str> {
        match self {
            JsonValue::Str(s) => Some(s),
            _ => None,
        }
    }

    fn as_f64(&self) -> Option<f64> {
        match self {
            JsonValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    fn as_bool(&self) -> Option<bool> {
        match self {
            JsonValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

// ========================================================
// Concept 7: Canvas-like Drawing Commands
// In real WASM, these would call the Canvas API via wasm-bindgen.
// ========================================================

#[derive(Debug, Clone, PartialEq)]
enum DrawCommand {
    MoveTo(f64, f64),
    LineTo(f64, f64),
    Rect(f64, f64, f64, f64),
    Circle(f64, f64, f64),
    FillColor(String),
    StrokeColor(String),
    Fill,
    Stroke,
    Text(String, f64, f64),
    Clear,
}

struct Canvas {
    commands: Vec<DrawCommand>,
    width: u32,
    height: u32,
}

impl Canvas {
    fn new(width: u32, height: u32) -> Self {
        Self {
            commands: Vec::new(),
            width,
            height,
        }
    }

    fn clear(&mut self) {
        self.commands.push(DrawCommand::Clear);
    }

    fn fill_color(&mut self, color: &str) {
        self.commands.push(DrawCommand::FillColor(color.to_string()));
    }

    fn stroke_color(&mut self, color: &str) {
        self.commands.push(DrawCommand::StrokeColor(color.to_string()));
    }

    fn rect(&mut self, x: f64, y: f64, w: f64, h: f64) {
        self.commands.push(DrawCommand::Rect(x, y, w, h));
    }

    fn circle(&mut self, cx: f64, cy: f64, r: f64) {
        self.commands.push(DrawCommand::Circle(cx, cy, r));
    }

    fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        self.commands.push(DrawCommand::MoveTo(x1, y1));
        self.commands.push(DrawCommand::LineTo(x2, y2));
        self.commands.push(DrawCommand::Stroke);
    }

    fn text(&mut self, content: &str, x: f64, y: f64) {
        self.commands.push(DrawCommand::Text(content.to_string(), x, y));
    }

    fn fill(&mut self) {
        self.commands.push(DrawCommand::Fill);
    }

    fn command_count(&self) -> usize {
        self.commands.len()
    }

    fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

// ========================================================
// Concept 8: Image Processing (pixel manipulation)
// In WASM, you'd work with ImageData from Canvas.
// ========================================================

#[derive(Debug, Clone)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

struct ImageBuffer {
    pixels: Vec<Pixel>,
    width: u32,
    height: u32,
}

impl ImageBuffer {
    fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        Self {
            pixels: vec![Pixel { r: 0, g: 0, b: 0, a: 255 }; size],
            width,
            height,
        }
    }

    fn get_pixel(&self, x: u32, y: u32) -> &Pixel {
        &self.pixels[(y * self.width + x) as usize]
    }

    fn set_pixel(&mut self, x: u32, y: u32, pixel: Pixel) {
        self.pixels[(y * self.width + x) as usize] = pixel;
    }

    fn grayscale(&self) -> ImageBuffer {
        let pixels: Vec<Pixel> = self.pixels.iter().map(|p| {
            let gray = (0.299 * p.r as f64 + 0.587 * p.g as f64 + 0.114 * p.b as f64) as u8;
            Pixel { r: gray, g: gray, b: gray, a: p.a }
        }).collect();
        ImageBuffer { pixels, width: self.width, height: self.height }
    }

    fn invert(&self) -> ImageBuffer {
        let pixels: Vec<Pixel> = self.pixels.iter().map(|p| {
            Pixel { r: 255 - p.r, g: 255 - p.g, b: 255 - p.b, a: p.a }
        }).collect();
        ImageBuffer { pixels, width: self.width, height: self.height }
    }

    fn brightness(&self, factor: f64) -> ImageBuffer {
        let pixels: Vec<Pixel> = self.pixels.iter().map(|p| {
            let r = (p.r as f64 * factor).min(255.0) as u8;
            let g = (p.g as f64 * factor).min(255.0) as u8;
            let b = (p.b as f64 * factor).min(255.0) as u8;
            Pixel { r, g, b, a: p.a }
        }).collect();
        ImageBuffer { pixels, width: self.width, height: self.height }
    }
}

fn main() {
    println!("=== WebAssembly Concepts Demo ===\n");

    // Demo 1: Exported functions
    println!("--- Exported Functions ---");
    println!("fibonacci(10) = {}", fibonacci(10));
    println!("fibonacci(20) = {}", fibonacci(20));
    println!("validate_email(\"alex@example.com\") = {}", validate_email("alex@example.com"));
    println!("validate_email(\"invalid\") = {}", validate_email("invalid"));
    println!("format_number(1234567) = {}", format_number(1234567));
    println!("format_number(-42000) = {}", format_number(-42000));

    println!();

    // Demo 2: WASM module state
    println!("--- Module State ---");
    let mut state = WasmState::new();
    println!("Counter: {}", state.increment());
    println!("Counter: {}", state.increment());
    println!("Counter: {}", state.decrement());

    state.add_item("Task 1".to_string());
    state.add_item("Task 2".to_string());
    state.add_item("Task 3".to_string());
    println!("Items: {:?}", state.get_items());

    state.remove_item(1);
    println!("After remove: {:?}", state.get_items());

    state.set_score("Alex".to_string(), 95.0);
    state.set_score("Sam".to_string(), 87.0);
    state.set_score("Jordan".to_string(), 92.0);
    println!("Top 2 scores: {:?}", state.top_scores(2));

    println!();

    // Demo 3: Virtual DOM
    println!("--- Virtual DOM ---");
    let page = VNode::element("div")
        .attr("class", "container")
        .child(VNode::element("h1")
            .child(VNode::text("Hello, WASM!"))
            .build())
        .child(VNode::element("p")
            .attr("class", "intro")
            .child(VNode::text("This is rendered by Rust."))
            .build())
        .child(VNode::element("ul")
            .children(vec![
                VNode::element("li").child(VNode::text("Item 1")).build(),
                VNode::element("li").child(VNode::text("Item 2")).build(),
            ])
            .build())
        .build();

    println!("{}", page.render_html());

    println!();

    // Demo 4: Diffing
    println!("--- Virtual DOM Diff ---");
    let old = VNode::text("Hello");
    let new = VNode::text("Hello, World!");
    println!("Text diff: {:?}", diff(&old, &new));

    let old = VNode::text("Same");
    let new = VNode::text("Same");
    println!("Same text: {:?}", diff(&old, &new));

    println!();

    // Demo 5: Reactive signals
    println!("--- Reactive Signals ---");
    let mut count = Signal::new(0);
    println!("Initial: {}", count.get());
    count.set(5);
    println!("After set: {}", count.get());
    count.set(10);
    println!("After set: {}", count.get());

    println!();

    // Demo 6: JSON bridge
    println!("--- JSON Bridge ---");
    let data = JsonValue::Object(vec![
        ("name".to_string(), JsonValue::Str("Alex".to_string())),
        ("age".to_string(), JsonValue::Number(25.0)),
        ("active".to_string(), JsonValue::Bool(true)),
        ("tags".to_string(), JsonValue::Array(vec![
            JsonValue::Str("rust".to_string()),
            JsonValue::Str("wasm".to_string()),
        ])),
    ]);
    println!("{}", data.to_json_string());

    println!();

    // Demo 7: Canvas commands
    println!("--- Canvas Drawing ---");
    let mut canvas = Canvas::new(800, 600);
    canvas.clear();
    canvas.fill_color("#ff0000");
    canvas.rect(10.0, 10.0, 100.0, 50.0);
    canvas.fill();
    canvas.stroke_color("#0000ff");
    canvas.circle(200.0, 200.0, 50.0);
    canvas.text("Hello WASM!", 50.0, 300.0);
    println!("Canvas {}x{} with {} commands", canvas.width, canvas.height, canvas.command_count());

    println!();

    // Demo 8: Image processing
    println!("--- Image Processing ---");
    let mut img = ImageBuffer::new(4, 4);
    img.set_pixel(0, 0, Pixel { r: 255, g: 0, b: 0, a: 255 });
    img.set_pixel(1, 0, Pixel { r: 0, g: 255, b: 0, a: 255 });
    img.set_pixel(2, 0, Pixel { r: 0, g: 0, b: 255, a: 255 });

    let gray = img.grayscale();
    let p = gray.get_pixel(0, 0);
    println!("Red -> grayscale: r={}, g={}, b={}", p.r, p.g, p.b);

    let inverted = img.invert();
    let p = inverted.get_pixel(0, 0);
    println!("Red inverted: r={}, g={}, b={}", p.r, p.g, p.b);

    println!("\n=== All demos completed! ===");
}

// ========================================================
// Tests
// ========================================================

#[cfg(test)]
mod tests {
    use super::*;

    // --- Exported functions ---

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci(20), 6765);
    }

    #[test]
    fn test_validate_email_valid() {
        assert!(validate_email("alex@example.com"));
        assert!(validate_email("user@sub.domain.com"));
    }

    #[test]
    fn test_validate_email_invalid() {
        assert!(!validate_email("invalid"));
        assert!(!validate_email("@domain.com"));
        assert!(!validate_email("user@"));
        assert!(!validate_email("user@domain"));
        assert!(!validate_email("user@.com"));
    }

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(0), "0");
        assert_eq!(format_number(999), "999");
        assert_eq!(format_number(1000), "1,000");
        assert_eq!(format_number(1234567), "1,234,567");
        assert_eq!(format_number(-42000), "-42,000");
    }

    // --- WASM State ---

    #[test]
    fn test_state_counter() {
        let mut state = WasmState::new();
        assert_eq!(state.get_counter(), 0);
        assert_eq!(state.increment(), 1);
        assert_eq!(state.increment(), 2);
        assert_eq!(state.decrement(), 1);
    }

    #[test]
    fn test_state_items() {
        let mut state = WasmState::new();
        assert_eq!(state.add_item("a".to_string()), 1);
        assert_eq!(state.add_item("b".to_string()), 2);
        assert_eq!(state.get_items(), &["a", "b"]);
        assert_eq!(state.remove_item(0), Some("a".to_string()));
        assert_eq!(state.get_items(), &["b"]);
    }

    #[test]
    fn test_state_remove_invalid() {
        let mut state = WasmState::new();
        assert_eq!(state.remove_item(0), None);
    }

    #[test]
    fn test_state_scores() {
        let mut state = WasmState::new();
        state.set_score("Alex".to_string(), 95.0);
        state.set_score("Sam".to_string(), 87.0);
        assert_eq!(state.get_score("Alex"), Some(95.0));
        assert_eq!(state.get_score("Unknown"), None);
    }

    #[test]
    fn test_state_top_scores() {
        let mut state = WasmState::new();
        state.set_score("A".to_string(), 80.0);
        state.set_score("B".to_string(), 95.0);
        state.set_score("C".to_string(), 90.0);
        let top = state.top_scores(2);
        assert_eq!(top[0].0, "B");
        assert_eq!(top[1].0, "C");
    }

    // --- Virtual DOM ---

    #[test]
    fn test_vnode_text() {
        let node = VNode::text("hello");
        assert_eq!(node.render_html(), "hello");
    }

    #[test]
    fn test_vnode_element() {
        let node = VNode::element("div")
            .attr("class", "test")
            .child(VNode::text("content"))
            .build();
        assert_eq!(node.render_html(), "<div class=\"test\">content</div>");
    }

    #[test]
    fn test_vnode_empty_element() {
        let node = VNode::element("br").build();
        assert_eq!(node.render_html(), "<br/>");
    }

    #[test]
    fn test_vnode_nested() {
        let node = VNode::element("ul")
            .child(VNode::element("li").child(VNode::text("item")).build())
            .build();
        assert_eq!(node.render_html(), "<ul><li>item</li></ul>");
    }

    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("<script>"), "&lt;script&gt;");
        assert_eq!(html_escape("a & b"), "a &amp; b");
    }

    // --- Diff ---

    #[test]
    fn test_diff_same_text() {
        let a = VNode::text("hello");
        let b = VNode::text("hello");
        assert_eq!(diff(&a, &b), Patch::NoChange);
    }

    #[test]
    fn test_diff_different_text() {
        let a = VNode::text("hello");
        let b = VNode::text("world");
        assert_eq!(diff(&a, &b), Patch::UpdateText("world".to_string()));
    }

    #[test]
    fn test_diff_different_tags() {
        let a = VNode::element("div").build();
        let b = VNode::element("span").build();
        assert!(matches!(diff(&a, &b), Patch::Replace(_)));
    }

    #[test]
    fn test_diff_text_to_element() {
        let a = VNode::text("hello");
        let b = VNode::element("div").build();
        assert!(matches!(diff(&a, &b), Patch::Replace(_)));
    }

    // --- Signal ---

    #[test]
    fn test_signal_get_set() {
        let mut signal = Signal::new(42);
        assert_eq!(signal.get(), 42);
        signal.set(100);
        assert_eq!(signal.get(), 100);
    }

    #[test]
    fn test_signal_string() {
        let mut signal = Signal::new("hello".to_string());
        assert_eq!(signal.get(), "hello");
        signal.set("world".to_string());
        assert_eq!(signal.get(), "world");
    }

    // --- JSON ---

    #[test]
    fn test_json_null() {
        assert_eq!(JsonValue::Null.to_json_string(), "null");
        assert!(JsonValue::Null.is_null());
    }

    #[test]
    fn test_json_bool() {
        assert_eq!(JsonValue::Bool(true).to_json_string(), "true");
        assert_eq!(JsonValue::Bool(false).as_bool(), Some(false));
    }

    #[test]
    fn test_json_number() {
        assert_eq!(JsonValue::Number(42.0).to_json_string(), "42");
        assert_eq!(JsonValue::Number(3.14).as_f64(), Some(3.14));
    }

    #[test]
    fn test_json_string() {
        assert_eq!(
            JsonValue::Str("hello".to_string()).to_json_string(),
            "\"hello\""
        );
        assert_eq!(
            JsonValue::Str("hello".to_string()).as_str(),
            Some("hello")
        );
    }

    #[test]
    fn test_json_array() {
        let arr = JsonValue::Array(vec![
            JsonValue::Number(1.0),
            JsonValue::Number(2.0),
        ]);
        assert_eq!(arr.to_json_string(), "[1,2]");
    }

    #[test]
    fn test_json_object() {
        let obj = JsonValue::Object(vec![
            ("name".to_string(), JsonValue::Str("Alex".to_string())),
            ("age".to_string(), JsonValue::Number(25.0)),
        ]);
        assert_eq!(obj.to_json_string(), "{\"name\":\"Alex\",\"age\":25}");
    }

    // --- Canvas ---

    #[test]
    fn test_canvas_dimensions() {
        let canvas = Canvas::new(800, 600);
        assert_eq!(canvas.dimensions(), (800, 600));
    }

    #[test]
    fn test_canvas_commands() {
        let mut canvas = Canvas::new(100, 100);
        canvas.clear();
        canvas.rect(0.0, 0.0, 50.0, 50.0);
        canvas.fill();
        assert_eq!(canvas.command_count(), 3);
    }

    #[test]
    fn test_canvas_line() {
        let mut canvas = Canvas::new(100, 100);
        canvas.line(0.0, 0.0, 100.0, 100.0);
        assert_eq!(canvas.command_count(), 3); // moveto, lineto, stroke
    }

    // --- Image Processing ---

    #[test]
    fn test_image_set_get_pixel() {
        let mut img = ImageBuffer::new(2, 2);
        img.set_pixel(0, 0, Pixel { r: 255, g: 0, b: 0, a: 255 });
        let p = img.get_pixel(0, 0);
        assert_eq!(p.r, 255);
        assert_eq!(p.g, 0);
    }

    #[test]
    fn test_image_grayscale() {
        let mut img = ImageBuffer::new(1, 1);
        img.set_pixel(0, 0, Pixel { r: 255, g: 0, b: 0, a: 255 });
        let gray = img.grayscale();
        let p = gray.get_pixel(0, 0);
        // 0.299 * 255 = ~76
        assert!(p.r > 70 && p.r < 80);
        assert_eq!(p.r, p.g);
        assert_eq!(p.g, p.b);
    }

    #[test]
    fn test_image_invert() {
        let mut img = ImageBuffer::new(1, 1);
        img.set_pixel(0, 0, Pixel { r: 200, g: 100, b: 50, a: 255 });
        let inverted = img.invert();
        let p = inverted.get_pixel(0, 0);
        assert_eq!(p.r, 55);
        assert_eq!(p.g, 155);
        assert_eq!(p.b, 205);
    }

    #[test]
    fn test_image_brightness() {
        let mut img = ImageBuffer::new(1, 1);
        img.set_pixel(0, 0, Pixel { r: 100, g: 100, b: 100, a: 255 });
        let bright = img.brightness(1.5);
        let p = bright.get_pixel(0, 0);
        assert_eq!(p.r, 150);
    }

    #[test]
    fn test_image_brightness_clamp() {
        let mut img = ImageBuffer::new(1, 1);
        img.set_pixel(0, 0, Pixel { r: 200, g: 200, b: 200, a: 255 });
        let bright = img.brightness(2.0);
        let p = bright.get_pixel(0, 0);
        assert_eq!(p.r, 255); // clamped
    }
}
