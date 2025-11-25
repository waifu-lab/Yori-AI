use serde::{Deserialize, Serialize};

/// The main container for the entire UI description, acting as the root of the UI tree.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UiSchema {
    pub root: Component,
}

/// Represents a single UI component. This enum is the core of the UI description.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Component {
    /// A container that displays a single child component within a styled card.
    Card(Card),
    /// A container that stacks its children vertically.
    VerticalStack(VerticalStack),
    /// A container that arranges its children horizontally.
    HorizontalStack(HorizontalStack),
    /// A component for displaying static or data-bound text.
    Text(Text),
    /// An input field for text.
    TextInput(TextInput),
    /// A clickable button that can trigger actions.
    Button(Button),
}

/// Defines content that can either be a fixed string or bound to a key in the plugin's settings.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Content {
    /// A static, unchanging string.
    Static(String),
    /// A key that binds the content to a value in the plugin's settings.
    Bound { key: String },
}

/// A container component that visually groups its child component.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Card {
    /// A unique identifier for this component.
    pub id: String,
    /// The title displayed at the top of the card.
    pub title: String,
    /// The single component contained within the card.
    pub child: Box<Component>,
}

/// A layout component that arranges multiple children in a vertical line.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VerticalStack {
    /// A unique identifier for this component.
    pub id: String,
    /// The list of components to be stacked vertically.
    pub children: Vec<Component>,
}

/// A layout component that arranges multiple children in a horizontal line.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HorizontalStack {
    /// A unique identifier for this component.
    pub id: String,
    /// The list of components to be arranged horizontally.
    pub children: Vec<Component>,
}

/// A component for displaying text.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Text {
    /// A unique identifier for this component.
    pub id: String,
    /// The content to be displayed, which can be static or data-bound.
    pub content: Content,
}

/// A component representing a text input field.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TextInput {
    /// A unique identifier for this component.
    pub id: String,
    /// The label displayed alongside the input field.
    pub label: String,
    /// The key used to bind the input's value to the plugin's settings.
    pub data_key: String,
}

/// A component representing a clickable button.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Button {
    /// A unique identifier for this component.
    pub id: String,
    /// The text displayed on the button.
    pub text: String,
}