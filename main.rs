const PI:f32 = 3.14;
static mut VARIAVEL_GLOBAL:u8 = 1;

fn main(){
   println!("PI ≈ {}", PI);
   
   unsafe {
      println!("tamanho da variável global é {} bytes.", std::mem::size_of_val(&VARIAVEL_GLOBAL));
   }
   

   let variavel:i8 = 127;
   println!("variável = {}, tamanho  = {} bytes", variavel, std::mem::size_of_val(&variavel));

   let decimal:f32 = 2.5;
   println!("variável = {}", decimal);
   
   let booleana:bool = true;
   println!("Booleana = {}, tamanho booleana = {}", booleana,  std::mem::size_of_val(&booleana));

   let letra:char = 'R';
   println!("Tamanho do char = {}", std::mem::size_of_val(&letra));

}