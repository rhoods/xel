//use std::collections::HashMap;

use crate::struc::programme::{MatiereProg,OptionProgramme};
use crate::struc::matiere::{Matiere};
use crate::struc::teacher::{Teacher};

use crate::app::filiere_window::Filiere;

use std::sync::{Arc, Mutex};
use crate::app::room_window::RoomType; //

use crate::app::filiere_window::Classe;
use std::collections::{HashMap};

//use super::programme::OptionProgramme;


#[derive(Clone, Debug)]
pub struct Groupe  {
    id:usize,
    name: usize,
    classe: Arc<Classe>,
    matiere: Arc<Matiere>,

}

impl Groupe {
    pub fn new( id: usize,name:usize, classe: Arc<Classe>, matiere: Arc<Matiere>,) -> Self{
        Self {
            id,
            name,
            classe,
            matiere,
            
        }
    }
    pub fn get_id(&self) -> &usize  {
        &self.id
    }
    pub fn get_name(&self) -> &usize  {
        &self.name
    }
    pub fn get_matiere(&self) -> &Arc<Matiere>  {
        &self.matiere
    }
    pub fn get_classe(&self) -> &Arc<Classe>  {
        &self.classe
    }

}

#[derive(Clone, Debug)]
pub struct Assignation  {
    id:usize,
    classe: Arc<Classe>,
    matiere: Arc<Matiere>,
    groupe : Arc<Groupe>,
    prof: Teacher,
    option: Arc<OptionProgramme>,
    matiere_prog: Arc<MatiereProg>,

}

impl Assignation {
    pub fn new( id: usize, classe: Arc<Classe>, matiere: Arc<Matiere>, groupe: Arc<Groupe>, prof: Teacher,option: Arc<OptionProgramme>,matiere_prog: Arc<MatiereProg>,) -> Self{
        Self {
            id,
            matiere,
            classe,
            groupe,
            prof,
            option,
            matiere_prog,
        }
    }

    pub fn get_matiere(&self) -> &Arc<Matiere>  {
        &self.matiere
    }
    pub fn get_groupe(&self) -> &Arc<Groupe>  {
        &self.groupe
    }
    pub fn get_classe(&self) -> &Arc<Classe>  {
        &self.classe
    }
    pub fn get_prof(&self) -> &Teacher  {
        &self.prof
    }
    pub fn get_option(&self) -> &Arc<OptionProgramme>  {
        &self.option
    }
    pub fn get_matiere_prog(&self) -> &Arc<MatiereProg>  {
        &self.matiere_prog
    }


}