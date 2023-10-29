use crate::{
	config::Config,
	querent::workflow::workflow::{Workflow, WorkflowManager},
};
use std::sync::Arc;
use tokio::sync::mpsc;

#[tokio::test]
async fn test_workflow_manager() {
	let manager = Arc::new(WorkflowManager::new());
	let workflow1 = Workflow {
		name: "Workflow 1".to_string(),
		id: "workflow1".to_string(),
		python_import_path: "asyncio".to_string(),
		python_start_function: "sleep".to_string(),
		python_stop_function: "sleep".to_string(),
		config: Config::default(),
	};
	manager.add_workflow(workflow1.clone()).unwrap();
	assert!(manager.get_workflow("workflow1").is_some());
	assert!(manager.get_workflow("workflow2").is_none());
	let (tx, mut _rx) = mpsc::channel(32);
	let _ = manager.start_workflow("workflow1".to_string(), tx).await;
}

// 	// Test adding and retrieving workflows
// 	let workflow1 = Workflow {
// 		name: "Workflow 1".to_string(),
// 		id: "workflow1".to_string(),
// 		python_import_path: "./python/mock_querent_workflow.py".to_string(),
// 		python_start_function: "start".to_string(),
// 		python_stop_function: "stop".to_string(),
// 		config: Config::default(),
// 	};

// 	manager.add_workflow(workflow1.clone()).unwrap();

// 	assert_eq!(manager.get_workflows().len(), 1);
// 	assert_eq!(manager.get_workflow("workflow1").unwrap().id, workflow1.id);

// 	// Test starting and stopping workflows
// 	let (tx, mut rx) = mpsc::channel(32);

// 	let manager_start = manager.clone();
// 	let manager_end = manager.clone();
// 	let tx_clone = tx.clone();

// 	let start_task = tokio::spawn(async move {
// 		manager_start.start_workflow("workflow1".to_string(), tx_clone).await.unwrap();
// 	});

// 	let stop_task = tokio::spawn(async move {
// 		manager_end.stop_workflow("workflow1".to_string(), tx).await.unwrap();
// 	});

// 	start_task.await.unwrap();
// 	stop_task.await.unwrap();

// 	assert_eq!(rx.recv().await.unwrap(), "Workflow workflow1 has started.");
// 	assert_eq!(rx.recv().await.unwrap(), "Workflow workflow1 has stopped.");
// }
