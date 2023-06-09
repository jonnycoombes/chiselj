//! Pretty-printer logic for use by various commands
use crate::cl_immediate;
use crate::errors::{ChiselError, ChiselResult};
use crate::render::display_lists::{DisplayList, DisplayListCommand, DisplayListMode, Draw};
use chisel_json::JsonValue;
use std::sync::mpsc::Sender;

/// Options that control the output from a given printer instance
pub struct FormatOptions {
    /// The level of indent to use
    pub indent: u16,

    /// The padding between KV pairs
    pub kvpadding: u16,
}

/// Default implementation uses some sensible default for the various options
impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            indent: 2,
            kvpadding: 1,
        }
    }
}

/// Pretty printer for [JsonValue]s
pub struct PrettyPrinter {
    /// The [ActionContext] associated with the printer
    pub pipeline: Sender<DisplayList>,

    /// The formatting options
    pub options: FormatOptions,
}

impl PrettyPrinter {
    /// Construct a new instance, based on a supplied context reference and set of options
    pub fn new(pipeline: Sender<DisplayList>, options: FormatOptions) -> Self {
        PrettyPrinter { pipeline, options }
    }

    /// Chuck a [DisplayList] at the rendering pipeline and perform error conversion if necessary
    #[inline]
    fn submit_command_list(&self, cmds: DisplayList) -> ChiselResult<()> {
        match self.pipeline.send(cmds) {
            Ok(_) => Ok(()),
            Err(_) => Err(ChiselError::DisplayListFailed),
        }
    }

    /// Recursively render a JSON value
    pub fn render_json(&self, value: JsonValue) -> ChiselResult<()> {
        self.render_json_value(0, value)
    }

    /// Draw a [JsonValue]
    fn render_json_value(&self, level: u16, value: JsonValue) -> ChiselResult<()> {
        match value {
            JsonValue::Object(kids) => self.render_json_object(level, kids),
            JsonValue::Array(kids) => self.render_json_array(level, kids),
            JsonValue::String(value) => self.render_json_string(value.into_owned()),
            JsonValue::Float(value) => self.render_json_float(value),
            JsonValue::Integer(value) => self.render_json_integer(value),
            JsonValue::Boolean(value) => self.render_json_boolean(value),
            JsonValue::Null => self.render_json_null(),
        }
    }

    /// Draw a json array
    fn render_json_array(&self, level: u16, kids: Vec<JsonValue>) -> ChiselResult<()> {
        let kidcount = kids.len();
        let empty = kids.is_empty();

        // opening bracket
        if !empty {
            self.submit_command_list(cl_immediate!(Draw::Char('['), Draw::NewLine))?;
        } else {
            self.submit_command_list(cl_immediate!(Draw::Char('['),))?;
        }

        for (i, value) in kids.into_iter().enumerate() {
            self.submit_command_list(cl_immediate!(Draw::Indent(
                (level + 1) * self.options.indent
            )))?;
            match value {
                JsonValue::Object(pairs) => self.render_json_object(level + 1, pairs)?,
                JsonValue::Array(kids) => self.render_json_array(level + 1, kids)?,
                JsonValue::String(value) => self.render_json_string(value.into_owned())?,
                JsonValue::Float(value) => self.render_json_float(value)?,
                JsonValue::Integer(value) => self.render_json_integer(value)?,
                JsonValue::Boolean(value) => self.render_json_boolean(value)?,
                JsonValue::Null => self.render_json_null()?,
            }
            if i != kidcount - 1 {
                self.submit_command_list(cl_immediate!(Draw::Char(','), Draw::NewLine))?
            } else {
                self.submit_command_list(cl_immediate!(Draw::NewLine))?
            }
        }

        // closing bracket
        if !empty {
            self.submit_command_list(cl_immediate!(
                Draw::Indent(level * self.options.indent),
                Draw::Char(']'),
            ))
        } else {
            self.submit_command_list(cl_immediate!(Draw::Char(']'),))
        }
    }

    /// Draw a string value
    fn render_json_string(&self, value: String) -> ChiselResult<()> {
        self.submit_command_list(cl_immediate!(Draw::Text(value)))
    }

    /// Draw an integer value
    fn render_json_integer(&self, value: i64) -> ChiselResult<()> {
        self.submit_command_list(cl_immediate!(Draw::Text(value.to_string())))
    }

    /// Draw an float value
    fn render_json_float(&self, value: f64) -> ChiselResult<()> {
        self.submit_command_list(cl_immediate!(Draw::Text(value.to_string())))
    }

    /// Draw a boolean value
    fn render_json_boolean(&self, value: bool) -> ChiselResult<()> {
        if value {
            self.submit_command_list(cl_immediate!(Draw::Slice("true")))
        } else {
            self.submit_command_list(cl_immediate!(Draw::Slice("false")))
        }
    }

    /// Draw a null value
    fn render_json_null(&self) -> ChiselResult<()> {
        self.submit_command_list(cl_immediate!(Draw::Slice("null")))
    }

    /// Surround an object with braces at the correct indent level, and recursively render
    /// children at the next indent level
    fn render_json_object(&self, level: u16, kids: Vec<(String, JsonValue)>) -> ChiselResult<()> {
        let kidcount = kids.len();
        let empty = kids.is_empty();

        // opening brace
        if !empty {
            self.submit_command_list(cl_immediate!(Draw::Char('{'), Draw::NewLine))?;
        } else {
            self.submit_command_list(cl_immediate!(Draw::Char('{'),))?;
        }

        // render the kids
        for (i, (key, value)) in kids.into_iter().enumerate() {
            if i == kidcount - 1 {
                self.render_json_pair(level + 1, false, key, value)?;
            } else {
                self.render_json_pair(level + 1, true, key, value)?;
            }
        }

        // closing brace with optional newline
        if !empty {
            self.submit_command_list(cl_immediate!(
                Draw::Indent(level * self.options.indent),
                Draw::Char('}'),
            ))
        } else {
            self.submit_command_list(cl_immediate!(Draw::Char('}'),))
        }
    }

    /// Output a KV pair from within an object
    fn render_json_pair(
        &self,
        level: u16,
        trailing: bool,
        key: String,
        value: JsonValue,
    ) -> ChiselResult<()> {
        // the key
        self.submit_command_list(cl_immediate!(
            Draw::Indent(level * self.options.indent),
            Draw::Text(key.to_string()),
            Draw::Indent(self.options.kvpadding),
            Draw::Slice(":"),
            Draw::Indent(self.options.kvpadding),
        ))?;

        // the value
        self.render_json_value(level, value)?;

        // add trailing comma as required
        if trailing {
            self.submit_command_list(cl_immediate!(Draw::Char(','), Draw::NewLine))?;
        } else {
            self.submit_command_list(cl_immediate!(Draw::NewLine))?;
        }

        Ok(())
    }
}
