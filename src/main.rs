
use sysinfo::{System, SystemExt};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();
//print information
    let usdram = sys.used_memory()/1000;
    let ttlram = sys.total_memory()/1000;
    let os = std::env::consts::OS;
    let fam = std::env::consts::FAMILY;
    let cpu = std::env::consts::ARCH;
   
 //print linux
 if os == "linux"{
    println!("
     .--.      OS     > {os}
    |o_o |     family > {fam}
    |:_/ |     cpu    > {cpu}
   //   ' '    ram    > {usdram}M / {ttlram}M
  (|     | )   
  _'_   __/    
 (___)=(___)
 ")
    }
 
 //print macOS
 if os == "macos"{
    println!("
        _
   ___ |'__
  /   --/  `.   OS     > {os}
 |        /     family > {fam}
 |        |     cpu    > {cpu}
  |        |    ram    > {usdram}M / {ttlram}M
   `__--__/
 " )
 }
} 
