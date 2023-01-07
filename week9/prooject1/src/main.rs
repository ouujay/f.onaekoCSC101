
 fn main() {
  use std::fs::File;
use std::io::prelude::*;

 let mut file = File::create("NIGERIAN bREWERY LIMITED")
   .expect("Error encountered while creating file!");
   file.write_all(b"
    LAGER       STOUT       NON-ALCOHOLIC
    33EXPORT    LEGEND      MALTINA
    DESPERADOS  TURBOKING   AMSTEEL MALTINA
    GOLDBERG    WILLIAMS    MALTAGOLD
    GULDER                  FAYROUZ
    HEINEKEN
    STAR
    ")
   .expect("Error while writing to file");
}


