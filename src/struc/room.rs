/*#[derive(Clone, Debug)]
pub struct RoomType {
    id:usize,
    name: String,
}

impl RoomType {

    pub fn new(id: usize, name: String,) -> Self {
        Self {
            id,
            name,
        }
    }

    pub fn set_name(&mut self, new_name:String) { 
        self.name = new_name;
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}



#[derive(Clone, Debug)]
pub struct Room<'b> {
    id:usize, 
    name: &'b String,
    room_type: &'b RoomType, // salle de tp, ou autre
}

impl<'b> Room<'b> {
    pub fn new( id: usize, name: &'b String, room_type: &'b RoomType,) -> Self {
        Self {
            id,
            name,
            room_type,
        }
    }
    /*pub fn update(&self, name: String, room_type: RoomType,)  -> Self {
        Self {
            id: self.id,
            name,
            room_type,
        }
    }*/

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_room_type(&self) -> &RoomType {
        &self.room_type
    }

    /*pub fn get_room_type_mut(&mut self) -> &RoomType {
        self.room_type
    }*/

}*/