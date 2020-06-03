#[test]
fn one_node() {
  use crate::{
    gedcom_data_format::from_str,
    gedcom_entities::{
      Address, BirthEvent, Business, ChangeDate, ChangeDateDateTime,
      CharacterSet::Utf8,
      DateExact, DateGreg, Gedcom,
      GedcomForm::LineageLinked,
      Header, Individual,
      Language::English,
      Month::{April, January},
      PersonalName, Record,
      SexValue::Male,
      Source, SubmissionRecord, TimeValue, TransmissionDateTime,
    },
  };

  // Arrange
  let input = include_str!("one-node.ged");

  // Act
  let records: Vec<Record> = from_str(input).expect("testy no crashy");

  // Assert
  assert_eq!(records.len(), 4);
  assert_eq!(
    records[0],
    Record::Header(Header {
      source: Source {
        id: "FINDMYPAST",
        name: Some("Findmypast Family Tree"),
        version: Some("2.0"),
        business: Some(Business {
          name: "DC Thomson Family History",
          address: Some(Address {
            address_line: "The Glebe, 6 Chapel Place, Rivington Street",
            city: Some("London"),
            post_code: Some("EC2A 3DQ"),
            country: Some("England")
          }),
          web_page: vec!["www.findmypast.com"]
        })
      },
      receiving_sytem_name: "FINDMYPAST",
      transmission_date_time: TransmissionDateTime {
        date: DateExact {
          day: 15,
          month: April,
          year: 2020
        },
        time: TimeValue {
          hours: 15,
          minutes: 21,
          seconds: Some(24),
        }
      },
      file_name: "Henderson Family Tree.ged",
      submission_record_id: "@SUBM1@",
      gedcom: Gedcom {
        version_number: "5.5.1",
        form: LineageLinked
      },
      character_set: Utf8,
      language: Some(English)
    })
  );
  assert_eq!(
    records[1],
    Record::SubmissionRecord(SubmissionRecord { name: "Not known" })
  );
  assert_eq!(
    records[2],
    Record::Individual(Individual {
      name: vec![PersonalName {
        given_name: "Gavin",
        surname: "Henderson",
        is_primary: true
      }],
      sex: Some(Male),
      birth_event: Some(BirthEvent {
        is_primary: true,
        place: Some("Dundee"),
        date: DateGreg {
          day: 1,
          month: January,
          year: 1990
        }
      }),
      uid: Some("9ACF01CA-A40C-4AF5-8905-D6678B6288BE"),
      change_date: Some(ChangeDate {
        date_time: ChangeDateDateTime {
          date: DateExact {
            day: 15,
            month: April,
            year: 2020
          },
          time: TimeValue {
            hours: 16,
            minutes: 19,
            seconds: Some(21)
          }
        }
      })
    })
  );
  assert_eq!(records[3], Record::Trailer);
}
