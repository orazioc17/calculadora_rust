use regex::Regex;

enum TipoOperacion {
    SUMA,
    RESTA,
    MULTIPLICACION,
    DIVISION,
}

fn loop_operaciones(string_regex: Regex, mut expresion: String, tipo_operacion: TipoOperacion) -> String {
    loop {
        // Aplicar Operaciones
        let caps = string_regex.captures(expresion.as_str());

        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let cap_expresion = caps.get(0).unwrap().as_str();

        // Aqui primero se accede a la posicion 1, se pasa a str, luego se parsea a f32 y un ultimo unwrap
        let left_value: f32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: f32 = caps.get(2).unwrap().as_str().parse().unwrap();

        println!("{:?} izq: {} der: {}", caps, left_value, right_value); // :? dentro de los corchetes hace que de mas informacion
        println!("{}", expresion);
        let resultado: f32;

        match tipo_operacion {
            TipoOperacion::SUMA => resultado = left_value + right_value,
            TipoOperacion::RESTA => resultado = left_value - right_value,
            TipoOperacion::MULTIPLICACION => resultado = left_value * right_value,
            TipoOperacion::DIVISION => resultado = left_value / right_value,
        }

        expresion = expresion.replace(cap_expresion, &resultado.to_string()); // amperson al principio transforma un String a un str
    }
    return expresion;
}

fn main() {

    // Regex
    // Esta es la regex que me funciono para este caso y que desarrolle sobre la marcha, buscar mejor solucion
    let re_suma = Regex::new(r"(\d+\.?\d*)\s*\+\s*(\d+\.?\d*)").unwrap();
    let re_mult = Regex::new(r"(\d+\.?\d*)\s*\*\s*(\d+\.?\d*)").unwrap();
    let re_resta = Regex::new(r"(\d+\.?\d*)\s*\-\s*(\d+\.?\d*)").unwrap();
    let re_division = Regex::new(r"(\d+\.?\d*)\s*/\s*(\d+\.?\d*)").unwrap();

    // (\d+) \s? \+ \s? (\d+)
    /*
    Regex dividida en 5 partes
    Esta regex va a buscar primero un numero de uno o mas digitos
    Luego va a buscar un espacio que seria opcional
    Luego un signo de mas
    Luego otro espacio opcional
    Y por ultimo el otro numero
    Los espacios opcionales son por si el usuario ingresa por ejemplo 1 +2 o 2+ 34 o 3 + 6
    Y pues tambien sirve sin los espacios opcionales 2+5
    */

    // Traer datos del usuario
    let mut expresion = String::new();
    println!("Ingrese la expresion: ");
    std::io::stdin().read_line(&mut expresion).unwrap();
 
    expresion = loop_operaciones(re_mult, expresion, TipoOperacion::MULTIPLICACION);
    expresion = loop_operaciones(re_division, expresion, TipoOperacion::DIVISION);
    expresion = loop_operaciones(re_suma, expresion, TipoOperacion::SUMA);
    expresion = loop_operaciones(re_resta, expresion, TipoOperacion::RESTA);

    // Mostrar resultado
    println!("Resultado: {}", expresion);
}
