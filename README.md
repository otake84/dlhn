# DLHN
DLHN is a blazing fast and small data serialization format.  
[Specification](https://dlhn.org)

<p align="center"><a href="https://dlhn.org/" target="_blank" alt="DLHN"><img src="https://user-images.githubusercontent.com/1064585/166881107-9a386366-0ab9-4558-8b81-2a44a32df26c.png"></a></p>

## Overview
DLHN ( Pronounced the same as "Dullahan" ) is a language and platform neutral binary serialization format that is inspired by JSON, CSV, MessagePack, and Protocol Buffers. It is designed for blazing fast serialization and deserialization with the smallest possible data size without the need for schema file.
However, we are also considering supporting schema file in the future.

## QuickStart
```toml
[dependencies]
dlhn = "0.1"
```

## Serialize and deserialize body
```rust
use dlhn::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Test {
    a: bool,
    b: u8,
    c: String,
}

fn main() {
    let body = Test {
        a: true,
        b: 123,
        c: "test".to_string(),
    };

    // Serialize body
    let mut output = Vec::new();
    let mut serializer = Serializer::new(&mut output);
    body.serialize(&mut serializer).unwrap();

    // Deserialize body
    let mut reader = output.as_slice();
    let mut deserializer = Deserializer::new(&mut reader);
    let deserialized_body = Test::deserialize(&mut deserializer).unwrap();

    assert_eq!(body, deserialized_body);
}
```

## Serialize and deserialize header
```rust
use dlhn::{DeserializeHeader, SerializeHeader, Header};

#[derive(SerializeHeader)]
struct Test {
    a: bool,
    b: u8,
    c: String,
}

fn main() {
    let mut output = Vec::new();

    // Serialize header
    Test::serialize_header(&mut output).unwrap();
    assert_eq!(
        output,
        [
            21, // Tuple code
            3,  // Tuple size
            2,  // Boolean code
            3,  // UInt8 code
            18, // String code
        ]
    );

    // Deserialize header
    let deserialized_header = output.as_slice().deserialize_header().unwrap();
    assert_eq!(
        deserialized_header,
        Header::Tuple(vec![Header::Boolean, Header::UInt8, Header::String])
    );
}
```

## Stream version serialize and deserialize bodies
```rust
use dlhn::{de::Error, Deserializer, Serializer};
use serde::{Deserialize, Serialize};

fn main() {
    let mut output = Vec::new();

    // Serialize body
    let mut serializer = Serializer::new(&mut output);
    true.serialize(&mut serializer).unwrap();
    false.serialize(&mut serializer).unwrap();
    assert_eq!(output, [1, 0]);

    // Deserialize body
    let mut reader = output.as_slice();
    let mut deserializer = Deserializer::new(&mut reader);
    assert_eq!(bool::deserialize(&mut deserializer), Ok(true));
    assert_eq!(bool::deserialize(&mut deserializer), Ok(false));
    assert_eq!(bool::deserialize(&mut deserializer), Err(Error::Read));
}
```

## Copyright
Copyright 2020-2022 Shogo Otake

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
