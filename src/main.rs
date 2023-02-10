
use macroquad::prelude::*;
// use glam::vec3;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Macroquad"),
        // window_width: 1260,
        // window_height: 768,
        fullscreen: true,
        ..Default::default()
    }
}
const COLOR_LIST: [Color; 10] = [
ORANGE	,
PINK	,
BLUE	,
GREEN	,
GOLD	,
LIME	,
MAGENTA	,
PURPLE	,
RED	,
YELLOW];





struct CameraStruct {
    yaw:f32,
    pitch:f32,
    right: Vec3,
    front: Vec3,
    position: Vec3
}

struct CubeStruct {
    para: BasePara,
    speed: f32,
    radius: f32,
    color: Color
}

struct PlayerStruct {
    para: BasePara,
    speed: f32,
    color: Color
}

fn get_front (yaw:f32, pitch:f32) -> Vec3 {
    vec3(
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos(),
    )
}

const BASE_SPEED: f32 = 3.;

fn player_movement (player: &mut PlayerStruct, delta:f32) {
    if is_key_down(KeyCode::Up) {
        player.para.y += delta * player.speed;
    }
    if is_key_down(KeyCode::Down) {
        player.para.y -= delta * player.speed;
    }
    if is_key_down(KeyCode::Right) {
        player.para.x -= delta * player.speed;
    }
    if is_key_down(KeyCode::Left) {
        player.para.x += delta * player.speed;
    }
    if player.para.x > 9.8 {player.para.x = 9.8}
    if player.para.x < -9.8 {player.para.x = -9.8}
    if player.para.y > 9.8 {player.para.y = 9.8}
    if player.para.y < -9.8 {player.para.y = -9.8}
}



fn screen_space (laps:i32) {
    set_default_camera();
    draw_text(format!("Level: {} ", laps).as_str(), 10.0, 20.0, 30.0, BLACK);
}

fn camera_space (camera: &CameraStruct) {
    set_camera(&Camera3D {
        position: camera.position,
        up: camera.right.cross(camera.front).normalize(),
        target:camera.position + camera.front,
        ..Default::default()
    });
    // draw_cube(vec3(0., 0., 0.), vec3(20., 0., 20.), None, WHITE);
    draw_grid(20, 1., BLACK, GRAY);
    draw_cube_wires(vec3(0., 0., 0.), vec3(20., 0., 20.), BLACK)
}

fn get_new_color (level:i32,) -> Color {
    COLOR_LIST[ ((level-1) % 10) as usize]
}

impl CubeStruct {
    pub fn new (x:f32, y:f32)->Self{
      let radius = if x.abs()>y.abs() {x.abs()} else {y.abs()};
        CubeStruct{
            para: BasePara::new(x, y, 1., 1., None, None),
            radius,
            speed: BASE_SPEED,
            color: get_new_color(1),
        }
    }
    pub fn new_border_cube (x:f32, y:f32,color:Color)->Self{
        let radius = if x.abs()>y.abs() {x.abs()} else {y.abs()};
        CubeStruct{
            para: BasePara::new(x, y, 1., 1., Some(0.75), Some(1.5)),
            radius,
            speed: 0.,
            color,
        }
    }
    pub fn move_cube (&mut self, delta:f32) {
        // inbound x 
        let movement = self.speed * delta;
        if self.para.y == -self.radius && self.para.x != self.radius {
            self.para.x += movement;
        } else if self.para.x == self.radius && self.para.y != self.radius {
            self.para.y += movement;
        } else if self.para.y == self.radius && self.para.x != -self.radius {
            self.para.x -= movement;
        } else if self.para.x == -self.radius {
            self.para.y -= movement;
        }
        
      

        if self.para.x > self.radius {self.para.x = self.radius}
        if self.para.x < -self.radius {self.para.x = -self.radius}
        if self.para.y > self.radius {self.para.y = self.radius}
        if self.para.y < -self.radius {self.para.y = -self.radius}
    }
}

#[derive(Clone)]
struct BasePara {
    x:f32,
    y:f32,
    x_width:f32,
    y_width:f32,
    z_width: f32,
    z: f32,
}







impl BasePara {
    pub fn new(x:f32, y:f32,x_width:f32, y_width: f32, z:Option<f32>, z_width:Option<f32>)->BasePara{
        BasePara {
            x,
            y,
            x_width,
            y_width,
            z: z.unwrap_or(0.5),
            z_width: z_width.unwrap_or(1.),
        }
    }
    pub fn paint(&self,color:Color, draw_wires:bool){
        draw_cube(vec3(self.x, self.z, self.y), vec3(self.x_width, self.z_width, self.y_width), None,color);
        if draw_wires {
            draw_cube_wires(vec3(self.x, self.z, self.y), vec3(self.x_width, self.z_width, self.y_width), BLACK);
        }
    }
    pub fn max_x (&self)->f32 {self.x+self.x_width/2.}
    pub fn max_y (&self)->f32 {self.y+self.y_width/2.}
    pub fn min_x (&self)->f32 {self.x-self.x_width/2.}
    pub fn min_y (&self)->f32 {self.y-self.y_width/2.}
}



fn do_intersect (first: &BasePara, second: &BasePara) -> bool {
    let x_overlap = first.max_x() > second.min_x() && first.min_x() < second.max_x();
    let y_overlap = first.max_y() > second.min_y() && first.min_y() < second.max_y();
    if x_overlap && y_overlap {return true};
    return false;
}


#[macroquad::main(conf)]
async fn main() {


    let world_up = vec3(0.0, 1.0, 0.0);
    let yaw = -17.27f32;
    let pitch = -0.82f32;
    let camera = CameraStruct {
        yaw: yaw,
        pitch: pitch,
        front: get_front(yaw, pitch),
        right: get_front(yaw, pitch).cross(world_up).normalize(),
        position: vec3(0.0, 15., -14.),
    };
   
    let end_zone = BasePara::new(7.5, -7.5, 5., 5., Some(0.), Some(0.));
    let player_starting_para = BasePara::new(0., 0., 0.2, 0.2, None, None);
    let mut player = PlayerStruct {
        para: player_starting_para.clone(), speed: 7., color: YELLOW
    };
    // set_mouse_pos((screen_width()/2.) as i32 , (screen_height() / 2.) as i32);
    // let default_mouse_position = get_mouse_pos();

    let mut level:i32 = 1;
    
    let mut move_cube_vec:[CubeStruct;15] = [
        // 9.5
        CubeStruct::new(9.5,-9.5),
        CubeStruct::new(-9.5,9.5),
        CubeStruct::new(0.5,-9.5),
        // 8.5
        CubeStruct::new(8.5,8.5),
        CubeStruct::new(-8.5,-8.5),
        CubeStruct::new(8.5,-8.5),
        // 7.5
        CubeStruct::new(0.5,7.5),
        CubeStruct::new(0.5,-7.5),
        CubeStruct::new(7.5,0.5),
        // 6.5
        CubeStruct::new(0.5,-6.5),
        CubeStruct::new(-6.5,0.5),
        CubeStruct::new(-6.5,6.5),
        // 5.5
        CubeStruct::new(-5.5,-5.5),
        CubeStruct::new(5.5,-5.5),
        CubeStruct::new(5.5,0.5),        
    ];
    let mut static_cube_vec:Vec<CubeStruct> = vec![];
    for i in 0..10 {
        let new_cube = CubeStruct::new_border_cube(4.5 - i as f32, 4.5, BLACK);
        static_cube_vec.push(new_cube);
    }
    for i in 0..5 {
        let new_cube = CubeStruct::new_border_cube(0.5 - i as f32, -4.5,BLACK);
        static_cube_vec.push(new_cube);
    }
    for i in 0..15 {
        let new_cube = CubeStruct::new_border_cube(4.5, 4.5 - i as f32,BLACK);
        static_cube_vec.push(new_cube);
    }
    for i in 0..10 {
        let new_cube = CubeStruct::new_border_cube(-4.5, 4.5 - i as f32,BLACK);
        static_cube_vec.push(new_cube);
    }
    let mut old_player_para: BasePara =player.para.clone();

    loop {
        let delta = get_frame_time();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        for cube in move_cube_vec.iter_mut() {cube.move_cube(delta)}
        player_movement(&mut player, delta);
        
        // camera_arrow_movement(&mut camera);
        // camera_mouse_movement(&mut camera, delta, default_mouse_position, world_up);

        for (_, cube) in static_cube_vec.iter().enumerate() {
            let intersected = do_intersect(&player.para, &cube.para);
            if intersected {
                player.para = old_player_para.clone();
                break;
            }
        }

        old_player_para = player.para.clone();
        for (_, cube) in move_cube_vec.iter().enumerate() {
            let intersected = do_intersect(&player.para, &cube.para);
            if intersected {
                player.para = player_starting_para.clone();
                break;
            }
        }
        let intersected = do_intersect(&player.para, &end_zone);
        if intersected {
            player.para = player_starting_para.clone();
            level += 1;
            for i in (((level - 2) % 5 *3))..(((level - 2) % 5 *3)+3) {
                move_cube_vec[i as usize].speed = BASE_SPEED + level as f32 * 0.3;
                move_cube_vec[i as usize].color = get_new_color(level);
            }
            // for cube in move_cube_vec.iter_mut() {
            //     let change = rand::gen_range(0, 10);
            //     if change > 7 {
            //         cube.speed = level as f32 + (level-1) as f32 * 0.2;
            //         cube.color = get_new_color(level);
            //     }
            // }
        }
        clear_background(LIGHTGRAY);
        camera_space(&camera);
        let radius = if player.para.x.abs() > player.para.y.abs() {player.para.x.abs()}  else {player.para.y.abs() }; 
        draw_cube_wires(vec3(0., 0., 0.), vec3(radius*2., 0., radius*2.), YELLOW);
        end_zone.paint(GREEN, true);
       
        for cube in move_cube_vec.iter() {
            cube.para.paint(cube.color, true);
        }
        for cube in static_cube_vec.iter() {
            cube.para.paint(cube.color, false);
        }
        
        player.para.paint(player.color, true);
        screen_space(level);
        next_frame().await
    }
}
            
            