use i_slint_core::graphics::{Image, ImageInner, SharedPixelBuffer, SharedImageBuffer};
use napi::{bindgen_prelude::*, Env, JsBoolean, JsNumber, JsString, JsUnknown, Result};
use slint_interpreter::{Value, ValueType};

#[napi(js_name = "ValueType")]
pub enum JsValueType {
    Void,
    Number,
    String,
    Bool,
    Model,
    Struct,
    Brush,
    Image,
}

impl From<slint_interpreter::ValueType> for JsValueType {
    fn from(value_type: slint_interpreter::ValueType) -> Self {
        match value_type {
            slint_interpreter::ValueType::Number => JsValueType::Number,
            slint_interpreter::ValueType::String => JsValueType::String,
            slint_interpreter::ValueType::Bool => JsValueType::Bool,
            slint_interpreter::ValueType::Model => JsValueType::Model,
            slint_interpreter::ValueType::Struct => JsValueType::Struct,
            slint_interpreter::ValueType::Brush => JsValueType::Brush,
            slint_interpreter::ValueType::Image => JsValueType::Image,
            _ => JsValueType::Void,
        }
    }
}

#[napi(js_name = "Property")]
pub struct JsProperty {
    pub name: String,
    pub value_type: JsValueType,
}

pub fn to_js_unknown(env: &Env, value: &Value) -> Result<JsUnknown> {
    match value {
        Value::Void => env.get_null().map(|v| v.into_unknown()),
        Value::Number(number) => env.create_double(*number).map(|v| v.into_unknown()),
        Value::String(string) => env.create_string(string).map(|v| v.into_unknown()),
        Value::Bool(value) => env.get_boolean(*value).map(|v| v.into_unknown()),
        Value::Image(image) => {
            Ok(JsImageData::from(image.clone()).into_instance(*env)?.as_object(*env).into_unknown())
        }
        //                        Image(image) => {} TODO: https://github.com/slint-ui/slint/issues/2474 - return struct that has same properties/etc./shape as ImageData: https://developer.mozilla.org/en-US/docs/Web/API/ImageData
        //                        Model(model) => {} TODO: Try to create a Rust type that stores ModelRc<Value> and exposes it in a nice JS API (see Model<T> interface in api/node/lib/index.ts)
        Value::Struct(struct_value) => {
            let mut o = env.create_object()?;
            for (field_name, field_value) in struct_value.iter() {
                o.set_property(env.create_string(field_name)?, to_js_unknown(env, field_value)?)?;
            }
            Ok(o.into_unknown())
        }
        //                      Brush(brush) => {}
        _ => env.get_undefined().map(|v| v.into_unknown()),
    }
}

pub fn to_value(env: &Env, unknown: JsUnknown, type_hint: ValueType) -> Result<Value> {
    Ok(match type_hint {
        ValueType::Void => Value::Void,
        ValueType::Number => {
            let js_number: JsNumber = unknown.try_into()?;
            Value::Number(js_number.get_double()?)
        }
        ValueType::String => {
            let js_string: JsString = unknown.try_into()?;
            Value::String(js_string.into_utf8()?.as_str()?.into())
        }
        ValueType::Bool => {
            let js_bool: JsBoolean = unknown.try_into()?;
            Value::Bool(js_bool.get_value()?)
        }
        ValueType::Image => {
            todo!()
        }
        ValueType::Model => {
            todo!("Instantiate a Rust type that implements Model<Value>, stores JsUnknown as JsObject and treats it as if it implements the Model<T> interface")
        }
        ValueType::Struct => {
            todo!("Use private interpreter API to find out what fields are expected; Then create slint_interpreter::Struct")
        }
        ValueType::Brush => {
            todo!()
        }
        _ => {
            todo!()
        }
    })
}

#[napi(js_name = ImageData)]
pub struct JsImageData {
    internal: Image,
}

impl From<Image> for JsImageData {
    fn from(image: Image) -> Self {
        Self { internal: image }
    }
}

#[napi]
impl JsImageData {
    #[napi(constructor)]
    pub fn new(width: u32, height: u32) -> Self {
        Self { internal: Image::from_rgba8(SharedPixelBuffer::new(width, height)) }
    }

    #[napi(constructor)]
    pub fn from_data_array(data_array: Buffer, width: u32) -> Self {
        Self {
            internal: Image::from_rgba8(SharedPixelBuffer::clone_from_slice(
                data_array.as_ref(),
                width,
                width / data_array.len() as u32,
            )),
        }
    }

    #[napi(getter)]
    pub fn width(&self) -> u32 {
        self.internal.size().width
    }

    #[napi(getter)]
    pub fn height(&self) -> u32 {
        self.internal.size().height
    }

    #[napi(getter)]
    pub fn data(&self) -> Buffer {
        let image_inner: &ImageInner = (&self.internal).into();
        if let Some(buffer) = image_inner.render_to_buffer(None) {
          match buffer {
                SharedImageBuffer::RGB8(buffer) => return Buffer::from(buffer.as_bytes()),
                SharedImageBuffer::RGBA8(buffer) => return Buffer::from(buffer.as_bytes()),
                SharedImageBuffer::RGBA8Premultiplied(buffer) => return Buffer::from(buffer.as_bytes()),
            }
        }

        Buffer::from(vec![0; (self.width() * self.height() * 4) as usize])
    }

    pub(crate) fn to_image(self) -> Image {
        self.internal
    }
}
