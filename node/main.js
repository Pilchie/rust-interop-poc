const cosmos = require('../rust/azure_data_cosmos_shared');

let orig = '{ "name": "Kevin" }';
let encoded = cosmos.node_encode(orig);
let decoded = cosmos.node_decode(encoded)
console.log("Roundtripped: '", orig, "' to get '", decoded, "'. It was", encoded.length, "bytes when encoded.")
