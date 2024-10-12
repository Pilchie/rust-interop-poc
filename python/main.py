import azure_data_cosmos_shared as azure_data_cosmos_shared

orig = '{ "name": "Kevin" }'
encoded = azure_data_cosmos_shared.python_encode(orig)
roundtripped = azure_data_cosmos_shared.python_decode(encoded)

print(f"Roundtripped: ' {orig} ' to get ' {roundtripped} '. It was {len(encoded)} bytes when encoded.")
