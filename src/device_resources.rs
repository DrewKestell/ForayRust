use crate::com::ComPtr;
use std::{
    ptr::null_mut,
    mem::zeroed
};
use winapi::{
    ctypes::c_void,
    Interface,
    shared::{
        winerror::{
            DXGI_ERROR_DEVICE_REMOVED,
            DXGI_ERROR_DEVICE_RESET
        },
        dxgi::{
            IDXGIDevice,
            DXGI_SWAP_EFFECT_FLIP_DISCARD,
            IDXGISurface
        },
        dxgi1_2::{
            IDXGIFactory2,
            IDXGISwapChain1,
            DXGI_SWAP_CHAIN_DESC1,
            DXGI_SCALING_STRETCH,
            DXGI_ALPHA_MODE_IGNORE,
            DXGI_SWAP_CHAIN_FULLSCREEN_DESC
            
        },
        dxgi1_3::{
            CreateDXGIFactory2,
            DXGIGetDebugInterface1,
            DXGI_CREATE_FACTORY_DEBUG
        },
        dxgitype::{
            DXGI_USAGE_RENDER_TARGET_OUTPUT,
            DXGI_SAMPLE_DESC,
            DXGI_RATIONAL
        },
        dxgiformat::{
            DXGI_FORMAT_B8G8R8A8_UNORM,
            DXGI_FORMAT_D32_FLOAT
        },
        windef::{
            HWND,
            RECT
        }
    },
    um::{
        d2d1::{
            D2D1_FACTORY_OPTIONS,
            D2D1_DEBUG_LEVEL_INFORMATION,
            D2D1_DEBUG_LEVEL_NONE,
            D2D1CreateFactory,
            D2D1_FACTORY_TYPE_MULTI_THREADED,
            ID2D1Image
        },
        d2d1_1::{
            D2D1_DEVICE_CONTEXT_OPTIONS_ENABLE_MULTITHREADED_OPTIMIZATIONS,
            D2D1_BITMAP_PROPERTIES1,
            D2D1_BITMAP_OPTIONS_TARGET,
            D2D1_BITMAP_OPTIONS_CANNOT_DRAW,
            ID2D1Bitmap1
        },
        d2d1_2::{
            ID2D1Factory2,
            ID2D1Device1,
            ID2D1DeviceContext1
        },
        dcommon::{
            D2D1_ALPHA_MODE_IGNORE,
            D2D1_PIXEL_FORMAT
        },
        d3dcommon::{
            D3D_FEATURE_LEVEL,
            D3D_FEATURE_LEVEL_11_1,
            D3D_FEATURE_LEVEL_11_0,
            D3D_FEATURE_LEVEL_10_1,
            D3D_FEATURE_LEVEL_10_0,
            D3D_FEATURE_LEVEL_9_3,
            D3D_FEATURE_LEVEL_9_2,
            D3D_FEATURE_LEVEL_9_1,
            D3D_DRIVER_TYPE_HARDWARE,
        },
        d3d11::{
            D3D11_CREATE_DEVICE_BGRA_SUPPORT,
            D3D11_CREATE_DEVICE_DEBUG,
            ID3D11Device,
            ID3D11DeviceContext,
            D3D11CreateDevice,
            D3D11_SDK_VERSION,
            ID3D11Texture2D,
            ID3D11RenderTargetView,
            ID3D11Resource,
            D3D11_TEXTURE2D_DESC,
            D3D11_USAGE_DEFAULT,
            D3D11_BIND_DEPTH_STENCIL,
            D3D11_DEPTH_STENCIL_VIEW_DESC,
            D3D11_DSV_DIMENSION_TEXTURE2DMS,
            ID3D11DepthStencilView,
            D3D11_BIND_RENDER_TARGET,
            D3D11_RENDER_TARGET_VIEW_DESC,
            D3D11_RTV_DIMENSION_TEXTURE2DMS,
            D3D11_VIEWPORT
        },
        d3d11_1::{
            ID3D11Device1,
            ID3D11DeviceContext1
        },
        d3d11sdklayers::{
            ID3D11Debug,
            D3D11_RLDO_SUMMARY,
            ID3D11InfoQueue,
            D3D11_MESSAGE_SEVERITY_CORRUPTION,
            D3D11_MESSAGE_SEVERITY_ERROR
        },
        dwrite::{
            DWriteCreateFactory,
            DWRITE_FACTORY_TYPE_SHARED
        },
        dwrite_2::{
            IDWriteFactory2
        },
        dxgidebug::{
            IDXGIInfoQueue,
            DXGI_DEBUG_ALL,
            DXGI_INFO_QUEUE_MESSAGE_SEVERITY_ERROR,
            DXGI_INFO_QUEUE_MESSAGE_SEVERITY_CORRUPTION
        },
        unknwnbase::IUnknown
    }
};

pub struct DeviceResources {
    handle: HWND,
    dxgi_factory: Option<ComPtr<IDXGIFactory2>>,
    d3d_device: Option<ComPtr<ID3D11Device1>>,
    d3d_device_context: Option<ComPtr<ID3D11DeviceContext1>>,
    write_factory: Option<ComPtr<IDWriteFactory2>>,
    d2d_factory: Option<ComPtr<ID2D1Factory2>>,
    d2d_device: Option<ComPtr<ID2D1Device1>>,
    d2d_device_context: Option<ComPtr<ID2D1DeviceContext1>>,
    dxgi_swap_chain: Option<ComPtr<IDXGISwapChain1>>,
    back_buffer_render_target: Option<ComPtr<ID3D11Texture2D>>,
    back_buffer_render_target_view: Option<ComPtr<ID3D11RenderTargetView>>,
    depth_stencil_buffer: Option<ComPtr<ID3D11Texture2D>>,
    depth_stencil_view: Option<ComPtr<ID3D11DepthStencilView>>,
    offscreen_render_target: Option<ComPtr<ID3D11Texture2D>>,
    offscreen_render_target_view: Option<ComPtr<ID3D11RenderTargetView>>,
    target_bitmap: Option<ComPtr<ID2D1Bitmap1>>,
    viewport: D3D11_VIEWPORT,
    output_size: RECT
}

impl DeviceResources {
    pub fn new() -> DeviceResources {
        return DeviceResources {
            handle: null_mut(),
            dxgi_factory: None,
            d3d_device: None,
            d3d_device_context: None,
            write_factory: None,
            d2d_factory: None,
            d2d_device: None,
            d2d_device_context: None,
            dxgi_swap_chain: None,
            back_buffer_render_target: None,
            back_buffer_render_target_view: None,
            depth_stencil_buffer: None,
            depth_stencil_view: None,
            offscreen_render_target: None,
            offscreen_render_target_view: None,
            target_bitmap: None,
            viewport: D3D11_VIEWPORT {
                TopLeftX: 0.0,
                TopLeftY: 0.0,
                Width: 0.0,
                Height: 0.0,
                MinDepth: 0.0,
                MaxDepth: 0.0
            },
            output_size: RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0
            }
        };
    }

    pub fn get_dxgi_factory(&mut self) -> *const IDXGIFactory2 {
        self.dxgi_factory.as_ref().unwrap().as_raw()
    }

    pub fn get_d3d_device(&mut self) -> *const ID3D11Device1 {
        self.d3d_device.as_ref().unwrap().as_raw()
    }

    pub fn get_d3d_device_context(&mut self) -> *const ID3D11DeviceContext1 {
        self.d3d_device_context.as_ref().unwrap().as_raw()
    }

    pub fn get_write_factory(&mut self) -> *const IDWriteFactory2 {
        self.write_factory.as_ref().unwrap().as_raw()
    }

    pub fn get_d2d_factory(&mut self) -> *const ID2D1Factory2 {
        self.d2d_factory.as_ref().unwrap().as_raw()
    }

    pub fn get_d2d_device(&mut self) -> *const ID2D1Device1 {
        self.d2d_device.as_ref().unwrap().as_raw()
    }

    pub fn get_d2d_device_context(&mut self) -> *const ID2D1DeviceContext1 {
        self.d2d_device_context.as_ref().unwrap().as_raw()
    }

    pub fn get_offscreen_render_target_view(&mut self) -> *mut ID3D11RenderTargetView {
        self.offscreen_render_target_view.as_ref().unwrap().as_raw()
    }
    
    pub fn get_depth_stencil_view(&mut self) -> *mut ID3D11DepthStencilView {
        self.depth_stencil_view.as_ref().unwrap().as_raw()
    }

    pub fn get_viewport(&mut self) -> D3D11_VIEWPORT {
        self.viewport
    }

    pub fn get_back_buffer_render_target(&mut self) -> *mut ID3D11Texture2D {
        self.back_buffer_render_target.as_ref().unwrap().as_raw()
    }

    pub fn get_offscreen_render_target(&mut self) -> *mut ID3D11Texture2D {
        self.offscreen_render_target.as_ref().unwrap().as_raw()
    }

    pub fn set_window(&mut self, handle: HWND, width: i32, height: i32) {
        self.handle = handle;
        self.output_size.right = width;
        self.output_size.bottom = height;
    }

    fn create_factory(&mut self) {
        unsafe {
            if cfg!(debug_assertions) {
                // initialize dxgi_info_queue
                let mut dxgi_info_queue: *mut IDXGIInfoQueue = zeroed();
                DXGIGetDebugInterface1(0, &IDXGIInfoQueue::uuidof(), &mut dxgi_info_queue as *mut *mut IDXGIInfoQueue as *mut *mut c_void);
    
                dxgi_info_queue.as_ref().unwrap().SetBreakOnSeverity(DXGI_DEBUG_ALL, DXGI_INFO_QUEUE_MESSAGE_SEVERITY_ERROR, 1);
                dxgi_info_queue.as_ref().unwrap().SetBreakOnSeverity(DXGI_DEBUG_ALL, DXGI_INFO_QUEUE_MESSAGE_SEVERITY_CORRUPTION, 1);
    
                dxgi_info_queue.as_ref().unwrap().Release();
    
                // create dxgi_factory
                let mut dxgi_factory: *mut IDXGIFactory2 = zeroed();
                CreateDXGIFactory2(DXGI_CREATE_FACTORY_DEBUG, &IDXGIFactory2::uuidof(), &mut dxgi_factory as *mut *mut IDXGIFactory2 as *mut *mut c_void);
                self.dxgi_factory = ComPtr::new(dxgi_factory);
            }
            else {
                // create dxgi_factory
                let mut dxgi_factory: *mut IDXGIFactory2 = zeroed();
                CreateDXGIFactory2(0, &IDXGIFactory2::uuidof(), &mut dxgi_factory as *mut *mut IDXGIFactory2 as *mut *mut c_void);
                self.dxgi_factory = ComPtr::new(dxgi_factory);
            }
        }
    }

    pub fn create_device_resources(&mut self) {
        unsafe {
            self.create_factory();

            // create d3d_device and d3d_device_context
            let mut creation_flags: u32 = D3D11_CREATE_DEVICE_BGRA_SUPPORT;

            if cfg!(debug_assertions) {
                creation_flags |= D3D11_CREATE_DEVICE_DEBUG;
            }

            let feature_levels: [D3D_FEATURE_LEVEL; 7] = [
                D3D_FEATURE_LEVEL_11_1,
                D3D_FEATURE_LEVEL_11_0,
                D3D_FEATURE_LEVEL_10_1,
                D3D_FEATURE_LEVEL_10_0,
                D3D_FEATURE_LEVEL_9_3,
                D3D_FEATURE_LEVEL_9_2,
                D3D_FEATURE_LEVEL_9_1
            ];

            let mut d3d_device: *mut ID3D11Device = zeroed();
            let mut d3d_feature_level: D3D_FEATURE_LEVEL = zeroed();
            let mut d3d_device_context: *mut ID3D11DeviceContext = zeroed();

            D3D11CreateDevice(
                null_mut(),
                D3D_DRIVER_TYPE_HARDWARE,
                null_mut(),
                creation_flags,
                feature_levels.as_ptr(),
                1,
                D3D11_SDK_VERSION,
                &mut d3d_device as *mut *mut ID3D11Device,
                &mut d3d_feature_level,
                &mut d3d_device_context as *mut *mut ID3D11DeviceContext );

            let mut d3d_device1: *mut ID3D11Device1 = null_mut();
            d3d_device.as_ref().unwrap().QueryInterface(
                &ID3D11Device1::uuidof(),
                &mut d3d_device1 as *mut *mut ID3D11Device1 as *mut *mut winapi::ctypes::c_void
            );
            self.d3d_device = ComPtr::new(d3d_device1);
            d3d_device.as_ref().unwrap().Release();

            let mut d3d_device_context1: *mut ID3D11DeviceContext1 = null_mut();
            d3d_device_context.as_ref().unwrap().QueryInterface(
                &ID3D11DeviceContext1::uuidof(),
                &mut d3d_device_context1 as *mut *mut ID3D11DeviceContext1 as *mut *mut winapi::ctypes::c_void
            );
            self.d3d_device_context = ComPtr::new(d3d_device_context1);
            d3d_device_context.as_ref().unwrap().Release();
                
            if cfg!(debug_assertions) {
                let d3d_debug = self.d3d_device.as_ref().unwrap().cast::<ID3D11Debug>().unwrap();
                let d3d_info_queue = d3d_debug.cast::<ID3D11InfoQueue>().unwrap().as_raw().as_ref().unwrap();

                d3d_info_queue.SetBreakOnSeverity(D3D11_MESSAGE_SEVERITY_CORRUPTION, 1);
                d3d_info_queue.SetBreakOnSeverity(D3D11_MESSAGE_SEVERITY_ERROR, 1);

                d3d_info_queue.Release();
            }

            // create write_factory
            let mut write_factory: *mut IDWriteFactory2 = zeroed();
            DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED, &IDWriteFactory2::uuidof(), &mut write_factory as *mut *mut IDWriteFactory2 as *mut *mut IUnknown);
            self.write_factory = ComPtr::new(write_factory);

            // create d2d_factory
            let options = D2D1_FACTORY_OPTIONS {
                debugLevel: if cfg!(debug_assertions) { D2D1_DEBUG_LEVEL_INFORMATION } else { D2D1_DEBUG_LEVEL_NONE }
            };
            let mut d2d_factory: *mut ID2D1Factory2 = zeroed();
            D2D1CreateFactory(D2D1_FACTORY_TYPE_MULTI_THREADED, &ID2D1Factory2::uuidof(), &options, &mut d2d_factory as *mut *mut ID2D1Factory2 as *mut *mut c_void);
            
            self.d2d_factory = ComPtr::new(d2d_factory);

            // create d2d_device
            let dxgi_device = self.d3d_device.as_ref().unwrap().cast::<IDXGIDevice>().unwrap().as_raw();

            let mut d2d_device: *mut ID2D1Device1 = zeroed();
            self.d2d_factory.as_ref().unwrap().CreateDevice(dxgi_device, &mut d2d_device as *mut *mut ID2D1Device1);
            self.d2d_device = ComPtr::new(d2d_device);

            dxgi_device.as_ref().unwrap().Release();

            let mut d2d_device_context: *mut ID2D1DeviceContext1 = zeroed();
            self.d2d_device.as_ref().unwrap().CreateDeviceContext(D2D1_DEVICE_CONTEXT_OPTIONS_ENABLE_MULTITHREADED_OPTIMIZATIONS, &mut d2d_device_context as *mut *mut ID2D1DeviceContext1);
            self.d2d_device_context = ComPtr::new(d2d_device_context);
        }
    }

    pub fn create_window_size_dependent_resources(&mut self) {
        unsafe {
            // create dxgi_swap_chain
            // todo: get these from elsewhere
            let render_target_width = 1400;
            let render_target_height = 900;
            let back_buffer_format = DXGI_FORMAT_B8G8R8A8_UNORM;
            let back_buffer_count = 2;

            if self.dxgi_swap_chain == None {
                let swap_chain_desc = DXGI_SWAP_CHAIN_DESC1 {
                    Width: render_target_width,
                    Height: render_target_height,
                    Format: back_buffer_format,
                    BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
                    BufferCount: back_buffer_count,
                    SampleDesc: DXGI_SAMPLE_DESC {
                        Count: 1,
                        Quality: 0
                    },
                    Scaling: DXGI_SCALING_STRETCH,
                    SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
                    AlphaMode: DXGI_ALPHA_MODE_IGNORE,
                    Flags: 0,
                    Stereo: 0
                };
                let swap_chain_fs_desc = DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
                    RefreshRate: DXGI_RATIONAL {
                        Numerator: 0,
                        Denominator: 0
                    },
                    ScanlineOrdering: 0,
                    Scaling: 0,
                    Windowed: 1
                };

                let mut swap_chain: *mut IDXGISwapChain1 = null_mut();
                self.dxgi_factory.as_ref().unwrap().CreateSwapChainForHwnd(
                    self.d3d_device.as_ref().unwrap().as_raw() as *mut IUnknown,
                    self.handle,
                    &swap_chain_desc,
                    &swap_chain_fs_desc,
                    null_mut(),
                    &mut swap_chain as *mut *mut IDXGISwapChain1
                );
                self.dxgi_swap_chain = ComPtr::new(swap_chain);
            }
            else {
                let hr = self.dxgi_swap_chain.as_ref().unwrap().ResizeBuffers(
                    back_buffer_count,
                    render_target_width,
                    render_target_height,
                    back_buffer_format,
                    0
                );

                if hr == DXGI_ERROR_DEVICE_REMOVED || hr == DXGI_ERROR_DEVICE_RESET {
                    self.handle_device_lost();
                    return;
                }
            }

            // create back_buffer_render_target
            let mut back_buffer_render_target: *mut ID3D11Texture2D = null_mut();
            self.dxgi_swap_chain.as_ref().unwrap().GetBuffer(0, &ID3D11Texture2D::uuidof(), &mut back_buffer_render_target as *mut *mut ID3D11Texture2D as *mut *mut c_void);
            self.back_buffer_render_target = ComPtr::new(back_buffer_render_target);

            // create_back_buffer_render_target_view
            let mut back_buffer_render_target_view: *mut ID3D11RenderTargetView = null_mut();
            self.d3d_device.as_ref().unwrap().CreateRenderTargetView(back_buffer_render_target as *mut ID3D11Resource, null_mut(), &mut back_buffer_render_target_view as *mut *mut ID3D11RenderTargetView);
            self.back_buffer_render_target_view = ComPtr::new(back_buffer_render_target_view);

            // create depth_stencil_buffer
            let msaa_count = 8;
            let mut msaa_quality = zeroed();
            self.d3d_device.as_ref().unwrap().CheckMultisampleQualityLevels(DXGI_FORMAT_B8G8R8A8_UNORM, msaa_count, &mut msaa_quality);

            let depth_stencil_desc = D3D11_TEXTURE2D_DESC {
                Format: DXGI_FORMAT_D32_FLOAT,
                Width: render_target_width,
                Height: render_target_height,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: msaa_count,
                    Quality: msaa_quality - 1
                },
                MipLevels: 1,
                ArraySize: 1,
                Usage: D3D11_USAGE_DEFAULT,
                BindFlags: D3D11_BIND_DEPTH_STENCIL,
                CPUAccessFlags: zeroed(),
                MiscFlags: zeroed(),
            };

            let mut depth_stencil_buffer: *mut ID3D11Texture2D = null_mut();
            self.d3d_device.as_ref().unwrap().CreateTexture2D(&depth_stencil_desc, null_mut(), &mut depth_stencil_buffer as *mut *mut ID3D11Texture2D);
            self.depth_stencil_buffer = ComPtr::new(depth_stencil_buffer);

            let depth_stencil_view_desc = D3D11_DEPTH_STENCIL_VIEW_DESC {
                ViewDimension: D3D11_DSV_DIMENSION_TEXTURE2DMS,
                Flags: zeroed(),
                Format: zeroed(),
                u: zeroed()
            };

            // create depth_stencil_view
            let mut depth_stencil_view: *mut ID3D11DepthStencilView = null_mut();
            self.d3d_device.as_ref().unwrap().CreateDepthStencilView(depth_stencil_buffer as *mut ID3D11Resource, &depth_stencil_view_desc, &mut depth_stencil_view as *mut *mut ID3D11DepthStencilView);
            self.depth_stencil_view = ComPtr::new(depth_stencil_view);

            // create offscreen_render_target
            let surface_desc = D3D11_TEXTURE2D_DESC {
                Format: DXGI_FORMAT_B8G8R8A8_UNORM,
                Width: render_target_width,
                Height: render_target_height,
                BindFlags: D3D11_BIND_RENDER_TARGET,
                MipLevels: 1,
                ArraySize: 1,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: msaa_count,
                    Quality: msaa_quality - 1
                },
                CPUAccessFlags: zeroed(),
                MiscFlags: zeroed(),
                Usage: D3D11_USAGE_DEFAULT
            };
            let mut offscreen_render_target: *mut ID3D11Texture2D = null_mut();
            self.d3d_device.as_ref().unwrap().CreateTexture2D(&surface_desc, null_mut(), &mut offscreen_render_target as *mut *mut ID3D11Texture2D);
            self.offscreen_render_target = ComPtr::new(offscreen_render_target);

            // create offscreen_render_target_view
            let render_target_view_desc = D3D11_RENDER_TARGET_VIEW_DESC {
                ViewDimension: D3D11_RTV_DIMENSION_TEXTURE2DMS,
                Format: zeroed(),
                u: zeroed()
            };
            let mut offscreen_render_target_view: *mut ID3D11RenderTargetView = null_mut();
            self.d3d_device.as_ref().unwrap().CreateRenderTargetView(offscreen_render_target as *mut ID3D11Resource, &render_target_view_desc, &mut offscreen_render_target_view as *mut *mut ID3D11RenderTargetView);
            self.offscreen_render_target_view = ComPtr::new(offscreen_render_target_view);

            // set d2d render target
            let bitmap_props = D2D1_BITMAP_PROPERTIES1 {
                pixelFormat: D2D1_PIXEL_FORMAT {
                    format: DXGI_FORMAT_B8G8R8A8_UNORM,
                    alphaMode: D2D1_ALPHA_MODE_IGNORE
                },
                dpiX: 96.0,
                dpiY: 96.0,
                bitmapOptions: D2D1_BITMAP_OPTIONS_TARGET | D2D1_BITMAP_OPTIONS_CANNOT_DRAW,
                colorContext: null_mut()
            };
            let dxgi_surface = self.offscreen_render_target.as_ref().unwrap().cast::<IDXGISurface>().unwrap().as_raw();
            let mut target_bitmap: *mut ID2D1Bitmap1 = null_mut();
            self.d2d_device_context.as_ref().unwrap().CreateBitmapFromDxgiSurface(dxgi_surface, &bitmap_props, &mut target_bitmap as *mut *mut ID2D1Bitmap1);
            self.target_bitmap = ComPtr::new(target_bitmap);
            dxgi_surface.as_ref().unwrap().Release();
            self.d2d_device_context.as_ref().unwrap().SetTarget(target_bitmap as *mut ID2D1Image);

            // initialize viewport
            self.viewport = D3D11_VIEWPORT {
                TopLeftX: 0.0,
                TopLeftY: 0.0,
                Width: render_target_width as f32,
                Height: render_target_height as f32,
                MinDepth: zeroed(),
                MaxDepth: zeroed()
            }
        }
    }

    pub fn handle_device_lost(&mut self) {
        unsafe {
            // TODO: notify game of device lost

            if cfg!(debug_assertions) {
                let d3d_debug = self.d3d_device.as_ref().unwrap().cast::<ID3D11Debug>().unwrap();
                d3d_debug.ReportLiveDeviceObjects(D3D11_RLDO_SUMMARY);
                d3d_debug.Release();
            }

            self.create_device_resources();
            self.create_window_size_dependent_resources();

            // TODO: notify game of device restores
        }
    }

    pub fn present(&mut self) {
        unsafe {
            let hr = self.dxgi_swap_chain.as_ref().unwrap().Present(0, 0);

            self.d3d_device_context.as_ref().unwrap().DiscardView(self.back_buffer_render_target_view.as_ref().unwrap().as_raw() as *mut ID3D11Resource);
            self.d3d_device_context.as_ref().unwrap().DiscardView(self.offscreen_render_target_view.as_ref().unwrap().as_raw() as *mut ID3D11Resource);
            self.d3d_device_context.as_ref().unwrap().DiscardView(self.depth_stencil_view.as_ref().unwrap().as_raw() as *mut ID3D11Resource);

            if hr == DXGI_ERROR_DEVICE_REMOVED || hr == DXGI_ERROR_DEVICE_RESET {
                self.handle_device_lost();
            }
            else {
                if self.dxgi_factory.as_ref().unwrap().IsCurrent() != 1 {
                    self.create_factory();
                }
            }
        }
    }

    pub fn on_window_size_changed(&mut self, width: i32, height: i32) {
        self.output_size.right = width;
        self.output_size.bottom = height;
        self.create_window_size_dependent_resources();
    }
}