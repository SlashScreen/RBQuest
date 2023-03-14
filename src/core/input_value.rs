use magnus::{value, RArray, Value, QFALSE, QTRUE};

pub enum InputValue {
    Boolean(bool),
    Input2D { x: f64, y: f64 },
    Input3D { x: f64, y: f64, z: f64 },
}

impl InputValue {
    pub fn into_rarray(&self) -> RArray {
        let args = RArray::new();
        match self {
            InputValue::Boolean(b) => args
                .push(if *b {
                    Value::from(QTRUE)
                } else {
                    Value::from(QFALSE)
                })
                .unwrap(),
            InputValue::Input2D { x, y } => {
                args.push(Value::from(*x)).unwrap();
                args.push(Value::from(*y)).unwrap();
            }
            InputValue::Input3D { x, y, z } => {
                args.push(Value::from(*x)).unwrap();
                args.push(Value::from(*y)).unwrap();
                args.push(Value::from(*z)).unwrap();
            }
        };
        args
    }
}
