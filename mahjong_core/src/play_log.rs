use std::{path::Path, sync::Arc};
use chrono::Utc;
use anyhow::anyhow;

//#[cfg(feature = "write-log")]
use std::fs::{self, File};
use arrow_array::builder::{Int32Builder, ListBuilder, StringBuilder, StructBuilder};
use arrow_array::types::{Int32Type, UInt32Type};
use arrow_array::FixedSizeListArray;
use arrow_schema::{DataType, Field};
use parquet::{arrow::arrow_writer::ArrowWriter, basic::{Compression, Encoding, GzipLevel}, file::properties::*};
use arrow_array::{array::{BooleanArray, UInt32Array, Int32Array, UInt64Array, StringArray, ListArray, ArrayRef}, RecordBatch};


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
    kyoku_log : KyokuLog,
    haipais_log: HaipaisLog,
    agaris_log: AgarisLog,
    nagare_log: NagareLog,
    actions_log: ActionsLog,
}


#[derive(Default)]
pub struct GamesLog {
    id_vec: Vec<String>,
    tonpu_vec: Vec<bool>,
    ariari_vec: Vec<bool>,
    has_aka_vec: Vec<bool>,
    is_demo_vec: Vec<bool>,
    is_soku_vec: Vec<bool>,
    level_vec: Vec<i32>,
    started_at_vec: Vec<u64>,
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
    pai_ids_vec: Vec<Option<Vec<Option<u32>>>>
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
    pai_id_vec: Vec<u32>
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
        kazes: &[Option<i32>]
    ) {
        self.id_vec.push(id);
        self.game_id_vec.push(game_id);
        self.kyoku_num_vec.push(kyoku_num);
        self.honba_vec.push(honba);
        self.riichi_bou_vec.push(riichi_bou);
        self.scores_vec.push(Some(scores.to_vec()));
        self.kazes_vec.push(Some(kazes.to_vec()));
    }

    pub fn save_to_parquet<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let file = File::create(path.as_ref()).map_err(|e| anyhow!("Failed to create file at {:?}: {}", path.as_ref(), e))?;

        let props = WriterProperties::builder()
            .set_writer_version(WriterVersion::PARQUET_2_0)
            .set_encoding(Encoding::PLAIN)
            .set_compression(Compression::GZIP(GzipLevel::default()))
            .build();

        let id_vec = UInt64Array::from(self.id_vec.clone());
        let game_id_vec = StringArray::from(self.game_id_vec.clone());
        let kyoku_num_vec = Int32Array::from(self.kyoku_num_vec.clone());
        let honba_vec = Int32Array::from(self.honba_vec.clone());
        let riichi_bou_vec = Int32Array::from(self.riichi_bou_vec.clone());
        let scores_vec = FixedSizeListArray::from_iter_primitive::<Int32Type, _, _>(self.scores_vec.clone(),4);
        let kazes_vec = FixedSizeListArray::from_iter_primitive::<Int32Type, _, _>(self.kazes_vec.clone(), 4);

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
        writer.write(&batch).map_err(|e| anyhow!("Failed to write batch to file: {}", e))?;

        writer.close()?;

        Ok(())
    }
}

impl AgarisLog {
    pub fn new() -> Self {
        // yakuはname(役の名前)とhan(翻数)の2つのフィールドを持つ構造体のリストが一つのレコードとして保存される
        let yaku_vec_builder = ListBuilder::new(
            StructBuilder::new(
                vec![
                    Field::new("name", DataType::Utf8, false),
                    Field::new("han", DataType::Int32, false),
                ],
                vec![
                    Box::new(StringBuilder::new()),
                    Box::new(Int32Builder::new())
                ],
            )
        );

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
            nukidora_vec: Vec::new()
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
        nukidora: u32
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
            self.yaku_vec_builder.values().field_builder::<StringBuilder>(0).unwrap().append_value(name);
            self.yaku_vec_builder.values().field_builder::<Int32Builder>(1).unwrap().append_value(*han);
            self.yaku_vec_builder.values().append(true);
        }

        self.yaku_vec_builder.append(true);
    }

    pub fn save_to_parquet<P: AsRef<Path>>(&mut self, path: P) -> anyhow::Result<()> {
        let file = File::create(path.as_ref()).map_err(|e| anyhow!("Failed to create file at {:?}: {}", path.as_ref(), e))?;

        let props = WriterProperties::builder()
            .set_writer_version(WriterVersion::PARQUET_2_0)
            .set_encoding(Encoding::PLAIN)
            .set_compression(Compression::GZIP(GzipLevel::default()))
            .build();

        let kyoku_id_vec = UInt64Array::from(self.kyoku_id_vec.clone());
        let machipai_vec = UInt32Array::from(self.machipai_vec.clone());
        let score_vec = Int32Array::from(self.score_vec.clone());
        let fu_vec = Int32Array::from(self.fu_vec.clone());
        let han_vec = Int32Array::from(self.han_vec.clone());
        let tehai_vec = StringArray::from(self.tehai_vec.clone());
        let pai_ids_vec = ListArray::from_iter_primitive::<UInt32Type, _, _>(self.pai_ids_vec.clone());
        let yaku_vec = self.yaku_vec_builder.finish();
        let dora_vec = ListArray::from_iter_primitive::<UInt32Type, _, _>(self.dora_vec.clone());
        let uradora_vec = ListArray::from_iter_primitive::<UInt32Type, _, _>(self.uradora_vec.clone());
        let dora_orig_vec = ListArray::from_iter_primitive::<UInt32Type, _, _>(self.dora_orig_vec.clone());
        let uradora_orig_vec = ListArray::from_iter_primitive::<UInt32Type, _, _>(self.uradora_orig_vec.clone());
        let who_vec = Int32Array::from(self.who_vec.clone());
        let by_vec = Int32Array::from(self.by_vec.clone());
        let score_diff_vec = FixedSizeListArray::from_iter_primitive::<Int32Type, _, _>(self.score_diff_vec.clone(), 4);
        let owari_vec = BooleanArray::from(self.owari_vec.clone());
        let nukidora_vec = UInt32Array::from(self.nukidora_vec.clone());

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
            ("nukidora", Arc::new(nukidora_vec) as ArrayRef)
        ])?;

        let mut writer = ArrowWriter::try_new(file, batch.schema(), Some(props))?;
        writer.write(&batch).map_err(|e| anyhow!("Failed to write batch to file: {}", e))?;
        writer.close()?;

        Ok(())
    }
}

// let pai_ids_vec = ListArray::from_iter_primitive::<Int32Type, _, _>(self.pai_ids_vec.clone());

impl HaipaisLog {
    pub fn append(
        &mut self,
        kyoku_id: u64,
        player_index: i32,
        pai_ids: &[Option<u32>]
    ) {
        self.kyoku_id_vec.push(kyoku_id);
        self.player_index_vec.push(player_index);
        self.haipai_vec.push(num_to_hai(pai_ids, 0));
        self.pai_ids_vec.push(Some(pai_ids.to_vec()));
    }

    pub fn save_to_parquet<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let file = File::create(path.as_ref()).map_err(|e| anyhow!("Failed to create file at {:?}: {}", path.as_ref(), e))?;

        let props = WriterProperties::builder()
            .set_writer_version(WriterVersion::PARQUET_2_0)
            .set_encoding(Encoding::PLAIN)
            .set_compression(Compression::GZIP(GzipLevel::default()))
            .build();

        let kyoku_id_vec = UInt64Array::from(self.kyoku_id_vec.clone());
        let player_index_vec = Int32Array::from(self.player_index_vec.clone());
        let haipai_vec = StringArray::from(self.haipai_vec.clone());
        let pai_ids_vec = FixedSizeListArray::from_iter_primitive::<UInt32Type, _, _>(self.pai_ids_vec.clone(), 13);

        let batch = RecordBatch::try_from_iter(vec![
            ("kyoku_id", Arc::new(kyoku_id_vec) as ArrayRef),
            ("player_index", Arc::new(player_index_vec) as ArrayRef),
            ("haipai", Arc::new(haipai_vec) as ArrayRef),
            ("pai_ids", Arc::new(pai_ids_vec) as ArrayRef),
        ])?;

        let mut writer = ArrowWriter::try_new(file, batch.schema(), Some(props))?;
        writer.write(&batch).map_err(|e| anyhow!("Failed to write batch to file: {}", e))?;

        writer.close()?;

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
        pai_id: u32
    ) {
        self.kyoku_id_vec.push(kyoku_id);
        self.player_index_vec.push(player_index);
        self.seq_vec.push(seq);
        self.type_vec.push(action_type);
        self.pais_vec.push(num_to_hai(&[Some(pai_id)], 0));
        self.pai_id_vec.push(pai_id);
    }

    // #[cfg(feature = "write-log")]
    pub fn save_to_parquet<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let file = File::create(path.as_ref()).map_err(|e| anyhow!("Failed to create file at {:?}: {}", path.as_ref(), e))?;

        let props = WriterProperties::builder()
            .set_writer_version(WriterVersion::PARQUET_2_0)
            .set_encoding(Encoding::PLAIN)
            .set_compression(Compression::GZIP(GzipLevel::default()))
            .build();

        let kyoku_id_vec = UInt64Array::from(self.kyoku_id_vec.clone());
        let player_index_vec = Int32Array::from(self.player_index_vec.clone());
        let seq_vec = Int32Array::from(self.seq_vec.clone());
        let type_vec = StringArray::from(self.type_vec.clone());
        let pais_vec = StringArray::from(self.pais_vec.clone());
        let pai_id_vec = UInt32Array::from(self.pai_id_vec.clone());

        let batch = RecordBatch::try_from_iter(vec![
            ("kyoku_id", Arc::new(kyoku_id_vec) as ArrayRef),
            ("player_index", Arc::new(player_index_vec) as ArrayRef),
            ("seq", Arc::new(seq_vec) as ArrayRef),
            ("type", Arc::new(type_vec) as ArrayRef),
            ("pais", Arc::new(pais_vec) as ArrayRef),
            ("pai_id", Arc::new(pai_id_vec.clone()) as ArrayRef),
        ])?;

        let mut writer = ArrowWriter::try_new(file, batch.schema(), Some(props))?;
        writer.write(&batch).map_err(|e| anyhow!("Failed to write batch to file: {}", e))?;

        writer.close()?;

        Ok(())
    }
}

impl NagareLog {
    pub fn append(
        &mut self,
        kyoku_id: u64,
        name: String,
        score_diff: &[Option<i32>]
    ) {
        self.kyoku_id_vec.push(kyoku_id);
        self.name_vec.push(name);
        self.score_diff_vec.push(Some(score_diff.to_vec()));
    }

    pub fn save_to_parquet<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let file = File::create(path.as_ref()).map_err(|e| anyhow!("Failed to create file at {:?}: {}", path.as_ref(), e))?;

        let props = WriterProperties::builder()
            .set_writer_version(WriterVersion::PARQUET_2_0)
            .set_encoding(Encoding::PLAIN)
            .set_compression(Compression::GZIP(GzipLevel::default()))
            .build();

        let kyoku_id_vec = UInt64Array::from(self.kyoku_id_vec.clone());
        let name_vec = StringArray::from(self.name_vec.clone());
        let score_diff_vec = FixedSizeListArray::from_iter_primitive::<Int32Type, _, _>(self.score_diff_vec.clone(), 4);

        let batch = RecordBatch::try_from_iter(vec![
            ("kyoku_id", Arc::new(kyoku_id_vec) as ArrayRef),
            ("name", Arc::new(name_vec) as ArrayRef),
            ("score_diff", Arc::new(score_diff_vec) as ArrayRef),
        ])?;

        let mut writer = ArrowWriter::try_new(file, batch.schema(), Some(props))?;
        writer.write(&batch).map_err(|e| anyhow!("Failed to write batch to file: {}", e))?;

        writer.close()?;

        Ok(())
    }
}

impl PlayLog {
    pub fn new() -> Self {
        Self {
            kyoku_log: KyokuLog::default(),
            haipais_log: HaipaisLog::default(),
            agaris_log: AgarisLog::new(),
            nagare_log: NagareLog::default(),
            actions_log: ActionsLog::default(),
        }
    }

    pub fn append_kyoku_log(
        &mut self,
        id: u64,
        game_id: String,
        kyoku_num: i32,
        honba: i32,
        riichi_bou: i32,
        scores: &[Option<i32>],
        kazes: &[Option<i32>]
    ) {
        self.kyoku_log.append(id, game_id, kyoku_num, honba, riichi_bou, scores, kazes);
    }

    pub fn append_haipais_log(
        &mut self,
        kyoku_id: u64,
        player_index: i32,
        pai_ids: &[Option<u32>]
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
        nukidora: u32
    ) {
        self.agaris_log.append(kyoku_id, machipai, score, fu, han, pai_ids, yaku, dora, uradora, dora_orig, uradora_orig, who, by, score_diff, owari, nukidora);
    }

    pub fn append_nagare_log(
        &mut self,
        kyoku_id: u64,
        name: String,
        score_diff: &[Option<i32>]
    ) {
        self.nagare_log.append(kyoku_id, name, score_diff);
    }

    pub fn append_actions_log(
        &mut self,
        kyoku_id: u64,
        player_index: i32,
        seq: i32,
        action_type: String,
        pai_id: u32
    ) {
        self.actions_log.append(kyoku_id, player_index, seq, action_type, pai_id);
    }

    pub fn write_to_parquet<P: AsRef<Path>>(&mut self, path: P) -> anyhow::Result<()> {
        let dt = Utc::now();
        let dtstr = dt.format("dt=%Y-%m-%d");

        fs::create_dir_all(path.as_ref().join(format!("kyokus/{}", dtstr)))?;
        fs::create_dir_all(path.as_ref().join(format!("haipais/{}", dtstr)))?;
        fs::create_dir_all(path.as_ref().join(format!("agaris/{}", dtstr)))?;
        fs::create_dir_all(path.as_ref().join(format!("actions/{}", dtstr)))?;
        fs::create_dir_all(path.as_ref().join(format!("nagares/{}", dtstr)))?;
        self.kyoku_log.save_to_parquet(path.as_ref().join(format!("kyokus/{}/kyoku-{}.parquet", dtstr, dt.timestamp())))?;
        self.haipais_log.save_to_parquet(path.as_ref().join(format!("haipais/{}/haipai-{}.parquet", dtstr, dt.timestamp())))?;
        self.agaris_log.save_to_parquet(path.as_ref().join(format!("agaris/{}/agari-{}.parquet", dtstr, dt.timestamp())))?;
        self.actions_log.save_to_parquet(path.as_ref().join(format!("actions/{}/action-{}.parquet", dtstr, dt.timestamp())))?;
        self.nagare_log.save_to_parquet(path.as_ref().join(format!("nagares/{}/nagare-{}.parquet", dtstr, dt.timestamp())))?;
        Ok(())
    }

}
