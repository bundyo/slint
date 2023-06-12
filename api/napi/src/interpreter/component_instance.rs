// Copyright © SixtyFPS GmbH <info@slint-ui.com>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-commercial

use slint_interpreter::{ComponentHandle, ComponentInstance};

#[napi(js_name = "JsComponentInstance")]
pub struct JsComponentInstance {
    internal: ComponentInstance,
}

impl From<ComponentInstance> for JsComponentInstance {
    fn from(instance: ComponentInstance) -> Self {
        Self { internal: instance }
    }
}

#[napi]
impl JsComponentInstance {
    #[napi(constructor)]
    pub fn new() -> Self {
        unreachable!("ComponentDefinition can only be created by using ComponentCompiler.")
    }

    #[napi]
    pub fn run(&self) {
        self.internal.run().unwrap()
    }
}
