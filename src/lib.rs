use neon::prelude::*;

mod cleandoc;
mod docstrings;
mod extract;
mod generate;

fn get_markdown(mut cx: FunctionContext) -> JsResult<JsString> {
    let code = cx.argument::<JsString>(0).unwrap().value(&mut cx);

    let markdown = generate::generate(&code);

    Ok(cx.string(markdown))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("get_markdown", get_markdown)?;
    Ok(())
}
