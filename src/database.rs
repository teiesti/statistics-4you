use {
    anyhow::{Ok, Result, bail},
    log::info,
    std::{
        collections::HashMap,
        fs::{File, OpenOptions, create_dir_all},
        io::{BufRead as _, BufReader, BufWriter, Write as _},
        path::{Path, PathBuf},
    },
};

pub(crate) struct Database {
    root: PathBuf,
    files: HashMap<Table, File>,
    last_records: HashMap<Table, Record>,
}

impl Database {
    pub(crate) fn open(path: PathBuf) -> Result<Self> {
        info!("Opening database at {}", path.display());

        if !path.is_dir() {
            bail!("Cannot open database at {}", path.display())
        }

        Ok(Self {
            root: path,
            files: HashMap::new(),
            last_records: HashMap::new(),
        })
    }

    pub(crate) fn store(&mut self, table: Table, record: Record) -> Result<()> {
        let file = match self.files.get_mut(&table) {
            Some(file) => file,

            None => {
                let path = self.root.join(table.path());

                create_dir_all(path.parent().unwrap())?;

                self.files.insert(
                    table.clone(),
                    OpenOptions::new()
                        .read(true)
                        .append(true)
                        .create(true)
                        .open(&path)?,
                );

                let mut file = self.files.get_mut(&table).unwrap();
                let lines = BufReader::new(&mut file).lines();

                let mut needs_header = true;
                for line in lines {
                    let line = line?;

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
                        assert!(columns.next().is_none());
                    }
                }

                if needs_header {
                    let mut writer = BufWriter::new(&mut file);
                    writer.write_all("timestamp,value\n".as_bytes())?;
                }

                file
            }
        };

        match self.last_records.get(&table) {
            Some(last) if record == *last => {}

            _ => {
                let mut writer = BufWriter::new(file);
                writer.write_all(format!("{},{}\n", record.timestamp, record.value).as_bytes())?;

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
        Path::new(&self.charge_point).join(format!("{}.csv", &self.property))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Record {
    pub(crate) timestamp: String,
    pub(crate) value: String,
}
