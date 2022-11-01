use anyhow::{anyhow, Result};
use rand::prelude::IteratorRandom;

/// Block of a piece.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    /// Index of the piece.
    pub index: usize,
    /// Byte offset within that piece.
    pub begin: usize,
    /// Block data.
    pub data: Vec<u8>,
}

/// Struct to keep track of finished/missing blocks, and assemble them once finished.
#[derive(Debug, Clone)]
pub struct Builder {
    pub finished: Vec<Block>,
    pub missing: Vec<Block>,

    pub piece_amount: usize,
    pub piece_length: usize,
}

impl Builder {
    /// Create a new builder.
    ///
    /// # Arguments
    ///
    /// * `piece_amount` - amount of pieces.
    /// * `piece_length` - length of each piece in bytes.
    /// * `block_size` - size to divide each piece up in.
    // TODO: Write tests
    pub fn new(piece_amount: usize, piece_length: usize, block_size: usize) -> Builder {
        // TODO: Refactor
        let block_amount = piece_length / block_size;
        let block_rest = piece_length % block_size;

        let mut missing = vec![];

        if block_amount == 0 && block_rest > 0 {
            missing.push(Block {
                index: 0,
                begin: 0,
                data: vec![0; block_rest],
            });
        }

        for piece_index in 0..piece_amount {
            for block_index in 0..block_amount {
                let data_size = if block_index == block_amount - 1 && block_rest > 0 {
                    block_rest
                } else {
                    block_size
                };

                missing.push(Block {
                    index: piece_index,
                    begin: block_index * block_size,
                    data: vec![0; data_size],
                });
            }
        }

        Builder {
            finished: vec![],
            missing,
            piece_amount,
            piece_length,
        }
    }

    pub fn assemble_piece(&self, index: usize) -> Vec<Option<u8>> {
        let mut out = vec![None; self.piece_length];

        for block in self.finished.iter().filter(|x| x.index == index) {
            out.splice(
                block.begin..block.begin + block.data.len(),
                block.data.iter().map(|x| Some(*x)),
            );
        }

        out
    }

    pub fn assemble(&self) -> Vec<Option<u8>> {
        let mut out = vec![];

        for index in 0..self.piece_amount {
            let mut piece = self.assemble_piece(index);
            out.append(&mut piece);
        }

        out
    }

    // TODO: Write tests
    pub fn get_finished_block(&self, index: usize, begin: usize, length: usize) -> Result<Block> {
        if let Some(piece) = self.assemble_piece(index).get(begin..begin + length) {
            let data = piece
                .iter()
                .copied()
                .collect::<Option<Vec<u8>>>()
                .ok_or_else(|| anyhow!("Missing block"))?;

            Ok(Block { index, begin, data })
        } else {
            Err(anyhow!("Missing block"))
        }
    }

    pub fn take_missing_relevant_block(&mut self, bitfield: &[u8]) -> Result<Block> {
        let mut block_index = None;

        for (row_index, mut row) in bitfield.iter().copied().enumerate() {
            while row > 0 && block_index.is_none() {
                let piece_index = row.leading_zeros() as usize + (row_index * 8);

                if let Some((i, _)) = self
                    .missing
                    .iter()
                    .enumerate()
                    .find(|x| x.1.index == piece_index)
                {
                    block_index = Some(i);
                } else {
                    row ^= 1 << (7 - row.leading_zeros());
                }
            }

            if let Some(block_index) = block_index {
                return Ok(self.missing.swap_remove(block_index));
            }
        }

        Err(anyhow!("Didn't find any blocks"))
    }

    pub fn take_random_missing_block(&mut self) -> Result<Block> {
        let index = (0..self.missing.len())
            .choose(&mut rand::thread_rng())
            .ok_or_else(|| anyhow!("No more blocks"))?;

        Ok(self.missing.swap_remove(index))
    }

    pub fn add_missing_block(&mut self, block: Block) -> Result<()> {
        self.missing.push(block);

        Ok(())
    }

    pub fn add_finished_block(&mut self, block: Block) -> Result<()> {
        self.finished.push(block);

        Ok(())
    }
}
