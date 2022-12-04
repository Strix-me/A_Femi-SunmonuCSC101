fn main() {
   //initialization of tuple with data types
   let datatype_tuple: (&str, f32, u8) = ("Rust", 3.14, 100);
   println!("Tuple Contents = {:?}", datatype_tuple );

   //initialization of tuple withput data type
   let no_datatype_tuple = ("Rust", "fun", 100);
   println!("Tuple Contents ={:?}", datatype_tuple );

   //accessing tuple element at index 0
   println!("Value at index 0 = {}",datatype_tuple.0 );

   //accessing tuple element at index 1
   println!("Value at index 1 = {}",datatype_tuple.1 );

   //acessing tuple element at index 2
   println!("Value ar Index 2 = {}", datatype_tuple.2 );
}
