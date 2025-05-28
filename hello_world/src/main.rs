// Installation und erstes Projekt siehe https://wiki.archlinux.org/title/Rust

use std::fs::File;

mod test;

fn main() { 
    let name = "Anja"; // konstante string kette typ &str
    println!("Hello {}!", name);

    let zahl = 123;
    println!("die erste Zahl ist {}", zahl +1);

    let mut changable_zahl = 123;
    changable_zahl += 1;
    if zahl == changable_zahl {
        print!("die zweite Zahl ist {}\nBis bald {}!\n", changable_zahl,name);
    }
    
    let s1 = String::from("Test"); // heap alloziert typ string
    let mut s2 = s1; // s1 wird ungültig
    println!("{}",s2);
    //println!("{}",s1); // Fehler -> Ownership

    test1(&s2);
    let s3 = s2.clone();
    test2(&mut s2);
    println!("{}",s2); // Wert geändert

    match s2.as_str() {
       "Test Zusatz" => println!("Nicht geändert {}", s2),
        _ => println!("Geändert {}", s2)
    }
    if s3 != s2 {
        println!("geändert {} vs {}", s2,s3);
    }

    test3();
    let x = 4;
    println!("Das Doppelte von {} ist {}\nAußerdem ist diese Zahl {}", x, double(x),vz(x));

    match wurzel(x.into()) {
        Ok(ergebnis) => println!("{}",ergebnis),
        Err(msg) => println!("{}", msg)
    }
    let _ = wurzel(-1.0);

    let mut vektor = vec![1,2]; //Vec::new();
    vektor.push(3);
    println!("{:?}",vektor);
    for zahl in &vektor {
        print!("{}",zahl)
    }
    println!("\nfirst {} or {}",
        match vektor.get(0) { // alternativ mit if let Some(zahl) = vektor.get(0) ...
            Some(zahl) => zahl.to_string(),
            None => "".to_string()
        }, vektor[0]);

    if let Ok(f) = File::open("beispiel.txt") {
        println!("Datei: {:?}", f);
    }
    println!("{:?}",if let Ok(inhalt) = std::fs::read_to_string("beispiel.txt") {inhalt} else {"".to_string()});

    struct Person {
        name: String,
        alter: u32,
    }

    let anja = Person {
        name: "Anja".to_string(),
        alter: 20
    };

    trait Lebewesen {
        fn hallo(&self) -> String;
    }

    impl Lebewesen for Person{
        // fn new(name: &str, alter: u32) -> Self {
        //     Person {
        //         name: name.to_string(),
        //         alter,
        //     }
        // }


        fn hallo(&self) -> String {
            format!("Hallo {}",self.name)
        }
    }
    // let anja = Nutzer::new("Anja",20);

    println!("{} {}",anja.hallo(),anja.alter);


    println!("Vorzeichen von 4 ist {}",test::vz(4))

}

fn test1(text: &String) { // Referenz des Werts wird mitgegeben
    println!("{}",text); // Funktion darf Wert lesen
}

fn test2(text: &mut String) { // Referenz des Werts wird mitgegeben
    let s3 = format!("{} Zusatz", text);
    println!("{}",s3);
    
    text.push_str(" Zusatz");
    println!("{}",text); // Funktion darf Wert lesen und schreiben
}

fn test3() {
    let mut number = 1;
    loop {
        println!("{}",number);
        number +=1;
        if number == 5 {
            break;
        }
    }

    while number != 11 {
        println!("{}", number);
        number += 1;
    }

    for i in 1..=10 {
        println!("{}",i);
    }
}

fn double(x: i32) -> i32 {
    x*2
}

fn vz(x:i32) -> String {
    if x < 0 {
        "negativ".to_string()
    } else if x == 0 {
        "null".to_string()
    } else {
        return String::from("positiv");
    }
}

fn wurzel(x:f64) -> Result<f64,String> {
    if x < 0.0 {
        Err(String::from("Negativ"))
    } else {
        Ok(x.sqrt())
    }
}