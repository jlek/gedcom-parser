#[test]
fn header_only() {
  use crate::{
    gedcom_data_format::from_str,
    gedcom_entities::{
      Address, Business, CharacterSet::Utf8, DateExact, Gedcom, GedcomForm::LineageLinked, Header,
      Language::English, Month::April, Source, TimeValue, TransmissionDateTime,
    },
  };

  // Arrange
  let input = include_str!("header_only.ged");

  // Act
  let header: Header = from_str(input).expect("testy no crashy");

  // Assert
  assert_eq!(
    header,
    Header {
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
    }
  )
}
