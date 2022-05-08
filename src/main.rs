mod sub;
use sub::sub;

fn main() {
  // use fn without "use" expression at import
  // sub::sub();
  // only works when sub fn is imported with `use sub::sub;`
  sub();

}