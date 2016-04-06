extern crate rustc_serialize;
use rustc_serialize::json;

// ### This part goes in your library ###

trait TypeIdentifier {
    /// Return the identifier for the type.
    fn identifier() -> &'static str;
}

trait MyEncodable {
    /// Serialize an object as some JSON. Behaves kinda like json::encode for enums.
    fn serialize(&self) -> json::Json;
}

impl<T: TypeIdentifier + rustc_serialize::Encodable> MyEncodable for T {
    fn serialize(&self) -> json::Json {
        let tn = String::from(T::identifier());
        let mut o = json::Object::new();
        // Serialize the type identifier
        o.insert("identifier".into(), json::Json::String(tn));
        // Serialize the value (this code is dumb)
        o.insert("value".into(), json::Json::from_str(json::encode(self).unwrap().as_ref()).unwrap());
        json::Json::Object(o)
    }
}

// ### This part goes in the user program ###

#[derive(RustcDecodable, RustcEncodable)]
struct A { x: i32, y: i32 }

impl TypeIdentifier for A {
    fn identifier() -> &'static str { "A" }
}

// ### Example of how that can be used ###

fn main() {
    let a = A { x: 5, y: 10 };
    let a: Box<MyEncodable> = Box::new(a);

    println!("{}", a.serialize());
}
