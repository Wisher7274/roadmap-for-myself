pub mod garden;
pub mod kitchen;
use kitchen::qin_ling;
use garden::vegetables::Asparagus;

fn main() {
    let vg1 = Asparagus {
        name: String::from("Asparagus"),
    };
    vg1.echo_name();
    
    qin_ling::cook_asparagus(vg1);
}
