//minimal example, S and ca without * import
use polars::prelude as pl;
fn main() {
    println!("Hello, world!");
    use pl::NamedFrom; //this trait is needed to create the
    let s: pl::Series = pl::Series::new("foo", &["a", "b", "c"]);
    let ca_utf8 = s.utf8(); //no traits needed
    println!("hello chunky {:?}", ca_utf8);
}
