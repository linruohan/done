use crate::icon_names;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use done_core::models::list::List;

#[derive(Debug, Clone, EnumIter, PartialEq)]
pub enum SidebarList {
	All,
	Today,
	Starred,
	Next7Days,
	Done,
	Custom(List),
}

impl Default for SidebarList {
	fn default() -> Self {
		Self::Custom(List::default())
	}
}

impl SidebarList {
	pub fn list() -> Vec<SidebarList> {
		let mut list: Vec<SidebarList> = SidebarList::iter().collect();
		list.pop();
		list
	}

	pub fn name(&self) -> String {
		let all: String = "all".to_string();
		let today: String = "today".to_string();
		let starred: String = "starred".to_string();
		let next_7_days: String = "next-7-days".to_string();
		let completed_list: String = "completed-list".to_string();
		match self {
			SidebarList::All => all.clone(),
			SidebarList::Today => today.clone(),
			SidebarList::Starred => starred.clone(),
			SidebarList::Next7Days => next_7_days.clone(),
			SidebarList::Done => completed_list.clone(),
			SidebarList::Custom(list) => list.name.clone(),
		}
	}

	pub fn description(&self) -> String {
		let all_desc: String = "all-desc".to_string();
		let today_desc: String = "today-desc".to_string();
		let starred_desc: String = "starred-desc".to_string();
		let next_7_days_desc: String = "next-7-days-desc".to_string();
		let completed_list_desc: String = "completed-list-desc".to_string();
		match self {
			SidebarList::All => all_desc.clone(),
			SidebarList::Today => today_desc.clone(),
			SidebarList::Starred => starred_desc.clone(),
			SidebarList::Next7Days => next_7_days_desc.clone(),
			SidebarList::Done => completed_list_desc.clone(),
			SidebarList::Custom(list) => list.description.clone(),
		}
	}

	pub fn icon(&self) -> Option<&str> {
		match self {
			SidebarList::All => Some(icon_names::CLIPBOARD),
			SidebarList::Today => Some(icon_names::IMAGE_ADJUST_BRIGHTNESS),
			SidebarList::Starred => Some(icon_names::STAR_FILLED_ROUNDED),
			SidebarList::Next7Days => Some(icon_names::WORK_WEEK),
			SidebarList::Done => Some(icon_names::CHECK_ROUND_OUTLINE),
			SidebarList::Custom(list) => list.icon.as_deref(),
		}
	}

	pub fn smart(&self) -> bool {
		!matches!(self, SidebarList::Custom(_))
	}
}
