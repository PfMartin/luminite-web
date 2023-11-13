use leptos::{create_signal, view, IntoView, SignalUpdate};

pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    let increment = move |_| set_count.update(|count| *count += 1);

    view! {
      <button
        class="py-1 px-2 m-1 bg-blue-400 rounded hover:bg-blue-500 transition-all"
        on:click=increment
      >
      "Increment Counter: " {count}
      </button>
    }
}
