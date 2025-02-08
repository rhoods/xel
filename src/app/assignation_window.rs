/*
 egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("tableau")
                .striped(true)
                .show(ui, |ui| {
                    for (i, matiere) in self.matieres.iter().enumerate() {
                        ui.label(matiere);

                        ui.vertical(|ui| {
                            for _ in 0..self.groupes[i].len() {
                                ui.horizontal(|ui| {
                                    ui.label("Groupe");
                                    if ui.button("➖").clicked() {
                                        // Supprimer le groupe
                                    }
                                });
                            }
                            if ui.button("➕").clicked() {
                                // Ajouter un groupe
                            }
                        });

                        ui.vertical(|ui| {
                            for _ in 0..self.groupes[i].len() {
                                egui::ComboBox::from_label("Classes")
                                    .selected_text("Sélectionner des classes")
                                    .show_ui(ui, |ui| {
                                        for classe in &self.classes {
                                            ui.selectable_value(&mut false, classe.clone(), classe);
                                        }
                                    });
                            }
                        });

                        ui.vertical(|ui| {
                            for _ in 0..self.groupes[i].len() {
                                egui::ComboBox::from_label("Professeurs")
                                    .selected_text("Sélectionner un professeur")
                                    .show_ui(ui, |ui| {
                                        for prof in &self.professeurs {
                                            ui.selectable_value(&mut false, prof.clone(), prof);
                                        }
                                    });
                            }
                        });

                        ui.end_row();
                    }
                });
        });
*/