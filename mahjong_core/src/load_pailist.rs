use std::{path::Path, fs::File};

use anyhow::ensure;
use arrow_array::{array::Int32Array, Array, FixedSizeListArray, ListArray, RecordBatch, StringArray, StructArray, UInt32Array};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

use crate::mahjong_generated::open_mahjong::{PaiT, Mentsu, MentsuT, MentsuFlag, MentsuType};

#[derive(Default, Debug)]
pub struct ParquetAgari {
    pub tehai: Vec<PaiT>,
    pub fulo: Vec<Mentsu>,
    pub yaku: Vec<(String, i32)>,
    pub fu: i32,
    pub han: i32,
    pub score: i32,
    pub machipai: PaiT,
    pub dora: Vec<PaiT>,
    pub uradora: Vec<PaiT>,
    pub nukidora: u32,
}

fn str_to_pais<T: AsRef<str>>(input: T) -> Vec<PaiT> {
    let s = input.as_ref();
    let mut ret: Vec<PaiT> = Vec::new();
    let mut suit: u8 = 0;
    for c in s.chars() {
        match c {
            'm' => suit = 0,
            'p' => suit = 1,
            's' => suit = 2,
            'z' => suit = 3,
            _ => {
                let pai = PaiT{
                    pai_num: suit * 9 + c.to_digit(10).unwrap() as u8 - 1,
                    id: 0,
                    is_nakare: false,
                    is_riichi: false,
                    is_tsumogiri: false
                };
                ret.push(pai);
            }
        }
    }

    ret
}

fn str_to_fulo(s: &str) -> Mentsu {
    let mut ret: MentsuT = Default::default();
    let mut suit: u8 = 0;
    let mut index: usize = 0;
    let mut naki = false;
    for c in s.chars() {
        match c {
            'm' => suit = 0,
            'p' => suit = 1,
            's' => suit = 2,
            'z' => suit = 3,
            '-' => {
                ret.pai_list[index-1].flag = MentsuFlag::FLAG_KAMICHA;
                naki = true;
            },
            '=' => {
                ret.pai_list[index-1].flag = MentsuFlag::FLAG_TOIMEN;
                naki = true;
            },
            '+' => {
                ret.pai_list[index-1].flag = MentsuFlag::FLAG_SIMOCHA;
                naki = true;
            },
            _ => {
                let pai_num = suit * 9 + c.to_digit(10).unwrap() as u8 - 1;
                ret.pai_list[index].pai_num = pai_num;
                index += 1;
            }
        }
    }

    if index == 4 {
        ret.mentsu_type = if naki { MentsuType::TYPE_MINKAN } else { MentsuType::TYPE_ANKAN };
    } else if ret.pai_list[0].pai_num == ret.pai_list[1].pai_num {
        ret.mentsu_type = MentsuType::TYPE_KOUTSU;
    } else {
        ret.mentsu_type = MentsuType::TYPE_SHUNTSU;
    }

    ret.pack()
}


impl ParquetAgari {
    pub fn parse_tehai_string(&mut self, s: &str) {
        let mut part  = s.split(",");

        let tehai = part.next().unwrap();

        self.tehai = str_to_pais(tehai);

        for hand in part {
            self.fulo.push(str_to_fulo(hand));
        }
    }

    pub fn get_row_with_types(&mut self, record_batch: &RecordBatch, row_index: usize) {
        record_batch.columns().iter().enumerate().for_each(|(i, column)| {
            let binding = record_batch.schema();
            let field = binding.field(i);
            let name = field.name();

            if name == &String::from("tehai") {
                let string_array = column.as_any().downcast_ref::<StringArray>().unwrap();
                let cell = string_array.value(row_index);
                self.parse_tehai_string(cell);
            }

            if name == &String::from("fu") {
                let int_array = column.as_any().downcast_ref::<Int32Array>().unwrap();
                self.fu = int_array.value(row_index);
            }

            if name == &String::from("yaku") {
                let string_array = column.as_any().downcast_ref::<ListArray>().unwrap();
                let cell = string_array.value(row_index);

                let yaku_array = cell.as_any().downcast_ref::<StructArray>().unwrap();

                let yaku_names = yaku_array.column(0).as_any().downcast_ref::<StringArray>().unwrap();
                let yaku_hans = yaku_array.column(1).as_any().downcast_ref::<Int32Array>().unwrap();

                for i in 0..yaku_array.len() {
                    let yaku_name = yaku_names.value(i);
                    let yaku_han = yaku_hans.value(i);

                    self.yaku.push((String::from(yaku_name), yaku_han));
                }
            }

            if name == &String::from("han") {
                let int_array = column.as_any().downcast_ref::<Int32Array>().unwrap();
                self.han = int_array.value(row_index);
            }

            if name == &String::from("score") {
                let int_array = column.as_any().downcast_ref::<Int32Array>().unwrap();
                self.score = int_array.value(row_index);
            }

            if name == &String::from("dora_orig") {
                let string_array = column.as_any().downcast_ref::<ListArray>().unwrap();
                let cell = string_array.value(row_index);

                let dora_array = cell.as_any().downcast_ref::<UInt32Array>().unwrap();

                self.dora = (0..dora_array.len()).map(|idx| {
                    let dora = dora_array.value(idx);
                    PaiT{
                        pai_num: (dora >> 2) as u8,
                        id: (dora & 3) as u8,
                        is_nakare: false,
                        is_riichi: false,
                        is_tsumogiri: false
                    }
                }).collect();
            }

            if name == &String::from("machipai") {
                let int_array = column.as_any().downcast_ref::<UInt32Array>().unwrap();
                let cell = int_array.value(row_index);

                self.machipai = PaiT{
                    pai_num: (cell >> 2) as u8,
                    id: (cell & 3) as u8,
                    is_nakare: false,
                    is_riichi: false,
                    is_tsumogiri: false
                };
            }

            if name == &String::from("uradora_orig") {
                let string_array = column.as_any().downcast_ref::<ListArray>().unwrap();
                let cell = string_array.value(row_index);

                let dora_array = cell.as_any().downcast_ref::<UInt32Array>().unwrap();

                self.uradora = (0..dora_array.len()).map(|idx| {
                    let dora = dora_array.value(idx);
                    PaiT{
                        pai_num: (dora >> 2) as u8,
                        id: (dora & 3) as u8,
                        is_nakare: false,
                        is_riichi: false,
                        is_tsumogiri: false
                    }
                }).collect();
            }

            if name == &String::from("nukidora") {
                let int_array = column.as_any().downcast_ref::<UInt32Array>().unwrap();
                let cell = int_array.value(row_index);

                self.nukidora = cell;
            }
        });
    }

}




pub fn load_pailist<P: AsRef<Path>>(path: P, row_index: usize) -> anyhow::Result<Vec<u32>>{
    let file = File::open(path)?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
    let mut reader = builder.build()?;
    let read_result = reader.next();

    if let Some(arrow_result) = read_result {
        let record_batch = arrow_result?;
        if let Some(column) = record_batch.column_by_name("pai_ids") {
            let row_list = column.as_any().downcast_ref::<FixedSizeListArray>();

            if let Some(rows) = row_list {
                ensure!(row_index < rows.len(), "row_index must be less than row length");
                let cell = rows.value(row_index);
                let ret = cell.as_any().downcast_ref::<UInt32Array>();

                if let Some(row) = ret {
                    let values = row.values().to_vec();

                    return Ok(values);
                } else {
                    ensure!(false, "cannot read cell data");
                }
            } else {
                ensure!(false, "cannot read columns by list");
            }
        } else {
            ensure!(false, "cannot load pai_ids column");
        }
    } else {
        ensure!(false, "cannot load parquet record");
    }

    Ok(vec![])
}

pub fn load_agari_tehai<P: AsRef<Path>>(path: P, row_index: usize) -> anyhow::Result<ParquetAgari>{
    let file = File::open(path)?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
    let mut reader = builder.build()?;
    let read_result = reader.next();
    let mut ret = ParquetAgari::default();

    if let Some(arrow_result) = read_result {
        let record_batch = arrow_result?;

        ret.get_row_with_types(&record_batch, row_index);

        return Ok(ret)
    } else {
        ensure!(false, "cannot load parquet record");
    }

    Ok(ret)
}
