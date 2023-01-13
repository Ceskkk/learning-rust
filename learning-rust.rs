// ! En Rust SIEMPRE usamos el punto y coma
/*
  Para usar programas de rust tenemos que instalar antes el compilador.
  Para compialr un programa usamos 'rustup {filePath}'.
  Esto creará un .exe que podemos ejecutar como cualquier otro programa '{filePath}'
*/

// ? Funciones
// Los programas de rust deben tener una funcion 'main'. 
// Es la primera que el compilador ejecuta.
fn main() {
  // ? Inicialización de variables
  // La variable creada con 'let' indica que es inmutable, es decir, no se puede reasignar un valor.
  // Para hacer una variable mutable lo indicamos con 'mut' delante del 'let'
  // También podemos asignar variables con 'const', pero son constantes en tiempo de compilación (ni mutar ni reasignar).
  let name = "Cesk";
  let name2;
  name2 = "Cesk inmutable asignado mas tarde";
  let mut name3 = "Cesk Mutable";
  name3 = "Cesk Mutado";
  const LUCKY_NUMBER: i32 = 7;
  // * Nunca se puede cambiar el tipo de variable, solo mutarlo.
  // * En ningún caso hace falta asignar la variable desde un principio, se puede hacer mas tarde.
  // * En las 'const' es obligatorio asignar el tipo

  // ? Escribir en consola
  // Este es nuestro primer Hello World.
  // Entedemos como escapar las dobles comillas y a interpolar variables
  println!("Hello \"World\"");
  println!("Hola, {name}, tu número favorito es el {LUCKY_NUMBER}");
  println!("Hola, {} {}", name2, name3)
  // * La exclamación antes de los parentesis indica que println es una 'macro'.
  // * Una macro es una forma de generar código en tiempo de compilación.

  // ? Tipos de variables
  // * Escalares
  // Bool
  // Char
  // Integers -> Sin decimales 
    // (i8, i16, i32, i64, i128, isize) Positivos y negativos
    // (u8, u16, u32, u64, u128, usize) Solo positivos
    // isize y usize dependen de la arquitectura de la maquina
  // Float -> Con decimales (f32, f64)
  // * Compuestos

}