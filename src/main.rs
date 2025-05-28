use eframe::egui;

#[derive(Default)]
struct RomanCalculatorApp {
    input1: String,
    input2: String,
    result: String,
    error_message: String,
    operation: Operation,
}

#[derive(PartialEq, Default)]
enum Operation {
    #[default]
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl RomanCalculatorApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Konfiguracja wyglądu
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Default::default()
    }

    fn calculate(&mut self) {
        self.error_message.clear();

        // Konwersja z rzymskich na arabskie
        let num1 = match roman_to_arabic(&self.input1) {
            Ok(n) => n,
            Err(e) => {
                self.error_message = format!("Błąd w pierwszej liczbie: {}", e);
                return;
            }
        };

        let num2 = match roman_to_arabic(&self.input2) {
            Ok(n) => n,
            Err(e) => {
                self.error_message = format!("Błąd w drugiej liczbie: {}", e);
                return;
            }
        };

        // Wykonanie operacji
        let result = match self.operation {
            Operation::Add => num1 + num2,
            Operation::Subtract => num1 - num2,
            Operation::Multiply => num1 * num2,
            Operation::Divide => {
                if num2 == 0 {
                    self.error_message = "Nie można dzielić przez zero!".to_string();
                    return;
                }
                num1 / num2
            }
        };

        // Sprawdzenie czy wynik jest w zakresie liczb rzymskich
        if result <= 0 || result > 3999 {
            self.error_message = "Wynik poza zakresem liczb rzymskich (1-3999)!".to_string();
            return;
        }

        // Konwersja wyniku na liczbę rzymską
        self.result = arabic_to_roman(result);
    }
}

impl eframe::App for RomanCalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Kalkulator Liczb Rzymskich");
            ui.separator();

            ui.add_space(10.0);

            // Pierwsza liczba
            ui.horizontal(|ui| {
                ui.label("Pierwsza liczba:");
                ui.text_edit_singleline(&mut self.input1);
            });

            ui.add_space(5.0);

            // Wybór operacji
            ui.horizontal(|ui| {
                ui.label("Operacja:");
                ui.selectable_value(&mut self.operation, Operation::Add, "➕ Dodawanie");
                ui.selectable_value(&mut self.operation, Operation::Subtract, "➖ Odejmowanie");
                ui.selectable_value(&mut self.operation, Operation::Multiply, "✖ Mnożenie");
                ui.selectable_value(&mut self.operation, Operation::Divide, "➗ Dzielenie");
            });

            ui.add_space(5.0);

            // Druga liczba
            ui.horizontal(|ui| {
                ui.label("Druga liczba:");
                ui.text_edit_singleline(&mut self.input2);
            });

            ui.add_space(10.0);

            // Przycisk oblicz
            if ui.button("Oblicz").clicked() {
                self.calculate();
            }

            ui.add_space(10.0);

            // Wyświetlenie wyniku
            if !self.result.is_empty() {
                ui.horizontal(|ui| {
                    ui.label("Wynik:");
                    ui.colored_label(egui::Color32::GREEN, &self.result);
                });
            }

            // Wyświetlenie błędu
            if !self.error_message.is_empty() {
                ui.colored_label(egui::Color32::RED, &self.error_message);
            }

            ui.add_space(20.0);
            ui.separator();

            // Instrukcje
            ui.collapsing("📖 Instrukcje", |ui| {
                ui.label("• Wprowadź liczby rzymskie (np. XIV, MCMLIV)");
                ui.label("• Obsługiwane liczby: I-MMMCMXCIX (1-3999)");
                ui.label("• Podstawowe symbole: I(1), V(5), X(10), L(50), C(100), D(500), M(1000)");
                ui.label("• Przykłady: IV=4, IX=9, XL=40, XC=90, CD=400, CM=900");
            });


        });
    }
}

// Konwersja z liczby rzymskiej na arabską
fn roman_to_arabic(roman: &str) -> Result<i32, String> {
    if roman.is_empty() {
        return Err("Pusta liczba".to_string());
    }

    let roman = roman.to_uppercase();
    let mut result = 0;
    let mut prev_value = 0;

    for ch in roman.chars().rev() {
        let value = match ch {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => return Err(format!("Nieprawidłowy symbol: {}", ch)),
        };

        if value < prev_value {
            result -= value;
        } else {
            result += value;
        }
        prev_value = value;
    }

    if result <= 0 || result > 3999 {
        return Err("Liczba poza zakresem (1-3999)".to_string());
    }

    Ok(result)
}

// Konwersja z liczby arabskiej na rzymską
fn arabic_to_roman(mut num: i32) -> String {
    let values = [
        (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"),
        (100, "C"), (90, "XC"), (50, "L"), (40, "XL"),
        (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I"),
    ];

    let mut result = String::new();

    for (value, symbol) in values.iter() {
        while num >= *value {
            result.push_str(symbol);
            num -= value;
        }
    }

    result
}

// Tworzenie ikony z rzymską cyfrą IV (4) - Jetbrains AI
fn create_icon_data() -> egui::IconData {
    let size = 32;
    let mut rgba = vec![0u8; size * size * 4];

    // Tło przezroczyste
    for i in (0..rgba.len()).step_by(4) {
        rgba[i + 3] = 0; // Alpha = 0 (przezroczyste)
    }

    // Rysowanie litery "I" (pionowa linia) - przesunięta wyżej i krótsza
    for y in 6..18 { // Zmieniono zakres z 6..26 na 6..18
        for x in 8..11 {
            let idx = (y * size + x) * 4;
            if idx + 3 < rgba.len() {
                rgba[idx] = 255;     // R
                rgba[idx + 1] = 255; // G  
                rgba[idx + 2] = 255; // B
                rgba[idx + 3] = 255; // A
            }
        }
    }

    // Rysowanie litery "V"
    for i in 0..12 {
        // Lewa linia ukośna
        let y = 6 + i;
        let x = 16 + (i / 2);
        if y < 26 && x < 26 {
            for dx in 0..3 {
                let idx = (y * size + (x + dx)) * 4;
                if idx + 3 < rgba.len() {
                    rgba[idx] = 255;
                    rgba[idx + 1] = 255;
                    rgba[idx + 2] = 255;
                    rgba[idx + 3] = 255;
                }
            }
        }

        // Prawa linia ukośna
        let x = 26 - (i / 2);
        if y < 26 && x >= 16 {
            for dx in 0..3 {
                let idx = (y * size + (x + dx)) * 4;
                if idx + 3 < rgba.len() {
                    rgba[idx] = 255;
                    rgba[idx + 1] = 255;
                    rgba[idx + 2] = 255;
                    rgba[idx + 3] = 255;
                }
            }
        }
    }

    egui::IconData {
        rgba,
        width: size as u32,
        height: size as u32,
    }
}



fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([450.0, 300.0])
            .with_min_inner_size([450.0, 300.0])
            .with_icon(create_icon_data()),
        ..Default::default()
    };

    eframe::run_native(
        "Kalkulator Liczb Rzymskich",
        options,
        Box::new(|cc| Ok(Box::new(RomanCalculatorApp::new(cc)))),
    )

}

// Testy jednostkowe
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_arabic() {
        assert_eq!(roman_to_arabic("IV").unwrap(), 4);
        assert_eq!(roman_to_arabic("IX").unwrap(), 9);
        assert_eq!(roman_to_arabic("XIV").unwrap(), 14);
        assert_eq!(roman_to_arabic("MCMLIV").unwrap(), 1954);
        assert_eq!(roman_to_arabic("MMMCMXCIX").unwrap(), 3999);
    }

    #[test]
    fn test_arabic_to_roman() {
        assert_eq!(arabic_to_roman(4), "IV");
        assert_eq!(arabic_to_roman(9), "IX");
        assert_eq!(arabic_to_roman(14), "XIV");
        assert_eq!(arabic_to_roman(1954), "MCMLIV");
        assert_eq!(arabic_to_roman(3999), "MMMCMXCIX");
    }
}