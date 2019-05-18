use chrono::Weekday;
use chrono::naive::NaiveTime;

/// A single regular shift slot.
///
/// The current implementation assumes that every shift starts and ends on the same day of the week.
pub struct RecurringShiftSlot {
    /// Weekday of the shift
    pub weekday: Weekday,
    /// Time at which the shift starts
    pub start: NaiveTime,
    /// Time at which the shift ends
    pub end: NaiveTime,
    /// The person in charge of the shift slot. `None` if vacant.
    pub barista: Option<String>,
}

/// A single slot for an irregular shift that happens only once.
pub struct IrregularShiftSlot {
    /// Time and Date at which the shift starts
    pub start: NaiveDateTime,
    /// Time and Date at which the shift ends
    pub end: NaiveDateTime,
    /// Person in charge of the shift slot. `None` if vacant.
    pub barista: Option<String>,
}

/// Wrapper for the different shift types.
pub enum ShiftType {
    /// A regular shiftplan.
    Regular(Vec<RecurringShiftSlot>),
    /// An irregular shiftplan.
    Irregular(Vec<IrregularShiftSlot>),
}

/// A single shift plan.
pub struct Shiftplan {
    /// Lower time bound this plan spans
    pub valid_from: NaiveDate,
    /// Upper bound this plan spans
    pub valid_until: NaiveDate,
    /// The type of plan as well as the shift slots themself
    pub plan: ShiftType,
}

/// Trait for all shift slot types.
pub trait ShiftSlot {
    pub fn add_barista(&mut self, barista: Option<String>);
    pub fn is_vacant(&self) -> bool;
    pub fn vacate(&mut self);
}

impl ShiftSlot for RecurringShiftSlot {
    fn add_barista(&mut self, barista: String) {
        self.barista = Some(barista);
    }

    fn is_vacant(&self) -> bool {
        self.barista.is_none()
    }

    fn vacate(&mut self) {
        self.barista = None;
    }
}

impl ShiftSlot for IrregularShiftSlot {
    fn add_barista(&mut self, barista: String) {
        self.barista = Some(barista);
    }

    fn is_vacant(&self) -> bool {
        self.barista.is_none()
    }

    fn vacate(&mut self) {
        self.barista = None;
    }
}
