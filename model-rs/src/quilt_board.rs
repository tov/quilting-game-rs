use piece::Piece;
use position::{Position, Dimension, Transformation};

/// The board on which the quilt is constructed.
///
/// Invariant:
///
///  - rows.len() == height
///
///  - for row in rows { row.len() == width }
pub struct QuiltBoard {
    dimension: Dimension,
    rows:      Box<[Box<[bool]>]>,
}

impl QuiltBoard {
    /// Creates a new, empty board of the given dimensions.
    pub fn new(width: usize, height: usize) -> Self {
        let mut rows = Vec::new();

        for _ in 0 .. height {
            rows.push(vec![false; width].into_boxed_slice());
        }

        QuiltBoard {
            dimension: Dimension::new(width, height),
            rows:      rows.into_boxed_slice(),
        }
    }

    /// Returns the number of squares covered by pieces.
    pub fn squares_covered(&self) -> usize {
        let mut result = 0;

        for row in &*self.rows {
            for &b in &**row {
                if b { result += 1; }
            }
        }

        result
    }

    /// Can the given piece be added at the given position under the given transformation?
    ///
    /// Returns `Err` of a reason if it cannot.
    pub fn can_add_piece(&self, position: Position,
                         piece: &Piece,
                         transformation: Transformation)
                         -> Result<(), &'static str>
    {
        for p in piece.positions(transformation) {
            let x = position.x + p.x;
            let y = position.y + p.y;

            if x >= self.dimension.width {
                return Err("Piece hangs off right edge of board");
            } else if y >= self.dimension.height {
                return Err("Piece hangs off bottom edge of board");
            } else if self.rows[y][x] {
                return Err("Piece overlaps other piece");
            }
        }

        Ok(())
    }

    /// Adds the given piece at the specified position under the given transformation.
    ///
    /// # Errors
    ///
    /// Panics if the piece cannot be added because it doesn't fit.
    pub fn add_piece(&mut self, position: Position, piece: &Piece, transformation: Transformation) {
        self.can_add_piece(position, piece, transformation).unwrap();

        for p in piece.positions(transformation) {
            self.rows[position.y + p.y][position.x + p.x] = true;
        }
    }
}