use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::path::PathBuf;

use dats::dat_format::DatFormat;
use dats::formats::auto_translate::AutoTranslate;
use dats::formats::dialog::Dialog;
use dats::formats::dmsg_table::DmsgTable;
use dats::formats::entity_names::EntityNames;
use dats::formats::events::Events;
use dats::formats::furniture_data::FurnitureData;
use dats::formats::item_info::ItemInfoTable;
use dats::formats::menu_table::MenuTable;
use dats::formats::status_info::StatusInfoTable;
use dats::formats::xistring_table::XiStringTable;

fn serde_to_pyobject(py: Python, val: &serde_json::Value) -> PyObject {
    match val {
        serde_json::Value::Null => py.None(),
        serde_json::Value::Bool(b) => b.into_pyobject(py).unwrap().to_owned().into_any().unbind(),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                i.into_pyobject(py).unwrap().into_any().unbind()
            } else if let Some(f) = n.as_f64() {
                f.into_pyobject(py).unwrap().into_any().unbind()
            } else {
                py.None()
            }
        }
        serde_json::Value::String(s) => s.into_pyobject(py).unwrap().into_any().unbind(),
        serde_json::Value::Array(arr) => {
            let items: Vec<PyObject> = arr.iter().map(|v| serde_to_pyobject(py, v)).collect();
            let list = PyList::new(py, &items).unwrap();
            list.into_pyobject(py).unwrap().into_any().unbind()
        }
        serde_json::Value::Object(map) => {
            let dict = PyDict::new(py);
            for (k, v) in map {
                dict.set_item(k, serde_to_pyobject(py, v)).unwrap();
            }
            dict.into_pyobject(py).unwrap().into_any().unbind()
        }
    }
}

fn parse_format<T: DatFormat + serde::Serialize>(py: Python, path: &str) -> PyResult<PyObject> {
    let dat_path = PathBuf::from(path);
    let data = T::from_path(&dat_path)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Parse error: {e}")))?;
    let json_val = serde_json::to_value(&data)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Serialize error: {e}")))?;
    Ok(serde_to_pyobject(py, &json_val))
}

#[pyfunction]
fn parse_menu_table(py: Python, path: &str) -> PyResult<PyObject> {
    parse_format::<MenuTable>(py, path)
}

#[pyfunction]
fn parse_dialog(py: Python, path: &str) -> PyResult<PyObject> {
    parse_format::<Dialog>(py, path)
}

#[pyfunction]
fn parse_dmsg_table(py: Python, path: &str) -> PyResult<PyObject> {
    parse_format::<DmsgTable>(py, path)
}

#[pyfunction]
fn parse_entity_names(py: Python, path: &str) -> PyResult<PyObject> {
    parse_format::<EntityNames>(py, path)
}

#[pyfunction]
fn parse_events(py: Python, path: &str) -> PyResult<PyObject> {
    parse_format::<Events>(py, path)
}

#[pyfunction]
fn parse_item_info(py: Python, path: &str) -> PyResult<PyObject> {
    parse_format::<ItemInfoTable>(py, path)
}

#[pyfunction]
fn parse_status_info(py: Python, path: &str) -> PyResult<PyObject> {
    parse_format::<StatusInfoTable>(py, path)
}

#[pyfunction]
fn parse_xistring_table(py: Python, path: &str) -> PyResult<PyObject> {
    parse_format::<XiStringTable>(py, path)
}

#[pyfunction]
fn parse_auto_translate(py: Python, path: &str) -> PyResult<PyObject> {
    parse_format::<AutoTranslate>(py, path)
}

#[pyfunction]
fn parse_furniture_data(py: Python, path: &str) -> PyResult<PyObject> {
    parse_format::<FurnitureData>(py, path)
}

#[pymodule]
fn xi_tinkerer(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_menu_table, m)?)?;
    m.add_function(wrap_pyfunction!(parse_dialog, m)?)?;
    m.add_function(wrap_pyfunction!(parse_dmsg_table, m)?)?;
    m.add_function(wrap_pyfunction!(parse_entity_names, m)?)?;
    m.add_function(wrap_pyfunction!(parse_events, m)?)?;
    m.add_function(wrap_pyfunction!(parse_item_info, m)?)?;
    m.add_function(wrap_pyfunction!(parse_status_info, m)?)?;
    m.add_function(wrap_pyfunction!(parse_xistring_table, m)?)?;
    m.add_function(wrap_pyfunction!(parse_auto_translate, m)?)?;
    m.add_function(wrap_pyfunction!(parse_furniture_data, m)?)?;
    Ok(())
}
