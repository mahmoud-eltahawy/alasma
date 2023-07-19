use leptos::*;

#[component]
pub fn ProgressBar(
    cx: Scope,
    #[prop(into)]
    progress: Signal<(i32,i32)>,
) -> impl IntoView {
    view! { cx,
	<Show
	when=move || {
	    let (p,max) = progress.get();
	    p != max
	}
	fallback=|_| view!{cx,<></>}
	>
	    <progress
		max= move|| progress.get().1
		value= move|| progress.get().0
	    />
	</Show>
    }
}
