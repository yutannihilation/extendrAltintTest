use extendr_api::prelude::*;

#[derive(Debug, Clone)]
struct MyCompactIntRange {
    start: i32,
    len: i32,
    step: i32,
}

impl AltrepImpl for MyCompactIntRange {
    fn length(&self) -> usize {
        self.len as usize
    }
}

impl AltIntegerImpl for MyCompactIntRange {
    fn elt(&self, index: usize) -> Rint {
        Rint(self.start + self.step * index as i32)
    }
}

/// @export
#[extendr]
fn get_intrange() -> Robj {
    // index 5 is missing
    let mystate = MyCompactIntRange {
        start: 0,
        len: 10,
        step: 1,
    };

    let class = Altrep::make_altinteger_class::<MyCompactIntRange>("cir", "mypkg");
    let obj = Altrep::from_state_and_class(mystate, class.clone(), false);
    let robj = Robj::from(obj.clone());
    let integers: Integers = robj.try_into().unwrap();
    integers.into()
}

extendr_module! {
    mod extendrAltintTest;
    fn get_intrange;
}
