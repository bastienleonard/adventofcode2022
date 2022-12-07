pub mod ex1;
pub mod ex2;
pub mod ex3;
pub mod ex4;

fn main() {
    assert!(ex1::ex1_1() == 65912);
    assert!(ex1::ex1_2() == 195625);
    assert!(ex2::ex2_1() == 13675);
    assert!(ex2::ex2_2() == 14184);
    assert!(ex3::ex3_1() == 7568);
    assert!(ex3::ex3_2() == 2780);
    assert!(ex4::ex4_1() == 441);
    assert!(ex4::ex4_2() == 861);
}
