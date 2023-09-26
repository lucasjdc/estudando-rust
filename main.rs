fn main(){
   let variavel:i32 = 127;
   println!("variavel = {}, tamanho  = {} bytes", variavel, std::mem::size_of_val(&variavel));

   let decimal:f32 = 2.5;
   println!("variavel = {}", decimal);
   
   let booleana:bool = true;
   println!("Booleana = {}, tamanho booleana = {}", booleana,  std::mem::size_of_val(&booleana));

   let letra:char = 'C';
   println!("Tamanho do char = {}", std::mem::size_of_val(&letra));

}