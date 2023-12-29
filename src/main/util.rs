use wbdl::Date;

pub fn find_date(vec: &mut Vec<String>) -> Option<Date>{
    Date::try_from(vec.pop()?).ok()
}