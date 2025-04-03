//use std::collections::HashMap;

use crate::struc::matiere::Matiere;
use crate::app::filiere_window::Filiere;

use std::sync::{Arc, Mutex};
use crate::app::room_window::RoomType; //

use crate::app::filiere_window::Classe;
use std::collections::{HashMap};



#[derive(Clone, Debug)]
pub struct MatiereInterClasse  {
    id:usize,
    matiere_prog:  Arc<MatiereProg>, //mettre le meme id pour matiere et nb_heure
    classe: Arc<Classe>
}

impl MatiereInterClasse {
    pub fn new( id: usize, matiere_prog: Arc<MatiereProg>, classe: Arc<Classe>) -> Self{
        Self {
            id,
            matiere_prog,
            classe,
        }
    }

    pub fn get_id(&self) -> &usize {
        &self.id
    }

    pub fn get_matiere_prog(&self) -> Arc<MatiereProg>{
        Arc::clone(&self.matiere_prog)
    }

    pub fn get_classe(&self) -> Arc<Classe>{
        Arc::clone(&self.classe)
    }
}



#[derive(Clone, Debug)]
pub struct OptionProgramme{
    id: usize,
    name: String,
    filiere: Arc<Filiere>,

}

impl  OptionProgramme {
    pub fn new(id:usize, name: String, filiere: Arc<Filiere>) -> Self {
        Self{
            id,
            name,
            filiere,
        }
    }

    pub fn get_id(&self) -> &usize{
        &self.id
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn get_filiere(&self) -> Arc<Filiere>{
        Arc::clone(&self.filiere)
    }
}


#[derive(Clone, Debug)]
pub struct MatiereProg  {
    id: usize,
    matiere: Arc<Matiere>, //mettre le meme id pour matiere et nb_heure
    nb_heure: usize,
    duree_minimum: usize,
    duree_maximum: usize,
    en_groupe: bool,
    nb_groupe: usize,
    en_groupe_inter_classe: bool,
    semaine: Arc<Semaine>,
    option: Arc<OptionProgramme>,
    //liste_classe: Arc<HashMap<usize, Arc<Classe>>>
}
impl  MatiereProg  {
    pub fn  new( id:usize, matiere: Arc<Matiere>, nb_heure:usize, duree_minimum:usize, duree_maximum:usize, en_groupe: bool, nb_groupe: usize, en_groupe_inter_classe: bool,semaine: Arc<Semaine>, option: Arc<OptionProgramme>) -> Self {
        Self {
            id,
            matiere,
            nb_heure,
            duree_minimum,
            duree_maximum,
            en_groupe,
            nb_groupe,
            en_groupe_inter_classe,
            semaine,
            option
            //liste_classe: Arc::new(HashMap::new()),
        }
    }
    pub fn get_id(&self) -> &usize {
        &self.id
    }

    pub fn get_matiere(&self) -> Arc<Matiere> {
        Arc::clone(&self.matiere)
    }
    /*pub fn get_liste_classe(&self) -> Arc<HashMap<usize, Arc<Classe>>> {
        Arc::clone(&self.liste_classe)
    }*/
    pub fn get_nb_heure(&self) -> &usize {
        &self.nb_heure
    }

    pub fn get_duree_minimum(&self) -> &usize {
        &self.duree_minimum
    }

    pub fn get_duree_maximum(&self) -> &usize {
        &self.duree_maximum
    }

    pub fn get_en_groupe(&self) -> &bool {
        &self.en_groupe
    }

    pub fn set_en_groupe(&mut self) {
        self.en_groupe = !self.en_groupe; 
    }
    pub fn get_en_groupe_inter_classe(&self) -> &bool {
        &self.en_groupe_inter_classe
    }

    pub fn get_semaine(&self) -> Arc<Semaine>{
        Arc::clone(&self.semaine)
    }
    pub fn get_nb_groupe(&self) -> &usize {
        &self.nb_groupe
    }

    pub fn get_option(&self) -> Arc<OptionProgramme> {
        Arc::clone(&self.option)
    }


}


#[derive(Clone, Debug)]
pub struct Semaine  {
    id: usize,
    filiere: Arc<Filiere>,
    //matiere: HashMap<usize,MatiereProg>,
    //nb_heure: Option<usize>,
}

impl  Semaine  {
    pub fn  new( id:usize, filiere: Arc<Filiere>,) -> Self {
        Self {
            id,
            filiere,
           // matiere: HashMap::new(),
            //nb_heure : None,
        }
    }
    pub fn get_id(&self) -> &usize {
        &self.id
    }
    pub fn get_filiere(&self) -> Arc<Filiere> {
        Arc::clone(&self.filiere)
    }
    /*pub fn get_programme(&self) -> Arc<Programme> {
        Arc::clone(&self.programme)
    }*/
    /*pub fn add_matiere(&mut self, matiere: MatiereProg) {
        self.matiere.insert(self.matiere.len(),matiere);
    }*/

    /*pub fn get_matiere_prog(&self, index: usize) -> Option<&MatiereProg>{
        self.matiere.get(&index)
        /*if let Some(matiere) = self.matiere.get(&index){
            matiere
        } else {
            None
        }*/
        
    }*/

    /*pub fn get_liste_matiere_prog(&self) -> &HashMap<usize,MatiereProg>{
        &self.matiere
    }*/


}

/*#[derive(Clone, Debug)]
pub struct Programme {
    id:usize,
    nb_semaine: usize, //HashMap<usize, Semaine>,
    filiere: Arc<Filiere>
    //matiere_prog: Vec<Box<MatiereProg>>, //mettre le meme id pour matiere et nb_heure
    //nb_heure: HashMap<usize, u8>,
}

impl  Programme {
    pub fn  new( id:usize, nb_semaine: usize, filiere: Arc<Filiere>) -> Self {
        Self {
            id,
            nb_semaine,
            filiere,
        }
    }
    pub fn get_id(&self) -> &usize{
        &self.id
    }

    pub fn get_nb_semaine(&self) -> &usize {
        &self.nb_semaine
    }

    /*pub fn get_liste_semaine(&self) -> &HashMap<usize, Semaine> {
        &self.semaines
    }*/

    pub fn get_filiere(&self) ->  Arc<Filiere> {
        Arc::clone(&self.filiere)
    }
    
    

    //pub fn get_filiere(&self) -> 

    
}*/