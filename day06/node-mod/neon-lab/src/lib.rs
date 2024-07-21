use std::{fs::File, io::{ BufReader, Read}};

use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}
fn request_sts_token(mut cx: FunctionContext) -> JsResult<JsObject> {
    let bucket_name = cx.argument::<JsString>(0)?.value(&mut cx);
    let object_key = cx.argument::<JsString>(1)?.value(&mut cx);
    let ttl_seconds = match cx.argument_opt(2)  {
        Some(hv) if hv.is_a::<JsNumber, _>(&mut cx) => {
            let nv = hv.downcast::<JsNumber, _>(&mut cx).unwrap_or(cx.number(3600));
            let n = nv.value(&mut cx);
            n as u32
        },
        _ => 3600u32
    };
    println!("test");
    // cx.throw_error("123")?;

    let token = cx.string("this  is a string");
    let bucket_name = cx.string(bucket_name);
    let object_key = cx.string(object_key);
    let ttl_seconds = cx.number(ttl_seconds);

    let obj = cx.empty_object();
    obj.set(&mut cx, "token", token)?;
    obj.set(&mut cx, "bucketName", bucket_name)?;
    obj.set(&mut cx, "objectKey", object_key)?;
    obj.set(&mut cx, "ttlSeconds", ttl_seconds)?;
    Ok(obj)
}
fn gen_digest(file_name: String) -> String {
    let file = File::open(file_name).unwrap();
    let mut reader = BufReader::new(file);
    // ring::digest::SHA256
    let mut ctx = ring::digest::Context::new(&ring::digest::SHA256);

    let mut buf_reader = [0; 1024*64];

    loop  {
        let n = reader.read(&mut buf_reader).unwrap();
        if n == 0 {
            break;
        }
        ctx.update(&buf_reader[..n]);
    };

    let result = ctx.finish();
    hex::encode(result)
}
fn digest_file(mut cx: FunctionContext) -> JsResult<JsString> {
    let file_name = cx.argument::<JsString>(0)?.value(&mut cx);
    let str = gen_digest(file_name);
    Ok(cx.string(str))
}
#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("requestStsToken", request_sts_token)?;
    cx.export_function("digestFile", digest_file)?;
    Ok(())
}
