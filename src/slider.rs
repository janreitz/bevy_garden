use bevy::{
    prelude::*, 
    render::pipeline::RenderPipeline, 
    ui::{
        FocusPolicy, 
        UI_PIPELINE_HANDLE
    }
};

pub struct SliderPlugin;
impl Plugin for SliderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SliderMaterials>()
            .add_system(slider_systen.system());
    }
}


#[derive(Debug, Clone)]
pub struct Slider {
    min: f32,
    max: f32,
    value: f32,
}

impl Default for Slider {
    fn default() -> Slider {
        Slider {
            min: 0.0,
            max: 1.0,
            value: 1.0
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct SliderBundle {
    pub node: Node,
    pub interaction: Interaction,
    pub focus_policy: FocusPolicy,
    pub slider: Slider,
    pub style: Style,
    pub mesh: Handle<Mesh>, // TODO: maybe abstract this out
    pub material_base: Handle<ColorMaterial>,
    pub draw: Draw,
    pub visible: Visible,
    pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}


// The focus system which updates 'Interaction' queries for this components:
// &Node,
// &GlobalTransform,
// Option<&mut Interaction>,
// Option<&FocusPolicy>,
impl Default for SliderBundle {
    fn default() -> Self {
        SliderBundle {
            slider: Slider::default(),
            mesh: bevy::sprite::QUAD_HANDLE.typed(),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                UI_PIPELINE_HANDLE.typed(),
            )]),
            interaction: Default::default(),
            focus_policy: Default::default(),
            node: Default::default(),
            style: Default::default(),
            material_base: Default::default(),
            draw: Default::default(),
            visible: Visible {
                is_transparent: true,
                ..Default::default()
            },
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}

fn button_system(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Mutated<Interaction>, With<Button>, Without<Slider>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.value = "Press".to_string();
                *material = button_materials.pressed.clone();
            }
            Interaction::Hovered => {
                text.value = "Hover".to_string();
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                text.value = "Button".to_string();
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn slider_systen(
    windows: Res<Windows>,
    slider_query: Query<(&Transform, &Slider, &Interaction, &Children)>,
    node_query: Query<&mut Node>,
) {
    let cursor_position = 
    if let Some(cursor_position) = windows
    .get_primary()
    .and_then(|window| window.cursor_position())
    {
        cursor_position
    } else {
        return;
    };

    for (transform, slider, interaction, child) in slider_query.iter() {
        
        
        // if cursor position is within slider region

        // if interaction::pressed

        // Map cursor position to slider range

        // Adjust slider bar to cursor position

    }
}

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}

struct SliderMaterials {
    base: Handle<ColorMaterial>,
    slider: Handle<ColorMaterial>,
    handle: Handle<ColorMaterial>,
}

impl FromResources for SliderMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        SliderMaterials {
            base: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            slider: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            handle: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}