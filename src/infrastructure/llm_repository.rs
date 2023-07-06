//! Esta es la implementacion usando chatgpt a travez de autogpt,
//! por lo que se esta usando pyo3 para llamar a la libreria autogpt
//! que esta echa en python.

use pyo3::prelude::*;
use pyo3::prepare_freethreaded_python;
use pyo3::types::IntoPyDict;
use tokio::task::JoinHandle;

use crate::domain::error::LlmError;
use crate::domain::llm_repository::{LlmRepository, LlmRepositoryImpl};

impl LlmRepositoryImpl for LlmRepository {
    async fn llm_execute(&self, ai_settings_path: String, workspace_directory: String) -> Result<String, LlmError> {
        let auto_gpt_result: JoinHandle<PyResult<String>> = tokio::task::spawn_blocking(move || {
            prepare_freethreaded_python();
            Python::with_gil(|py| {
                let result = run_auto_gpt(
                    py,
                    Some(true),
                    None,
                    Some(ai_settings_path),
                    None,
                    Some(true),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(workspace_directory),
                    None,
                );
                Ok(result?.to_string())
            })
        });

        let result = auto_gpt_result.await??;
        Ok(result)
    }
}

fn run_auto_gpt(
    py: Python,
    continuous: Option<bool>,
    continuous_limit: Option<u16>,
    ai_settings: Option<String>,
    prompt_settings: Option<String>,
    skip_reprompt: Option<bool>,
    speak: Option<bool>,
    debug: Option<bool>,
    gpt3only: Option<bool>,
    gpt4only: Option<bool>,
    memory_type: Option<String>,
    browser_name: Option<String>,
    allow_downloads: Option<bool>,
    skip_news: Option<bool>,
    workspace_directory: Option<String>,
    install_plugin_deps: Option<bool>,
) -> Result<PyObject, PyErr> {
    let run_auto_gpt: Py<PyAny> = PyModule::import(py, "autogpt.main")?
        .getattr("run_auto_gpt")?
        .into();

    let args: [(String, PyObject); 15] = [
        ("continuous".to_string(), continuous.to_object(py)),
        ("continuous_limit".to_string(), continuous_limit.to_object(py)),
        ("ai_settings".to_string(), ai_settings.to_object(py)),
        ("prompt_settings".to_string(), prompt_settings.to_object(py)),
        ("skip_reprompt".to_string(), skip_reprompt.to_object(py)),
        ("speak".to_string(), speak.to_object(py)),
        ("debug".to_string(), debug.to_object(py)),
        ("gpt3only".to_string(), gpt3only.to_object(py)),
        ("gpt4only".to_string(), gpt4only.to_object(py)),
        ("memory_type".to_string(), memory_type.to_object(py)),
        ("browser_name".to_string(), browser_name.to_object(py)),
        ("allow_downloads".to_string(), allow_downloads.to_object(py)),
        ("skip_news".to_string(), skip_news.to_object(py)),
        ("workspace_directory".to_string(), workspace_directory.to_object(py)),
        ("install_plugin_deps".to_string(), install_plugin_deps.to_object(py)),
    ];
    let args = args.into_py_dict(py);
    let auto_gpt = run_auto_gpt.call(py, (), Some(args))?;
    Ok(auto_gpt)
}
