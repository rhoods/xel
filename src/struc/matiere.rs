//use std::collections::HashMap;


use crate::app::room_window::RoomType;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct Matiere {
    id: usize,
    name: String,
    type_salle: Arc<RoomType>,
    
}

impl Matiere {
    pub fn new(id: usize, name: String, type_salle: Arc<RoomType>) -> Self {
        Self {
            id,
            name,
            type_salle,
        }
    }
    pub fn get_id(&self) -> &usize {
        &self.id
    }
    
    pub fn get_name(&self) -> String{
        self.name.clone()
    }
    pub fn set_name(&mut self, new_name:String) { 
        self.name = new_name;
    }

    pub fn get_room_type(&self) -> Arc<RoomType> {
        Arc::clone(&self.type_salle)
    }

    pub fn set_type_salle(&self) -> Arc<RoomType> {
        Arc::clone(&self.type_salle)
    }

    /*pub fn set_room_type_exist(&self) -> bool{
        if self.type_salle.is_none(){
            false
        }
        else{
            true
        }
    }*/
}
