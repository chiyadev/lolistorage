use once_cell::sync::Lazy;
use rocket_contrib::templates::handlebars::{
    Context, Handlebars, Helper, Output, RenderContext, RenderError, Renderable,
};
use serde_json::Value;
use std::collections::HashMap;

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
        .ok_or_else(|| RenderError::new("Param not found for helper \"if_not_null\""))?;

    let value = *param.value() != Value::Null;

    let tmpl = if value { h.template() } else { h.inverse() };

    match tmpl {
        Some(ref t) => t.render(r, ctx, rc, out),
        None => Ok(()),
    }
}

macro_rules! insert_many {
    ($map:ident, $value: expr, $($key:expr),*) => {
        $(
            $map.insert($key, $value);
        )*
    }
}

// https://www.computerhope.com/issues/ch001789.htm
static ICON_EXTS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut map = HashMap::new();

    insert_many!(
        map,
        "icons/music-note",
        ".aif",
        ".cda",
        ".mid",
        ".mp3",
        ".mpa",
        ".ogg",
        ".wav",
        ".wma",
        ".wpl"
    );

    insert_many!(
        map,
        "icons/archive",
        ".7z",
        ".arj",
        ".deb",
        ".gz",
        ".pkg",
        ".rar",
        ".rpm",
        ".tar",
        ".tar.gz",
        ".z",
        ".zip"
    );

    insert_many!(
        map,
        "icons/database",
        ".bin",
        ".dmg",
        ".iso",
        ".toast",
        ".vcd",
        ".dat",
        ".db",
        ".dbf",
        ".mdb",
        ".sav",
        ".sql",
        ".pdb"
    );

    insert_many!(
        map,
        "icons/mail",
        ".email",
        ".eml",
        ".emlx",
        ".msg",
        ".oft",
        ".ost",
        ".pst",
        ".vcf"
    );

    insert_many!(
        map,
        "icons/terminal",
        ".apk",
        ".bat",
        ".cmd",
        ".com",
        ".exe",
        ".gadget",
        ".jar",
        ".msi",
        ".sh",
        ".wsf",
        ".command"
    );

    insert_many!(
        map,
        "icons/photograph",
        ".ai",
        ".bmp",
        ".gif",
        ".ico",
        ".jpeg",
        ".jpg",
        ".png",
        ".ps",
        ".psd",
        ".svg",
        ".tif",
        ".tiff"
    );

    insert_many!(
        map,
        "icons/presentation-chart-bar",
        ".key",
        ".odp",
        ".pps",
        ".ppt",
        ".pptx"
    );

    insert_many!(
        map,
        "icons/code",
        ".asp",
        ".aspx",
        ".cer",
        ".cfm",
        ".cgi",
        ".css",
        ".htm",
        ".html",
        ".js",
        ".jsp",
        ".md",
        ".part",
        ".php",
        ".py",
        ".rss",
        ".xhtml",
        ".c",
        ".class",
        ".cpp",
        ".cs",
        ".h",
        ".java",
        ".pl",
        ".swift",
        ".vb",
        ".sln",
        ".rs"
    );

    insert_many!(map, "icons/table", ".ods", ".xls", ".xlsm", ".xlsx");

    insert_many!(
        map,
        "icons/cog",
        ".bak",
        ".cab",
        ".cfg",
        ".cpl",
        ".cur",
        ".dll",
        ".dmp",
        ".drv",
        ".icns",
        ".ini",
        ".lnk",
        ".sys",
        ".tmp",
        ".so",
        ".dylib"
    );

    insert_many!(
        map,
        "icons/film",
        ".3g2",
        ".3gp",
        ".avi",
        ".flv",
        ".h264",
        ".m4v",
        ".mkv",
        ".mov",
        ".mp4",
        ".mpg",
        ".rm",
        ".swf",
        ".vob",
        ".wmv"
    );

    insert_many!(
        map,
        "icons/document-text",
        ".csv",
        ".doc",
        ".docs",
        ".odt",
        ".pdf",
        ".rtf",
        ".tex",
        ".txt",
        ".wpd",
        ".hbs",
        ".xml",
        ".json",
        ".yml",
        ".yaml",
        ".toml"
    );

    map
});

pub fn icon_helper<'reg, 'rc>(
    h: &Helper<'reg, 'rc>,
    r: &'reg Handlebars<'reg>,
    c: &'rc Context,
    _: &mut RenderContext<'reg, 'rc>,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h
        .param(0)
        .ok_or_else(|| RenderError::new("Param not found for helper \"icon\""))?;

    let value = param.value().as_str().unwrap_or("");
    let ext: &str = &value[value.rfind('.').unwrap_or(0)..].to_lowercase();

    out.write(&r.render_with_context(ICON_EXTS.get(ext).unwrap_or(&"icons/document"), c)?)?;

    Ok(())
}
