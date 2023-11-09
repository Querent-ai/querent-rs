use querent_rs::{
	config::Config,
	querent::workflow::{Workflow, WorkflowManager},
};

#[pyo3_asyncio::tokio::main]
async fn main() -> pyo3::PyResult<()> {
	pyo3_asyncio::testing::main().await
}

#[pyo3_asyncio::tokio::test]
async fn test_basic() -> pyo3::PyResult<()> {
	Ok(())
}

#[pyo3_asyncio::tokio::test]
async fn workflow_manager_basic_tests() -> pyo3::PyResult<()> {
	let workflow_manager = WorkflowManager::new();
	let config = Config::default();
	let test_flow: Workflow = Workflow {
		name: "test_flow_basic".to_string(),
		id: "id1".to_string(),
		python_import_path: "asyncio".to_string(),
		python_start_function: "sleep".to_string(),
		arguments: config,
	};
	assert!(workflow_manager.add_workflow(test_flow).is_ok());
	Ok(())
}
