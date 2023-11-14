use querent_rs::{
	cross::{CLRepr, StringType},
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
	//let config = Config::default();
	let mut args: Vec<CLRepr> = Vec::new();
	args.push(CLRepr::Int(1));
	let test_flow: Workflow = Workflow {
		name: "test_flow_basic".to_string(),
		id: "id1".to_string(),
		import: "asyncio".to_string(),
		attr: "sleep".to_string(),
		arguments: args,
		code: None,
	};
	assert!(workflow_manager.add_workflow(test_flow).is_ok());
	match workflow_manager.start_workflows().await {
		Ok(_) => assert!(true),
		Err(e) => panic!("Error starting workflows: {}", e),
	}
	Ok(())
}

// const CODE: &str = r#"
// def function(*args):
//     assert args == ("hello",)
//     return "called with args and kwargs"
// "#;

// #[pyo3_asyncio::tokio::test]
// async fn workflow_manager_with_code_tests() -> pyo3::PyResult<()> {
// 	let workflow_manager = WorkflowManager::new();

// 	let mut args: Vec<CLRepr> = Vec::new();
// 	args.push(CLRepr::String("Hello".to_string(), StringType::Normal));

// 	let test_flow_with_code: Workflow = Workflow {
// 		name: "test_flow_with_code".to_string(),
// 		id: "id2".to_string(),
// 		import: "".to_string(),
// 		attr: "function".to_string(),
// 		arguments: args,
// 		code: Some(CODE.to_string()),
// 	};

// 	assert!(workflow_manager.add_workflow(test_flow_with_code).is_ok());

// 	match workflow_manager.start_workflows().await {
// 		Ok(_) => {
// 			assert!(true);
// 		},
// 		Err(e) => panic!("Error starting workflows: {}", e),
// 	}
// 	Ok(())
// }
