// uncomment this to hide the console
// #![windows_subsystem = "windows"]
pub mod com;
pub mod game_timer;
pub mod events;
pub mod ui;

mod device_resources;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::iter::once;
use std::ptr::null_mut;
use std::io::Error;
use std::mem;
use std::mem::size_of;
use device_resources::DeviceResources;

use winapi::um::libloaderapi::GetModuleHandleW;

use winapi::um::combaseapi::{
    CoInitializeEx,
    COINITBASE_MULTITHREADED
};

use winapi::um::d3d11::{
    D3D11_CLEAR_DEPTH,
    D3D11_CLEAR_STENCIL,
    ID3D11Resource
};

use winapi::shared::windef::{
    HWND,
    HICON,
    RECT
};

use winapi::shared::dxgiformat::{
    DXGI_FORMAT_B8G8R8A8_UNORM
};

use winapi::um::winuser::{
    DefWindowProcW,
    RegisterClassExW,
    CreateWindowExW,
    TranslateMessage,
    DispatchMessageW,
    LoadImageW,
    WNDCLASSEXW,
    CS_HREDRAW,
    CS_VREDRAW,
    WS_OVERLAPPEDWINDOW,
    WS_VISIBLE,
    MSG,
    IMAGE_ICON,
    LR_LOADFROMFILE,
    LR_DEFAULTSIZE,
    LR_SHARED,
    WM_QUIT,
    PeekMessageW,
    PM_REMOVE
};

fn win32_string(value: &str ) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}

struct Window {
    handle: HWND
}

fn create_window(name: &str, title: &str) -> Result<Window, Error> {
    let name = win32_string(name);
    let title = win32_string(title);

    unsafe {
        CoInitializeEx(null_mut(), COINITBASE_MULTITHREADED);

        let hinstance = GetModuleHandleW(null_mut());

        let icon = LoadImageW(
            null_mut(),
            win32_string("Foray.ico").as_ptr(),
            IMAGE_ICON,
            0,
            0,
            LR_LOADFROMFILE | LR_DEFAULTSIZE | LR_SHARED
        );

        let wnd_class = WNDCLASSEXW {
            cbSize : size_of::<WNDCLASSEXW>() as u32,
            hIcon : icon as HICON,
            hIconSm : null_mut(),
            style : CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc : Some( DefWindowProcW ),
            hInstance : hinstance,
            lpszClassName : name.as_ptr(),
            cbClsExtra : 0,
            cbWndExtra : 0,
            hCursor: null_mut(),
            hbrBackground: null_mut(),
            lpszMenuName: null_mut(),
        };

        RegisterClassExW(&wnd_class);

        let client_width = 1400;
        let client_height = 900;

        let rect = RECT { left: 0, top: 0, right: client_width, bottom: client_height };

        let handle = CreateWindowExW(
            0,
            name.as_ptr(),
            title.as_ptr(),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            0,
            0,
            rect.right - rect.left,
            rect.bottom - rect.top,
            null_mut(),
            null_mut(),
            hinstance,
            null_mut() );

        if handle.is_null() {
            Err(Error::last_os_error())
        } else {
            Ok(Window { handle })
        }
    }
}

fn main() {
    let window = create_window( "foray_window", "Foray" ).unwrap();
    let mut dr = DeviceResources::new();
    dr.set_window(window.handle, 1400, 900);
    dr.create_device_resources();
    dr.create_window_size_dependent_resources();

    // should this be a static/global variable somewhere?
    let game_timer = game_timer::GameTimer::new();
    let event_handler = events::EventHandler::new();
    
    unsafe {
        let mut msg: MSG = mem::uninitialized();
        while msg.message != WM_QUIT {
            if PeekMessageW(&mut msg as *mut MSG, null_mut(), 0, 0, PM_REMOVE) != 0 {
                TranslateMessage(&msg as *const MSG);
                DispatchMessageW(&msg as *const MSG);
            }
            else {
                let cornflower_blue = [0.392156899, 0.584313750, 0.929411829, 1.000000000];
                let render_target_view = dr.get_offscreen_render_target_view();
                let d3d_device_context = dr.get_d3d_device_context().as_ref().unwrap();
                d3d_device_context.ClearRenderTargetView(render_target_view, &cornflower_blue);

                let depth_stencil = dr.get_depth_stencil_view();
                d3d_device_context.ClearDepthStencilView(depth_stencil, D3D11_CLEAR_DEPTH | D3D11_CLEAR_STENCIL, 1.0, 0);
                
                d3d_device_context.OMSetRenderTargets(1, &render_target_view, depth_stencil);

                let viewport = dr.get_viewport();
                d3d_device_context.RSSetViewports(1, &viewport);

                let backbuffer_render_target = dr.get_back_buffer_render_target();
                let offscreen_render_target = dr.get_offscreen_render_target();
                d3d_device_context.ResolveSubresource(backbuffer_render_target as *mut ID3D11Resource, 0, offscreen_render_target as *mut ID3D11Resource, 0, DXGI_FORMAT_B8G8R8A8_UNORM);

                dr.present();
            }
        }
    }
}