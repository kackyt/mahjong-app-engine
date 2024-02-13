use std::{fmt::Display, ops::Range};

use crate::mahjong_generated::open_mahjong::{
    FixedString, FixedStringT, Pai, PaiT, PlayerT, Taku, TakuT,
};
use anyhow::{bail, ensure};
use rand::prelude::SliceRandom;

// 牌の表示
impl Display for PaiT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let colors = ["m", "p", "s", "z"];

        let num = self.pai_num;

        let suit = (num / 9) as usize;

        write!(f, "{}{}", colors[suit], (num % 9) + 1)
    }
}

// 牌の並び替え
impl PartialOrd for PaiT {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.pai_num.partial_cmp(&other.pai_num) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.id.partial_cmp(&other.id) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }

        None
    }
}

impl Eq for PaiT {}

impl PaiT {
    pub fn get_pai_id(&self) -> u32 {
        (self.pai_num as u32) * 4 + self.id as u32
    }
}

impl Ord for PaiT {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.pai_num.cmp(&other.pai_num) {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }

        self.id.cmp(&other.id)
    }
}

fn copy_slice_partial<T: Copy>(dst: &mut [T], src: &[T]) {
    let min_len = src.len().min(dst.len());
    dst[..min_len].copy_from_slice(&src[..min_len]);
}

impl From<&[u8]> for FixedStringT {
    fn from(src: &[u8]) -> Self {
        let mut s = Self::default();
        let mut chunks = src.chunks(32);

        let n1 = chunks.next();

        if let Some(n) = n1 {
            copy_slice_partial(&mut s.n1, n);
        } else {
            return s;
        }

        let n2 = chunks.next();

        if let Some(n) = n2 {
            copy_slice_partial(&mut s.n2, n);
        } else {
            return s;
        }

        let n3 = chunks.next();

        if let Some(n) = n3 {
            copy_slice_partial(&mut s.n3, n);
        } else {
            return s;
        }

        let n4 = chunks.next();

        if let Some(n) = n4 {
            copy_slice_partial(&mut s.n4, n);
            return s;
        }

        let n5 = chunks.next();

        if let Some(n) = n5 {
            copy_slice_partial(&mut s.n5, n);
        } else {
            return s;
        }

        let n6 = chunks.next();

        if let Some(n) = n6 {
            copy_slice_partial(&mut s.n6, n);
        } else {
            return s;
        }

        let n7 = chunks.next();

        if let Some(n) = n7 {
            copy_slice_partial(&mut s.n7, n);
        } else {
            return s;
        }

        let n8 = chunks.next();

        if let Some(n) = n8 {
            copy_slice_partial(&mut s.n8, n);
        }

        s
    }
}

impl Into<Vec<u8>> for FixedStringT {
    fn into(self) -> Vec<u8> {
        let mut t: Vec<u8> = Vec::new();

        t.extend_from_slice(&self.n1);
        t.extend_from_slice(&self.n2);
        t.extend_from_slice(&self.n3);
        t.extend_from_slice(&self.n4);
        t.extend_from_slice(&self.n5);
        t.extend_from_slice(&self.n6);
        t.extend_from_slice(&self.n7);
        t.extend_from_slice(&self.n8);

        t
    }
}

impl From<&[u8]> for FixedString {
    fn from(value: &[u8]) -> Self {
        let t = FixedStringT::from(value);

        t.pack()
    }
}

impl Into<Vec<u8>> for FixedString {
    fn into(self) -> Vec<u8> {
        let t = self.unpack();

        t.into()
    }
}

pub trait GetTsumo {
    fn get_tsumohai(&self) -> Option<PaiT>;
}

impl PlayerT {
    pub fn is_registered(&self) -> bool {
        self.name.n1[0] != 0
    }
}

pub trait TakuControl {
    fn load(list: &[u32]) -> Self;
    fn create_shuffled() -> Self;
    fn search(&self, target: &PaiT) -> anyhow::Result<usize>;
    fn get(&self, index: usize) -> anyhow::Result<PaiT>;
    fn get_range(&self, r: Range<usize>) -> anyhow::Result<Vec<PaiT>>;
}

impl GetTsumo for PlayerT {
    fn get_tsumohai(&self) -> Option<PaiT> {
        if self.is_tsumo {
            Some(self.tsumohai.clone())
        } else {
            None
        }
    }
}

impl TakuControl for Taku {
    fn create_shuffled() -> Self {
        // Pai配列の初期化
        let mut hai_array: Vec<Pai> = Vec::new();
        let mut rng = rand::thread_rng();
        let mut dst = [Pai::new(0, 0, false, false, false); 32];
        let mut dst2 = [Pai::new(0, 0, false, false, false); 8];
        let mut s = Self::new(&dst, &dst, &dst, &dst, &dst2, 0);

        for pai_num in 0..34u8 {
            for id in 1..=4u8 {
                hai_array.push(Pai::new(pai_num, id, false, false, false));
            }
        }

        // shuffleの実行
        hai_array.shuffle(&mut rng);

        // 配列へコピー
        dst.copy_from_slice(&hai_array[0..32]);
        s.set_n1(&dst);
        dst.copy_from_slice(&hai_array[32..64]);
        s.set_n2(&dst);
        dst.copy_from_slice(&hai_array[64..96]);
        s.set_n3(&dst);
        dst.copy_from_slice(&hai_array[96..128]);
        s.set_n4(&dst);
        dst2.copy_from_slice(&hai_array[128..136]);
        s.set_n5(&dst2);

        s.set_length(136);

        s
    }

    fn search(&self, target: &PaiT) -> anyhow::Result<usize> {
        self.unpack().search(target)
    }

    fn get(&self, index: usize) -> anyhow::Result<PaiT> {
        self.unpack().get(index)
    }

    fn get_range(&self, r: Range<usize>) -> anyhow::Result<Vec<PaiT>> {
        self.unpack().get_range(r)
    }

    fn load(list: &[u32]) -> Self {
        let hai_array: Vec<Pai> = list
            .into_iter()
            .map(|x| Pai::new((x >> 2) as u8, (x & 3) as u8, false, false, false))
            .collect();
        let mut dst = [Pai::new(0, 0, false, false, false); 32];
        let mut dst2 = [Pai::new(0, 0, false, false, false); 8];
        let mut s = Self::new(&dst, &dst, &dst, &dst, &dst2, 0);

        dst.copy_from_slice(&hai_array[0..32]);
        s.set_n1(&dst);
        dst.copy_from_slice(&hai_array[32..64]);
        s.set_n2(&dst);
        dst.copy_from_slice(&hai_array[64..96]);
        s.set_n3(&dst);
        dst.copy_from_slice(&hai_array[96..128]);
        s.set_n4(&dst);
        dst2.copy_from_slice(&hai_array[128..136]);
        s.set_n5(&dst2);

        s.set_length(136);

        s
    }
}

impl TakuControl for TakuT {
    fn create_shuffled() -> Self {
        Taku::create_shuffled().unpack()
    }

    fn search(&self, target: &PaiT) -> anyhow::Result<usize> {
        if let Some(idx) = self.n1.iter().position(|item| item == target) {
            return Ok(idx);
        }
        if let Some(idx) = self.n2.iter().position(|item| item == target) {
            return Ok(idx + 32);
        }
        if let Some(idx) = self.n3.iter().position(|item| item == target) {
            return Ok(idx + 64);
        }
        if let Some(idx) = self.n4.iter().position(|item| item == target) {
            return Ok(idx + 96);
        }
        if let Some(idx) = self.n5.iter().position(|item| item == target) {
            return Ok(idx + 128);
        }
        bail!("not found")
    }

    fn get(&self, index: usize) -> anyhow::Result<PaiT> {
        if index < 32 {
            return Ok(self.n1[index].clone());
        }
        if index < 64 {
            return Ok(self.n2[index - 32].clone());
        }
        if index < 96 {
            return Ok(self.n3[index - 64].clone());
        }
        if index < 128 {
            return Ok(self.n4[index - 96].clone());
        }
        if index < 136 {
            return Ok(self.n5[index - 128].clone());
        }

        bail!("index out of range")
    }

    fn get_range(&self, r: Range<usize>) -> anyhow::Result<Vec<PaiT>> {
        // range check
        ensure!(r.end < self.length as usize, "range out of range");
        let st = (r.start / 32, r.start % 32);
        let ed = (r.end / 32, r.end % 32);
        let mut v: Vec<PaiT> = Vec::new();
        let mut rstart = 0usize;
        let mut rend = 0usize;

        if st.0 == 0 {
            rstart = st.1;
            if ed.0 == 0 {
                rend = ed.1;
            } else {
                rend = self.n1.len();
            }
            let mut nx: Vec<PaiT> = self.n1[rstart..rend].iter().cloned().collect();

            v.append(&mut nx);
        }

        if st.0 <= 1 && ed.0 >= 1 {
            if st.0 == 1 {
                rstart = st.1;
            } else {
                rstart = 0;
            }
            if ed.0 == 1 {
                rend = ed.1;
            } else {
                rend = self.n2.len();
            }
            let mut nx: Vec<PaiT> = self.n2[rstart..rend].iter().cloned().collect();

            v.append(&mut nx);
        }

        if st.0 <= 2 && ed.0 >= 2 {
            if st.0 == 2 {
                rstart = st.1;
            } else {
                rstart = 0;
            }
            if ed.0 == 2 {
                rend = ed.1;
            } else {
                rend = self.n3.len();
            }
            let mut nx: Vec<PaiT> = self.n3[rstart..rend].iter().cloned().collect();

            v.append(&mut nx);
        }

        if st.0 <= 3 && ed.0 >= 3 {
            if st.0 == 3 {
                rstart = st.1;
            } else {
                rstart = 0;
            }
            if ed.0 == 3 {
                rend = ed.1;
            } else {
                rend = self.n4.len();
            }
            let mut nx: Vec<PaiT> = self.n4[rstart..rend].iter().cloned().collect();

            v.append(&mut nx);
        }

        if st.0 <= 4 && ed.0 >= 4 {
            if st.0 == 4 {
                rstart = st.1;
            } else {
                rstart = 0;
            }
            if ed.0 == 4 {
                rend = ed.1;
            } else {
                rend = self.n5.len();
            }
            let mut nx: Vec<PaiT> = self.n5[rstart..rend].iter().cloned().collect();

            v.append(&mut nx);
        }

        Ok(v)
    }

    fn load(list: &[u32]) -> Self {
        Taku::load(list).unpack()
    }
}
