use rocket_contrib::templates::handlebars::{
    Context, Handlebars, Helper, Output, RenderContext, RenderError, Renderable,
};
use serde_json::Value;

pub fn if_exists_helper<'reg, 'rc>(
    h: &Helper<'reg, 'rc>,
    r: &'reg Handlebars<'reg>,
    ctx: &'rc Context,
    rc: &mut RenderContext<'reg, 'rc>,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h
        .param(0)
        .ok_or_else(|| RenderError::new("Param not found for helper \"if_exists\""))?;

    let value = !param.is_value_missing();

    let tmpl = if value { h.template() } else { h.inverse() };

    match tmpl {
        Some(ref t) => t.render(r, ctx, rc, out),
        None => Ok(()),
    }
}

pub fn if_not_null_helper<'reg, 'rc>(
    h: &Helper<'reg, 'rc>,
    r: &'reg Handlebars<'reg>,
    ctx: &'rc Context,
    rc: &mut RenderContext<'reg, 'rc>,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h
        .param(0)
        .ok_or_else(|| RenderError::new("Param not found for helper \"if_exists\""))?;

    let value = *param.value() != Value::Null;

    let tmpl = if value { h.template() } else { h.inverse() };

    match tmpl {
        Some(ref t) => t.render(r, ctx, rc, out),
        None => Ok(()),
    }
}
