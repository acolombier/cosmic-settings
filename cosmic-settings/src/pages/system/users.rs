// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{
    iced::{color, Alignment, ContentFit, Length},
    theme,
    widget::{self, column, icon, settings, text},
    Apply, Element,
};
use cosmic_settings_page::{self as page, section, Section};
use slab::Slab;
use slotmap::SlotMap;
use std::path::PathBuf;

use crate::pages;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct User {
    avatar: PathBuf,
    full_name: String,
    password: String,
    username: String,
    is_admin: bool,
    is_enabled: bool,
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
enum Editing {
    #[default]
    None,
    Username(String),
    FullName(String),
    Password(String),
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
enum Dialog {
    #[default]
    None,
    Authenticate(String),
    AddNewUser(User),
}

#[derive(Default, Clone, Debug)]
pub struct Page {
    entity: page::Entity,
    users: Vec<User>,
    selected_user_idx: Option<usize>,
    editing: Editing,
    dialog: Dialog,
    editing_field: Option<cosmic::widget::Id>,
}

#[derive(Clone, Debug)]
pub enum Message {
    UserList(Vec<User>),
    SelectUser(usize),
    Dialog(Dialog),
    NewUser(User),
    Authenticate(String),
    SelectedUserSetAdmin(bool),
    SelectedUserDelete,
    SelectedUserChangeAvatar,
    Edit(Editing),
    CompleteEdit(bool),
}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(user_list()),
            sections.insert(add_user_button()),
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new("users", "system-users-symbolic")
            .title(fl!("users"))
            .description(fl!("users", "desc"))
    }

    fn dialog(&self) -> Option<Element<pages::Message>> {
        let theme = cosmic::theme::active();
        let theme = theme.cosmic();
        match &self.dialog {
         Dialog::AddNewUser(user) => Some(widget::dialog(fl!("users", "add-user"))
            .control(widget::ListColumn::default()
                .add(
                    widget::container(
                        widget::text_input("", &user.full_name)
                            .label("Full Name")
                            .on_input(|value|crate::pages::Message::User(Message::Dialog(Dialog::AddNewUser(User { full_name: value, ..user.clone() }))))
                    )
                    .padding([0, theme.space_s().into()])
                )
                .add(
                    widget::container(
                    widget::text_input("", &user.full_name)
                        .label("Username")
                        .on_input(|value|crate::pages::Message::User(Message::Dialog(Dialog::AddNewUser(User { username: value, ..user.clone() }))))
                    )
                    .padding([0, theme.space_s().into()])
                )
                .add(
                    widget::container(
                    widget::text_input("", &user.full_name)
                        .label("Password")
                        .on_input(|value|crate::pages::Message::User(Message::Dialog(Dialog::AddNewUser(User { password: value, ..user.clone() }))))
                    )
                    .padding([0, theme.space_s().into()])
                )
                .add(settings::item_row(vec![
                    column::with_capacity(2)
                        .push(text::body("Administrator"))
                        .push(text::caption("Administrators can change settings for all users, add and remove other users.")).into(),
                        widget::horizontal_space().width(Length::Fill).into(),
                    widget::toggler(user.is_admin).on_toggle(|value|crate::pages::Message::User(Message::Dialog(Dialog::AddNewUser(User { is_admin: value, ..user.clone() })))).into(),
                ]))
            )
            .primary_action(
                widget::button::suggested(fl!("users", "add-user"))
                    .on_press(pages::Message::User(Message::NewUser(user.clone()))),
            )
            .secondary_action(
                widget::button::standard(fl!("users", "cancel"))
                    .on_press(pages::Message::User(Message::Dialog(Dialog::None))),
            ).apply(cosmic::Element::from)),
        Dialog::Authenticate(password) => Some(widget::dialog(fl!("dialog", "title"))
            .control(widget::text_input("", password).label("Joe").password().on_input(|value|crate::pages::Message::User(Message::Dialog(Dialog::Authenticate(value)))))
            .primary_action(
                widget::button::suggested("Foo")
                    .on_press(pages::Message::User(Message::Authenticate(password.clone()))),
            )
            .secondary_action(
                widget::button::standard(fl!("users", "cancel"))
                    .on_press(pages::Message::User(Message::Dialog(Dialog::None))),
        ).apply(cosmic::Element::from)),
            _ => None
        }
    }

    fn on_enter(
        &mut self,
        _sender: tokio::sync::mpsc::Sender<crate::pages::Message>,
    ) -> cosmic::Task<crate::pages::Message> {
        cosmic::command::future(async move {
            // let users = Battery::update_battery().await;
            let users = vec![
                User {
                    avatar: PathBuf::from(r"/usr/share/pixmaps/faces/tree.jpg"),
                    full_name: "John Doe".to_owned(),
                    username: "johndoe".to_owned(),
                    is_admin: false,
                    is_enabled: true,
                    password: String::new(),
                },
                User {
                    avatar: PathBuf::from(r"/usr/share/pixmaps/faces/butterfly.png"),
                    full_name: "Bob Ross".to_owned(),
                    username: "bobross".to_owned(),
                    is_admin: true,
                    is_enabled: true,
                    password: String::new(),
                },
                User {
                    avatar: PathBuf::from(r"/usr/share/pixmaps/faces/puppy.jpg"),
                    full_name: "Johnny-Michael Jones".to_owned(),
                    username: "jmjones".to_owned(),
                    is_admin: false,
                    is_enabled: false,
                    password: String::new(),
                },
            ];
            Message::UserList(users)
        })
        .map(crate::pages::Message::User)
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> cosmic::Task<crate::app::Message> {
        match message {
            Message::UserList(users) => {
                self.users = users;
            }
            Message::SelectUser(user_idx) => {
                match self.selected_user_idx {
                    Some(currently_selected_idx) if currently_selected_idx == user_idx => {
                        self.selected_user_idx = None;
                    }
                    _ => {
                        self.selected_user_idx = Some(user_idx);
                    }
                };
                // self.selected_user = if user >= 0 {
                //     self.users.get(user)
                // } else {
                //     None
                // };
            }
            Message::Edit(editing) => {
                self.editing = editing;
                if matches!(self.editing, Editing::None) {
                    self.editing_field = None;
                    return cosmic::Task::none();
                }
                self.editing_field = Some(widget::Id::unique());
                return cosmic::Task::batch(vec![
                    cosmic::widget::text_input::focus(self.editing_field.as_ref().unwrap().clone()),
                    cosmic::widget::text_input::select_all(
                        self.editing_field.as_ref().unwrap().clone(),
                    ),
                ]);
            }
            Message::SelectedUserDelete => {}
            Message::Dialog(dialog) => {
                self.dialog = dialog;
            }
            Message::NewUser(user) => {
                self.dialog = Dialog::None;
                // self.add_new_user = false;
            }
            Message::SelectedUserSetAdmin(_is_admin) => {}
            Message::Authenticate(_password) => {}
            Message::SelectedUserChangeAvatar => {
                // file_chooser::open::Dialog::new()
                //         .title(fl!("wallpaper", "folder-dialog"))
                //         .accept_label(fl!("dialog-add"))
                //         .modal(false)
                //         .open_folder()
                //         .await
                //         .map(|response| response.url().to_owned());
            }
            Message::CompleteEdit(_save) => {
                self.editing = Editing::None;
            }
        };
        cosmic::Task::none()
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

fn user_list() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let user_type_standard = descriptions.insert(fl!("users", "standard"));
    let user_type_admin = descriptions.insert(fl!("users", "admin"));

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            let section = settings::section().title(&section.title);
            let theme = cosmic::theme::active();
            let theme = theme.cosmic();

            page.users
                .iter()
                .enumerate()
                .flat_map(|(idx, user)| {
                    let expanded = matches!(page.selected_user_idx, Some(user_idx) if user_idx == idx);
                    vec![
                        Some(cosmic::iced::widget::MouseArea::new(settings::item_row(
                            vec![
                                widget::row::with_capacity(2)
                                    .push(
                                        widget::container(widget::button::icon(icon::from_path(user.avatar.clone()))
                                                .width(Length::Fixed(32.0))
                                                .height(Length::Fixed(32.0)),
                                            )
                                        .style(|_theme| {
                                            widget::container::Style {
                                                border: cosmic::iced::Border {
                                                    radius: cosmic::iced::Radius::from(32),
                                                    ..Default::default()
                                                },
                                                ..Default::default()
                                            }
                                        })
                                    )
                                    .push(widget::horizontal_space().width(Length::Fixed(theme.space_xxs().into())))
                                    .push(
                                        column::with_capacity(2)
                                            .push(text::body(&user.full_name).class(if expanded { cosmic::theme::Text::Accent } else { cosmic::theme::Text::Default }))
                                            .push(text::caption(if user.is_admin {
                                                &descriptions[user_type_admin]
                                            } else {
                                                &descriptions[user_type_standard]
                                            }).class(if expanded { cosmic::theme::Text::Accent } else { cosmic::theme::Text::Default }))
                                            .height(Length::Fixed(32.0)),
                                    )
                                    .align_y(Alignment::Center)
                                    .spacing(theme.space_xxs())
                                    .into(),
                                widget::horizontal_space().width(Length::Fill).into(),
                                    icon::from_name(if expanded {
                                        "go-up-symbolic"
                                    } else {
                                        "go-next-symbolic"
                                    }).icon().class(if expanded {
                                        theme::Svg::custom(|_theme| widget::svg::Style {
                                            color: Some(color!(0xDEDEDE)),
                                        })
                                    } else {
                                        theme::Svg::Default
                                    },)
                                    .size(16)
                                .into(),
                            ],
                        )).on_press(Message::SelectUser(idx))
                        .apply(cosmic::Element::from)),
                        if expanded { Some(
                        widget::ListColumn::default()
                                .add(settings::item(
                                    "Full name",
                                    widget::row::with_capacity(2)
                                        .push(if let Editing::FullName(full_name) = &page.editing {
                                            widget::text_input::text_input("Type a full name", full_name)
                                                .id(page.editing_field.as_ref().unwrap().clone())
                                                .on_submit(Message::CompleteEdit(true))
                                                .padding([0, theme.space_xxxs()])
                                                .on_input(|value|Message::Edit(Editing::FullName(value)))
                                                .apply(cosmic::Element::from)
                                        }  else {
                                            widget::text(&user.full_name)
                                                .apply(cosmic::Element::from)
                                        })
                                        .push(
                                            widget::button::icon(
                                                icon::from_name(
                                                    if matches!(page.editing, Editing::FullName(..)) {
                                                        "window-close-symbolic"
                                                    }  else {
                                                        "edit-symbolic"
                                                    }
                                                )
                                            ).on_press(if matches!(page.editing, Editing::FullName(..)) {
                                                    Message::CompleteEdit(false)
                                                } else {
                                                    Message::Edit(Editing::FullName(user.full_name.clone()))
                                                })
                                        )
                                        .spacing(theme.space_xxs()).align_y(Alignment::Center),
                                ))
                                .add(settings::item(
                                    "Username",
                                    widget::row::with_capacity(2)
                                        .push(if let Editing::Username(username) = &page.editing {
                                            widget::text_input::text_input("Type an username", username)
                                                .id(page.editing_field.as_ref().unwrap().clone())
                                                .on_submit(Message::CompleteEdit(true))
                                                .padding([0, theme.space_xxxs()])
                                                .on_input(|value|Message::Edit(Editing::Username(value)))
                                                .apply(cosmic::Element::from)
                                        }  else {
                                            widget::text(&user.username)
                                                .apply(cosmic::Element::from)
                                        })
                                        .push(
                                            widget::button::icon(
                                                icon::from_name(
                                                    if matches!(page.editing, Editing::Username(..)) {
                                                        "window-close-symbolic"
                                                    }  else {
                                                        "edit-symbolic"
                                                    }
                                                )
                                            ).on_press(if matches!(page.editing, Editing::Username(..)) {
                                                    Message::CompleteEdit(false)
                                                } else {
                                                    Message::Edit(Editing::Username(user.username.clone()))
                                                })
                                        )
                                        .spacing(theme.space_xxs()).align_y(Alignment::Center),
                                ))
                                .add(settings::item(
                                    "Password",
                                    widget::row::with_capacity(2)
                                        .push(if let Editing::Password(password) = &page.editing {
                                            widget::text_input::text_input("", password)
                                            .padding([theme.space_xxs(), 0])
                                            .width(Length::Fixed(80.0))
                                            .size(14.0)
                                            .style(widget::text_input::Style::Default)
                                            .id(page.editing_field.as_ref().unwrap().clone())
                                            .password()
                                            .on_submit(Message::CompleteEdit(true)).on_input(|value|Message::Edit(Editing::Password(value))).apply(cosmic::Element::from) }  else { widget::text("••••••••••••").apply(cosmic::Element::from) })
                                        .push(widget::button::icon(icon::from_name(if matches!(page.editing, Editing::Password(..)) { "window-close-symbolic" }  else { "edit-symbolic" })).on_press(if  matches!(page.editing, Editing::Password(..)) { Message::CompleteEdit(false) }  else { Message::Edit(Editing::Password(String::new())) }))
                                        .spacing(theme.space_xxs()).align_y(Alignment::Center),
                                ))
                                .add(settings::item_row(vec![
                                    column::with_capacity(2)
                                        .push(text::body("Administrator"))
                                        .push(text::caption("Administrators can change settings for all users, add and remove other users.")).into(),
                                        widget::horizontal_space().width(Length::Fill).into(),
                                    widget::toggler(user.is_admin).on_toggle(Message::SelectedUserSetAdmin).into(),
                                ]))
                                .add(settings::item_row(vec![
                                    widget::horizontal_space().width(Length::Fill).into(),
                                    widget::button::destructive("Remove")
                                        .padding([0, 16])
                                        .on_press(Message::SelectedUserDelete)
                                        .into(),
                                ]))
                                .padding([0, theme.space_xs()])
                                .apply(cosmic::Element::from)) } else { None},
                    ]
                    // .on_press(Message::ShowTrackpadGestureInfo(
                    //     !page.show_trackpad_gesture,
                    // ))
                })
                .flatten()
                .fold(section, settings::Section::add)
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::User)
        })
}

fn add_user_button() -> Section<crate::pages::Message> {
    Section::default().view::<Page>(move |_binder, _page, _section| {
        widget::Column::with_children(vec![
            widget::horizontal_space().width(Length::Fill).into(),
            widget::button::custom(widget::container("Add user").padding([0, 16]))
                .on_press(Message::Dialog(Dialog::AddNewUser(User::default())))
                .into(),
        ])
        .width(Length::Fill)
        .apply(cosmic::Element::from)
        .map(crate::pages::Message::User)
    })
}
