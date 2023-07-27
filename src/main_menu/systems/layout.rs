use bevy::prelude::*;

use crate::main_menu::components::*;
use crate::main_menu::styles::*;

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu (mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single(){
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
    .spawn( (
        NodeBundle {
        style: MAIN_MENU_STYLE,
        background_color: Color::ALICE_BLUE.into(),
        ..default()
       },
     MainMenu {}
    ))
    .with_children(|parent| { 
        //Title
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                size: Size::new(Val::Px(300.0), Val::Px(120.0)),
                ..default()
            },
            ..default()
            }).with_children(|parent|{
                //nogglesimg
                parent.spawn(ImageBundle {
                    style: Style {
                        size: Size::new(Val::Px(64.0), Val::Px(64.0)),
                        margin: UiRect::new(
                            Val::Px(8.0),
                            Val::Px(8.0),
                            Val::Px(8.0),
                            Val::Px(8.0),
                        ),
                        ..default()
                    },
                    image: asset_server.load("sprites/nounmonarca.png").into(),
                    ..default()
                });
                //título
                parent.spawn(TextBundle {
                    text: Text {
                        sections: vec! [TextSection::new(
                            "Catch the Noggles!",
                            get_title_text_style(&asset_server),
                        )],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                });

                //nounimg
                parent.spawn(ImageBundle {
                    style: Style {
                        size: Size::new(Val::Px(96.0), Val::Px(96.0)),
                        margin: UiRect::new(
                            Val::Px(8.0),
                            Val::Px(8.0),
                            Val::Px(8.0),
                            Val::Px(8.0),
                        ),
                        ..default()
                    },
                    image: asset_server.load("sprites/lentesazul.png").into(),
                    ..default()
                });

            });

       
        //PlayButton
        parent.spawn(
         (
            ButtonBundle {
                style: BUTTON_STYLE,
                background_color:NORMAL_BUTTON_COLOR.into(),
                ..default()
            },
            PlayButton {},
        ))
        .with_children(|parent|{
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec! [TextSection::new(
                        "Play",
                        get_button_text_style(&asset_server),
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
        });
         
         //QuitButton
         parent.spawn(
            (
               ButtonBundle {
                   style: BUTTON_STYLE,
                   background_color:NORMAL_BUTTON_COLOR.into(),
                   ..default()
               },
               QuitButton {},
           ))
           .with_children(|parent|{
               parent.spawn(TextBundle {
                   text: Text {
                       sections: vec! [TextSection::new(
                           "Quit",
                           get_button_text_style(&asset_server),
                       )],
                       alignment: TextAlignment::Center,
                       ..default()
                   },
                   ..default()
               });
           });
    })


    .id();



   
    main_menu_entity
}