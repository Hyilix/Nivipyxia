use nivipyxia::app::run_application;

// Pixel representation:
// Screen (ScreenBuffer) -> 0x00RRGGBB
// Canvas (hand-made)    -> 0xAARRGGBB

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_application()
}
