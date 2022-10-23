use regex::Regex;

fn main() {
    println!("Hello, world!");

    // Regex
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_mult = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();

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

    // loop multiplicacion
    loop {
        // Aplicar Operaciones
        let caps = re_mult.captures(expresion.as_str());

        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let cap_expresion = caps.get(0).unwrap().as_str();

        // Aqui primero se accede a la posicion 1, se pasa a str, luego se parsea a i32 y un ultimo unwrap
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        println!("{:?} izq: {} der: {}", caps, left_value, right_value); // :? dentro de los corchetes hace que de mas informacion

        let mult = left_value * right_value;

        expresion = expresion.replace(cap_expresion, &mult.to_string()); // amperson al principio transforma un String a un str
    }

    // loop suma
    loop {
        // Aplicar Operaciones
        let caps = re_add.captures(expresion.as_str());

        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let cap_expresion = caps.get(0).unwrap().as_str();

        // Aqui primero se accede a la posicion 1, se pasa a str, luego se parsea a i32 y un ultimo unwrap
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        println!("{:?} izq: {} der: {}", caps, left_value, right_value); // :? dentro de los corchetes hace que de mas informacion

        let suma = left_value + right_value;

        expresion = expresion.replace(cap_expresion, &suma.to_string()); // amperson al principio transforma un String a un str
    }

    // Mostrar resultado
    println!("Resultado: {}", expresion);
}
