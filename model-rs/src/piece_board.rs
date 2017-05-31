use std::collections::{vec_deque, VecDeque};
use std::default::Default;

use rand;
use serde_json;

use piece::Piece;

use result::{QResult, PlayerError};

/// The default set of pieces, serialized.
const PIECES_JSON: &'static [u8] = include_bytes!("../data/pieces.json");

/// The default depth at which we can take pieces (0-based).
const DEFAULT_DEPTH: usize = 2;

/// Builder for constructing and configuring `PieceBoard`s.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PieceBoardBuilder {
    piece_queue: VecDeque<Piece>,
    depth:       usize,
}

impl PieceBoardBuilder {
    /// Will build a `PieceBoard` with the default depth and default set of pieces.
    pub fn new() -> Self {
        let result = Self::empty();
        result.extend_from_slice(PIECES_JSON).unwrap()
    }

    /// Will build a `PieceBoard` with the default depth and no pieces.
    pub fn empty() -> Self {
        PieceBoardBuilder {
            piece_queue: VecDeque::new(),
            depth:       DEFAULT_DEPTH,
        }
    }

    /// Sets the piece taking depth.
    pub fn depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
    }

    /// Adds the given sequence of pieces to the piece queue.
    pub fn extend<I>(mut self, pieces: I) -> Self
        where I: IntoIterator<Item = Piece>
    {
        self.piece_queue.extend(pieces);
        self
    }

    /// Deserializes pieces from a `&[u8]` of JSON, adding to the piece queue.
    pub fn extend_from_slice(self, pieces: &[u8]) -> serde_json::Result<Self> {
        let pieces: Vec<Piece> = serde_json::from_slice(pieces)?;
        Ok(self.extend(pieces))
    }

    /// Builds the `PieceBoard`, shuffling the pieces.
    pub fn build(mut self) -> PieceBoard {
        shuffle(&mut rand::thread_rng(), &mut self.piece_queue);
        self.build_in_order()
    }

    /// Builds the `PieceBoard` without shuffling the pieces.
    pub fn build_in_order(self) -> PieceBoard {
        PieceBoard {
            piece_queue: self.piece_queue,
            depth:       self.depth,
        }
    }
}

/// [Fisher-Yates shuffle](https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle).
fn shuffle<R: rand::Rng, T>(rng: &mut R, vd: &mut VecDeque<T>) {
    use rand::distributions::{IndependentSample, Range};

    for i in (1 .. vd.len()).rev() {
        let range = Range::new(0, i);
        let j = range.ind_sample(rng);
        vd.swap(i, j);
    }
}

/// The queue of pieces to be taken.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PieceBoard {
    piece_queue: VecDeque<Piece>,
    depth: usize,
}

impl PieceBoard {
    pub fn new() -> Self {
        PieceBoardBuilder::new().build()
    }

    pub fn is_empty(&self) -> bool {
        self.piece_queue.is_empty()
    }

    pub fn len(&self) -> usize {
        self.piece_queue.len()
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn pieces(&self) -> Pieces {
        Pieces(self.piece_queue.iter())
    }

    pub fn take(&mut self, depth: usize) -> QResult<Piece> {
        if depth > self.depth {
            Err(PlayerError::TakeOverDepth)
        } else if depth >= self.piece_queue.len() {
            Err(PlayerError::OutOfPieces)
        } else {
            let mut stack = Vec::new();
            for _ in 0..depth {
                stack.push(self.piece_queue.pop_front().unwrap());
            }
            let result = self.piece_queue.pop_front().unwrap();
            for piece in stack.into_iter().rev() {
                self.piece_queue.push_front(piece)
            }
            Ok(result)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pieces<'a>(vec_deque::Iter<'a, Piece>);

impl<'a> Iterator for Pieces<'a> {
    type Item = &'a Piece;

    fn next(&mut self) -> Option<&'a Piece> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a> ExactSizeIterator for Pieces<'a> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a> DoubleEndedIterator for Pieces<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl Default for PieceBoardBuilder {
    fn default() -> Self {
        PieceBoardBuilder::new()
    }
}

impl Default for PieceBoard {
    fn default() -> Self {
        PieceBoard::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use piece::*;

    fn pieces() -> Vec<Piece> {
        use self::examples::*;
        vec![piece1(), piece2(), piece3(), piece4()]
    }

    #[test]
    fn make_board() {
        let builder = PieceBoardBuilder::empty()
            .extend(pieces());
        let board = builder.build();
        assert_eq!(board.len(), 4);
    }
}
