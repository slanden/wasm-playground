wasmtime::component::bindgen!({
  async: false,
  path: "./wit/world.wit",
  additional_derives: [
      Eq,
      PartialEq,
      Ord,
      PartialOrd
  ]
});
