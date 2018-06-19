#[macro_use]
extern crate neon;
extern crate fuzzy_phrase;

use neon::mem::Handle;
use neon::vm::{This, Lock, FunctionCall, JsResult};
use neon::js::{JsFunction, Object, JsString, Value, JsUndefined, JsArray, JsBoolean, JsInteger, JsValue};
use neon::js::class::{JsClass, Class};

use fuzzy_phrase::glue::{FuzzyPhraseSetBuilder, FuzzyPhraseSet};

// check argument type: https://github.com/Brooooooklyn/sourcemap-decoder/blob/master/native/src/lib.rs#L21-L29
trait CheckArgument {
  fn check_argument<V: Value>(&mut self, i: i32) -> JsResult<V>;
}

impl<'a, T: This> CheckArgument for FunctionCall<'a, T> {
  fn check_argument<V: Value>(&mut self, i: i32) -> JsResult<V> {
    self.arguments.require(self.scope, i)?.check::<V>()
  }
}

declare_types! {
    pub class JsFuzzyPhraseSetBuilder as JsFuzzyPhraseSetBuilder for Option<FuzzyPhraseSetBuilder> {
        init(mut call) {
            let filename = call
                .check_argument::<JsString>(0)
                ?.value();
            let build = FuzzyPhraseSetBuilder::new(filename).unwrap();
            Ok(Some(build))
        }

        method insert(call) {
            let phrase_array = call.arguments.require(call.scope, 0)?.check::<JsArray>()?;

            let mut v: Vec<String> = Vec::new();

            for i in 0..phrase_array.len() {
                let string = phrase_array.get(call.scope, i)
                ?.check::<JsString>()
                ?.value();

                v.push(string);
            }

            let mut this: Handle<JsFuzzyPhraseSetBuilder> = call.arguments.this(call.scope);

            // let mut result: JsResult<JsValue> = JsUndefined::new().upcast();
            this.grab(|fuzzyphrasesetbuilder| {
                match fuzzyphrasesetbuilder {
                    Some(builder) => {
                        builder.insert(&v[..]).unwrap();
                        // match builder.insert(&v[..]) {
                        //     Ok(()) => {},
                        //     Err(e) => {
                        //         result = JsString::new(call.scope, e.description());
                        //         println!("{:?}", e);
                        //     }
                        // }
                    },
                    None => {
                        // result = JsString::new(call.scope, "ERROR");
                        panic!("FuzzyPhraseSetBuilder not available for insertion");
                    }
                };
            });

            // Ok(result.unwrap())
            // needs error handling
            // match parse() {
            //     Ok(JsUndefined::new().upcast()),
            //     Err(e) => {
            //         println!("{:?}", e);
            //         Ok((JsNull::new().as_value(scope)))
            //     }
            // }
            Ok(JsUndefined::new().upcast())
        }

        method finish(call) {
            let scope = call.scope;
                let mut this: Handle<JsFuzzyPhraseSetBuilder> = call.arguments.this(scope);

                this.grab(|fuzzyphrasesetbuilder| {
                    match fuzzyphrasesetbuilder.take() {
                        Some(builder) => {
                            builder.finish().unwrap();
                        },
                        None => {
                            panic!("SetBuilder not available for finish()");
                        }
                    }
                });
            Ok(JsUndefined::new().upcast())
        }
    }

    pub class JsFuzzyPhraseSet as JsFuzzyPhraseSet for FuzzyPhraseSet {
        init(mut call) {
            let filepath = call
                .check_argument::<JsString>(0)
                ?.value();
            let set = { FuzzyPhraseSet::from_path(filepath).unwrap() };
            Ok(set)
        }

        method contains(call) {
            let phrase_array = call.arguments.require(call.scope, 0)?.check::<JsArray>()?;

            let mut v: Vec<String> = Vec::new();

            for i in 0..phrase_array.len() {
                let string = phrase_array.get(call.scope, i)
                ?.check::<JsString>()
                ?.value();

                v.push(string);
            }

            let mut this: Handle<JsFuzzyPhraseSet> = call.arguments.this(call.scope);

            Ok(JsBoolean::new(
                call.scope,
                this.grab(|set| {
                    set.contains(&v[..]).unwrap()
                })
            ).upcast())
        }

        method contains_prefix(call) {
            let phrase_array = call.arguments.require(call.scope, 0)?.check::<JsArray>()?;

            let mut v: Vec<String> = Vec::new();

            for i in 0..phrase_array.len() {
                let string = phrase_array.get(call.scope, i)
                ?.check::<JsString>()
                ?.value();

                v.push(string);
            }

            let mut this: Handle<JsFuzzyPhraseSet> = call.arguments.this(call.scope);

            Ok(JsBoolean::new(
                call.scope,
                this.grab(|set| {
                    set.contains_prefix(&v[..]).unwrap()
                })
            ).upcast())
        }

        method fuzzy_match(call) {
            let phrase_array = call.arguments.require(call.scope, 0)?.check::<JsArray>()?;
            let max_word_dist: u8 = call.arguments.require(call.scope, 1)?.check::<JsInteger>()
                ?.value() as u8;
            let max_phrase_dist: u8 = call.arguments.require(call.scope, 2)?.check::<JsInteger>()
                ?.value() as u8;

            let mut v: Vec<String> = Vec::new();

            for i in 0..phrase_array.len() {
                let string = phrase_array.get(call.scope, i)
                ?.check::<JsString>()
                ?.value();

                v.push(string);
            }

            let mut this: Handle<JsFuzzyPhraseSet> = call.arguments.this(call.scope);

            this.grab(|set| {
                set.fuzzy_match(&v[..], max_word_dist, max_phrase_dist).unwrap()
            });

            Ok(JsUndefined::new().upcast())
        }

        method fuzzy_match_prefix(call) {
            let phrase_array = call.arguments.require(call.scope, 0)?.check::<JsArray>()?;
            let max_word_dist: u8 = call.arguments.require(call.scope, 1)?.check::<JsInteger>()
                ?.value() as u8;
            let max_phrase_dist: u8 = call.arguments.require(call.scope, 2)?.check::<JsInteger>()
                ?.value() as u8;

            let mut v: Vec<String> = Vec::new();

            for i in 0..phrase_array.len() {
                let string = phrase_array.get(call.scope, i)
                ?.check::<JsString>()
                ?.value();

                v.push(string);
            }

            let mut this: Handle<JsFuzzyPhraseSet> = call.arguments.this(call.scope);

            this.grab(|set| {
                set.fuzzy_match_prefix(&v[..], max_word_dist, max_phrase_dist).unwrap()
            });

            Ok(JsUndefined::new().upcast())
        }
    }
}

register_module!(m, {

    let class: Handle<JsClass<JsFuzzyPhraseSetBuilder>> = try!(JsFuzzyPhraseSetBuilder::class(m.scope));
    let constructor: Handle<JsFunction<JsFuzzyPhraseSetBuilder>> = try!(class.constructor(m.scope));
    try!(m.exports.set("FuzzyPhraseSetBuilder", constructor));

    let class: Handle<JsClass<JsFuzzyPhraseSet>> = try!(JsFuzzyPhraseSet::class(m.scope));
    let constructor: Handle<JsFunction<JsFuzzyPhraseSet>> = try!(class.constructor(m.scope));
    try!(m.exports.set("FuzzyPhraseSet", constructor));

    Ok(())
});
