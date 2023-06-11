#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct GridCoord {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl std::fmt::Debug for GridCoord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(usize, usize)> for GridCoord {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

pub(crate) struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Grid<T>
where
    T: Default + Clone,
{
    pub(crate) fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![T::default(); width * height],
        }
    }

    fn in_bounds(&self, coord: GridCoord) -> bool {
        coord.x < self.width && coord.y < self.height
    }

    pub(crate) fn cell_mut(&mut self, coord: GridCoord) -> Option<&mut T> {
        if !self.in_bounds(coord) {
            return None;
        }
        Some(&mut self.data[coord.y * self.width + coord.x])
    }

    pub(crate) fn cell(&self, coord: GridCoord) -> Option<&T> {
        if !self.in_bounds(coord) {
            return None;
        }
        Some(&self.data[coord.y * self.width + coord.x])
    }

    pub(crate) fn width(&self) -> usize {
        self.width
    }

    pub(crate) fn height(&self) -> usize {
        self.height
    }
}
