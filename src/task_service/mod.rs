use bson::ordered::OrderedDocument;
use bson::{doc, Bson, Document};
use mongodb::results::{DeleteResult, UpdateResult};
use mongodb::{error::Error, results::InsertOneResult, Collection};
use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id:i64,
    pub name: String,
    pub completed: bool,
    pub createdat: i64,
}



#[derive(Clone)]
pub struct TaskService {
    collection: Collection,
}
///
/// Build user from inputs
/// # Example :
///
/// ```
/// let user = build_user(
///     "hela",
///     "ben khalfallah",
///     "hela@hotmail.fr",
///     "helabenkhalfallah",
///     "azerty"
/// )
/// println!("user  = {:?}", user);
/// ```

fn build_task(
    id:i64,
    name: String,
    completed: bool,
    createdat:i64,
) -> Task {
    Task {
        id,
        name,
        completed,
        createdat,
    }
}

///
/// Transform mongo db document to User



fn task_from_document(document: Document) -> Task {
    let mut _id=4563; 
    let mut _name = "".to_string();
    let mut _completed=false;
    let mut _createdat=123544; 

    if let Some(&Bson::I64(ref id)) = document.get("id") {
        _id = *id;
    }
    if let Some(&Bson::String(ref name)) = document.get("name") {
        _name = name.to_string();
    }
    if let Some(&Bson::Boolean(ref completed)) = document.get("completed") {
        _completed = *completed;
    }
    if let Some(&Bson::I64(ref createdat)) = document.get("createdat") {
        _createdat = *createdat;
    }
   

    build_task(_id,_name, _completed,_createdat)
}

/// Transform task to mongo db document
/// 



fn task_to_document(task: &Task) -> Document {
    let Task {
        id,
        name,
        completed,
        createdat,
    } = task;
    doc! {
        "id":id,
        "name": name,
        "completed": completed,
        "createdat":createdat,
    }
}







impl TaskService {
    pub fn new(collection: Collection) -> TaskService {
        TaskService { collection }
    }

    /// Insert task in mongo db (task)
    pub fn create(&self, task: &Task) -> Result<InsertOneResult, Error> {
        self.collection.insert_one(task_to_document(task), None)
    }

    /// Update existing task in mongo db (email)
    pub fn update(&self, task: &Task) -> Result<UpdateResult, Error> {
        let Task {
            id,
            name:_name,
            completed: _completed,
            createdat:_createdat,
            
        } = task;
        self.collection
            .update_one(doc! { "id": id}, task_to_document(task), None)
    }

    /// Delete existing task in mongo db (email)
    pub fn delete(&self, id: &i64) -> Result<DeleteResult, Error> {
        self.collection
            .delete_one(doc! { "id": id}, None)
    }

    /// get all users
    pub fn get(&self) -> Result<Vec<Task>, Error> {
        let cursor = self.collection.find(None, None).unwrap();
        let mut data: Vec<Task> = Vec::new();

        for result in cursor {
            if let Ok(item) = result {
                data.push(task_from_document(item))
            }
        }

        Ok(data)
    }

    /// Retrieve task by (name)
    pub fn get_task_id(&self, id: &i64) -> Result<Option<OrderedDocument>, Error> {
        self.collection.find_one(doc! { "id": id}, None)
    }
}

