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
	let workflow_manager_res = WorkflowManager::new();
	if let Err(e) = workflow_manager_res {
		panic!("Error creating workflow manager: {}", e);
	}
	let workflow_manager = workflow_manager_res.unwrap();
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

const _CODE: &str = r#"
def print_querent(text: str) -> str:
    """Attempt to parse and execute input as Python"""
    try:
        output = eval(text, globals(), globals())
    except SyntaxError as error:
        try:
            output = exec(compile(text, "<string>", "exec"), globals(), globals())
        except Exception as exception:
            output = f"{exception=}"
    return str(output)
                   "#;

#[pyo3_asyncio::tokio::test]
async fn workflow_manager_python_tests() -> pyo3::PyResult<()> {
	let workflow_manager_res = WorkflowManager::new();
	if let Err(e) = workflow_manager_res {
		panic!("Error creating workflow manager: {}", e);
	}
	let workflow_manager = workflow_manager_res.unwrap();
	//let config = Config::default();
	let mut args: Vec<CLRepr> = Vec::new();
	args.push(CLRepr::String("Querent".to_string(), StringType::Normal));
	let test_flow: Workflow = Workflow {
		name: "test_flow_python".to_string(),
		id: "id2".to_string(),
		import: "".to_string(),
		attr: "print_querent".to_string(),
		arguments: args,
		code: Some(_CODE.to_string()),
	};
	assert!(workflow_manager.add_workflow(test_flow).is_ok());
	match workflow_manager.start_workflows().await {
		Ok(_) => assert!(true),
		Err(e) => panic!("Error starting workflows: {}", e),
	}
	Ok(())
}
