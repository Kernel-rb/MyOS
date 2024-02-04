#[allow(dead_code)] // allow unused code
const BUFFER_HEIGHT: usize = 25; // hauteur du buffer
const BUFFER_WIDTH: usize = 80; // largeur du buffer



#[derive(Debug, Clone, Copy, PartialEq, Eq)] // utilise derive pour générer des implémentations de traits
#[repr(C)] // représente la structure sous forme de C
struct ScreenChar{ // bit de caractère à afficher
    ascii_character: u8, // caractère à afficher
    color_code: ColorCode, // code de couleur
}
struct ColorCode(u8); // représente le code de couleur
impl ColorCode { 
    fn new(foreground: Color, background: Color) -> ColorCode { // fonction qui retourne un ColorCode
        ColorCode((background as u8) << 4 | (foreground as u8)) // retourne le code de couleur
    }
}


#[repr(transparent)]  // assurer que la taille de la structure est égale à celle de son champ
struct Buffer { // structure qui représente le buffer
    chars:[[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT], // tableau de caractères
}


#[repr(u8)] // représente le type de donnée sous forme d'un u8
pub enum  Color { // enumération des couleurs 
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

pub struct Writer { // structure qui représente l'écran
    column_position: usize, // position de la colonne
    color_code: ColorCode, // code de couleur
    buffer: &'static mut Buffer, // buffer avec lifetime statique ( https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotation-syntax)
    // valid pour toute la durée du programme
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8){
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH { // si la position de la colonne est supérieure à la largeur du buffer retourne à la ligne
                    self.new_line();
                }
                let row = BUFFER_HEIGHT - 1; // la ligne est égale à la hauteur du buffer - 1
                let col = self.column_position; // la colonne est égale à la position de la colonne
                let color_code = self.color_code; // ne change pas la couleur
                self.buffer.chars[row][col] = ScreenChar { // caractère à afficher
                    ascii_character: byte,  // caractère à afficher
                    color_code, // code de couleur
                };
                self.column_position += 1; // incrémente la position de la colonne
            }
        }
    }
    fn new_line(&mut self){
        /*!TODO */
    }
}

impl Writer{
    pub fn write_string(&mut self , s: &str){
        for byte in s.bytes(){
            match byte {
                // print only ASCII byte or new line
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // if not an ASCII byte print a ■
                _ => self.write_byte(0xfe),// 0xfe is a solid block character (■)
            }
        }
    }

}


pub fn print_something(){
    let mut writer =  Writer{
        column_position: 0,
        color_code: ColorCode::new(Color::Red, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }, // on utilise unsafe pour accéder à la mémoire
    };
    writer.write_string(b'I'); // b is used to specify a byte literal , pour spécifier un type de donnée de byte
    writer.write_string("'m");
    writer.write_string("Alive!");
    
}

