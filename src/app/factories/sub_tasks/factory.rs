use crate::icon_names;
use adw::prelude::{
	ButtonExt, CheckButtonExt, EditableExt, EntryRowExt, ListBoxRowExt,
	PreferencesRowExt, WidgetExt,
};
use relm4::RelmWidgetExt;
use relm4::{
	FactorySender, adw,
	factory::FactoryView,
	gtk,
	prelude::{DynamicIndex, FactoryComponent},
};

use done_core::models::status::Status;

use super::{
	messages::{SubTaskInput, SubTaskOutput},
	model::{SubTaskInit, SubTaskModel},
};

#[relm4::factory(pub)]
impl FactoryComponent for SubTaskModel {
	type ParentWidget = adw::PreferencesGroup;
	type Input = SubTaskInput;
	type Output = SubTaskOutput;
	type Init = SubTaskInit;
	type CommandOutput = ();
	view! {
		#[root]
		adw::EntryRow {
			set_title: "Sub task",
			set_enable_emoji_completion: true,
			set_show_apply_button: true,
			set_text: self.sub_task.title.as_str(),
			add_prefix = &gtk::CheckButton {
				set_active: self.sub_task.status == Status::Completed,
				connect_toggled[sender, index] => move |checkbox| {
					sender.input(SubTaskInput::SetStatus(index.clone(), checkbox.is_active()));
				}
			},

			add_suffix = &gtk::Button {
				set_valign: gtk::Align::Center,
				set_icon_name: icon_names::ARROW_CIRCULAR_TOP_RIGHT,
				set_css_classes: &["error", "circular"],
				set_tooltip: "remove-sub-task",
				connect_clicked[sender, index] => move |_| {
					sender.input(SubTaskInput::Remove(index.clone()));
				}
			},
			connect_activate[sender, index] => move |entry| {
				let buffer = entry.text().to_string();
				sender.input(SubTaskInput::ModifyTitle(index.clone(), buffer));
			},
			connect_apply[sender, index] => move |entry| {
				let buffer = entry.text().to_string();
				sender.input(SubTaskInput::ModifyTitle(index.clone(), buffer));
			},
		}
	}

	fn init_model(
		init: Self::Init,
		index: &DynamicIndex,
		_sender: FactorySender<Self>,
	) -> Self {
		Self {
			sub_task: init.sub_task,
			index: index.clone(),
		}
	}

	fn init_widgets(
		&mut self,
		index: &DynamicIndex,
		root: Self::Root,
		_returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
		sender: FactorySender<Self>,
	) -> Self::Widgets {
		let widgets = view_output!();
		widgets
	}

	fn update(&mut self, message: Self::Input, sender: FactorySender<Self>) {
		match message {
			SubTaskInput::SetStatus(index, completed) => {
				if completed {
					self.sub_task.status = Status::Completed;
				} else {
					self.sub_task.status = Status::NotStarted;
				}
				sender
					.output(SubTaskOutput::Update(index, self.sub_task.clone()))
					.unwrap_or_default()
			},
			SubTaskInput::ModifyTitle(index, title) => {
				self.sub_task.title = title;
				sender
					.output(SubTaskOutput::Update(index, self.sub_task.clone()))
					.unwrap_or_default()
			},
			SubTaskInput::Remove(index) => sender
				.output(SubTaskOutput::Remove(index))
				.unwrap_or_default(),
		}
	}
}
