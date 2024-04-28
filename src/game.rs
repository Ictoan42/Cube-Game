use std::{sync::Arc, time::{Duration, Instant}};

use cgmath::{Deg, Quaternion, Rotation3, Vector2};
use rand::{thread_rng, Rng};
use rodio::OutputStreamHandle;
use winit::{dpi::{PhysicalPosition, PhysicalSize}, event::{ElementState, MouseButton}, window::Window};

use crate::{config::*, d2::{backgroundmanager::BackgroundManager, mouseutils::{convert_mouse_coords, is_in_rounded_rect}, net::Net, number::Number, rectangle::{depth_sort, Rectangle}, spiral::Spiral, tovertind2d::ToVertInd2D}, d3::columngrid::ColumnGrid, mathsutils::lerp, soundmanager::SoundManager};

pub struct GameState<'a> {
    gpustate: crate::gpustate::State<'a>,
    animstate: AnimState,
    current_column_grid: ColumnGrid,
    current_nets: Vec<Net>,
    current_correct_index: usize,
    background_manager: BackgroundManager,
    // option because the game can run without sound
    sound_manager: Option<SoundManager>,
    current_answer_buttons: Vec<Rectangle>,
    timer_graphic: Spiral,
    counter_graphic: Number,
    best_counter_graphic: Number,
    timer: f32, // frames
    timer_max: f32,
    counter: u32,
    best_counter: u32,
    frame: u64,
    mouse_pos: Vector2<f32>,
    mouse_is_on_button: Option<usize>,
    mouse_was_on_button: Option<usize>,
    mouse_clicked: (bool, bool),
    last_mouse_clicked: (bool, bool),
    last_answer_was_correct: bool,
}

impl<'a> GameState<'a> {
    pub async fn new(window: Arc<Window>, tex_arr: Vec<&[u8]>, sound_arr: Vec<(&'static[u8],f32)>, sound_stream_handle: Option<OutputStreamHandle>) -> Self {
        let gpustate = crate::gpustate::State::new(window, tex_arr, DEFAULT_CLEARCOL).await;

        let background_manager = BackgroundManager::new();

        let sound_manager: Option<SoundManager>;
        if let Some(osh) = sound_stream_handle {
            sound_manager = Some(SoundManager::new(sound_arr, osh));
        } else {
            sound_manager = None;
        }

        let timer_graphic = Spiral::new(
            [0.0,0.35].into(),
			0.0,
			0.5,
			0.45,
			0.5,
			TIMER_TEX_INDEX,
			0.7
		);

		let counter_graphic = Number::new(
		    0,
			[1.8,0.825].into(),
			0.0,
			0,
			0.8,
			0.15,
			0.3,
			0.0,
			true
		);

		let best_counter_graphic = Number::new(
		    0,
			[1.76,0.6].into(),
			0.0,
			0,
			0.5,
			0.075,
			0.15,
			0.0,
			true
		);

		let current_column_grid = gen_cg(COLUMN_GRID_SIDELEN);
		let (
		    current_answer_buttons,
		    current_nets,
		    current_correct_index
		) = gen_next_nets(&current_column_grid);


        let gs = Self {
            gpustate,
            animstate: AnimState::SlidingIn(ANIM_SLIDE_IN_LEN_FRAMES),
            background_manager,
            sound_manager,
            current_column_grid,
            current_nets,
            current_correct_index,
            current_answer_buttons,
            timer_graphic,
            timer: TIMER_DEFAULT_MAX,
            timer_max: TIMER_DEFAULT_MAX,
            counter: 0,
            counter_graphic,
            best_counter: 0,
            best_counter_graphic,
            frame: 0,
            mouse_pos: [100.0;2].into(),
            mouse_clicked: (false, false),
            mouse_is_on_button: None,
            mouse_was_on_button: None,
            last_mouse_clicked: (false, false),
            last_answer_was_correct: false
        };

        gs
    }
    pub fn update(&mut self) {
        // dbg!(self.animstate.clone());

        // stuff that is independent of current animstate
        self.frame += 1;
        self.gpustate.update();
        self.background_manager.update(self.frame);
        self.gpustate.set_bg_col(self.background_manager.current());

        // stuff that is dependent on current animstate
        match self.animstate {
            AnimState::Static(mut should_switch) => {
                // highlight buttons
                self.mouse_is_on_button = None; // code after this will overwrite immediately
                                                // if the mouse is on a button, so this is just
                                                // set as a default
                let mut clicked = -1;
                for (i, b) in self.current_answer_buttons.iter_mut().enumerate() {
                    if is_in_rounded_rect(self.mouse_pos, b, 0.14) {
                        self.mouse_is_on_button = Some(i);
                        if self.mouse_clicked.0 && !self.last_mouse_clicked.0 {
                            clicked = i as i32;
                        } else if self.mouse_clicked.0 {
                            b.set_opacity(0.60);
                        } else {
                            b.set_opacity(0.3);
                        }
                    } else  {
                        b.set_opacity(0.2);
                    }
                }
                if self.mouse_is_on_button != self.mouse_was_on_button && self.mouse_is_on_button.is_some() {
                    self.try_play_sound(0);
                }
                self.mouse_was_on_button = self.mouse_is_on_button;

                // update timer graphic
                self.timer -= 1.0;
                let n = ( self.timer - self.timer_max ) / self.timer_max;
                self.timer_graphic.set_depth(lerp(1.0-0.25, 1.0, -n));
                if self.timer <= 10.0 {
                    // return answer outside of bounds so it's always wrong
                    self.process_answer(NET_COUNT + 1);
                    should_switch = true;
                }

                if clicked >= 0 {
                    self.process_answer(clicked as usize);
                    should_switch = true;
                    // remove hovered effect from any buttons
                    for b in self.current_answer_buttons.iter_mut() {
                        b.set_opacity(0.2);
                    }
                }

                // switch if neccessary
                if should_switch {
                    self.animstate = AnimState::SlidingOut(ANIM_SLIDE_OUT_LEN_FRAMES);
                }
            },
            AnimState::SlidingIn(t) => {
                let n = t / ANIM_SLIDE_IN_LEN_FRAMES;
                self.timer_graphic.set_opacity(lerp(0.0, TIMER_OPACITY_MAX, 1.0-n));
                for net in self.current_nets.iter_mut() {
                    net.set_opacity(
                        lerp(0.0, 1.0, 1.0-n)
                    )
                }

                if self.current_column_grid.pos.x > 0.0 {
                    let offset = ( self.current_column_grid.pos.x+0.2 ) * COLUMN_GRID_SLIDE_MULTIPLIER;
                    self.current_column_grid.translate([-offset,0.0,0.0].into());
                }

                // decrement anim timer and switch if neccessary
                if t != 0.0 {
                    self.animstate = AnimState::SlidingIn(t - 1.0);
                } else {
                    self.animstate = AnimState::Static(false);
                }
            },
            AnimState::SlidingOut(t) => {
                let n = t / ANIM_SLIDE_OUT_LEN_FRAMES;
                self.timer_graphic.set_opacity(lerp(TIMER_OPACITY_MAX, 0.0, 1.0-n));
                for net in self.current_nets.iter_mut() {
                    net.set_opacity(
                        lerp(1.0, 0.0, 1.0-n)
                    )
                }

                if self.current_column_grid.pos.x > -30.0 {
                    let offset = ( self.current_column_grid.pos.x ) * COLUMN_GRID_SLIDE_MULTIPLIER;
                    self.current_column_grid.translate([-(0.2-offset),0.0,0.0].into());
                }

                // decrement anim timer and switch if neccessary
                if t != 0.0 {
                    self.animstate = AnimState::SlidingOut(t - 1.0);
                } else {
                    self.animstate = AnimState::Between(ANIM_BETWEEN_SLIDES_FRAMES);
                }
            },
            AnimState::Between(t) => {
                // only run on first frame of between
                if t == ANIM_BETWEEN_SLIDES_FRAMES {
                    if self.last_answer_was_correct {
                        self.timer_max *= TIMER_REDUCTION_MULTIPLER;
                    } else {
                        self.timer_max = TIMER_DEFAULT_MAX;
                    }
                    self.timer = self.timer_max;
                    let n = ( self.timer - self.timer_max ) / self.timer_max;
                    self.timer_graphic.set_depth(lerp(1.0-0.25, 1.0, -n));

                    self.current_column_grid = gen_cg(COLUMN_GRID_SIDELEN);
                    (
                        self.current_answer_buttons,
                        self.current_nets,
                        self.current_correct_index
                    ) = gen_next_nets(&self.current_column_grid);
                    for net in self.current_nets.iter_mut() {
                        net.set_opacity(0.0)
                    }
                    self.current_column_grid.translate([30.0,0.0,0.0].into());
                }

                // decrement anim timer and switch if neccessary
                if t != 0.0 {
                    self.animstate = AnimState::Between(t - 1.0);
                } else {
                    self.animstate = AnimState::SlidingIn(ANIM_SLIDE_IN_LEN_FRAMES);
                }
            }
        }
    }
    pub fn process_answer(&mut self, ans_index: usize) {
        if ans_index == self.current_correct_index {
            self.answer_correct();
        } else {
            self.answer_incorrect();
        }
        self.animstate = AnimState::Static(true);
    }
    pub fn answer_correct(&mut self) {
        self.try_play_sound(2);
        self.background_manager.set_bg([0.5,1.0,0.5,1.0]);
        self.background_manager.start_anim(DEFAULT_CLEARCOL, 30);
        self.counter += 1;
        self.last_answer_was_correct = true;
        if self.counter > self.best_counter {
            self.best_counter = self.counter;
            self.best_counter_graphic.set(self.best_counter);
        }
        self.counter_graphic.set(self.counter);
    }
    pub fn answer_incorrect(&mut self) {
        self.try_play_sound(3);
        self.background_manager.set_bg([1.0,0.5,0.5,1.0]);
        self.background_manager.start_anim(DEFAULT_CLEARCOL, 30);
        self.last_answer_was_correct = false;
        self.counter = 0;
        self.counter_graphic.set(self.counter);
    }
    pub fn render(&mut self) -> Result<Duration, wgpu::SurfaceError> {
        let start = Instant::now();
        let mut cgs: Vec<ColumnGrid> = vec![];
        cgs.push(self.current_column_grid.clone());

        let nets: Vec<Net> = self.current_nets.clone();

        let mut v: Vec<Box<dyn ToVertInd2D>> = vec![];

        v.extend(nets.iter().map(|n| Box::new(n) as Box<dyn ToVertInd2D> ));
        v.extend(self.current_answer_buttons.iter().map(|n| Box::new(n) as Box<dyn ToVertInd2D> ));
        v.push(Box::new(self.timer_graphic.clone()));
        v.push(Box::new(self.counter_graphic.clone()));
        v.push(Box::new(self.best_counter_graphic.clone()));

        let ds = depth_sort(v);
        
        let end = Instant::now();
        let rt = self.gpustate.render(cgs, ds)?;
        let t = (end - start) + rt;
        Ok(t)
    }
    pub fn print_net_debug(&self) {
        for (i,n) in self.current_nets.iter().enumerate() {
            println!(
                "Net {i}:\n\n{}",
                n.square_debug_info()
            );

            // test for equality against every other net
            let mut matches: String = "Matches with nets: ".into();
            for (i, n2) in self.current_nets.iter().enumerate() {
                if n.is_identical(n2) {matches += &format!("{} ", i)}
            }
            println!("{}\n", matches)
        }
    }
    pub fn window(&self) -> &Window {
        self.gpustate.window()
    }
    pub fn resize_window(&mut self, s: PhysicalSize<u32>) {
        self.gpustate.resize(s);
    }
    pub fn refresh_window(&mut self) {
        self.gpustate.fake_resize();
    }
    pub fn try_play_sound(&mut self, index: u32) {
        if self.sound_manager.is_some() {
            self.sound_manager.as_mut().unwrap().play(index)
        }
    }
    pub fn mouse_pos_update(&mut self, p: PhysicalPosition<f64>) {
        self.mouse_pos = convert_mouse_coords(p, self.gpustate.size, self.gpustate.aspect_uniform.aspect);
    }
    pub fn mouse_click_update(&mut self, state: ElementState, button: MouseButton) {
        match button {
            MouseButton::Left => {
                self.mouse_clicked.0 = match state {
                    ElementState::Pressed => {true}
                    ElementState::Released => {false}
                }
            }
            MouseButton::Right => {
                self.mouse_clicked.1 = match state {
                    ElementState::Pressed => {true}
                    ElementState::Released => {false}
                }
            }
            _ => {}
        }
    }
}

// f32s store how long the animation has left
// bool stores whether an answer has just been given
#[derive(Debug, Clone)]
pub enum AnimState {
    Static(bool), // when waiting for input
    SlidingOut(f32),
    SlidingIn(f32),
    Between(f32) // small buffer between questions
}

pub fn gen_cg(sidelen: u8) -> ColumnGrid {
    loop {
        let next_cg = ColumnGrid::new_random(
            COLUMN_GRID_POS,
            Some(Quaternion::from_angle_y(Deg(135.0))),
            sidelen
        );
        if next_cg.count_occupied_columns() > 5 && next_cg.highest_column() != 1 {
            break next_cg
        }
    }
}

// returns (answer buttons, nets, correct index)
pub fn gen_next_nets(cg: &ColumnGrid) -> (Vec<Rectangle>, Vec<Net>, usize) {

    let mut rng = thread_rng();
    let next_correct = rng.gen_range(0..NET_COUNT);
    let mut next_nets: Vec<Net> = vec![];

    let mut answer_buttons = vec![];

    let correct_net = Net::from_columngrid(
        &cg,
        NET_TEX_INDEX,
        NET_LAYER,
        [0.0;2].into(),
        NET_SCALE,
        NET_EDGE_THICKNESS,
        1.0
    );


    for i in 0..NET_COUNT {
        let mut x = -((NET_COUNT-1) as f32 * NET_GAP) * 0.5;
        x += NET_GAP * i as f32;
        let pos = Vector2::new(x, NET_Y_OFFSET);

        let mut n: Net;
        if i == next_correct {
            n = Net::from_columngrid(
                &cg,
                NET_TEX_INDEX,
                NET_LAYER,
                pos,
                NET_SCALE,
                NET_EDGE_THICKNESS,
                1.0
            );
        } else {
            loop {
                let cg = loop {
                    let mut c = cg.clone();
                    c.random_changes(NET_RANDOM_CHANGES);
                    if c.highest_column() > 1 {
                        break c
                    }                    };
                n = Net::from_columngrid(
                    &cg,
                    NET_TEX_INDEX,
                    NET_LAYER,
                    pos,
                    NET_SCALE,
                    NET_EDGE_THICKNESS,
                    1.0
                );

                let mut usable = true;
                if n.is_identical(&correct_net) {usable = false}
                for n2 in next_nets.iter() {
                    if n2.is_identical(&n) {
                        usable = false
                    }
                }

                if usable {break}
            }
        }
        next_nets.push(n);

        let bg = Rectangle::new(
            0.75,
            0.75,
            pos,
            0.0,
            2,
            BUTTON_TEX_INDEX,
            false,
            0.2
        );
        answer_buttons.push(bg);
    };

    (answer_buttons, next_nets, next_correct)
}
