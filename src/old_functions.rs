use winapi::shared::windef::POINT;
use rand;
const MOVE_SPEED: f32 = 0.1;
const LOOK_SPEED: f32 = 0.1;

fn camera_arrow_movement (camera: &mut CameraStruct) {
    if is_key_down(KeyCode::W) {
        camera.position += camera.front * MOVE_SPEED;
    }
    if is_key_down(KeyCode::S) {
        camera.position -= camera.front * MOVE_SPEED;
    }
    if is_key_down(KeyCode::A) {
        camera.position -= camera.right * MOVE_SPEED;
    }
    if is_key_down(KeyCode::D) {
        camera.position += camera.right * MOVE_SPEED;
    }
    if is_key_down(KeyCode::Space) {
        camera.position[1] += MOVE_SPEED;
    }
    if is_key_down(KeyCode::F) {
        camera.position[1] -= MOVE_SPEED;
    }
}

fn set_mouse_pos(x: i32, y: i32) {
    unsafe {
        winapi::um::winuser::SetCursorPos(x, y);
        // if winapi::um::winuser::SetCursorPos(x, y) == 0 {
        //     Err("SetCursorPos failed".into())
        // } else {
        //     Ok(())
        // }
    }
}

fn get_mouse_pos() -> (i32, i32) {
    let mut point = POINT { x: 0, y: 0 };
    unsafe { ::winapi::um::winuser::GetCursorPos(&mut point as *mut POINT) };
    (point.x, point.y)
}

fn camera_mouse_movement (camera: &mut CameraStruct, delta:f32, default_mouse_position: (i32, i32), world_up:Vec3) {
    
    let mouse_position = get_mouse_pos();
    let mouse_delta:Vec2 = vec2((mouse_position.0 - default_mouse_position.0) as f32,(mouse_position.1 - default_mouse_position.1) as f32);
    set_mouse_pos(default_mouse_position.0,default_mouse_position.1);
    camera.yaw += mouse_delta.x * delta * LOOK_SPEED;
    let mut temp_pitch = camera.pitch + mouse_delta.y * delta * -LOOK_SPEED;

    temp_pitch = if temp_pitch > 1.5 { 1.5 } else { temp_pitch };
    temp_pitch = if temp_pitch < -1.5 { -1.5 } else { temp_pitch };
    camera.pitch = temp_pitch;

    camera.front = get_front(camera.yaw, camera.pitch);
    camera.right = camera.front.cross(world_up).normalize();

}