use std::path::{Path, PathBuf};

pub fn relative(path: &Path, base: &Path) -> Option<PathBuf> {
	use std::path::Component;

	if path.is_absolute() != base.is_absolute() {
		if path.is_absolute() {
			Some(PathBuf::from(path))
		} else {
			None
		}
	} else {
		let mut ita = path.components();
		let mut itb = base.components();
		let mut comps: Vec<Component> = vec![];
		loop {
			match (ita.next(), itb.next()) {
				(None, None) => break,
				(Some(a), None) => {
					comps.push(a);
					comps.extend(ita.by_ref());
					break;
				}
				(None, _) => comps.push(Component::ParentDir),
				(Some(a), Some(b)) if comps.is_empty() && a == b => (),
				(Some(a), Some(b)) if b == Component::CurDir => comps.push(a),
				(Some(_), Some(b)) if b == Component::ParentDir => return None,
				(Some(a), Some(_)) => {
					comps.push(Component::ParentDir);
					for _ in itb {
						comps.push(Component::ParentDir);
					}
					comps.push(a);
					comps.extend(ita.by_ref());
					break;
				}
			}
		}
		Some(comps.iter().map(|c| c.as_os_str()).collect())
	}
}
