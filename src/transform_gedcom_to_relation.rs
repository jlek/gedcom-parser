use crate::gedcom_entities::{
  Individual, Record,
  SexValue::{Female, Male, Undetermined},
};
use crate::relation_entities::{Fact, FamilyTree, Name, Person, Place};
use chrono::Local;

pub fn transform_gedcom_to_relation<'input>(
  gedcom_records: &'input Vec<Record<'input>>,
) -> FamilyTree<'input> {
  if let Record::Individual(individual) = &gedcom_records[2] {
    let person = transform_indivual_to_person(individual);
    return FamilyTree {
      persons: vec![person],
      familys: vec![],
      childs: vec![],
      source_repos: vec![],
      master_sources: vec![],
      medias: vec![],
      fact_types: vec![],
    };
  } else {
    panic!();
  }
}

fn transform_indivual_to_person<'input>(individual: &'input Individual) -> Person<'input> {
  let birth_event = individual.birth_event.as_ref().unwrap();
  let now = Local::now();

  Person {
    is_living: true,
    gender: individual
      .sex
      .map(|sex| match sex {
        Female => 2,
        Male => 1,
        Undetermined => 0,
      })
      .unwrap_or(0),
    date_created: now.to_rfc2822(),
    names: vec![Name {
      fact_type_id: 100,
      given_names: individual.name[0].given_name,
      surnames: individual.name[0].surname,
    }],
    facts: vec![Fact {
      fact_type_id: 405,
      date_detail: format!(
        "{} {} {}",
        birth_event.date.day, birth_event.date.month, birth_event.date.year
      ),
      place: Place {
        place_name: birth_event.place,
      },
      preferred: true,
    }],
  }
}
