use std::collections::HashSet;

pub type IType = i32;

pub type Grid = Vec<Option<IType>>;

fn has_duplicates<T: Eq + std::hash::Hash>(slice: &[T]) -> bool {
	let mut set = HashSet::new();
	for item in slice {
		if !set.insert(item) {
			return true; // duplicate found
		}
	}
	false
}

pub fn sudoku_indices(n: usize) -> Vec<Vec<usize>> {
	let n2 = n * n;
	let mut indices: Vec<Vec<usize>> = vec![];
	for i in 0..n2 {
		// rows
		indices.push((n2 * i..n2 * (i + 1)).collect());
		// columns
		indices.push((0..n2).map(|j| j * n2 + i).collect());
		// blocks
		indices.push(
			(0..n2)
				.map(|j| (i / n) * n2 * (n - 1) + i * n + (j / n) * n2 + (j % n))
				.collect(),
		);
	}
	indices
}

pub fn valid_sudoku(grid: &Grid) -> bool {
	let n = (grid.len() as f32).powf(0.25) as usize;
	for group in sudoku_indices(n) {
		let vs: Vec<i32> = group.iter().filter_map(|&i| grid[i]).collect();
		if has_duplicates(&vs) {
			return false;
		}
	}
	true
}

pub fn solve_sudoku(grid: &mut Grid) -> bool {
	if !valid_sudoku(grid) {
		return false;
	}
	let first_empty = grid.iter().enumerate().find(|(_, &x)| x.is_none());
	if first_empty.is_none() {
		return true;
	}
	let (first_empty_idx, _) = first_empty.unwrap();
	let n = (grid.len() as f32).sqrt() as usize;
	for i in 0..n {
		grid[first_empty_idx] = Some((i + 1) as IType);
		if solve_sudoku(grid) {
			return true;
		} else {
			grid[first_empty_idx] = None;
		}
	}
	return false;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let n = 2;
		let indices = sudoku_indices(n);
		assert_eq!(
			indices,
			vec![
				[0, 1, 2, 3],
				[0, 4, 8, 12],
				[0, 1, 4, 5],
				[4, 5, 6, 7],
				[1, 5, 9, 13],
				[2, 3, 6, 7],
				[8, 9, 10, 11],
				[2, 6, 10, 14],
				[8, 9, 12, 13],
				[12, 13, 14, 15],
				[3, 7, 11, 15],
				[10, 11, 14, 15]
			]
		);
	}
}
