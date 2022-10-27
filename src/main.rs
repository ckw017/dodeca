use roaring::bitmap::RoaringBitmap;

// List of edges defined by the two points they span
const EDGE_POINTS: [(u32, u32); 30] = [
    (0, 1),   //0
    (1, 2),   //1
    (2, 3),   //2
    (3, 4),   //3
    (4, 0),   //4
    (1, 5),   //5
    (2, 6),   //6
    (3, 7),   //7
    (4, 8),   //8
    (0, 9),   //9
    (9, 10),  //10
    (5, 11),  //11
    (6, 12),  //12
    (7, 13),  //13
    (8, 14),  //14
    (5, 10),  //15
    (6, 11),  //16
    (7, 12),  //17
    (8, 13),  //18
    (9, 14),  //19
    (10, 15), //20
    (11, 16), //21
    (12, 17), //22
    (13, 18), //23
    (14, 19), //24
    (15, 16), //25
    (16, 17), //26
    (17, 18), //27
    (18, 19), //28
    (19, 15), //29
];

const ROT1_MASK_A: u32 = 0b111101111011110111101111011110;
const ROT1_MASK_B: u32 = 0b000010000100001000010000100001;

const ROT2_MAP: [u8; 30] = [
    5, 11, 16, 6, 1, 15, 21, 12, 2, 0, 9, 20, 26, 17, 3, 10, 25, 22, 7, 4, 19, 29, 27, 13, 8, 24,
    28, 23, 18, 14,
];

fn rot1(repr: u32) -> u32 {
    let tmp = (repr & ROT1_MASK_A) >> 1;
    tmp | (repr & ROT1_MASK_B) << 4
}

fn rot2(mut repr: u32) -> u32 {
    let mut result = 0;
    for offset in ROT2_MAP {
        if repr == 0 {
            break;
        }
        result |= (repr % 2) << offset;
        repr >>= 1;
    }
    result
}

fn skip_rot1(repr: u32, skip: &mut RoaringBitmap) {
    let mut base = repr;
    skip.insert(base);
    for _ in 0..4 {
        base = rot1(base);
        skip.insert(base);
    };
}

fn skip_repr_no_flip(repr: u32, skip: &mut RoaringBitmap) {
    let start0 = repr;
    let start1 = rot2(start0);
    let start2 = rot2(start1);
    let start3 = rot2(start2);
    let start4 = rot2(rot2(rot1(rot1(rot1(rot1(start0))))));
    let start5 = rot2(rot2(rot1(rot1(rot1(start0)))));
    skip_rot1(start0, skip);
    skip_rot1(start1, skip);
    skip_rot1(start2, skip);
    skip_rot1(start3, skip);
    skip_rot1(start4, skip);
    skip_rot1(start5, skip);
}

fn skip_repr(repr: u32, skip: &mut RoaringBitmap) {
    let flipped = rot1(rot2(rot1(repr)));
    skip_repr_no_flip(repr, skip);
    skip_repr_no_flip(flipped, skip);
}

fn main() {
    let mut edge_masks = Vec::new();
    for (u, v) in EDGE_POINTS {
        let mask: u32 = 2u32.pow(u) | 2u32.pow(v);
        edge_masks.push(mask)
    }

    let mut pair_masks = Vec::new();
    for i in 0..edge_masks.len() {
        for j in (i + 1)..edge_masks.len() {
            if edge_masks[i] & edge_masks[j] != 0 {
                let mask: u32 = 2u32.pow(i as u32) | 2u32.pow(j as u32);
                pair_masks.push(mask);
            }
        }
    }

    println!("{}", pair_masks.len());

    // Contains all of the valid edge sets
    let mut all = RoaringBitmap::new();
    // Contains all of the edge sets where `i` edges are set
    let mut curr = RoaringBitmap::new();
    // Contains all the edge sets that we can skip in curr
    let mut skip = RoaringBitmap::new();
    // Contains all of the edge sets where `i + 1` edges are set
    let mut next = RoaringBitmap::new();
    // Seed both maps with a single edge to start
    curr.insert(1);
    all.insert(1);

    for i in 1..31 {
        let mut round: u32 = 0;
        for parent in &curr {
            if skip.contains(parent) {
                continue;
            }
            skip_repr(parent, &mut skip);
            all.insert(parent);
            round += 1;
            for mask in &pair_masks {
                if (mask & parent) != 0 && (mask | parent) != parent {
                    next.insert(mask | parent);
                }
            }
        }
        println!("{} {}", i, round);
        curr = next;
        next = RoaringBitmap::new();
        skip = RoaringBitmap::new()
    }
    println!("{}", all.len())
}
