mod explicit;
mod method;
mod _struct;
mod _trait;
mod constraint;
mod transform;
mod _static;

pub fn exec_all_fn() {
  explicit::failed_borrow();

  let num = 4i32;
  method::invalid_output(&num);

  _struct::exec_structs();

  let t1 = 10;
  let t = constraint::Ref(&t1);
  constraint::print_cons_life(&t);

  transform::exec_transform_fn();
}