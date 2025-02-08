use std::collections::HashMap;

// Structure pour représenter une plage horaire
#[derive(Default, Clone, Debug)]
pub struct TimeSlot {
    is_not_available: bool,
}

impl TimeSlot {
    fn update(&mut self) {
        self.is_not_available = !self.is_not_available;
    }
    pub fn get_available(&self) -> bool {
        self.is_not_available
    }

    pub fn set_available(&mut self, not_available: bool) {
        self.is_not_available = not_available;
    }
    pub fn charge(&mut self, not_available: bool)  {
        self.is_not_available = not_available;
     
    }


}

// Structure pour représenter un professeur et ses disponibilités

#[derive(Clone, Debug)]
pub struct Teacher {
    id: usize,
    name: String,
    schedule: HashMap<(usize,usize), TimeSlot>, // Clé : (jour,créneau ), Valeur : TimeSlot
}

impl Teacher {
    pub fn new(id: usize, name: String) -> Self {
        Self {
            id,
            name,
            schedule: HashMap::new(),
        }
    }
    /*pub fn update(&self, new_name: String) -> Self{
        Self {
            id: self.id,
            name:new_name,
            schedule: self.schedule.clone(),
        }
    }*/
    pub fn get_name(&self) -> String{
        self.name.clone()
    }
    pub fn set_name(&mut self, new_name:String) { 
        self.name = new_name;
    }

   /* pub fn set_availability(&mut self, day: usize, hour: usize, is_not_available: bool) {
        let slot = self.schedule.entry((day, hour)).or_default();
        slot.is_not_available = is_not_available;
    }

    pub fn is_available(&self, day: usize, hour: usize) -> bool {
        !self.schedule
            .get(&(day, hour))
            .map_or(false, |slot| slot.is_not_available)
    }*/
    pub fn set_availability(&mut self, day: usize, hour: usize){
        self.schedule.entry((day,hour)).or_insert(TimeSlot::default()).update();
        //self.schedule[day][hour].update();
    }
    pub fn get_available(&self, day: usize, hour: usize) -> bool {
        self.schedule.get(&(day,hour)).map_or(false, |slot| slot.get_available())
        //self.schedule[day][hour].get_available()
    }

    pub fn get_creneau_mut(&mut self, day: usize, hour: usize ) -> Option<&mut TimeSlot>{
        self.schedule.get_mut(&(day,hour))
    }
    pub fn get_not_available_liste(&self) -> &HashMap<(usize,usize), TimeSlot>{
        &self.schedule

    }
    pub fn charge_creneau(&mut self, day:usize, hour:usize, not_available: bool) {
        let mut time_slot = TimeSlot::default();
        time_slot.charge(not_available);
        self.schedule.entry((day,hour)).or_insert(time_slot);
    }


}

#[derive(Clone, Debug)]
pub struct PlanningTeacher {
    id: usize,
    teacher: Teacher,
    schedule: Vec<Vec<Option<usize>>>, // Clé : (jour, créneau), Valeur : id de la classe
}