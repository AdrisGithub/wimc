use wbdl::Date;

pub fn find_date(vec: &mut Vec<String>) -> Option<Date> {
    Date::try_from(vec.pop()?).ok()
}
pub fn is_due(date: &Date) -> bool {
    Date::now().map(|val| &val > date).unwrap_or_default()
}
