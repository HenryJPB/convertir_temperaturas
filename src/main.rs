use std::io;

// -------------------------------------------------------------
// Bqto : 28 de Agosto 2026 10:39   
// convertir_temperaturas.rs  
// Aistido x Google Gemini. 
// Autor : HenryJPB  
// -------------------------------------------------------------
fn main() {
    println!("Hello, world!");
    println!("===========================================");
    println!("  * Convertir Grados Celsius ⇆ Farenheit * ");
    println!("===========================================");

    let c = Celsius { valor_convertir: 71.0 };
    println!("Resultado: {:.2}", c.resultado_conversion());
  
    let mut opcion: Option<char> = None;
    // let opcion : char = null;  // Error!!!
    // NOTA: La estrategia consiste en un 1er loop basado en un while q ejecuta una serie de tareas hasta q el usuario 
    //       T)ermine; y el 2do basado en loop dentro de un Clousure para certificar que el usuario tipee solo el Char 
    //       del menu de opciones !!  
    while opcion != Some('T') {
        println!();
        println!("=============================================================");
        println!(" MI MENU:");
        println!("=========");
        println!(" - Convertir Farenheit a C)elsius");
        println!(" - Convertir Celsius a   F)arenheit");
        println!("                         T)erminar");

        // loop aqui es un ejemplo de funcion Anonima !
        opcion = loop {
            let mut entrada = String::new();
            println!("Ingresa una opción ('F', 'C' o 'T'):");

            io::stdin()
                .read_line(&mut entrada)
                .expect("Error al leer la línea");

            let texto_limpio = entrada.trim().to_uppercase();

            match texto_limpio.chars().next() {
                Some('F') | Some('C') | Some('T') if texto_limpio.len() == 1 => {
                    break texto_limpio.chars().next(); // Devuelve directamente Some(char)
                }
                _ => println!(" Opción inválida. Solo se permite 'F', 'C' o 'T'.\n"),
            }
        }; // loop de opcion 

        if let Some(c) = opcion {
             if c != 'T' {
                procesar_conversion(c);
            }
        }
    }   
        // Usamos .unwrap() o un match para mostrar el char limpio  
        //if let Some(c) = opcion {
        //     println!("Seleccionaste la opción: {}", c );     
    println!("."); 
    println!("..");
    println!("...");
    println!("==================(THAT'S ALL FOLKS)====================");

} // main() 

//-----------------------------------------------------------------------
fn procesar_conversion( opcion : char ) {
    println!("Introduce un valor de temperatura en grados a convertir ( solo numeros ) :");  

    // 1. Creación de la variable String mutable para almacenar la entrada del teclado
    let mut entrada = String::new();

    // 2. Leemos la línea y la guardamos en la cadena de texto
    io::stdin()
        .read_line(&mut entrada)
        .expect("Error al leer la línea");

    // 3. Limpiamos espacios/saltos de línea con .trim() y convertimos a f32 con .parse()
    let valor_temperatura: f32 = entrada
        .trim()
        .parse()
        .expect("Por favor, ingresa un número válido");

    //println!("La temperatura ingresada es: {}", valor_temperatura);

    println!("You valor_temperatura : {valor_temperatura}");
    
    match opcion  {
        'C' => {
                println!("*Gestion convertir grados Farenheit a Celsius *"); 
                let c = Celsius { valor_convertir: valor_temperatura };
                println!("RESULTADO conversion {:.2} Farenheit 👉 Celsius= {:.2}",c.valor_convertir, c.resultado_conversion()); 
                println!();  
            }
        'F' => {
                println!("*Gestion convertir grados Celsius a Farenheit *"); 
                let f = Farenheit { valor_convertir :  valor_temperatura };
                println!("RESULTADO conversion {:.2} Celsius 👉 Farenheit= {:.2} ", f.valor_convertir,f.resultado_conversion());
            }   
        _ => {
                println!("*Gestion convertir NO APLICA *"); 
            }   
    } // match

} // procesar_conversion 

// Definimos el contrato: quien use este trait debe saber "imprimir_resumen"
pub trait ConvertirTemperatura {
    fn resultado_conversion(&self) -> f32;   
} // trait Temperatura

struct Celsius {
    valor_convertir : f32,
}

struct Farenheit {
    valor_convertir : f32, 
}

// Implementamos el trait para Celsius - recibiendo el valor en FaFactura
impl ConvertirTemperatura for Celsius {
    
    fn resultado_conversion(&self) -> f32 {
        ( self.valor_convertir - 32.00 ) * ( 5.00 / 9.00  )
    }
    
}

// Implementamos el trait para Farenheit
impl ConvertirTemperatura for Farenheit {
    fn resultado_conversion(&self) -> f32 {
        self.valor_convertir * ( 9.00 / 5.00 ) + 32.00       
    }

}
