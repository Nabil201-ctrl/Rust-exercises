//Simulate a coin flip (random true/false using rand crate).
use rand:Rng;


enum Coinflip{
    Head(bool),
    Tail(bool)

}


fn main (){
    let mut flip = rand::thread_rng();
    let randomBool = flip.gen::<bool>();

    if randomBool == true {
        Coinflip::Head(true);

    }
    else {
        Coinflip::Tail(false);
    }
}
