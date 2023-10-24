class SubjectCallback:
    def handle_event(self, event):
        print("Python: SubjectCallback.handle_event()")

if __name__=="__main__":
  import trait_exposure

  myModel = Model()
  my_rust_model = trait_exposure.UserModel(myModel)
  my_rust_model.set_variables([2.0])
  print("Print value from Python: ", myModel.inputs)
  my_rust_model.compute()
  print("Print value from Python through Rust: ", my_rust_model.get_results())
  print("Print value directly from Python: ", myModel.get_results())
