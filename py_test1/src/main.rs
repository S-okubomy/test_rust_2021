// use pyo3::prelude::*;
// use pyo3::types::IntoPyDict;

// fn main() -> PyResult<()> {
//     let arg1 = "arg1";
//     let arg2 = "arg2";
//     let arg3 = "arg3";

//     Python::with_gil(|py| {
//         let fun: Py<PyAny> = PyModule::from_code(
//             py,
//             "def example(*args):
//                 if args != ():
//                     print('called with args', args)
//                 if args == ():
//                     print('called with no arguments')",
//             "",
//             "",
//         )?.getattr("example")?.into();

//         // 引数なしで関数を呼び出す
//         fun.call0(py)?;

//         // 引数を渡して関数を呼び出す
//         let args = (arg1, arg2, arg3);
//         fun.call1(py, args)?;
//         Ok(())
//     })
// }

// use pyo3::prelude::*;

// fn main() -> PyResult<()> {
//     let py_foo = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python_app/utils/foo.py"));
//     let py_app = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python_app/app.py"));
//     let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
//         PyModule::from_code(py, py_foo, "utils.foo", "utils.foo")?;
//         let app: Py<PyAny> = PyModule::from_code(py, py_app, "", "")?
//             .getattr("run")?
//             .into();
//         app.call0(py)
//     });

//     println!("py: {}", from_python?);
//     Ok(())
// }

use pyo3::prelude::*;
use pyo3::types::PyList;
use std::env;

fn main() -> PyResult<()> {
    let py_foo = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python_app/utils/aiNangoQa.py"));
    let py_app = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python_app/app2.py"));
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        PyModule::from_code(py, py_foo, "utils.aiNangoQa", "utils.aiNangoQa")?;
        let app: Py<PyAny> = PyModule::from_code(py, py_app, "", "")?
            .getattr("run")?
            .into();
        app.call0(py)
    });

    let from_python2 = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        // PyModule::from_code(py, py_foo, "utils.aiNangoQa", "utils.aiNangoQa")?;
        let app: Py<PyAny> = PyModule::from_code(py, py_foo, "", "")?
            .getattr("getAns")?
            .into();
        app.call0(py)
    });


    // println!("py: {}", from_python?);

    println!("py: {}", from_python2?);

    Ok(())
}
