use leptos::*;

/// A renderer that shows an <input> tag and emits the `on_change` event when the <input> is changed.
#[component]
pub fn InputCellRenderer<F>(
    /// The class attribute for the cell element. Generated by the classes provider.
    #[prop(into)]
    class: MaybeSignal<String>,
    /// The value to display.
    #[prop(into)]
    value: MaybeSignal<String>,
    /// Event handler called when the cell is changed. In this default renderer this will never happen.
    on_change: F,
    /// The index of the column. Starts at 0.
    index: usize,
) -> impl IntoView
where
    F: Fn(String) + 'static,
{
    view! {
        <td class=class>
            <input type="text" value=value on:change=move |evt| { on_change(event_target_value(&evt)); } />
        </td>
    }
}
