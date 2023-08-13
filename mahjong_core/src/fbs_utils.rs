use crate::mahjong_generated::open_mahjong::{Bahai, BahaiT, FixedString, FixedStringT, Pai, PaiT};
use rand::prelude::SliceRandom;

impl From<&[u8]> for FixedStringT {
    fn from(src: &[u8]) -> Self {
        let mut s = Self::default();
        let mut chunks = src.chunks(32);

        let n1 = chunks.next();

        if let Some(n) = n1 {
            s.n1.copy_from_slice(n);
        } else {
            return s;
        }

        let n2 = chunks.next();

        if let Some(n) = n2 {
            s.n2.copy_from_slice(n);
        } else {
            return s;
        }

        let n3 = chunks.next();

        if let Some(n) = n3 {
            s.n3.copy_from_slice(n);
        } else {
            return s;
        }

        let n4 = chunks.next();

        if let Some(n) = n4 {
            s.n4.copy_from_slice(n);
        } else {
            return s;
        }

        let n5 = chunks.next();

        if let Some(n) = n5 {
            s.n5.copy_from_slice(n);
        } else {
            return s;
        }

        let n6 = chunks.next();

        if let Some(n) = n6 {
            s.n6.copy_from_slice(n);
        } else {
            return s;
        }

        let n7 = chunks.next();

        if let Some(n) = n7 {
            s.n7.copy_from_slice(n);
        } else {
            return s;
        }

        let n8 = chunks.next();

        if let Some(n) = n8 {
            s.n8.copy_from_slice(n);
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

pub trait BahaiControl {
    fn create_shuffled() -> Self;
    fn search(&self, target: &PaiT) -> Result<usize, ()>;
    fn get(&self, index: usize) -> Result<PaiT, ()>;
}

impl BahaiControl for Bahai {
    fn create_shuffled() -> Self {
        // Pai配列の初期化
        let mut hai_array: Vec<Pai> = Vec::new();
        let mut rng = rand::thread_rng();
        let mut dst = [Pai::new(0, 0, false, false, false); 32];
        let mut dst2 = [Pai::new(0, 0, false, false, false); 8];
        let mut s = Self::new(&dst, &dst, &dst, &dst, &dst2);

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

        s
    }

    fn search(&self, target: &PaiT) -> Result<usize, ()> {
        self.unpack().search(target)
    }

    fn get(&self, index: usize) -> Result<PaiT, ()> {
        self.unpack().get(index)
    }
}

impl BahaiControl for BahaiT {
    fn create_shuffled() -> Self {
        Bahai::create_shuffled().unpack()
    }

    fn search(&self, target: &PaiT) -> Result<usize, ()> {
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
        Err(())
    }

    fn get(&self, index: usize) -> Result<PaiT, ()> {
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

        Err(())
    }
}
