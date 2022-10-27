use anyhow::{anyhow, Result};
use rand::prelude::IteratorRandom;

/// Block of a piece.
#[derive(Debug)]
pub struct Block {
    /// Index of the piece.
    pub index: usize,
    /// Byte offset within that piece.
    pub begin: usize,
    /// Block data.
    pub data: Vec<u8>,
}

/// Struct to keep track of finished/missing blocks, and assemble them once finished.
#[derive(Debug)]
pub struct Builder {
    pub finished: Vec<Block>,
    pub missing: Vec<Block>,
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
        }
    }

    // TODO: Write tests
    pub fn get_finished_block(&self, index: usize, begin: usize, length: usize) -> Result<Block> {
        let mut buf = vec![None; length];

        for block in self.finished.iter() {
            if block.index == index as usize {
                buf.splice(begin..(begin + length), block.data.iter().map(|x| Some(*x)));
            }
        }

        let data = buf
            .into_iter()
            .collect::<Option<Vec<u8>>>()
            .ok_or_else(|| anyhow!("Missing block"))?;

        Ok(Block { index, begin, data })
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
