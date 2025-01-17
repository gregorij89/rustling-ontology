use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::helpers;
use rustling_ontology_moment::{Weekday, Grain};


pub fn rules_datetime(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    // Basic
    b.rule_2("intersect",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |a, b| a.value().intersect(b.value())
    );
    b.rule_3("intersect by 'and' or ','",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             b.reg(r#"e|,"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |a, _, b| a.value().intersect(b.value())
    );
//    b.rule_3("intersect by 'and' or ','",
//             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
//             b.reg(r#"e|,"#)?,
//             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
//             |a, _, b| a.value().intersect(b.value())
//    );
    b.rule_3("intersect by 'of'",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             b.reg(r#"del(?:l['oa])?|di"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |a, _, b| a.value().intersect(b.value())
    );
    b.rule_3("intersect by 'but/for example/rather'",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             b.reg(r#"ma"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |a, _, b| a.value().intersect(b.value())
    );
    b.rule_2("in <named-month>",
             b.reg(r#"(?:ad?|in|nel)"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, a| Ok(a.value().clone())
    );
//    b.rule_2("for <datetime>",
//             b.reg(r#"per"#)?,
//             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
//             |_, a| Ok(a.value().clone())
//    );
// Removed constraints (latent() And Not(TimeOfDay)) on datetime  (in cov test : 'per mezzanotte e venti' for ex) but need to check if this is a mistake to remove constraints
    b.rule_2("for <datetime>",
             b.reg(r#"per|durante|nel|in"#)?,
             datetime_check!(),
             |_, a| Ok(a.value().clone())
    );
    b.rule_2("for <celebration>",
             b.reg(r#"a"#)?,
             datetime_check!(|datetime: &DatetimeValue| form!(Form::Celebration)(datetime)),
             |_, a| Ok(a.value().clone())
    );
    // Days, months
    b.rule_1_terminal("named-day",
                      b.reg(r#"lun(?:ed[íìi]|\.)?"#)?,
                      |_| helpers::day_of_week(Weekday::Mon)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"mar(?:ted[íìi]|\.)?"#)?,
                      |_| helpers::day_of_week(Weekday::Tue)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"mer(?:coled[íìi]|\.)?"#)?,
                      |_| helpers::day_of_week(Weekday::Wed)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"gio(?:ved[íìi]|v?\.)?"#)?,
                      |_| helpers::day_of_week(Weekday::Thu)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"ven(?:erd[íìi]|\.)?"#)?,
                      |_| helpers::day_of_week(Weekday::Fri)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"sab(?:at[oi]|\.)?"#)?,
                      |_| helpers::day_of_week(Weekday::Sat)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"dom(?:enic(?:a|he)|\.)?"#)?,
                      |_| helpers::day_of_week(Weekday::Sun)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"gennaio|genn?\.?"#)?,
                      |_| helpers::month(1)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"febbraio|feb\.?"#)?,
                      |_| helpers::month(2)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"marzo|mar\.?"#)?,
                      |_| helpers::month(3)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"aprile|apr\.?"#)?,
                      |_| helpers::month(4)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"maggio|mag\.?"#)?,
                      |_| helpers::month(5)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"giugno|giu\.?"#)?,
                      |_| helpers::month(6)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"luglio|lug\.?"#)?,
                      |_| helpers::month(7)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"agosto|ago\.?"#)?,
                      |_| helpers::month(8)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"settembre|sett?\.?"#)?,
                      |_| helpers::month(9)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"ottobre|ott\.?"#)?,
                      |_| helpers::month(10)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"novembre|nov\.?"#)?,
                      |_| helpers::month(11)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"dicembre|dic\.?"#)?,
                      |_| helpers::month(12)
    );
    // Celebration days
    // TODO: Holidays/Celebrations
    // END TODO
    // Deictic lexemes
    b.rule_1_terminal("now",
                      b.reg(r#"ora|adesso|subito|in questo (?:momento esatto|preciso istante)|immediatamente|proprio(?: ora| adesso)|attualmente"#)?,
                      |_| helpers::cycle_nth(Grain::Second, 0)
    );
    b.rule_1_terminal("today",
                      b.reg(r#"oggi|in questo(?: preciso)? momento|in questa giornata"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 0)
    );
    b.rule_1_terminal("tomorrow",
                      b.reg(r#"(?:l' ?in)?domani|il giorno (?:seguente|dopo|successivo)"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 1)
    );
    b.rule_1_terminal("yesterday",
                      b.reg(r#"ieri|la vigilia|il giorno pr(?:ima|ecedente)"#)?,
                      |_| helpers::cycle_nth(Grain::Day, -1)
    );
    b.rule_1_terminal("the day before yesterday",
                      b.reg(r#"(?:l' ?)?altro ?ieri|ieri l'altro|avant'? ?ieri"#)?,
                      |_| helpers::cycle_nth(Grain::Day, -2)
    );
    b.rule_1_terminal("the day after tomorrow",
                      b.reg(r#"dopo ?domani"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 2)
    );

    // Deictic expressions with units
    b.rule_2("this <day-of-week>",
             b.reg(r#"quest[oa']"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             |_, datetime| datetime.value().the_nth_not_immediate(0)
    );
    b.rule_2("this <datetime>",
             b.reg(r#"quest[oa']"#)?,
             datetime_check!(),
             |_, datetime| datetime.value().the_nth(0)
    );
    b.rule_2("next <day-of-week>",
             b.reg(r#"(?:il |la )?prossim[oa]"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             |_, datetime| datetime.value().the_nth_not_immediate(0)
    );
    b.rule_2("<day-of-week> next",
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"prossim[oa]|seguent[ei]|che viene|dopo|successiv[oa]"#)?,
             |datetime, _| datetime.value().the_nth_not_immediate(0)
    );
    b.rule_2("next <named-month>",
             b.reg(r#"(?:il |la |nel corso del )?prossim[oa]"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, datetime| datetime.value().the_nth_not_immediate(0)
    );
    b.rule_2("<named-month> next",
             datetime_check!(form!(Form::Month(_))),
             b.reg(r#"prossim[oa]|seguent[ei]|che viene|dopo|successiv[oa]"#)?,
             |datetime, _| datetime.value().the_nth_not_immediate(0)
    );
    b.rule_3("the <named-month> next",
             b.reg(r#"il|l['ao]"#)?,
             datetime_check!(form!(Form::Month(_))),
             b.reg(r#"prossim[oa]"#)?,
             |_, datetime, _| datetime.value().the_nth_not_immediate(0)
    );
    b.rule_2("<named-month> following",
             datetime_check!(form!(Form::Month(_))),
             b.reg(r#"prossim[oa]|seguent[ei]|che viene|dopo|successiv[oa]"#)?,
             |datetime, _| datetime.value().the_nth_not_immediate(0)
    );
    b.rule_2("<datetime> following",
             datetime_check!(),
             b.reg(r#"prossim[oa]|seguent[ei]|che viene|dopo|successiv[oa]"#)?,
             |datetime, _| datetime.value().the_nth_not_immediate(0)
    );
    b.rule_2("next <datetime> ",
             b.reg(r#"prossim[oa]"#)?,
             datetime_check!(),
             |_, datetime| datetime.value().the_nth_not_immediate(0)
    );
    b.rule_3("the <named-month> following",
             b.reg(r#"il|l['ao]"#)?,
             datetime_check!(form!(Form::Month(_))),
             b.reg(r#"prossim[oa]|seguent[ei]|che viene|dopo|successiv[oa]"#)?,
             |_, datetime, _| datetime.value().the_nth_not_immediate(0)
    );
    b.rule_2("<named-month|named-day> last/past",
             datetime_check!(),
             b.reg(r#"scors[oa]|passat[oa]"#)?,
             |datetime, _| datetime.value().the_nth(-1)
    );
    b.rule_2("last/past <named-month|named-day>",
             b.reg(r#"(?:nell[oa] )?scors[oa]|passat[oa]"#)?,
             datetime_check!(),
             |_, datetime| datetime.value().the_nth(-1)
    );
    b.rule_4("last <day-of-week> of <datetime> (latent)",
             b.reg(r#"ultim[oa]"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"d(?:['i]|el(?:l['ao])?)"#)?,
             datetime_check!(),
             |_, dow, _, datetime| dow.value().last_of(datetime.value())
    );
    b.rule_2("the day after the <datetime>",
             b.reg(r#"(?:l'indomani|il giorno dopo) (?:di|del(?:l[ao'])?)"#)?,
             datetime_check!(),
             |_, datetime| helpers::cycle_nth_after_not_immediate(Grain::Day, 1, datetime.value())
    );
    b.rule_2("the day before the <datetime>",
             b.reg(r#"(la vigilia|il giorno prima) (di|del(l(a|o|'))?)"#)?,
             datetime_check!(),
             |_, datetime| helpers::cycle_nth_after_not_immediate(Grain::Day, -1, datetime.value())
    );
    b.rule_4("last <day-of-week> of <datetime> (latent)",
             b.reg(r#"ultim[oa]"#)?,
             cycle_check!(),
             b.reg(r#"d(?:['i]|el(?:l['ao])?)"#)?,
             datetime_check!(),
             |_, cycle, _, datetime| cycle.value().last_of(datetime.value())
    );

    b.rule_5("the <ordinal> <datetime> of <datetime>",
             b.reg(r#"il|l['ao]"#)?,
             ordinal_check!(), // the first
             datetime_check!(excluding_form!(Form::Month(_))), // Thursday // exclude Month to avoid confusion with 'il primo febbraio del prossimo anno'
             b.reg(r#"d(?:['i]|el(?:l['ao])?)"#)?, // of
             datetime_check!(), // march
             |_, ordinal, a, _, b| {
                 b.value().intersect(a.value())?.the_nth(ordinal.value().value - 1)
             }
    );
    b.rule_4("<ordinal> <datetime> of <datetime>",
             ordinal_check!(), // the first
             datetime_check!(excluding_form!(Form::Month(_))), // Thursday // exclude Month to avoid confusion with 'il primo febbraio del prossimo anno'
             b.reg(r#"d(?:['i]|el(?:l['ao])?)"#)?, // of
             datetime_check!(), // march
             |ordinal, a, _, b| {
                 b.value().intersect(a.value())?.the_nth(ordinal.value().value - 1)
             }
    );
    b.rule_4("<ordinal> week-end of <datetime>",
             b.reg(r#"il|l['ao]"#)?,
             ordinal_check!(),
             b.reg(r#"(?:week(?:\s|-)?end|fine[- ]?settimana) (?:d['i]|del mese d['i])"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, ordinal, _, datetime| {
                 let week_day_start = helpers::day_of_week(Weekday::Fri)?.intersect(&helpers::hour(18, false)?)?;
                 let week_day_end = helpers::day_of_week(Weekday::Mon)?.intersect(&helpers::hour(0, false)?)?;
                 let week_day = week_day_start.span_to(&week_day_end, false)?;
                 let week_ends_of_time = datetime.value().intersect(&week_day)?;
                 week_ends_of_time.the_nth(ordinal.value().value - 1)
             }
    );
    b.rule_3("<ordinal> week-end of <datetime>",
             ordinal_check!(),
             b.reg(r#"(?:week(?:\s|-)?end|fine[- ]?settimana) (?:d['i]|del mese d['i])"#)?,
             datetime_check!(form!(Form::Month(_))),
             |ordinal, _, datetime| {
                 let week_day_start = helpers::day_of_week(Weekday::Fri)?.intersect(&helpers::hour(18, false)?)?;
                 let week_day_end = helpers::day_of_week(Weekday::Mon)?.intersect(&helpers::hour(0, false)?)?;
                 let week_day = week_day_start.span_to(&week_day_end, false)?;
                 let week_ends_of_time = datetime.value().intersect(&week_day)?;
                 week_ends_of_time.the_nth(ordinal.value().value - 1)
             }
    );
    b.rule_2("last week-end of <datetime>",
             b.reg(r#"(?:l')?ultimo (?:week[ -]?end|fine[- ]?settimana) (?:d['i]|del mese d['i])"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, datetime| {
                 let week_day_start = helpers::day_of_week(Weekday::Fri)?.intersect(&helpers::hour(18, false)?)?;
                 let week_day_end = helpers::day_of_week(Weekday::Mon)?.intersect(&helpers::hour(0, false)?)?;
                 let week_day = week_day_start.span_to(&week_day_end, false)?;
                 week_day.last_of(datetime.value())
             }
    );
    // Years
    b.rule_1("year",
             integer_check_by_range!(1000, 2100),
             |integer| {
                 helpers::year(integer.value().value as i32)
             }
    );
    b.rule_1("year (latent)",
             integer_check_by_range!(-1000, 999),
             |integer| {
                 Ok(helpers::year(integer.value().value as i32)?.latent())
             }
    );
    b.rule_2("in <year>",
             b.reg(r#"[dn]el(?: corso del(?:l'anno)?)?"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && form!(Form::Year(_))(datetime)),
             |_, year| Ok(year.value().clone())
    );
    b.rule_2("in <year>",
             b.reg(r#"nel corso del|(?:del)?l'anno"#)?,
             datetime_check!(|datetime: &DatetimeValue| form!(Form::Year(_))(datetime)),
             |_, year| Ok(year.value().clone())
    );
    b.rule_1("year (latent)",
             integer_check_by_range!(2101, 3000),
             |integer| {
                 Ok(helpers::year(integer.value().value as i32)?.latent())
             }
    );
    // Days of the month
    b.rule_1_terminal("first of the month",
                      b.reg(r#"(?:il )?(?:1|prim)[o°](?: di)?"#)?,
                      |_| helpers::day_of_month(1)
    );
    b.rule_2("the <day-of-month> (non ordinal)",
             b.reg(r#"il|l['ao]"#)?,
             integer_check_by_range!(1, 31),
             |_, integer| helpers::day_of_month(integer.value().value as u32)
    );
    b.rule_4("the <day-of-month> at <datetime>",
             b.reg(r#"il|l['oa]"#)?,
             integer_check_by_range!(1, 31),
             b.reg(r#"a(?:ll['e])?"#)?,
             datetime_check!(),
             |_, integer, _, datetime| {
                 let day_of_month = helpers::day_of_month(integer.value().value as u32)?;
                 day_of_month.intersect(&datetime.value())
             }
    );
    b.rule_2("<day-of-month> <named-month>",
             integer_check_by_range!(1, 31),
             datetime_check!(form!(Form::Month(_))),
             |integer, month| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_3("the <day-of-month> <named-month>",
             b.reg(r#"il|l[oa']"#)?,
             integer_check_by_range!(1, 31),
             datetime_check!(form!(Form::Month(_))),
             |_, integer, month| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_3("<day-of-month> of <named-month>",
             integer_check_by_range!(1, 31),
             b.reg(r#"d(?:['i]|el(?:l['ao])?)"#)?,
             datetime_check!(form!(Form::Month(_))),
             |integer, _, month| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_4("the <day-of-month> of <named-month>",
             b.reg(r#"il|l['oa]"#)?,
             integer_check_by_range!(1, 31),
             b.reg(r#"d(?:['i]|el(?:l['ao])?)"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, integer, _, month| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_2("<day-of-week> <datetime>",
             datetime_check!(form!(Form::DayOfWeek{..})),
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, datetime| Ok(datetime.value().clone())
    );
    b.rule_3("the <day-of-week> <datetime>",
             b.reg(r#"il|l['oa]"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             datetime_check!(),
             |_, _, datetime| Ok(datetime.value().clone())
    );
    b.rule_2("<day-of-week> <day-of-month>)",
             datetime_check!(form!(Form::DayOfWeek{..})),
             integer_check_by_range!(1, 31),
             |_, integer| helpers::day_of_month(integer.value().value as u32)
    );
 // ex: wrong parsing: 'mercoledi venti sei del mese prossimo' --> day-of-week('mercoledi') + integer('vinte') + time-of-day('sei')
 //   b.rule_3("<day-of-week> <day-of-month> at <time-of-day>)",
 //            datetime_check!(form!(Form::DayOfWeek{..})),
 //            integer_check_by_range!(1, 31),
 //            datetime_check!(|datetime: &DatetimeValue| !!!datetime.latent && form!(Form::TimeOfDay(_)(datetime))),
             //datetime_check!(form!(Form::TimeOfDay(_))),
 //            |_, integer, tod| helpers::day_of_month(integer.value().value as u32)
 //                ?.intersect(tod.value())
 //   );
    // FIXME: Why can't this sort of thing be caught with intersect?
//    b.rule_4("the <day-of-month> <named-month> at <time-of-day>)",
//             b.reg(r#"il|l['oa]"#)?,
//             integer_check_by_range!(1, 31),
//             datetime_check!(form!(Form::Month(_))),
//             datetime_check!(form!(Form::TimeOfDay(_))),
//             |_, integer, month, tod| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
//                 ?.intersect(tod.value())
//    );
//    b.rule_4("the <day-of-week> <day-of-month> at <time-of-day>)",
//             b.reg(r#"il|l['oa]"#)?,
//             datetime_check!(form!(Form::DayOfWeek{..})),
//             integer_check_by_range!(1, 31),
//             datetime_check!(form!(Form::TimeOfDay(_))),
//             |_, _, integer, tod| helpers::day_of_month(integer.value().value as u32)
//                 ?.intersect(tod.value())
//    );
    b.rule_2("next <date>",
             b.reg(r#"(?:il )?prossim[oa]"#)?,
             datetime_check!(|datetime: &DatetimeValue| datetime.form.is_day()),
             |_, datetime| datetime.value().the_nth_not_immediate(0)
    );
    b.rule_3("the <date> next",
             b.reg(r#"(?:il )?"#)?,
             datetime_check!(|datetime: &DatetimeValue| datetime.form.is_day()),
             b.reg(r#"prossim[oa]"#)?,
             |_, datetime, _| datetime.value().the_nth_not_immediate(0)
    );
    // Time of day
    b.rule_1("time-of-day (latent)",
             integer_check_by_range!(1, 23),
             |integer| Ok(helpers::hour(integer.value().value as u32, integer.value().value < 12)?.latent())
    );
    b.rule_1("time-of-day (latent)",
             integer_check_by_range!(0, 0),
             |_| Ok(helpers::hour(0, false)?.latent())
    );
    b.rule_1_terminal("noon",
                      b.reg(r#"mezzo(giorno|d[íìi])"#)?,
                      |_| helpers::hour(12, false)
    );
    b.rule_1_terminal("midnight",
                      b.reg(r#"mezzanotte"#)?,
                      |_| helpers::hour(0, false)
    );
    b.rule_2("<time-of-day> hours",
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))),
             b.reg(r#"or[ae]"#)?,
             |a, _| Ok(a.value().clone().not_latent())
    );
    b.rule_2("hours <time-of-day>",
             b.reg(r#"or[ae]"#)?,
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))),
             |_, a| Ok(a.value().clone().not_latent())
    );
    b.rule_2("at <time-of-day>",
             b.reg(r#"a(?:l(?:l['e]))?"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, a| Ok(a.value().clone().not_latent())
    );
    b.rule_2("at <time-of-day> oclock",
             b.reg(r#"a(?:l(?:l['e]))?"#)?,
             integer_check_by_range!(1, 23),
             |_, integer| Ok(helpers::hour(integer.value().value as u32, integer.value().value < 12)?.not_latent())
    );
    b.rule_2("<time-of-day> oclock",
             b.reg(r#"l['e]"#)?,
             integer_check_by_range!(1, 23),
             |_, integer| Ok(helpers::hour(integer.value().value as u32, integer.value().value < 12)?.not_latent())
    );
    b.rule_2("around <time-of-day>",
             b.reg(r#"intorno a(?:ll[ae])?|verso|(?:approssim|indic|orient)ativamente|(?:all'in)?circa|più o meno|pressappoco|suppergiù|grosso modo"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, a| Ok(a.value().clone().not_latent().precision(Precision::Approximate))
    );
    b.rule_2("<time-of-day> around",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:all'in)?circa"#)?,
             |a, _| Ok(a.value().clone().not_latent().precision(Precision::Approximate))
    );
    b.rule_2("<time-of-day> exactly",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"in punto|precis[ea]|esatte"#)?,
             |a, _| Ok(a.value().clone().not_latent().precision(Precision::Exact))
    );
    b.rule_2("exactly <time-of-day>",
             b.reg(r#"(?:precis|esatt)amente"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, a| Ok(a.value().clone().not_latent().precision(Precision::Exact))
    );
    b.rule_3("relative minutes before <hour>",
             relative_minute_check!(),
             b.reg(r#"prima d(i|ell[ae])"#)?,
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour {.. }))),
             |relative_minute, _, datetime| Ok(helpers::hour_relative_minute(
                 datetime.value().form_time_of_day()?.full_hour(),
                 -1 * relative_minute.value().0,
                 true)?
                 .precision(datetime.value().precision))
    );
    b.rule_3("relative minutes after <hour>",
             relative_minute_check!(),
             b.reg(r#"dopo le"#)?,
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour {.. }))),
             |relative_minute, _, datetime| Ok(helpers::hour_relative_minute(
                 datetime.value().form_time_of_day()?.full_hour(),
                 relative_minute.value().0,
                 true)?.precision(datetime.value().precision))
    );
    b.rule_1_terminal("hh(:|h)mm (time-of-day)",
                      b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:\.h]([0-5]\d)"#)?,
                      |text_match| {
                          let hour: u32 = text_match.group(1).parse()?;
                          let minute: u32 = text_match.group(2).parse()?;
                          helpers::hour_minute(hour, minute, hour < 12)
                      }
    );
    b.rule_1_terminal("hh:mm:ss",
                      b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:\.h]([0-5]\d)[:.]([0-5]\d)"#)?,
                      |text_match| helpers::hour_minute_second(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(3).parse()?,
                          false
                      )

    );
    b.rule_1_terminal("hhmm (military time-of-day)",
                      b.reg(r#"((?:[01]?\d)|(?:2[0-3]))([0-5]\d)"#)?,
                      |text_match| Ok(helpers::hour_minute(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?,
                          false
                      )?.latent())
    );
    b.rule_1_terminal("quarter (relative minutes)",
                      b.reg(r#"(?:(?:e )?un )?quarto"#)?,
                      |_| Ok(RelativeMinuteValue(15))
    );
    b.rule_1_terminal("half (relative minutes)",
                      b.reg(r#"e mezz[oa]"#)?,
                      |_| Ok(RelativeMinuteValue(30))
    );
    b.rule_1_terminal("three quarters (relative minutes)",
                      b.reg(r#"(?:3|tre) quarti(?: d'ora)?"#)?,
                      |_| Ok(RelativeMinuteValue(45))
    );
    b.rule_1("number (as relative minutes)",
             integer_check_by_range!(1, 59),
             |a| Ok(RelativeMinuteValue(a.value().value as i32))
    );
    b.rule_2("number minutes (as relative minutes)",
             integer_check_by_range!(1, 59),
             b.reg(r#"min(?:\.|ut[oi])?"#)?,
             |a, _| Ok(RelativeMinuteValue(a.value().value as i32))
    );
    // check datetime islatent() removed (I don't understand why it is not working with this check)
    b.rule_2("<hour-of-day> <integer> (as relative minutes)",
             datetime_check!(|datetime: &DatetimeValue| form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))(datetime)),
             relative_minute_check!(),
             |datetime, minutes| helpers::hour_relative_minute(
                 datetime.value().form_time_of_day()?.full_hour(),
                 minutes.value().0,
                 datetime.value().form_time_of_day()?.is_12_clock()
             )
    );
    // check datetime islatent() removed (I don't understand why it is not working with this check)
    b.rule_3("<hour-of-day> minus <integer> (as relative minutes)",
             datetime_check!(|datetime: &DatetimeValue| form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))(datetime)),
             b.reg(r#"meno"#)?,
             relative_minute_check!(),
             |datetime, _, minutes| helpers::hour_relative_minute(
                 datetime.value().form_time_of_day()?.full_hour(),
                 -1 * minutes.value().0,
                 datetime.value().form_time_of_day()?.is_12_clock()
             )
    );
    // check datetime islatent() removed (I don't understand why it is not working with this check)
    b.rule_3("<hour-of-day> and/past <relative minutes>",
             datetime_check!(|datetime: &DatetimeValue| form!(Form::TimeOfDay(TimeOfDayForm::Hour { .. }))(datetime)),
             b.reg(r#"e"#)?,
             relative_minute_check!(),
             |datetime, _, minutes| helpers::hour_relative_minute(
                 datetime.value().form_time_of_day()?.full_hour(),
                 minutes.value().0,
                 datetime.value().form_time_of_day()?.is_12_clock()
             )
    );
    // Written dates in numeric formats
    b.rule_1_terminal("yyyy-mm-dd - ISO",
                      b.reg(r#"(\d{4})[-/](0?[1-9]|1[0-2])[-/](3[01]|[12]\d|0?[1-9])"#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(3).parse()?)
    );
    b.rule_1_terminal("dd/mm/yy or dd/mm/yyyy",
                      b.reg(r#"(0?[1-9]|[12]\d|3[01])[-\./](0?[1-9]|1[0-2])[-\./](\d{2,4})"#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(3).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(1).parse()?,
                      )
    );
    b.rule_1_terminal("dd/mm",
                      b.reg(r#"(0?[1-9]|[12]\d|3[01])[-\./](1[0-2]|0?[1-9])"#)?,
                      |text_match| helpers::month_day(
                          text_match.group(2).parse()?,
                          text_match.group(1).parse()?)
    );
    // End of Written dates in numeric formats
    // Parts of the day
    b.rule_1_terminal("morning",
                      b.reg(r#"mattin(?:o|a(?:ta)?)"#)?,
                      |_| Ok(helpers::hour(4, false)?
                          .span_to(&helpers::hour(12, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Morning)))
    );
    b.rule_1_terminal("beginning of morning (French 'matinée')",
                      b.reg(r#"(?:inizio(?: del(?:la)?)?|prim[ao]) mattin(?:o|a(?:ta)?)|mattin(?:o|a(?:ta)?) (?:sul )?presto"#)?,
                      |_| Ok(helpers::hour(4, false)?
                          .span_to(&helpers::hour(9, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Morning)))
    );
    b.rule_1_terminal("breakfast",
                      b.reg(r#"(?:prima )?colazione"#)?,
                      |_| Ok(helpers::hour(5, false)?
                          .span_to(&helpers::hour(10, false)?, false)?
                          .latent()
                          .form(Form::Meal))
    );
    b.rule_1_terminal("middle of morning",
                      b.reg(r#"(?:met[aà]|mezzo) (?:del(?:la)? )?mattin(?:o|a(?:ta)?)"#)?,
                      |_| Ok(helpers::hour(9, false)?
                          .span_to(&helpers::hour(11, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Morning)))
    );
    b.rule_1_terminal("brunch",
                      b.reg(r#"brunch"#)?,
                      |_| Ok(helpers::hour(10, false)?
                          .span_to(&helpers::hour(15, false)?, false)?
                          .latent()
                          .form(Form::Meal))
    );
    b.rule_1_terminal("end of morning",
                      b.reg(r#"(?:tard[ao] |fine (?:del(?:la)? ))?mattin(?:o|a(?:ta)?)|seconda mattina(?:ta)?|mattin(?:o|a(?:ta))? (?:sul )?tardi"#)?,
                      |_| Ok(helpers::hour(10, false)?
                          .span_to(&helpers::hour(12, false)?, false)?
                          .latent()
                          .form(Form::PartOfDay(PartOfDayForm::Morning)))
    );
    b.rule_1_terminal("lunch",
                      b.reg(r#"(?:all' ?ora di )?pranzo|seconda colazione"#)?,
                      |_| Ok(helpers::hour(12, false)?
                          .span_to(&helpers::hour(14, false)?, false)?
                          .latent()
                          .form(Form::Meal))
    );
    b.rule_1_terminal("after lunch",
                      b.reg(r#"dopo (?:il )?pranzo"#)?,
                      |_| {
                          let period = helpers::hour(13, false)?
                              .span_to(&helpers::hour(17, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 0)?.intersect(&period)?.form(Form::PartOfDay(PartOfDayForm::Afternoon)))
                      }
    );
    b.rule_1_terminal("before lunch",
                      b.reg(r#"prima d(?:i|el) pranzo"#)?,
                      |_| {
                          let period = helpers::hour(10, false)?
                              .span_to(&helpers::hour(12, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 0)?.intersect(&period)?.form(Form::PartOfDay(PartOfDayForm::Morning)))
                      }
    );
    b.rule_1_terminal("before work",
                      b.reg(r#"prima (?:del(?:l'orario di)? lavoro|di (?:andare a )?lavorare)"#)?,
                      |_| {
                          let period = helpers::hour(7, false)?
                              .span_to(&helpers::hour(10, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 0)?.intersect(&period)?.form(Form::PartOfDay(PartOfDayForm::Morning)))
                      }
    );
    b.rule_1_terminal("during work",
                      b.reg(r#"durante (?:il|(?:l'orario|le ore) di) lavoro"#)?,
                      |_| {
                          let period = helpers::hour(9, false)?
                              .span_to(&helpers::hour(19, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 0)?.intersect(&period)?.form(Form::PartOfDay(PartOfDayForm::None)))
                      }
    );
    b.rule_1_terminal("after work",
                      b.reg(r#"dopo (?:il|l'orario di) lavoro"#)?,
                      |_| {
                          let period = helpers::hour(17, false)?
                              .span_to(&helpers::hour(21, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 0)?.intersect(&period)?.form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );
    b.rule_1_terminal("afternoon",
                      b.reg(r#"pomeriggio"#)?,
                      |_| {
                          Ok(helpers::hour(12, false)?
                              .span_to(&helpers::hour(19, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
                      }
    );
    b.rule_1_terminal("beginning of afternoon (French: 'début d après-midi')",
                      b.reg(r#"(?:primo|inizio(?: del)?) pomeriggio|pomeriggio (?:sul )?presto"#)?,
                      |_| {
                          Ok(helpers::hour(12, false)?
                              .span_to(&helpers::hour(15, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
                      }
    );
    b.rule_1_terminal("middle of afternoon (French: 'milieu d après-midi')",
                      b.reg(r#"(?:met[aà]|mezzo) (?:del )?pomeriggio"#)?,
                      |_| {
                          Ok(helpers::hour(15, false)?
                              .span_to(&helpers::hour(17, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
                      }
    );
    b.rule_1_terminal("afternoon snack time (French 'goûter')",
                      b.reg(r#"(?:per |durante |al)?(?:l'ora dell[ao]|l[ao])? ?(?:merenda|spuntino)"#)?,
                      |_| Ok(helpers::hour(16, false)?
                          .span_to(&helpers::hour(18, false)?, false)?
                          .form(Form::Meal))
    );
    b.rule_1_terminal("tea time",
                      b.reg(r#"(?:per |durante |al)?(?:l'ora del|il) t(?:h)?(?:è|é|e)"#)?,
                      |_| Ok(helpers::hour(15, false)?
                          .span_to(&helpers::hour(17, false)?, false)?
                          .form(Form::Meal))
    );
    b.rule_1_terminal("coffee time",
                      b.reg(r#"(?:per |durante |al)?(?:l'ora del|il) caff(?:è|é|e)"#)?,
                      |_| Ok(helpers::hour(14, false)?
                          .span_to(&helpers::hour(16, false)?, false)?
                          .form(Form::Meal))
    );
    b.rule_1_terminal("end of afternoon",
                      b.reg(r#"(?:tardo|secondo|fine(?: del)?) pomeriggio|pomeriggio (?:sul )?tardi"#)?,
                      |_| {
                          Ok(helpers::hour(17, false)?
                              .span_to(&helpers::hour(19, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
                      }
    );
    b.rule_1_terminal("drinks",
                      b.reg(r#"aperitivo"#)?,
                      |_| {
                          Ok(helpers::hour(18, false)?
                              .span_to(&helpers::hour(19, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
                      }
    );
    b.rule_1_terminal("beginining of the day",
                      b.reg(r#"inizio (?:del(?:la)? )?giorn(?:o|ata)"#)?,
                      |_| {
                          Ok(helpers::hour(6, false)?
                              .span_to(&helpers::hour(10, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Morning)))
                      }
    );
    b.rule_1_terminal("middle of the day",
                      b.reg(r#"(?:met[aà]|mezzo) (?:del(?:la)? )?giorn(?:o|ata)"#)?,
                      |_| {
                          Ok(helpers::hour(11, false)?
                              .span_to(&helpers::hour(16, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::None)))
                      }
    );
    b.rule_1_terminal("end of the day (French 'fin de journée' sounds earlier than 9pm...)",
                      b.reg(r#"fine (?:del(?:la)? )?giorn(?:o|ata)"#)?,
                      |_| {
                          Ok(helpers::hour(17, false)?
                              .span_to(&helpers::hour(21, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );
    b.rule_1_terminal("evening",
                      b.reg(r#"sera(?:ta)?"#)?,
                      |_| {
                          Ok(helpers::hour(18, false)?
                              .span_to(&helpers::hour(0, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );
    b.rule_1_terminal("beginning of the evening",
                      b.reg(r#"inizio (?:della )?sera(?:ta)?"#)?,
                      |_| {
                          Ok(helpers::hour(18, false)?
                              .span_to(&helpers::hour(21, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );
    b.rule_1_terminal("end of the evening",
                      b.reg(r#"fine (?:della )?sera(?:ta)?"#)?,
                      |_| {
                          Ok(helpers::hour(21, false)?
                              .span_to(&helpers::hour(0, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );
    b.rule_1_terminal("dinner time",
                      b.reg(r#"(?:ora di )?cena"#)?,
                      |_| Ok(helpers::hour(18, false)?
                          .span_to(&helpers::hour(23, false)?, false)?
                          .form(Form::Meal))
    );
    b.rule_1_terminal("night",
                      b.reg(r#"notte"#)?,
                      |_| {
                          Ok(helpers::hour(22, false)?
                              .span_to(&helpers::hour(6, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Night)))
                      }
    );
    b.rule_2("<meal> time",
             b.reg(r#"a|(?:per|durante) (?:il|l[ao])?|(?:al)?l'ora d(?:i|el(?:l[ao])?)"#)?,
             datetime_check!(|datetime: &DatetimeValue| datetime.latent && form!(Form::Meal)(datetime)),
             |_, a| Ok(a.value().clone().not_latent())
    );
    b.rule_2("<dim time> <meal>",
             datetime_check!(),
             datetime_check!(form!(Form::Meal)),
             |a, b| a.value().intersect(b.value())
    );
    b.rule_2("in the <part-of-day>",
             b.reg(r#"(?:durante )?(?:il|la)|d(?:['i]|el(?:l['ao])?)|in|nel(?:la)?|nel corso del(?:la)?|a(?:ll(?:e|' ?)?(?: ora d[i'])?)?"#)?,
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             |_, a| Ok(a.value().clone().not_latent())
    );
    b.rule_2("this <part-of-day>",
             b.reg(r#"quest[ao]"#)?,
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             |_, a| Ok(helpers::cycle_nth(Grain::Day, 0)?.intersect(a.value())?.form(a.value().form.clone()))
    );
    b.rule_1_terminal("this morning",
                      b.reg(r#"stamattina"#)?,
                      |_| {
                          let period = helpers::hour(4, false)?.span_to(&helpers::hour(12, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 0)?
                              .intersect(&period)?
                              .form(Form::PartOfDay(PartOfDayForm::Morning)))
                      }
    );
    b.rule_1_terminal("tomorrow morning",
                      b.reg(r#"domattina"#)?,
                      |_| {
                          let period = helpers::hour(4, false)?.span_to(&helpers::hour(12, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 1)?
                              .intersect(&period)?
                              .form(Form::PartOfDay(PartOfDayForm::Morning)))
                      }
    );
    b.rule_1_terminal("tonight",
                      b.reg(r#"stasera"#)?,
                      |_| {
                          let period = helpers::hour(18, false)?.span_to(&helpers::hour(0, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 0)?
                              .intersect(&period)?
                              .form(Form::PartOfDay(PartOfDayForm::Night)))
                      }
    );
    b.rule_1_terminal("this night",
                      b.reg(r#"stanotte"#)?,
                      |_| {
                          let period = helpers::hour(20, false)?.span_to(&helpers::hour(0, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 0)?
                              .intersect(&period)?
                              .form(Form::PartOfDay(PartOfDayForm::Night)))
                      }
    );
    b.rule_2("<dim time> <part-of-day>",
             //datetime_check!(excluding_form!(Form::TimeOfDay(_))),
             datetime_check!(|datetime: &DatetimeValue| !!!datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             |a, b| a.value().intersect(b.value())
    );
    b.rule_2("<dim time> in the morning",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:(?:il|la)|di|in|[dn](?:el(?:la)?)) mattin(?:o|a(?:ta)?)"#)?,
             |a, _| {
                 let period = helpers::hour(0, false)?
                     .span_to(&helpers::hour(12, false)?, false)?;
                 a.value().intersect(&period)
             }
    );
    b.rule_2("<dim time> in the evening",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:d(?:i|ella)|la) (?:sera|stasera)|(?:nella|in) serata"#)?,
             |a, _| {
                 let period = helpers::hour(16, false)?
                     .span_to(&helpers::hour(0, false)?, false)?;
                 a.value().intersect(&period)
             }
    );
    // TODO + nel|del|di|il pomeriggio (in the afternoon),
    // TODO + nella|della|di|la notte (in the night)
    b.rule_3("<part-of-day> of <dim time>",
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             b.reg(r#"di|del|della|dello|dell'"#)?,
             datetime_check!(),
             |a, _, b| b.value().intersect(a.value())
    );
    // Parts of the week
    b.rule_1_terminal("week-end",
                      b.reg(r#"week(?:\s|-)?end|fine(?:\s|-)?settimana"#)?,
                      |_| {
                          let friday = helpers::day_of_week(Weekday::Fri)?
                              .intersect(&helpers::hour(18, false)?)?;
                          let monday = helpers::day_of_week(Weekday::Mon)?
                              .intersect(&helpers::hour(0, false)?)?;
                          friday.span_to(&monday, false)
                      }
    );
    b.rule_1_terminal("beginning of the week",
                      b.reg(r#"(?:a |all')?inizio (?:della |di questa )?settimana"#)?,
                      |_| helpers::day_of_week(Weekday::Mon)
                          ?.span_to(&helpers::day_of_week(Weekday::Tue)?, false)
    );
    b.rule_1_terminal("middle of the week",
                      b.reg(r#"(?:(?:a(?:lla)? )?met[aà]|(?:nel )?mezzo) (?:della |di questa )?settimana"#)?,
                      |_| helpers::day_of_week(Weekday::Wed)
                          ?.span_to(&helpers::day_of_week(Weekday::Thu)?, false)
    );
    b.rule_1_terminal("end of the week",
                      b.reg(r#"(?:alla )?fine (?:della|di questa) settimana"#)?,
                      |_| helpers::day_of_week(Weekday::Thu)
                          ?.span_to(&helpers::day_of_week(Weekday::Sun)?, false)
    );
    b.rule_1_terminal("during the week / week days",
                      b.reg(r#"(?:durante la|nel corso della|in) settimana"#)?,
                      |_| helpers::day_of_week(Weekday::Mon)
                          ?.span_to(&helpers::day_of_week(Weekday::Fri)?, false)
    );
    b.rule_2("the <datetime>",
             b.reg(r#"il|l['oa]"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |_, a| Ok(a.value().clone())
    );
    // Seasons
    b.rule_1_terminal("summer",
                      b.reg(r#"(?:quest(?:a |')|nell')?estate"#)?,
                      |_| helpers::month_day(6, 21)?.span_to(&helpers::month_day(9, 23)?, false)
    );
    b.rule_1_terminal("autumn",
                      b.reg(r#"(?:quest(?:o |')|nell')?autunno"#)?,
                      |_| helpers::month_day(9, 23)?.span_to(&helpers::month_day(12, 21)?, false)
    );
    b.rule_1_terminal("winter",
                      b.reg(r#"(?:quest(?:o |')|nell')?inverno"#)?,
                      |_| helpers::month_day(12, 21)?.span_to(&helpers::month_day(3, 20)?, false)
    );
    b.rule_1_terminal("spring",
                      b.reg(r#"(?:questa |nella )?primavera"#)?,
                      |_| helpers::month_day(3, 20)?.span_to(&helpers::month_day(6, 21)?, false)
    );
    // Dates
    // TODO x1 - Understand what this is ????
//    b.rule_2("le <datetime>",
//             b.reg(r#"l[ea]"#)?,
//             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
//             |_, a| Ok(a.value().clone())
//    );
    // Date intervals
    b.rule_2("dd-dd <month>(interval)",
             b.reg(r#"(3[01]|[12]\d|0?[1-9])(?: ?\- ?|al(?:l')? )(3[01]|[12]\d|0?[1-9])"#)?,
             datetime_check!(form!(Form::Month(_))),
             |a, month| {
                 let start = month.value().intersect(&helpers::day_of_month(a.group(1).parse()?)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(a.group(2).parse()?)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_3("<datetime>-dd <month>(interval)",
             datetime_check!(),
             b.reg(r#"(\-|al(?:l')? ?3[01]|[12]\d|0?[1-9])"#)?,
             datetime_check!(form!(Form::Month(_))),
             |datetime, text_match, month| {
                 let start = month.value().intersect(datetime.value())?;
                 let end = month.value().intersect(&helpers::day_of_month(text_match.group(1).parse()?)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_5("<datetime>-<day-of-week> dd <month>(interval)",
             datetime_check!(),
             b.reg(r#"\-|a(?:l(?:l['o]))"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             datetime_check!(form!(Form::Month(_))),
             |datetime, _, _, text_match, month| {
                 let start = month.value().intersect(datetime.value())?;
                 let end = month.value().intersect(&helpers::day_of_month(text_match.group(1).parse()?)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_6("<day-of-week> 1st-<day-of-week> dd <month> to (interval)",
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"(?:1|prim)o"#)?,
             b.reg(r#"\-|a(?:l(?:l['o]))"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, _, _, _, text_match, month| {
                 let start = month.value().intersect(&helpers::day_of_month(1)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(text_match.group(1).parse()?)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_6("from <datetime> to <day-of-week> dd <month> (interval)",
             b.reg(r#"dal?(?:l['o])?"#)?,
             datetime_check!(),
             b.reg(r#"\-|a(?:l(?:l['o]))"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, datetime, _, _, a, month| {
                 let start = month.value().intersect(datetime.value())?;
                 let end = month.value().intersect(&helpers::day_of_month(a.group(1).parse()?)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_6("from dd-<day-of-week> dd <month> to (interval)",
             b.reg(r#"dal?(?:l['o])?"#)?,
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             b.reg(r#"\-|a(?:l(?:l['o]))"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, a, _, _, b, month| {
                 let start = month.value().intersect(&helpers::day_of_month(a.group(1).parse()?)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(b.group(1).parse()?)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_5("from dd to dd <month> (interval)",
             b.reg(r#"dal?(?:l['o])?"#)?,
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             b.reg(r#"\-|(?:fino )?a(?:l(?:l['o])?)?"#)?,
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, a, _, b, month| {
                 let start = month.value().intersect(&helpers::day_of_month(a.group(1).parse()?)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(b.group(1).parse()?)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_6("from dd <month> to dd <month> (interval)",
             b.reg(r#"dal?(?:l['o])?"#)?,
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             datetime_check!(form!(Form::Month(_))),
             b.reg(r#"(?:fino )?a(?:l(?:l['o])?)?"#)?,
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, a, month_a, _, b, month_b| {
                 let start = month_a.value().intersect(&helpers::day_of_month(a.group(1).parse()?)?)?;
                 let end = month_b.value().intersect(&helpers::day_of_month(b.group(1).parse()?)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_6("from dd-<day-of-week> dd <month> to (interval)",
             b.reg(r#"dal?(?:l['o])?"#)?,
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             b.reg(r#"\-|a(?:l(?:l['o]))"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, a, _, _, b, month| {
                 let start = month.value().intersect(&helpers::day_of_month(a.group(1).parse()?)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(b.group(1).parse()?)?)?;
                 start.span_to(&end, true)
             }
    );
    // TODO: fix this, doesn't make sense
//    b.rule_6("from dd-<day-of-week> dd <month> to (interval)",
//             b.reg(r#"dal?(?:l['o])?"#)?,
//             datetime_check!(form!(Form::DayOfWeek{..})),
//             b.reg(r#"\-|a(?:l(?:l['o]))"#)?,
//             datetime_check!(form!(Form::DayOfWeek{..})),
//             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
//             datetime_check!(form!(Form::Month(_))),
//             |_, datetime, _, _, text_match, month| {
//                 let start = month.value().intersect(datetime.value())?;
//                 let end = month.value().intersect(&helpers::day_of_month(text_match.group(1).parse()?)?)?;
//                 start.span_to(&end, true)
//             }
//    );
    // END TODO
    // TODO x1
//    b.rule_4("during the night from <day-of-week> to <day-of-week>",
//             b.reg(r#""#)?,
//             datetime_check!(form!(Form::DayOfWeek{..})),
//             b.reg(r#"\-|a(?:l(?:l['o]))"#)?,
//             datetime_check!(form!(Form::DayOfWeek{..})),
//             |_, start, _, end| {
//                 let start = start.value().intersect(&helpers::hour(22, false)?)?;
//                 let end = end.value().intersect(&helpers::hour(6, false)?)?;
//                 start.span_to(&end, false)
//             }
//    );
    b.rule_5("between dd and dd <month>(interval)",
             b.reg(r#"tra(?: il|l')?"#)?,
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             b.reg(r#"e(?: il|l')?"#)?,
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, a, _, b, month| {
                 let start = month.value().intersect(&helpers::day_of_month(a.group(1).parse()?)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(b.group(1).parse()?)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_2_terminal("from dd to dd(interval)",
          b.reg(r#"dal(?:l'?)? ?(3[01]|[12]\d|0?[1-9])"#)?,
          b.reg(r#"\-|al(?:l')(3[01]|[12]\d|0?[1-9])"#)?,
          |a, b| {
              let start = helpers::day_of_month(a.group(1).parse()?)?;
              let end = helpers::day_of_month(b.group(1).parse()?)?;
              start.span_to(&end, true)
          }
    );
    // Parts of the month
    b.rule_2("end (of) <named-month>(interval) (French: 'fin janvier')",
             b.reg(r#"fine (?:(?:del mese )?d[i'])?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, month| {
                 let start = month.value().intersect(&helpers::day_of_month(25)?)?;
                 let end = helpers::cycle(Grain::Day)?.last_of(month.value())?;
                 start.span_to(&end, true)
             }
    );
    b.rule_2("beginning (of) <named-month>(interval) (French: 'début janvier')",
             b.reg(r#"inizio (?:(?:del mese )?d[i'])?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, month| {
                 let start = month.value().intersect(&helpers::day_of_month(1)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(5)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_2("first half of <named-month>(interval) (French: 'quinzaine')",
             b.reg(r#"prima (met[a|à]|parte|quindicina|15ina) d[i']"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, month| {
                 let start = month.value().intersect(&helpers::day_of_month(1)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(14)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_2("second half of <named-month>(interval) (French: 'quinzaine')",
             b.reg(r#"seconda (met[a|à]|parte|quindicina|15ina) d[i']"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, month| {
                 let start = month.value().intersect(&helpers::day_of_month(15)?)?;
                 let end = helpers::cycle(Grain::Day)?.last_of(month.value())?;
                 start.span_to(&end, true)
             }
    );
    b.rule_2("mid - <named-month>",
             b.reg(r#"met[àa] (?:d[i'])?"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, month| {
                 let start = month.value().intersect(&helpers::day_of_month(10)?)?;
                 let end = month.value().intersect(&helpers::day_of_month(19)?)?;
                 start.span_to(&end, true)
             }
    );
    // FIXME: This doesn't have a named month. Gaps in rule coverage wrt. deictic vs. named units?
    b.rule_1_terminal("end of the month",
                      b.reg(r#"(?:a |alla )?fine (?:del )?mese"#)?,
                      |_| {
                          let month = helpers::cycle_nth(Grain::Month, 1)?;
                          Ok(helpers::cycle_nth_after(Grain::Day, -10, &month)?
                              .span_to(&month, false)?
                              .latent()
                              .form(Form::PartOfMonth))
                      }
    );
    // Datetime intervals
    b.rule_3("<datetime> - <datetime> (interval)",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#"(?:fino )?a(?:l(?:l['o])?)?"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             |a, _, b| a.value().span_to(b.value(), true)
    );
    b.rule_3("<datetime> avant <time-of-day> (interval)",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#"(?:fino )?a(?:l(?:l['o])?)?|prima(?: d[i'])"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |a, _, b| a.value().span_to(b.value(), false)
    );
    b.rule_4("from <datetime> - <datetime> (interval)",
             b.reg(r#"dal?(?:l['oe])?"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#"(?:fino )?a(?:l(?:l['oe])?)?"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             |_, a, _, b| a.value().span_to(b.value(), true)
    );
    b.rule_4("between <datetime> and <datetime> (interval)",
             b.reg(r#"[tf]ra"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#"e"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             |_, a, _, b| a.value().span_to(b.value(), true)
    );
    // Specific case with years
    b.rule_5("from <datetime> - <datetime> <year> (interval)",
             b.reg(r#"dal?(?:l['oe])?"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#"(?:fino )?a(?:l(?:l['o])?)?"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime) && datetime.is_coarse_grain_greater_than(Grain::Year)),
             datetime_check!(form!(Form::Year(_))),
             |_, a, _, b, year| a.value().span_to(b.value(), true)?.intersect(year.value())
    );
    b.rule_5("between <datetime> and <datetime> <year> (interval)",
             b.reg(r#"[ft]ra"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#"e"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime) && datetime.is_coarse_grain_greater_than(Grain::Year)),
             datetime_check!(form!(Form::Year(_))),
             |_, a, _, b, year| a.value().span_to(b.value(), true)?.intersect(year.value())
    );
    b.rule_3("<time-of-day> - <time-of-day> (interval)",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#"(?:fino )?a(?:l(?:l['oe])?)?"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && form!(Form::TimeOfDay(_))(datetime)),
             |a, _, b| a.value().smart_span_to(b.value(), false)
    );
    b.rule_4("from <time-of-day> - <time-of-day> (interval)",
             b.reg(r#"dal?(?:l['oe])?"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:fino )?a(?:l(?:l['oe])?)?"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, a, _, b| a.value().smart_span_to(b.value(), false)
    );
    b.rule_4("between <time-of-day> and <time-of-day> (interval)",
             b.reg(r#"[ft]ra"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"e"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, a, _, b| a.value().smart_span_to(b.value(), false)
    );
    b.rule_2("before <time-of-day>",
             b.reg(r#"prima(?: d(?:i|el(?:l[ea])?)?)?|entro|[sf]ino a(?:l(?:l[eoa])?)?"#)?,
             datetime_check!(),
             |_, datetime| Ok(datetime.value().clone().mark_before_end())
    );
    b.rule_2("after <time-of-day>",
             b.reg(r#"dopo"#)?,
             datetime_check!(),
             |_, datetime| Ok(datetime.value().clone().mark_after_end())
    );
    b.rule_2("after <time-of-day>",
             b.reg(r#"dopo|a partire da(?:l(?:l['e])?)?|da(?:l(?:l[eo])?)?"#)?,
             datetime_check!(),
             |_, datetime| Ok(datetime.value().clone().mark_after_start())
    );
    b.rule_2("after the <day-of-month>",
             b.reg(r#"dopo|a partire da(?:l(?:l['e])?)?"#)?,
             integer_check_by_range!(1, 31),
             |_, integer| Ok(helpers::day_of_month(integer.value().value as u32)?.mark_after_end())
    );
    b.rule_2("after le <day-of-month>",
             b.reg(r#"dopo|a partire da(?:l(?:l['e])?)?"#)?,
             integer_check_by_range!(1, 31),
             |_, integer| Ok(helpers::day_of_month(integer.value().value as u32)?.mark_after_start())
    );
    Ok(())
}

pub fn rules_datetime_with_duration(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("by/before <duration> (French: 'd ici'",
             b.reg(r#"prima|entro"#)?,
             duration_check!(),
             |_, duration| {
                 let start = helpers::cycle_nth(Grain::Second, 0)?;
                 let end = duration.value().in_present()?;
                 start.span_to(&end, false)
             }
    );
    // Durations with modifiers / timeline positioning
    b.rule_2("in-future <duration> (French 'dans 2 mois')",
             b.reg(r#"[tf]ra"#)?,
             duration_check!(),
             |_, duration| duration.value().in_present()
    );
    b.rule_2("<duration> later",
             duration_check!(),
             b.reg(r"(dopo|più tardi)")?,
             |duration, _| duration.value().in_present()
    );
    b.rule_2("<duration> ago",
             duration_check!(),
             b.reg(r#"fa"#)?,
             |duration, _| duration.value().ago()
    );
    b.rule_3("in <duration> from now",
             b.reg(r#"[tf]ra"#)?,
             duration_check!(),
             b.reg(r#"da (?:adesso|ora)?"#)?,
             |_, duration, _| duration.value().in_present()
    );
    b.rule_2("since <duration>",
             b.reg(r#"da(?: |l(?:l['oaie])?)"#)?,
             duration_check!(),
             |_, duration| {
                 duration.value().ago()?
                     .span_to(&helpers::cycle_nth(Grain::Second, 0)?, false)
             });
    b.rule_3("<duration> after <datetime>",
             duration_check!(),
             b.reg(r#"dopo"#)?,
             datetime_check!(),
             |duration, _, datetime| duration.value().after(datetime.value())
    );
    b.rule_3("<duration> before <datetime>",
             duration_check!(),
             b.reg(r#"prima"#)?,
             datetime_check!(),
             |duration, _, datetime| duration.value().after(datetime.value())
    );
    Ok(())

}

pub fn rules_datetime_with_nth_cycle(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    // Cycles with modifiers / timeline positioning
    b.rule_2("this / in the <cycle>",
             b.reg(r#"(?:di )?quest[oa']|in"#)?,
             cycle_check!(),
             |_, cycle| helpers::cycle_nth(cycle.value().grain, 0)
    );
    b.rule_2("last <cycle>",
             b.reg(r#"(?:nell[oa] |nell'|nel corso (?:dell[oa] |dell')?)?(scors[oa]|passat[oa]|quest'ultim[ao])"#)?,
             cycle_check!(),
             |_, cycle| helpers::cycle_nth(cycle.value().grain, -1)
    );
    b.rule_2("<cycle> last",
             cycle_check!(),
             b.reg(r#"scors[oa]|passat[oa]"#)?,
             |cycle, _| helpers::cycle_nth(cycle.value().grain, -1)
    );
    b.rule_2("the next <cycle>",
             b.reg(r#"(?:il |la |nella |nel (?:corso del )?)?prossim[oa]"#)?,
             cycle_check!(),
             |_, cycle| helpers::cycle_nth(cycle.value().grain, 1)
    );
    b.rule_2("next <cycle>",
             b.reg(r#"prossim[oa]"#)?,
             cycle_check!(),
             |_, cycle| helpers::cycle_nth(cycle.value().grain, 1)
    );
    b.rule_2("<cycle> next",
             cycle_check!(),
             b.reg(r#"prossim[oa]|seguent[ei]|che viene|dopo|successiv[oa]"#)?,
             |cycle, _| helpers::cycle_nth(cycle.value().grain, 1)
    );
    b.rule_3("the <cycle> next",
             b.reg(r#"il|l['ao]"#)?,
             cycle_check!(),
             b.reg(r#"prossim[oa]|seguent[ei]|che viene|dopo|successiv[oa]"#)?,
             |_, cycle, _| helpers::cycle_nth(cycle.value().grain, 1)
    );
    b.rule_3("n <cycle> before",
             integer_check_by_range!(2, 9999),
             cycle_check!(),
             b.reg(r#"prima|fa"#)?,
             |integer, cycle, _| helpers::cycle_nth(cycle.value().grain, -1 * integer.value().value)
    );
    b.rule_3("n <cycle> after",
             integer_check_by_range!(2, 9999),
             cycle_check!(),
             b.reg(r#"dopo"#)?,
             |integer, cycle, _| helpers::cycle_nth(cycle.value().grain, integer.value().value)
    );
    // TODO: resolution is not correct for times, i.e. rounds at grain?
    b.rule_3("last n <cycle>",
             b.reg(r#"gli scorsi|(?:(?:gli|nelle|(?:per|durante)? le|nel corso de(?:lle|gli)?|nell'arco di questi)? ultim[ei])?"#)?,
             integer_check_by_range!(1, 9999),
             cycle_check!(),
             |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    // TODO: Same as above ?
    b.rule_3("next n <cycle>",
             b.reg(r#"(?:[nd]ei|i|nelle)? prossim[ie]"#)?,
             integer_check_by_range!(1, 9999),
             cycle_check!(),
             |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    // TODO: more <cycle> combinations with N + past/future
    // LATER
    // END TODO
    b.rule_4("<ordinal> <cycle> of <datetime>",
             ordinal_check_by_range!(1, 9999),
             cycle_check!(),
             b.reg(r#"d(?:['i]|el(?:l['ao])?)"#)?,
             datetime_check!(),
             |ordinal, cycle, _, datetime| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value - 1, datetime.value())
    );
    b.rule_5("the <ordinal> <cycle> of <datetime>",
             b.reg(r#"il|l['ao]"#)?,
             ordinal_check_by_range!(1, 9999),
             cycle_check!(),
             b.reg(r#"d(?:['i]|el(?:l['ao])?)"#)?,
             datetime_check!(),
             |_, ordinal, cycle, _, datetime| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value - 1, datetime.value())
    );
    b.rule_4("the <cycle> of <datetime>",
             b.reg(r#"il|l['ao]"#)?,
             cycle_check!(),
             b.reg(r#"d(?:['i]|el(?:l['ao])?)"#)?,
             datetime_check!(),
             |_, cycle, _, datetime| helpers::cycle_nth_after_not_immediate(cycle.value().grain, 0, datetime.value())
    );
    Ok(())
}


pub fn rules_cycle(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    // Cycle units
    b.rule_1_terminal("second (cycle)",
                      b.reg(r#"second[oi]"#)?,
                      |_| CycleValue::new(Grain::Second)
    );
    b.rule_1_terminal("minute (cycle)",
                      b.reg(r#"minut[oi]"#)?,
                      |_| CycleValue::new(Grain::Minute)
    );
    b.rule_1_terminal("hour (cycle)",
                      b.reg(r#"or[ae]"#)?,
                      |_| CycleValue::new(Grain::Hour)
    );
    b.rule_1_terminal("day (cycle)",
                      b.reg(r#"giorn(?:[oi]|ata)"#)?,
                      |_| CycleValue::new(Grain::Day)
    );
    b.rule_1_terminal("week (cycle)",
                      b.reg(r#"settiman[ae]"#)?,
                      |_| CycleValue::new(Grain::Week)
    );
    b.rule_1_terminal("month (cycle)",
                      b.reg(r#"mes[ei]"#)?,
                      |_| CycleValue::new(Grain::Month)
    );
    b.rule_1_terminal("Quarter (cycle)",
                      b.reg(r#"trimestre"#)?,
                      |_| CycleValue::new(Grain::Quarter)
    );
    b.rule_1_terminal("year (cycle)",
                      b.reg(r#"ann[oi]"#)?,
                      |_| CycleValue::new(Grain::Year)
    );
    Ok(())
}