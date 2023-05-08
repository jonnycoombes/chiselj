//! Pretty-printer logic for use by various commands
use crate::cl_immediate;
use crate::errors::{ChiselError, ChiselResult};
use crate::render::commands::{CommandList, CommandListMode, PipelineCommand, RenderCommand};
use chisel_json::JsonValue;
use std::sync::mpsc::Sender;

/// Options that control the output from a given printer instance
pub struct PrintFormatOptions {
    /// The level of indentation to use
    pub indentation: u16,

    /// The spacing between KV pairs
    pub kvspacing: u16,
}

/// Default implementation uses some sensible default for the various options
impl Default for PrintFormatOptions {
    fn default() -> Self {
        Self {
            indentation: 2,
            kvspacing: 1,
        }
    }
}

/// Pretty printer for [JsonValue]s
pub struct Printer {
    /// The [ActionContext] associated with the printer
    pub pipeline: Sender<CommandList>,

    /// The formatting options
    pub options: PrintFormatOptions,
}

impl Printer {
    /// Construct a new instance, based on a supplied context reference and set of options
    pub fn new(pipeline: Sender<CommandList>, options: PrintFormatOptions) -> Self {
        Printer { pipeline, options }
    }

    /// Chuck a [CommandList] at the rendering pipeline and perform error conversion if necessary
    #[inline]
    fn submit_command_list(&self, cmds: CommandList) -> ChiselResult<()> {
        match self.pipeline.send(cmds) {
            Ok(_) => Ok(()),
            Err(_) => Err(ChiselError::RenderPipelineSendFailed),
        }
    }

    /// Recursively render a JSON value
    pub fn render_json(&self, value: JsonValue) -> ChiselResult<()> {
        self.render_json_value(0, value)
    }

    /// Render a [JsonValue]
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

    /// Render a json array
    fn render_json_array(&self, level: u16, kids: Vec<JsonValue>) -> ChiselResult<()> {
        // opening bracket
        self.submit_command_list(cl_immediate!(
            RenderCommand::Char('['),
            RenderCommand::NewLine
        ))?;

        let kidcount = kids.len();
        for (i, value) in kids.into_iter().enumerate() {
            self.submit_command_list(cl_immediate!(RenderCommand::Indent(
                (level + 1) * self.options.indentation
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
                self.submit_command_list(cl_immediate!(
                    RenderCommand::Char(','),
                    RenderCommand::NewLine
                ))?
            } else {
                self.submit_command_list(cl_immediate!(RenderCommand::NewLine))?
            }
        }

        // closing bracket
        self.submit_command_list(cl_immediate!(
            RenderCommand::Indent(level * self.options.indentation),
            RenderCommand::Char(']'),
        ))
    }

    /// Render a string value
    fn render_json_string(&self, value: String) -> ChiselResult<()> {
        self.submit_command_list(cl_immediate!(RenderCommand::Text(value)))
    }

    /// Render an integer value
    fn render_json_integer(&self, value: i64) -> ChiselResult<()> {
        self.submit_command_list(cl_immediate!(RenderCommand::Text(value.to_string())))
    }

    /// Render an float value
    fn render_json_float(&self, value: f64) -> ChiselResult<()> {
        self.submit_command_list(cl_immediate!(RenderCommand::Text(value.to_string())))
    }

    /// Render a boolean value
    fn render_json_boolean(&self, value: bool) -> ChiselResult<()> {
        if value {
            self.submit_command_list(cl_immediate!(RenderCommand::Slice("true")))
        } else {
            self.submit_command_list(cl_immediate!(RenderCommand::Slice("false")))
        }
    }

    /// Render a null value
    fn render_json_null(&self) -> ChiselResult<()> {
        self.submit_command_list(cl_immediate!(RenderCommand::Slice("null")))
    }

    /// Surround an object with braces at the correct indentation level, and recursively render
    /// children at the next indentation level
    fn render_json_object(&self, level: u16, kids: Vec<(String, JsonValue)>) -> ChiselResult<()> {
        let kidcount = kids.len();

        // opening brace
        self.submit_command_list(cl_immediate!(
            RenderCommand::Char('{'),
            RenderCommand::NewLine
        ))?;

        // render the kids
        for (i, (key, value)) in kids.into_iter().enumerate() {
            if i == kidcount - 1 {
                self.render_json_pair(level + 1, false, key, value)?;
            } else {
                self.render_json_pair(level + 1, true, key, value)?;
            }
        }

        // closing brace with optional newline
        self.submit_command_list(cl_immediate!(
            RenderCommand::Indent(level * self.options.indentation),
            RenderCommand::Char('}'),
        ))
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
            RenderCommand::Indent(level * self.options.indentation),
            RenderCommand::Text(key.to_string()),
            RenderCommand::Indent(self.options.kvspacing),
            RenderCommand::Slice(":"),
            RenderCommand::Indent(self.options.kvspacing),
        ))?;

        // the value
        self.render_json_value(level, value)?;

        // add trailing comma as required
        if trailing {
            self.submit_command_list(cl_immediate!(
                RenderCommand::Char(','),
                RenderCommand::NewLine
            ))?;
        } else {
            self.submit_command_list(cl_immediate!(RenderCommand::NewLine))?;
        }

        Ok(())
    }
}