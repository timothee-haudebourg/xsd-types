fn check<C: Iterator<Item = u8>>(mut chars: C) -> bool {
	enum State {
		Initial,
		P,
		DuYearMonFragOrDuDayTimeFrag
	}

	let mut state = State::Initial;

	loop {
		state = match state {
			State::Initial => match chars.next() {
				Some(b'-') => State::P,
				Some(b'P') => State::DuYearMonFragOrDuDayTimeFrag,
				_ => todo!()
			},
			State::P => match chars.next() {
				Some(b'P') => State::DuYearMonFragOrDuDayTimeFrag,
				_ => todo!()
			}
			State::DuYearMonFragOrDuDayTimeFrag => match chars.next() {
				Some(b'P') => State::DuYearMonFragOrDuDayTimeFrag,
				_ => todo!()
			}
		}
	}
}