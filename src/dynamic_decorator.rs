// If you need to construct or swap decorators conditionally at runtime (e.g., adding an encryption
// layer only if a user ticks a checkbox), you must use Trait Objects (dyn Trait). This approach
// introduces heap allocation and dynamic dispatch overhead.

// ============ //
// 1. The Trait //
// ============ //

trait TextProcessor {
    fn process(&self, text: &str) -> String;
}

// ===================== //
// 2. The Base Component //
// ===================== //

struct SimpleProcessor;

impl TextProcessor for SimpleProcessor {
    fn process(&self, text: &str) -> String {
        text.to_string()
    }
}

// ================================================================= //
// 3. The Dynamic Decorator (Uses references or Boxes to dyn traits) //
// ================================================================= //

struct UppercaseDecorator<'a> {
    wrapped: &'a dyn TextProcessor,
}

impl<'a> TextProcessor for UppercaseDecorator<'a> {
    fn process(&self, text: &str) -> String {
        // Enhance base behavior
        self.wrapped.process(text).to_uppercase()
    }
}

// ===== //
// Usage //
// ===== //

fn main() {
    let base = SimpleProcessor;

    // Wrapped dynamically at runtime using references
    let decorated = UppercaseDecorator { wrapped: &base };

    println!("{}", decorated.process("hello rust!")); // Outputs: HELLO RUST!
}
