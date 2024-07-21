use anyhow::anyhow;
use chrono::Utc;
use parquet::arrow::arrow_reader::{ParquetRecordBatchReader, ParquetRecordBatchReaderBuilder};
use std::{path::Path, sync::Arc};

//#[cfg(feature = "write-log")]
use arrow_array::builder::{Int32Builder, ListBuilder, StringBuilder, StructBuilder};
use arrow_array::types::{Int32Type, UInt32Type};
use arrow_array::FixedSizeListArray;
use arrow_array::{
    array::{ArrayRef, BooleanArray, Int32Array, ListArray, StringArray, UInt32Array, UInt64Array},
    RecordBatch,
};
use arrow_schema::{DataType, Field};
use parquet::{
    arrow::arrow_writer::ArrowWriter,
    basic::{Compression, Encoding, GzipLevel},
    file::properties::*,
};
use std::fs::{self, File};
use walkdir::{IntoIter, WalkDir};

fn num_to_hai(num_list: &[Option<u32>], has_aka: u32) -> String {
    let colors = ['m', 'p', 's', 'z'];
    let mut ret = String::new();
    let mut suit = 255;

    for opn in num_list {
        if let Some(pn) = opn {
            let s = *pn / 36;

            if s != suit {
                suit = s;
                ret.push(colors[s as usize]);
            }

            let mut num = *pn % 36 / 4 + 1;
            let id = *pn % 4;

            if s < 3 {
                let aka_num = has_aka >> (s * 2) & 0b11;
                if num == 5 && (id as u32) < aka_num {
                    num = 0;
                }
            }

            ret.push_str(&format!("{}", num));
        }
    }

    ret
}

pub struct PlayLog {
    game_log: GameLog,
    game_player_log: GamePlayerLog,
    rule_log: RuleLog,
    kyoku_log: KyokuLog,
    haipais_log: HaipaisLog,
    agaris_log: AgarisLog,
    nagare_log: NagareLog,
    actions_log: ActionsLog,
}

pub struct PaiyamaBatch {
    entries: IntoIter,
    batch_reader: Option<ParquetRecordBatchReader>,
    id_list: Option<UInt64Array>,
    pai_ids_list: Option<FixedSizeListArray>,
    index: usize,
}

fn next_batch(
    batch_reader: &mut ParquetRecordBatchReader,
) -> anyhow::Result<(Option<UInt64Array>, Option<FixedSizeListArray>)> {
    let op_record_batch = batch_reader.next();
    if let Some(record_batch_r) = op_record_batch {
        let record_batch = record_batch_r?;
        let id_list = record_batch
            .column(0)
            .as_any()
            .downcast_ref::<UInt64Array>()
            .ok_or_else(|| anyhow!("Failed to downcast to UInt64Array"))?
            .clone();
        let pai_ids_list = record_batch
            .column(1)
            .as_any()
            .downcast_ref::<FixedSizeListArray>()
            .ok_or_else(|| anyhow!("Failed to downcast to FixedSizeListArray"))?
            .clone();
        return Ok((Some(id_list), Some(pai_ids_list)));
    }
    Ok((None, None))
}

fn next_entry(
    entry: &mut IntoIter,
) -> anyhow::Result<(
    Option<ParquetRecordBatchReader>,
    Option<UInt64Array>,
    Option<FixedSizeListArray>,
)> {
    loop {
        match entry.next() {
            Some(Ok(entry)) => {
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension == "parquet" {
                            let file = File::open(path)?;
                            let mut reader =
                                ParquetRecordBatchReaderBuilder::try_new(file)?.build()?;

                            let op_record_batch = reader.next();
                            if let Some(record_batch_r) = op_record_batch {
                                let record_batch = record_batch_r?;
                                let id_list = record_batch
                                    .column(0)
                                    .as_any()
                                    .downcast_ref::<UInt64Array>()
                                    .ok_or_else(|| anyhow!("Failed to downcast to UInt64Array"))?
                                    .clone();
                                let pai_ids_list = record_batch
                                    .column(1)
                                    .as_any()
                                    .downcast_ref::<FixedSizeListArray>()
                                    .ok_or_else(|| {
                                        anyhow!("Failed to downcast to FixedSizeListArray")
                                    })?
                                    .clone();
                                return Ok((Some(reader), Some(id_list), Some(pai_ids_list)));
                            }
                        }
                    }
                }
            }
            Some(Err(e)) => return Err(anyhow!(e)),
            None => return Ok((None, None, None)),
        }
    }
}

impl PaiyamaBatch {
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        let entries = WalkDir::new(root).into_iter();
        Self {
            entries,
            batch_reader: None,
            id_list: None,
            pai_ids_list: None,
            index: 0,
        }
    }
}

impl Iterator for PaiyamaBatch {
    type Item = anyhow::Result<(u64, Vec<u32>)>;

    fn next(&mut self) -> Option<Self::Item> {
        match (
            &mut self.batch_reader,
            &mut self.id_list,
            &mut self.pai_ids_list,
            self.index,
        ) {
            (None, _, _, _) => {
                let result = next_entry(&mut self.entries);
                if let Ok((Some(reader), Some(id_list), Some(pai_ids_list))) = result {
                    let id = id_list.value(0);
                    let array = pai_ids_list.value(0);
                    let pai_ids = array
                        .as_any()
                        .downcast_ref::<UInt32Array>()
                        .unwrap()
                        .values();
                    self.index = 1;
                    self.batch_reader = Some(reader);
                    self.id_list = Some(id_list);
                    self.pai_ids_list = Some(pai_ids_list);
                    return Some(Ok((id, pai_ids.to_vec())));
                } else {
                    return None;
                }
            }
            (Some(ref mut batch_reader), Some(ref id_list), Some(ref pai_ids_list), index) => {
                if index < id_list.len() {
                    let id = id_list.value(index);
                    let array = pai_ids_list.value(index);
                    let pai_ids = array
                        .as_any()
                        .downcast_ref::<UInt32Array>()
                        .unwrap()
                        .values();
                    self.index += 1;
                    return Some(Ok((id, pai_ids.to_vec())));
                } else {
                    let result = next_batch(batch_reader);
                    if let Ok((Some(id_list), Some(pai_ids_list))) = result {
                        let id = id_list.value(0);
                        let array = pai_ids_list.value(0);
                        let pai_ids = array
                            .as_any()
                            .downcast_ref::<UInt32Array>()
                            .unwrap()
                            .values();
                        self.id_list = Some(id_list);
                        self.pai_ids_list = Some(pai_ids_list);
                        self.index = 1;
                        return Some(Ok((id, pai_ids.to_vec())));
                    } else {
                        let result = next_entry(&mut self.entries);
                        if let Ok((Some(reader), Some(id_list), Some(pai_ids_list))) = result {
                            let id = id_list.value(0);
                            let array = pai_ids_list.value(0);
                            let pai_ids = array
                                .as_any()
                                .downcast_ref::<UInt32Array>()
                                .unwrap()
                                .values();
                            self.index = 1;
                            self.batch_reader = Some(reader);
                            self.id_list = Some(id_list);
                            self.pai_ids_list = Some(pai_ids_list);
                            return Some(Ok((id, pai_ids.to_vec())));
                        } else {
                            return None;
                        }
                    }
                }
            }
            (_, _, _, _) => None,
        }
    }
}

#[derive(Default)]
pub struct GameLog {
    id_vec: Vec<String>,
    started_at_vec: Vec<u64>,
}

#[derive(Default)]
pub struct GamePlayerLog {
    game_id_vec: Vec<String>,
    name_vec: Vec<String>,
    player_index_vec: Vec<i32>,
}

#[derive(Default)]
pub struct RuleLog {
    game_id_vec: Vec<String>,
    enable_kuitan_vec: Vec<bool>,
    enable_atozuke_vec: Vec<bool>,
    enable_pao_vec: Vec<bool>,
    enable_tobi_vec: Vec<bool>,
    enable_wareme_vec: Vec<bool>,
    enable_kuinaoshi_vec: Vec<bool>,
    enable_kiriage_vec: Vec<bool>,
    enable_agariyame_vec: Vec<bool>,
    enable_minus_riichi_vec: Vec<bool>,
    enable_ryanhan_shibari_vec: Vec<bool>,
    enable_keiten_vec: Vec<bool>,
    enable_glass_pai: Vec<bool>,
    aka_type_vec: Vec<u32>,
    shanyu_score_vec: Vec<i32>,
    nannyu_score_vec: Vec<i32>,
    uradora_type_vec: Vec<i32>,
    furiten_riichi_type: Vec<u32>,
    oyanagare_type: Vec<u32>,
    double_ron_type_vec: Vec<u32>,
    initial_score_vec: Vec<u32>,
    kan_in_riichi_type_vec: Vec<u32>,
    is_demo_vec: Vec<bool>,
    is_soku_vec: Vec<bool>,
    is_sanma_vec: Vec<bool>,
    level_vec: Vec<i32>,
}

#[derive(Default)]
pub struct KyokuLog {
    id_vec: Vec<u64>,
    game_id_vec: Vec<String>,
    kyoku_num_vec: Vec<i32>,
    honba_vec: Vec<i32>,
    riichi_bou_vec: Vec<i32>,
    scores_vec: Vec<Option<Vec<Option<i32>>>>,
    kazes_vec: Vec<Option<Vec<Option<i32>>>>,
}

#[derive(Default)]
pub struct HaipaisLog {
    kyoku_id_vec: Vec<u64>,
    player_index_vec: Vec<i32>,
    haipai_vec: Vec<String>,
    pai_ids_vec: Vec<Option<Vec<Option<u32>>>>,
}

pub struct AgarisLog {
    kyoku_id_vec: Vec<u64>,
    machipai_vec: Vec<u32>,
    score_vec: Vec<i32>,
    fu_vec: Vec<i32>,
    han_vec: Vec<i32>,
    tehai_vec: Vec<String>,
    pai_ids_vec: Vec<Option<Vec<Option<u32>>>>,
    yaku_vec_builder: ListBuilder<StructBuilder>,
    dora_vec: Vec<Option<Vec<Option<u32>>>>,
    uradora_vec: Vec<Option<Vec<Option<u32>>>>,
    dora_orig_vec: Vec<Option<Vec<Option<u32>>>>,
    uradora_orig_vec: Vec<Option<Vec<Option<u32>>>>,
    who_vec: Vec<i32>,
    by_vec: Vec<i32>,
    score_diff_vec: Vec<Option<Vec<Option<i32>>>>,
    owari_vec: Vec<bool>,
    nukidora_vec: Vec<u32>,
}

#[derive(Default)]
pub struct NagareLog {
    kyoku_id_vec: Vec<u64>,
    name_vec: Vec<String>,
    score_diff_vec: Vec<Option<Vec<Option<i32>>>>,
}

#[derive(Default)]
pub struct ActionsLog {
    kyoku_id_vec: Vec<u64>,
    player_index_vec: Vec<i32>,
    seq_vec: Vec<i32>,
    type_vec: Vec<String>,
    pais_vec: Vec<String>,
    pai_id_vec: Vec<u32>,
}

impl GameLog {
    pub fn append(&mut self, id: String, started_at: u64) {
        self.id_vec.push(id);
        self.started_at_vec.push(started_at);
    }

    pub fn save_to_parquet<P: AsRef<Path>>(self, path: P) -> anyhow::Result<()> {
        let file = File::create(path.as_ref())
            .map_err(|e| anyhow!("Failed to create file at {:?}: {}", path.as_ref(), e))?;

        let props = WriterProperties::builder()
            .set_writer_version(WriterVersion::PARQUET_2_0)
            .set_encoding(Encoding::PLAIN)
            .set_compression(Compression::GZIP(GzipLevel::default()))
            .build();

        let id_vec = StringArray::from(self.id_vec);
        let started_at_vec = UInt64Array::from(self.started_at_vec);

        let batch = RecordBatch::try_from_iter(vec![
            ("id", Arc::new(id_vec) as ArrayRef),
            ("started_at", Arc::new(started_at_vec) as ArrayRef),
        ])?;

        let mut writer = ArrowWriter::try_new(file, batch.schema(), Some(props))?;
        writer
            .write(&batch)
            .map_err(|e| anyhow!("Failed to write batch to file: {}", e))?;

        writer
            .close()
            .map_err(|e| anyhow!("Failed to close writer: {}", e))?;

        Ok(())
    }
}

impl GamePlayerLog {
    pub fn append(&mut self, name: String, game_id: String, player_index: i32) {
        self.name_vec.push(name);
        self.game_id_vec.push(game_id);
        self.player_index_vec.push(player_index);
    }

    pub fn save_to_parquet<P: AsRef<Path>>(self, path: P) -> anyhow::Result<()> {
        let file = File::create(path.as_ref())
            .map_err(|e| anyhow!("Failed to create file at {:?}: {}", path.as_ref(), e))?;

        let props = WriterProperties::builder()
            .set_writer_version(WriterVersion::PARQUET_2_0)
            .set_encoding(Encoding::PLAIN)
            .set_compression(Compression::GZIP(GzipLevel::default()))
            .build();

        let name_vec = StringArray::from(self.name_vec);
        let game_id_vec = StringArray::from(self.game_id_vec);
        let player_index_vec = Int32Array::from(self.player_index_vec);

        let batch = RecordBatch::try_from_iter(vec![
            ("game_id", Arc::new(game_id_vec) as ArrayRef),
            ("name", Arc::new(name_vec) as ArrayRef),
            ("player_index", Arc::new(player_index_vec) as ArrayRef),
        ])?;

        let mut writer = ArrowWriter::try_new(file, batch.schema(), Some(props))?;
        writer
            .write(&batch)
            .map_err(|e| anyhow!("Failed to write batch to file: {}", e))?;

        writer
            .close()
            .map_err(|e| anyhow!("Failed to close writer: {}", e))?;

        Ok(())
    }
}

impl RuleLog {
    pub fn append(
        &mut self,
        game_id: String,
        enable_kuitan: bool,
        enable_atozuke: bool,
        enable_pao: bool,
        enable_tobi: bool,
        enable_wareme: bool,
        enable_kuinaoshi: bool,
        enable_kiriage: bool,
        enable_agariyame: bool,
        enable_minus_riichi: bool,
        enable_ryanhan_shibari: bool,
        enable_keiten: bool,
        enable_glass_pai: bool,
        aka_type: u32,
        shanyu_score: i32,
        nannyu_score: i32,
        uradora_type: i32,
        furiten_riichi_type: u32,
        oyanagare_type: u32,
        double_ron_type: u32,
        initial_score: u32,
        kan_in_riichi_type: u32,
        is_demo: bool,
        is_sanma: bool,
    ) {
        self.game_id_vec.push(game_id);
        self.enable_kuitan_vec.push(enable_kuitan);
        self.enable_atozuke_vec.push(enable_atozuke);
        self.enable_pao_vec.push(enable_pao);
        self.enable_tobi_vec.push(enable_tobi);
        self.enable_wareme_vec.push(enable_wareme);
        self.enable_kuinaoshi_vec.push(enable_kuinaoshi);
        self.enable_kiriage_vec.push(enable_kiriage);
        self.enable_agariyame_vec.push(enable_agariyame);
        self.enable_minus_riichi_vec.push(enable_minus_riichi);
        self.enable_ryanhan_shibari_vec.push(enable_ryanhan_shibari);
        self.enable_keiten_vec.push(enable_keiten);
        self.enable_glass_pai.push(enable_glass_pai);
        self.aka_type_vec.push(aka_type);
        self.shanyu_score_vec.push(shanyu_score);
        self.nannyu_score_vec.push(nannyu_score);
        self.uradora_type_vec.push(uradora_type);
        self.furiten_riichi_type.push(furiten_riichi_type);
        self.oyanagare_type.push(oyanagare_type);
        self.double_ron_type_vec.push(double_ron_type);
        self.initial_score_vec.push(initial_score);
        self.kan_in_riichi_type_vec.push(kan_in_riichi_type);
        self.is_demo_vec.push(is_demo);
        self.is_soku_vec.push(false);
        self.is_sanma_vec.push(is_sanma);
        self.level_vec.push(0);
    }

    pub fn save_to_parquet<P: AsRef<Path>>(self, path: P) -> anyhow::Result<()> {
        let file = File::create(path.as_ref())
            .map_err(|e| anyhow!("Failed to create file at {:?}: {}", path.as_ref(), e))?;

        let props = WriterProperties::builder()
            .set_writer_version(WriterVersion::PARQUET_2_0)
            .set_encoding(Encoding::PLAIN)
            .set_compression(Compression::GZIP(GzipLevel::default()))
            .build();

        let game_id_vec = StringArray::from(self.game_id_vec);
        let enable_kuitan_vec = BooleanArray::from(self.enable_kuitan_vec);
        let enable_atozuke_vec = BooleanArray::from(self.enable_atozuke_vec);
        let enable_pao_vec = BooleanArray::from(self.enable_pao_vec);
        let enable_tobi_vec = BooleanArray::from(self.enable_tobi_vec);
        let enable_wareme_vec = BooleanArray::from(self.enable_wareme_vec);
        let enable_kuinaoshi_vec = BooleanArray::from(self.enable_kuinaoshi_vec);
        let enable_kiriage_vec = BooleanArray::from(self.enable_kiriage_vec);
        let enable_agariyame_vec = BooleanArray::from(self.enable_agariyame_vec);
        let enable_minus_riichi_vec = BooleanArray::from(self.enable_minus_riichi_vec);
        let enable_ryanhan_shibari_vec = BooleanArray::from(self.enable_ryanhan_shibari_vec);
        let enable_keiten_vec = BooleanArray::from(self.enable_keiten_vec);
        let enable_glass_pai = BooleanArray::from(self.enable_glass_pai);
        let aka_type_vec = UInt32Array::from(self.aka_type_vec);
        let shanyu_score_vec = Int32Array::from(self.shanyu_score_vec);
        let nannyu_score_vec = Int32Array::from(self.nannyu_score_vec);
        let uradora_type_vec = Int32Array::from(self.uradora_type_vec);
        let furiten_riichi_type = UInt32Array::from(self.furiten_riichi_type);
        let oyanagare_type = UInt32Array::from(self.oyanagare_type);
        let double_ron_type_vec = UInt32Array::from(self.double_ron_type_vec);
        let initial_score_vec = UInt32Array::from(self.initial_score_vec);
        let kan_in_riichi_type_vec = UInt32Array::from(self.kan_in_riichi_type_vec);
        let is_demo_vec = BooleanArray::from(self.is_demo_vec);
        let is_soku_vec = BooleanArray::from(self.is_soku_vec);
        let is_sanma_vec = BooleanArray::from(self.is_sanma_vec);
        let level_vec = Int32Array::from(self.level_vec);

        let batch = RecordBatch::try_from_iter(vec![
            ("game_id", Arc::new(game_id_vec) as ArrayRef),
            ("enable_kuitan", Arc::new(enable_kuitan_vec) as ArrayRef),
            ("enable_atozuke", Arc::new(enable_atozuke_vec) as ArrayRef),
            ("enable_pao", Arc::new(enable_pao_vec) as ArrayRef),
            ("enable_tobi", Arc::new(enable_tobi_vec) as ArrayRef),
            ("enable_wareme", Arc::new(enable_wareme_vec) as ArrayRef),
            (
                "enable_kuinaoshi",
                Arc::new(enable_kuinaoshi_vec) as ArrayRef,
            ),
            ("enable_kiriage", Arc::new(enable_kiriage_vec) as ArrayRef),
            (
                "enable_agariyame",
                Arc::new(enable_agariyame_vec) as ArrayRef,
            ),
            (
                "enable_minus_riichi",
                Arc::new(enable_minus_riichi_vec) as ArrayRef,
            ),
            (
                "enable_ryanhan_shibari",
                Arc::new(enable_ryanhan_shibari_vec) as ArrayRef,
            ),
            ("enable_keiten", Arc::new(enable_keiten_vec) as ArrayRef),
            ("enable_glass_pai", Arc::new(enable_glass_pai) as ArrayRef),
            ("aka_type", Arc::new(aka_type_vec) as ArrayRef),
            ("shanyu_score", Arc::new(shanyu_score_vec) as ArrayRef),
            ("nannyu_score", Arc::new(nannyu_score_vec) as ArrayRef),
            ("uradora_type", Arc::new(uradora_type_vec) as ArrayRef),
            (
                "furiten_riichi_type",
                Arc::new(furiten_riichi_type) as ArrayRef,
            ),
            ("oyanagare_type", Arc::new(oyanagare_type) as ArrayRef),
            ("double_ron_type", Arc::new(double_ron_type_vec) as ArrayRef),
            ("initial_score", Arc::new(initial_score_vec) as ArrayRef),
            (
                "kan_in_riichi_type",
                Arc::new(kan_in_riichi_type_vec) as ArrayRef,
            ),
            ("is_demo", Arc::new(is_demo_vec) as ArrayRef),
            ("is_soku", Arc::new(is_soku_vec) as ArrayRef),
            ("is_sanma", Arc::new(is_sanma_vec) as ArrayRef),
            ("level", Arc::new(level_vec) as ArrayRef),
        ])?;

        let mut writer = ArrowWriter::try_new(file, batch.schema(), Some(props))?;
        writer
            .write(&batch)
            .map_err(|e| anyhow!("Failed to write batch to file: {}", e))?;
        writer
            .close()
            .map_err(|e| anyhow!("Failed to close writer: {}", e))?;
        Ok(())
    }
}

impl KyokuLog {
    pub fn append(
        &mut self,
        id: u64,
        game_id: String,
        kyoku_num: i32,
        honba: i32,
        riichi_bou: i32,
        scores: &[Option<i32>],
        kazes: &[Option<i32>],
    ) {
        self.id_vec.push(id);
        self.game_id_vec.push(game_id);
        self.kyoku_num_vec.push(kyoku_num);
        self.honba_vec.push(honba);
        self.riichi_bou_vec.push(riichi_bou);
        self.scores_vec.push(Some(scores.to_vec()));
        self.kazes_vec.push(Some(kazes.to_vec()));
    }

    pub fn save_to_parquet<P: AsRef<Path>>(self, path: P) -> anyhow::Result<()> {
        let file = File::create(path.as_ref())
            .map_err(|e| anyhow!("Failed to create file at {:?}: {}", path.as_ref(), e))?;

        let props = WriterProperties::builder()
            .set_writer_version(WriterVersion::PARQUET_2_0)
            .set_encoding(Encoding::PLAIN)
            .set_compression(Compression::GZIP(GzipLevel::default()))
            .build();

        let id_vec = UInt64Array::from(self.id_vec);
        let game_id_vec = StringArray::from(self.game_id_vec);
        let kyoku_num_vec = Int32Array::from(self.kyoku_num_vec);
        let honba_vec = Int32Array::from(self.honba_vec);
        let riichi_bou_vec = Int32Array::from(self.riichi_bou_vec);
        let scores_vec =
            FixedSizeListArray::from_iter_primitive::<Int32Type, _, _>(self.scores_vec, 4);
        let kazes_vec =
            FixedSizeListArray::from_iter_primitive::<Int32Type, _, _>(self.kazes_vec, 4);

        let batch = RecordBatch::try_from_iter(vec![
            ("id", Arc::new(id_vec) as ArrayRef),
            ("game_id", Arc::new(game_id_vec) as ArrayRef),
            ("kyoku_num", Arc::new(kyoku_num_vec) as ArrayRef),
            ("honba", Arc::new(honba_vec) as ArrayRef),
            ("reachbou", Arc::new(riichi_bou_vec) as ArrayRef),
            ("scores", Arc::new(scores_vec) as ArrayRef),
            ("kazes", Arc::new(kazes_vec) as ArrayRef),
        ])?;

        let mut writer = ArrowWriter::try_new(file, batch.schema(), Some(props))?;
        writer
            .write(&batch)
            .map_err(|e| anyhow!("Failed to write batch to file: {}", e))?;

        writer
            .close()
            .map_err(|e| anyhow!("Failed to close writer: {}", e))?;

        Ok(())
    }
}

impl AgarisLog {
    pub fn new() -> Self {
        // yakuはname(役の名前)とhan(翻数)の2つのフィールドを持つ構造体のリストが一つのレコードとして保存される
        let yaku_vec_builder = ListBuilder::new(StructBuilder::new(
            vec![
                Field::new("name", DataType::Utf8, false),
                Field::new("han", DataType::Int32, false),
            ],
            vec![
                Box::new(StringBuilder::new()),
                Box::new(Int32Builder::new()),
            ],
        ));

        Self {
            kyoku_id_vec: Vec::new(),
            machipai_vec: Vec::new(),
            score_vec: Vec::new(),
            fu_vec: Vec::new(),
            han_vec: Vec::new(),
            tehai_vec: Vec::new(),
            pai_ids_vec: Vec::new(),
            yaku_vec_builder: yaku_vec_builder,
            dora_vec: Vec::new(),
            uradora_vec: Vec::new(),
            dora_orig_vec: Vec::new(),
            uradora_orig_vec: Vec::new(),
            who_vec: Vec::new(),
            by_vec: Vec::new(),
            score_diff_vec: Vec::new(),
            owari_vec: Vec::new(),
            nukidora_vec: Vec::new(),
        }
    }

    pub fn append(
        &mut self,
        kyoku_id: u64,
        machipai: u32,
        score: i32,
        fu: i32,
        han: i32,
        pai_ids: &[Option<u32>],
        yaku: &[(String, i32)],
        dora: &[Option<u32>],
        uradora: &[Option<u32>],
        dora_orig: &[Option<u32>],
        uradora_orig: &[Option<u32>],
        who: i32,
        by: i32,
        score_diff: &[Option<i32>],
        owari: bool,
        nukidora: u32,
    ) {
        self.kyoku_id_vec.push(kyoku_id);
        self.machipai_vec.push(machipai);
        self.score_vec.push(score);
        self.fu_vec.push(fu);
        self.han_vec.push(han);
        self.tehai_vec.push(num_to_hai(pai_ids, 0));
        self.pai_ids_vec.push(Some(pai_ids.to_vec()));
        self.dora_vec.push(Some(dora.to_vec()));
        self.uradora_vec.push(Some(uradora.to_vec()));
        self.dora_orig_vec.push(Some(dora_orig.to_vec()));
        self.uradora_orig_vec.push(Some(uradora_orig.to_vec()));
        self.who_vec.push(who);
        self.by_vec.push(by);
        self.score_diff_vec.push(Some(score_diff.to_vec()));
        self.owari_vec.push(owari);
        self.nukidora_vec.push(nukidora);

        for (name, han) in yaku {
            self.yaku_vec_builder
                .values()
                .field_builder::<StringBuilder>(0)
                .unwrap()
                .append_value(name);
            self.yaku_vec_builder
                .values()
                .field_builder::<Int32Builder>(1)
                .unwrap()
                .append_value(*han);
            self.yaku_vec_builder.values().append(true);
        }

        self.yaku_vec_builder.append(true);
    }

    pub fn save_to_parquet<P: AsRef<Path>>(mut self, path: P) -> anyhow::Result<()> {
        let file = File::create(path.as_ref())
            .map_err(|e| anyhow!("Failed to create file at {:?}: {}", path.as_ref(), e))?;

        let props = WriterProperties::builder()
            .set_writer_version(WriterVersion::PARQUET_2_0)
            .set_encoding(Encoding::PLAIN)
            .set_compression(Compression::GZIP(GzipLevel::default()))
            .build();

        let kyoku_id_vec = UInt64Array::from(self.kyoku_id_vec);
        let machipai_vec = UInt32Array::from(self.machipai_vec);
        let score_vec = Int32Array::from(self.score_vec);
        let fu_vec = Int32Array::from(self.fu_vec);
        let han_vec = Int32Array::from(self.han_vec);
        let tehai_vec = StringArray::from(self.tehai_vec);
        let pai_ids_vec = ListArray::from_iter_primitive::<UInt32Type, _, _>(self.pai_ids_vec);
        let yaku_vec = self.yaku_vec_builder.finish();
        let dora_vec = ListArray::from_iter_primitive::<UInt32Type, _, _>(self.dora_vec);
        let uradora_vec = ListArray::from_iter_primitive::<UInt32Type, _, _>(self.uradora_vec);
        let dora_orig_vec = ListArray::from_iter_primitive::<UInt32Type, _, _>(self.dora_orig_vec);
        let uradora_orig_vec =
            ListArray::from_iter_primitive::<UInt32Type, _, _>(self.uradora_orig_vec);
        let who_vec = Int32Array::from(self.who_vec);
        let by_vec = Int32Array::from(self.by_vec);
        let score_diff_vec =
            FixedSizeListArray::from_iter_primitive::<Int32Type, _, _>(self.score_diff_vec, 4);
        let owari_vec = BooleanArray::from(self.owari_vec);
        let nukidora_vec = UInt32Array::from(self.nukidora_vec);

        let batch = RecordBatch::try_from_iter(vec![
            ("kyoku_id", Arc::new(kyoku_id_vec) as ArrayRef),
            ("machipai", Arc::new(machipai_vec) as ArrayRef),
            ("score", Arc::new(score_vec) as ArrayRef),
            ("fu", Arc::new(fu_vec) as ArrayRef),
            ("han", Arc::new(han_vec) as ArrayRef),
            ("tehai", Arc::new(tehai_vec) as ArrayRef),
            ("pai_ids", Arc::new(pai_ids_vec) as ArrayRef),
            ("yaku", Arc::new(yaku_vec) as ArrayRef),
            ("dora", Arc::new(dora_vec) as ArrayRef),
            ("uradora", Arc::new(uradora_vec) as ArrayRef),
            ("dora_orig", Arc::new(dora_orig_vec) as ArrayRef),
            ("uradora_orig", Arc::new(uradora_orig_vec) as ArrayRef),
            ("who", Arc::new(who_vec) as ArrayRef),
            ("by", Arc::new(by_vec) as ArrayRef),
            ("score_diff", Arc::new(score_diff_vec) as ArrayRef),
            ("owari", Arc::new(owari_vec) as ArrayRef),
            ("nukidora", Arc::new(nukidora_vec) as ArrayRef),
        ])?;

        let mut writer = ArrowWriter::try_new(file, batch.schema(), Some(props))?;
        writer
            .write(&batch)
            .map_err(|e| anyhow!("Failed to write batch to file: {}", e))?;

        writer
            .close()
            .map_err(|e| anyhow!("Failed to close writer: {}", e))?;

        Ok(())
    }
}

// let pai_ids_vec = ListArray::from_iter_primitive::<Int32Type, _, _>(self.pai_ids_vec.clone());

impl HaipaisLog {
    pub fn append(&mut self, kyoku_id: u64, player_index: i32, pai_ids: &[Option<u32>]) {
        self.kyoku_id_vec.push(kyoku_id);
        self.player_index_vec.push(player_index);
        self.haipai_vec.push(num_to_hai(pai_ids, 0));
        self.pai_ids_vec.push(Some(pai_ids.to_vec()));
    }

    pub fn save_to_parquet<P: AsRef<Path>>(self, path: P) -> anyhow::Result<()> {
        let file = File::create(path.as_ref())
            .map_err(|e| anyhow!("Failed to create file at {:?}: {}", path.as_ref(), e))?;

        let props = WriterProperties::builder()
            .set_writer_version(WriterVersion::PARQUET_2_0)
            .set_encoding(Encoding::PLAIN)
            .set_compression(Compression::GZIP(GzipLevel::default()))
            .build();

        let kyoku_id_vec = UInt64Array::from(self.kyoku_id_vec);
        let player_index_vec = Int32Array::from(self.player_index_vec);
        let haipai_vec = StringArray::from(self.haipai_vec);
        let pai_ids_vec =
            FixedSizeListArray::from_iter_primitive::<UInt32Type, _, _>(self.pai_ids_vec, 13);

        let batch = RecordBatch::try_from_iter(vec![
            ("kyoku_id", Arc::new(kyoku_id_vec) as ArrayRef),
            ("player_index", Arc::new(player_index_vec) as ArrayRef),
            ("haipai", Arc::new(haipai_vec) as ArrayRef),
            ("pai_ids", Arc::new(pai_ids_vec) as ArrayRef),
        ])?;

        let mut writer = ArrowWriter::try_new(file, batch.schema(), Some(props))?;
        writer
            .write(&batch)
            .map_err(|e| anyhow!("Failed to write batch to file: {}", e))?;

        writer
            .close()
            .map_err(|e| anyhow!("Failed to close writer: {}", e))?;

        Ok(())
    }
}

impl ActionsLog {
    pub fn append(
        &mut self,
        kyoku_id: u64,
        player_index: i32,
        seq: i32,
        action_type: String,
        pai_id: u32,
    ) {
        self.kyoku_id_vec.push(kyoku_id);
        self.player_index_vec.push(player_index);
        self.seq_vec.push(seq);
        self.type_vec.push(action_type);
        self.pais_vec.push(num_to_hai(&[Some(pai_id)], 0));
        self.pai_id_vec.push(pai_id);
    }

    // #[cfg(feature = "write-log")]
    pub fn save_to_parquet<P: AsRef<Path>>(self, path: P) -> anyhow::Result<()> {
        let file = File::create(path.as_ref())
            .map_err(|e| anyhow!("Failed to create file at {:?}: {}", path.as_ref(), e))?;

        let props = WriterProperties::builder()
            .set_writer_version(WriterVersion::PARQUET_2_0)
            .set_encoding(Encoding::PLAIN)
            .set_compression(Compression::GZIP(GzipLevel::default()))
            .build();

        let kyoku_id_vec = UInt64Array::from(self.kyoku_id_vec);
        let player_index_vec = Int32Array::from(self.player_index_vec);
        let seq_vec = Int32Array::from(self.seq_vec);
        let type_vec = StringArray::from(self.type_vec);
        let pais_vec = StringArray::from(self.pais_vec);
        let pai_id_vec = UInt32Array::from(self.pai_id_vec);

        let batch = RecordBatch::try_from_iter(vec![
            ("kyoku_id", Arc::new(kyoku_id_vec) as ArrayRef),
            ("player_index", Arc::new(player_index_vec) as ArrayRef),
            ("seq", Arc::new(seq_vec) as ArrayRef),
            ("type", Arc::new(type_vec) as ArrayRef),
            ("pais", Arc::new(pais_vec) as ArrayRef),
            ("pai_id", Arc::new(pai_id_vec) as ArrayRef),
        ])?;

        let mut writer = ArrowWriter::try_new(file, batch.schema(), Some(props))?;
        writer
            .write(&batch)
            .map_err(|e| anyhow!("Failed to write batch to file: {}", e))?;

        writer
            .close()
            .map_err(|e| anyhow!("Failed to close writer: {}", e))?;

        Ok(())
    }
}

impl NagareLog {
    pub fn append(&mut self, kyoku_id: u64, name: String, score_diff: &[Option<i32>]) {
        self.kyoku_id_vec.push(kyoku_id);
        self.name_vec.push(name);
        self.score_diff_vec.push(Some(score_diff.to_vec()));
    }

    pub fn save_to_parquet<P: AsRef<Path>>(self, path: P) -> anyhow::Result<()> {
        let file = File::create(path.as_ref())
            .map_err(|e| anyhow!("Failed to create file at {:?}: {}", path.as_ref(), e))?;

        let props = WriterProperties::builder()
            .set_writer_version(WriterVersion::PARQUET_2_0)
            .set_encoding(Encoding::PLAIN)
            .set_compression(Compression::GZIP(GzipLevel::default()))
            .build();

        let kyoku_id_vec = UInt64Array::from(self.kyoku_id_vec);
        let name_vec = StringArray::from(self.name_vec);
        let score_diff_vec =
            FixedSizeListArray::from_iter_primitive::<Int32Type, _, _>(self.score_diff_vec, 4);

        let batch = RecordBatch::try_from_iter(vec![
            ("kyoku_id", Arc::new(kyoku_id_vec) as ArrayRef),
            ("name", Arc::new(name_vec) as ArrayRef),
            ("score_diff", Arc::new(score_diff_vec) as ArrayRef),
        ])?;

        let mut writer = ArrowWriter::try_new(file, batch.schema(), Some(props))?;
        writer
            .write(&batch)
            .map_err(|e| anyhow!("Failed to write batch to file: {}", e))?;

        writer
            .close()
            .map_err(|e| anyhow!("Failed to close writer: {}", e))?;

        Ok(())
    }
}

impl PlayLog {
    pub fn new() -> Self {
        Self {
            game_log: GameLog::default(),
            game_player_log: GamePlayerLog::default(),
            rule_log: RuleLog::default(),
            kyoku_log: KyokuLog::default(),
            haipais_log: HaipaisLog::default(),
            agaris_log: AgarisLog::new(),
            nagare_log: NagareLog::default(),
            actions_log: ActionsLog::default(),
        }
    }

    pub fn append_game_log(&mut self, id: String, started_at: u64) {
        self.game_log.append(id, started_at);
    }

    pub fn append_game_player_log(&mut self, name: String, game_id: String, player_index: i32) {
        self.game_player_log.append(name, game_id, player_index);
    }

    pub fn append_rule_log(
        &mut self,
        game_id: String,
        enable_kuitan: bool,
        enable_atozuke: bool,
        enable_pao: bool,
        enable_tobi: bool,
        enable_wareme: bool,
        enable_kuinaoshi: bool,
        enable_kiriage: bool,
        enable_agariyame: bool,
        enable_minus_riichi: bool,
        enable_ryanhan_shibari: bool,
        enable_keiten: bool,
        enable_glass_pai: bool,
        aka_type: u32,
        shanyu_score: i32,
        nannyu_score: i32,
        uradora_type: i32,
        furiten_riichi_type: u32,
        oyanagare_type: u32,
        double_ron_type: u32,
        initial_score: u32,
        kan_in_riichi_type: u32,
        is_demo: bool,
        is_sanma: bool,
    ) {
        self.rule_log.append(
            game_id,
            enable_kuitan,
            enable_atozuke,
            enable_pao,
            enable_tobi,
            enable_wareme,
            enable_kuinaoshi,
            enable_kiriage,
            enable_agariyame,
            enable_minus_riichi,
            enable_ryanhan_shibari,
            enable_keiten,
            enable_glass_pai,
            aka_type,
            shanyu_score,
            nannyu_score,
            uradora_type,
            furiten_riichi_type,
            oyanagare_type,
            double_ron_type,
            initial_score,
            kan_in_riichi_type,
            is_demo,
            is_sanma,
        );
    }

    pub fn append_kyoku_log(
        &mut self,
        id: u64,
        game_id: String,
        kyoku_num: i32,
        honba: i32,
        riichi_bou: i32,
        scores: &[Option<i32>],
        kazes: &[Option<i32>],
    ) {
        self.kyoku_log
            .append(id, game_id, kyoku_num, honba, riichi_bou, scores, kazes);
    }

    pub fn append_haipais_log(
        &mut self,
        kyoku_id: u64,
        player_index: i32,
        pai_ids: &[Option<u32>],
    ) {
        self.haipais_log.append(kyoku_id, player_index, pai_ids);
    }

    pub fn append_agaris_log(
        &mut self,
        kyoku_id: u64,
        machipai: u32,
        score: i32,
        fu: i32,
        han: i32,
        pai_ids: &[Option<u32>],
        yaku: &[(String, i32)],
        dora: &[Option<u32>],
        uradora: &[Option<u32>],
        dora_orig: &[Option<u32>],
        uradora_orig: &[Option<u32>],
        who: i32,
        by: i32,
        score_diff: &[Option<i32>],
        owari: bool,
        nukidora: u32,
    ) {
        self.agaris_log.append(
            kyoku_id,
            machipai,
            score,
            fu,
            han,
            pai_ids,
            yaku,
            dora,
            uradora,
            dora_orig,
            uradora_orig,
            who,
            by,
            score_diff,
            owari,
            nukidora,
        );
    }

    pub fn append_nagare_log(&mut self, kyoku_id: u64, name: String, score_diff: &[Option<i32>]) {
        self.nagare_log.append(kyoku_id, name, score_diff);
    }

    pub fn append_actions_log(
        &mut self,
        kyoku_id: u64,
        player_index: i32,
        seq: i32,
        action_type: String,
        pai_id: u32,
    ) {
        self.actions_log
            .append(kyoku_id, player_index, seq, action_type, pai_id);
    }

    // 最後に呼び出すことで、全てのログをparquetファイルに書き出す
    pub fn write_to_parquet<P: AsRef<Path>>(self, path: P) -> anyhow::Result<()> {
        let dt = Utc::now();
        let dtstr = dt.format("dt=%Y-%m-%d");

        fs::create_dir_all(path.as_ref().join(format!("games/{}", dtstr)))?;
        fs::create_dir_all(path.as_ref().join(format!("game_players/{}", dtstr)))?;
        fs::create_dir_all(path.as_ref().join(format!("rules/{}", dtstr)))?;
        fs::create_dir_all(path.as_ref().join(format!("kyokus/{}", dtstr)))?;
        fs::create_dir_all(path.as_ref().join(format!("haipais/{}", dtstr)))?;
        fs::create_dir_all(path.as_ref().join(format!("agaris/{}", dtstr)))?;
        fs::create_dir_all(path.as_ref().join(format!("actions/{}", dtstr)))?;
        fs::create_dir_all(path.as_ref().join(format!("nagares/{}", dtstr)))?;

        self.game_log.save_to_parquet(path.as_ref().join(format!(
            "games/{}/game-{}.parquet",
            dtstr,
            dt.timestamp()
        )))?;
        self.game_player_log
            .save_to_parquet(path.as_ref().join(format!(
                "game_players/{}/game_player-{}.parquet",
                dtstr,
                dt.timestamp()
            )))?;
        self.rule_log.save_to_parquet(path.as_ref().join(format!(
            "rules/{}/rule-{}.parquet",
            dtstr,
            dt.timestamp()
        )))?;
        self.kyoku_log.save_to_parquet(path.as_ref().join(format!(
            "kyokus/{}/kyoku-{}.parquet",
            dtstr,
            dt.timestamp()
        )))?;
        self.haipais_log
            .save_to_parquet(path.as_ref().join(format!(
                "haipais/{}/haipai-{}.parquet",
                dtstr,
                dt.timestamp()
            )))?;
        self.agaris_log.save_to_parquet(path.as_ref().join(format!(
            "agaris/{}/agari-{}.parquet",
            dtstr,
            dt.timestamp()
        )))?;
        self.actions_log
            .save_to_parquet(path.as_ref().join(format!(
                "actions/{}/action-{}.parquet",
                dtstr,
                dt.timestamp()
            )))?;
        self.nagare_log.save_to_parquet(path.as_ref().join(format!(
            "nagares/{}/nagare-{}.parquet",
            dtstr,
            dt.timestamp()
        )))?;
        Ok(())
    }
}
