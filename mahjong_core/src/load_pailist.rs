use std::{path::Path, fs::File};

use anyhow::ensure;
use arrow_array::{array::Int32Array, Array, FixedSizeListArray};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

pub fn load_pailist<P: AsRef<Path>>(path: P, row_index: usize) -> anyhow::Result<Vec<i32>>{
    let file = File::open(path)?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
    println!("Converted arrow schema is : {}\r", builder.schema());
    let mut reader = builder.build()?;
    let read_result = reader.next();

    if let Some(arrow_result) = read_result {
        let record_batch = arrow_result?;
        if let Some(column) = record_batch.column_by_name("hai_ids") {
            let row_list = column.as_any().downcast_ref::<FixedSizeListArray>();

            if let Some(rows) = row_list {
                ensure!(row_index < rows.len(), "row_index must be less than row length");
                let cell = rows.value(row_index);
                let ret = cell.as_any().downcast_ref::<Int32Array>();

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
