pub fn vz(x:i32) -> String {
     if x < 0 {
         "negativ".to_string()
     } else if x == 0 {
         "null".to_string()
     } else {
         return String::from("positiv");
     }
 }