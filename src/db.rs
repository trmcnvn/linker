use firestore_db_and_auth::{documents, dto, Credentials, ServiceSession};
use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Firestore {
    credentials: Credentials,
    session: OnceCell<Arc<Mutex<ServiceSession>>>,
}

impl Firestore {
    pub fn new(credentials: Credentials) -> Self {
        Self {
            credentials,
            session: OnceCell::new(),
        }
    }

    pub fn find<T>(&self, collection: &str, field: &str, query: String) -> anyhow::Result<T>
    where
        for<'b> T: serde::Deserialize<'b>,
    {
        let session = self.get_session()?;
        let session = session.lock().unwrap();
        let mut values: documents::Query = documents::query(
            &*session,
            collection,
            query.into(),
            dto::FieldOperator::EQUAL,
            field,
        )?;
        if let Some(document) = values.next() {
            documents::read_by_name(&*session, document.name).map_err(|err| anyhow::anyhow!(err))
        } else {
            Err(anyhow::anyhow!("Not found"))
        }
    }

    pub fn insert<T>(&self, collection: &str, document: &T) -> anyhow::Result<String>
    where
        T: serde::Serialize,
    {
        let session = self.get_session()?;
        let session = session.lock().unwrap();
        let result = documents::write(
            &*session,
            collection,
            None as Option<String>,
            document,
            documents::WriteOptions::default(),
        )?;
        Ok(result.document_id)
    }

    pub fn find_or_insert<T>(
        &self,
        collection: &str,
        field: &str,
        query: String,
        document: &T,
    ) -> anyhow::Result<T>
    where
        T: serde::Serialize + for<'b> serde::Deserialize<'b> + Clone,
    {
        match self.find(collection, field, query) {
            Ok(link) => Ok(link),
            Err(_) => match self.insert(collection, document) {
                Ok(_) => Ok(document.clone()),
                Err(_) => Err(anyhow::anyhow!("Failed to insert")),
            },
        }
    }

    fn get_session(&self) -> anyhow::Result<Arc<Mutex<ServiceSession>>> {
        let session = self.session.get_or_init(|| {
            let session = ServiceSession::new(self.credentials.clone()).unwrap();
            Arc::new(Mutex::new(session))
        });
        Ok(session.clone())
    }
}
