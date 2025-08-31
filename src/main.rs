use dioxus::prelude::*;

use sudoku::util::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Clone)]
struct SudokuState {
	grid: Signal<Vec<Option<IType>>>,
}

fn main() {
	dioxus::launch(App);
}

#[component]
fn App() -> Element {
	rsx! {
		document::Link { rel: "stylesheet", href: MAIN_CSS }
		Header {}
		Sudoku {}
	}
}

#[component]
pub fn Header() -> Element {
	rsx! {
		div { id: "title",
			h1 { "Sudoku! 🧩"}
		}
	}
}

#[component]
pub fn Sudoku() -> Element {
	const INITIAL_N: IType = 3;
	let state = use_context_provider(|| SudokuState {
		grid: Signal::new(vec![None; INITIAL_N.pow(4) as usize]),
	});
	let mut grid = state.grid;
	rsx! {
		input {
			value: (grid.read().len() as f32).powf(0.25) as IType,
			onchange: move |event| {
				if let Ok(n) = event.value().parse::<IType>() {
					if 0 < n && n < 5 {
						let x: &mut Vec<Option<IType>> = &mut grid.write();
						*x = vec![None; n.pow(4) as usize];
					}
				}
			}
		}
		div {
			button {
				onclick: move |_event| {
					solve_sudoku(&mut *grid.write());
				},
				"Solve"
			}
			button {
				onclick: move |_event| {
					grid.write().iter_mut().for_each(|x| *x = None);
				},
				"Clear"
			}
		}
		SudokuGrid { }
	}
}

#[component]
pub fn SudokuGrid() -> Element {
	let mut grid = use_context::<SudokuState>().grid;
	let grid_ref: &Vec<Option<IType>> = &grid.read();
	let n = (grid.read().len() as f32).powf(0.25) as usize;
	rsx! {
		table {
			border: if valid_sudoku(grid_ref) { "4px solid rgb(0 0 0)"} else {"4px solid rgb(255 0 0)"},
			for (i, x) in grid.read().iter().enumerate() {
				if i % n == 0 && i != 0 { td {} }
				if i % (n * n) == 0 { tr {} }
				if i % (n * n * n) == 0 { tr { td {} } }
				td {
					input {
						value: *x,
						onchange: move |event| {
							grid.write()[i] =
								event.value().parse::<IType>().ok().map_or(
									None,
									|x| if 0 < x && x <= (n * n) as IType { Some(x) }
									else { None }
							);
						}
					}
				}
			}
		}
	}
}
