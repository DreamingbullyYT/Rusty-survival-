use rusty_survival_engine_core::{BlockPos, ChunkPos, CHUNK_WIDTH};

#[test]
fn positive_chunk_boundaries_map_to_expected_local_coordinates() {
    let cases = [(0, 0, 0), (15, 0, 15), (16, 1, 0), (31, 1, 15), (32, 2, 0)];

    for (block, expected_chunk, expected_local) in cases {
        let address = BlockPos::new(block, 70, block).chunk_address();
        assert_eq!(
            address.chunk(),
            ChunkPos::new(expected_chunk, expected_chunk)
        );
        assert_eq!(address.local_x(), expected_local);
        assert_eq!(address.local_z(), expected_local);
    }
}

#[test]
fn negative_coordinates_use_euclidean_division() {
    let cases = [
        (-1, -1, 15),
        (-15, -1, 1),
        (-16, -1, 0),
        (-17, -2, 15),
        (-32, -2, 0),
    ];

    for (block, expected_chunk, expected_local) in cases {
        let address = BlockPos::new(block, -64, block).chunk_address();
        assert_eq!(
            address.chunk(),
            ChunkPos::new(expected_chunk, expected_chunk)
        );
        assert_eq!(address.local_x(), expected_local);
        assert_eq!(address.local_z(), expected_local);
    }
}

#[test]
fn block_chunk_mapping_round_trips_across_negative_and_positive_range() {
    for coordinate in -4097..=4097 {
        let block = BlockPos::new(coordinate, 123, -coordinate);
        let reconstructed = block
            .chunk_address()
            .to_block_pos(block.y)
            .expect("an address derived from an i32 block coordinate must round-trip");
        assert_eq!(reconstructed, block);
    }
}

#[test]
fn integer_extremes_round_trip_without_overflow() {
    for coordinate in [i32::MIN, i32::MIN + 1, i32::MAX - 1, i32::MAX] {
        let block = BlockPos::new(coordinate, 0, coordinate);
        assert_eq!(block.chunk_address().to_block_pos(0), Some(block));
    }
}

#[test]
fn chunk_width_is_sixteen_blocks() {
    assert_eq!(CHUNK_WIDTH, 16);
}
