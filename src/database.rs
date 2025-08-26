use {
    anyhow::{Ok, Result, bail},
    std::{
        collections::HashMap,
        path::{Path, PathBuf},
    },
    tokio::{
        fs::{File, OpenOptions},
        io::{AsyncBufReadExt as _, AsyncWriteExt, BufReader, BufWriter},
    },
};

pub(crate) struct Database {
    root: PathBuf,
    files: HashMap<Table, File>,
    last_records: HashMap<Table, Record>,
}

impl Database {
    pub(crate) fn open(path: PathBuf) -> Result<Self> {
        if !path.is_dir() {
            bail!("Cannot open database at {}", path.display())
        }

        Ok(Self {
            root: path,
            files: HashMap::new(),
            last_records: HashMap::new(),
        })
    }

    pub(crate) async fn store(&mut self, table: Table, record: Record) -> Result<()> {
        let file = match self.files.get_mut(&table) {
            Some(file) => file,
            None => {
                let path = self.root.join(table.path());

                self.files.insert(
                    table.clone(),
                    OpenOptions::new()
                        .read(true)
                        .append(true)
                        .create(true)
                        .open(&path)
                        .await?,
                );

                let mut file = self.files.get_mut(&table).unwrap();
                let mut lines = BufReader::new(&mut file).lines();

                let mut needs_header = true;
                while let Some(line) = lines.next_line().await? {
                    if needs_header {
                        needs_header = false;
                    } else {
                        let mut columns = line.split(",");
                        self.last_records.insert(
                            table.clone(),
                            Record {
                                timestamp: columns.next().unwrap().to_string(),
                                value: columns.next().unwrap().parse().unwrap(),
                            },
                        );
                    }
                }

                if needs_header {
                    let mut writer = BufWriter::new(&mut file);
                    writer.write_all("timestamp,value\n".as_bytes()).await?;
                    writer.flush().await?;
                }

                file
            }
        };

        match self.last_records.get(&table) {
            Some(last) if record == *last => {}

            _ => {
                let mut writer = BufWriter::new(file);
                writer
                    .write_all(format!("{},{}\n", record.timestamp, record.value).as_bytes())
                    .await?;
                writer.flush().await?;

                self.last_records.insert(table, record);
            }
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub(crate) struct Table {
    pub(crate) charge_point: String,
    pub(crate) property: String,
}

impl Table {
    fn path(&self) -> PathBuf {
        Path::new(&self.charge_point).join(&self.property)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Record {
    pub(crate) timestamp: String,
    pub(crate) value: String,
}
