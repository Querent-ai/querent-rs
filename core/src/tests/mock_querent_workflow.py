class MockQuerentWorkflow:
    def __init__(self, config):
        self.config = config

    def start(self):
        print("Mock workflow started with configuration:")
        print(self.config)

    def stop(self):
        print("Mock workflow stopped.")


def start(config):
    workflow = MockQuerentWorkflow(config)
    workflow.start()


def stop():
    workflow = MockQuerentWorkflow(None)  # No need for config to stop
    workflow.stop()
