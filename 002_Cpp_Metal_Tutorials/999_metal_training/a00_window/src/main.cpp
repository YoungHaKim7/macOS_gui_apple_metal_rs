/*
 *
 * Copyright 2022 Apple Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include <cassert>
#include <cstdio>

#define NS_PRIVATE_IMPLEMENTATION
#define MTL_PRIVATE_IMPLEMENTATION
#define MTK_PRIVATE_IMPLEMENTATION
#define CA_PRIVATE_IMPLEMENTATION
#include <AppKit/AppKit.hpp>
#include <Metal/Metal.hpp>
#include <MetalKit/MetalKit.hpp>

#pragma region Declarations {

class Renderer {
  public:
    Renderer(MTL::Device *pDevice);
    ~Renderer();
    void draw(MTK::View *pView);

  private:
    MTL::Device *_pDevice;
    MTL::CommandQueue *_pCommandQueue;
};

class MyMTKViewDelegate : public MTK::ViewDelegate {
  public:
    MyMTKViewDelegate(MTL::Device *pDevice);
    virtual ~MyMTKViewDelegate() override;
    virtual void drawInMTKView(MTK::View *pView) override;

  private:
    Renderer *_pRenderer;
};

class MyAppDelegate : public NS::ApplicationDelegate {
  public:
    ~MyAppDelegate();

    NS::Menu *createMenuBar();

    virtual void
    applicationWillFinishLaunching(NS::Notification *pNotification) override;
    virtual void
    applicationDidFinishLaunching(NS::Notification *pNotification) override;
    virtual bool applicationShouldTerminateAfterLastWindowClosed(
        NS::Application *pSender) override;

  private:
    NS::Window *_pWindow;
    MTK::View *_pMtkView;
    MTL::Device *_pDevice;
    MyMTKViewDelegate *_pViewDelegate = nullptr;
};

#pragma endregion Declarations }

int main() {
    NS::AutoreleasePool *pAutoreleasePool =
        NS::AutoreleasePool::alloc()->init();

    MyAppDelegate del;

    NS::Application *pSharedApplication = NS::Application::sharedApplication();
    pSharedApplication->setDelegate(&del);
    pSharedApplication->run();

    pAutoreleasePool->release();

    return 0;
}

#pragma mark - AppDelegate
#pragma region AppDelegate {

MyAppDelegate::~MyAppDelegate() {
    if (_pViewDelegate != nullptr) {
        delete _pViewDelegate;
        _pViewDelegate = nullptr;
    }

    if (_pMtkView != nullptr) {
        _pMtkView->release();
        _pMtkView = nullptr;
    }

    if (_pWindow != nullptr) {
        _pWindow->release();
        _pWindow = nullptr;
    }

    if (_pDevice != nullptr) {
        _pDevice->release();
        _pDevice = nullptr;
    }
}

NS::Menu *MyAppDelegate::createMenuBar() {
    using NS::StringEncoding::UTF8StringEncoding;

    NS::Menu *pMainMenu = NS::Menu::alloc()->init();
    NS::MenuItem *pAppMenuItem = NS::MenuItem::alloc()->init();
    NS::Menu *pAppMenu = NS::Menu::alloc()->init(
        NS::String::string("Appname", UTF8StringEncoding));

    NS::String *appName =
        NS::RunningApplication::currentApplication()->localizedName();
    NS::String *quitItemName = NS::String::string("Quit ", UTF8StringEncoding)
                                   ->stringByAppendingString(appName);
    SEL quitCb = NS::MenuItem::registerActionCallback(
        "appQuit", [](void *, SEL, const NS::Object *pSender) {
            auto pApp = NS::Application::sharedApplication();
            pApp->terminate(pSender);
        });

    NS::MenuItem *pAppQuitItem = pAppMenu->addItem(
        quitItemName, quitCb, NS::String::string("q", UTF8StringEncoding));
    pAppQuitItem->setKeyEquivalentModifierMask(NS::EventModifierFlagCommand);
    pAppMenuItem->setSubmenu(pAppMenu);

    NS::MenuItem *pWindowMenuItem = NS::MenuItem::alloc()->init();
    NS::Menu *pWindowMenu = NS::Menu::alloc()->init(
        NS::String::string("Window", UTF8StringEncoding));

    SEL closeWindowCb = NS::MenuItem::registerActionCallback(
        "windowClose", [](void *, SEL, const NS::Object *) {
            auto pApp = NS::Application::sharedApplication();
            pApp->windows()->object<NS::Window>(0)->close();
        });
    NS::MenuItem *pCloseWindowItem = pWindowMenu->addItem(
        NS::String::string("Close Window", UTF8StringEncoding), closeWindowCb,
        NS::String::string("w", UTF8StringEncoding));
    pCloseWindowItem->setKeyEquivalentModifierMask(
        NS::EventModifierFlagCommand);

    pWindowMenuItem->setSubmenu(pWindowMenu);

    pMainMenu->addItem(pAppMenuItem);
    pMainMenu->addItem(pWindowMenuItem);

    pAppMenuItem->release();
    pWindowMenuItem->release();
    pAppMenu->release();
    pWindowMenu->release();

    return pMainMenu->autorelease();
}

void MyAppDelegate::applicationWillFinishLaunching(
    NS::Notification *pNotification) {
    NS::Menu *pMenu = createMenuBar();
    NS::Application *pApp =
        reinterpret_cast<NS::Application *>(pNotification->object());
    pApp->setMainMenu(pMenu);
    pApp->setActivationPolicy(NS::ActivationPolicy::ActivationPolicyRegular);
}

void MyAppDelegate::applicationDidFinishLaunching(
    NS::Notification *pNotification) {

    // Create the application window.
    CGRect frame{CGPoint{100.0, 100.0}, CGSize{512.0, 512.0}};

    _pWindow = NS::Window::alloc()->init(
        frame, NS::WindowStyleMaskClosable | NS::WindowStyleMaskTitled,
        NS::BackingStoreBuffered, false);

    if (_pWindow == nullptr) {
        return;
    }

    // Create the default Metal device.
    _pDevice = MTL::CreateSystemDefaultDevice();

    if (_pDevice == nullptr) {
        std::fprintf(stderr, "Metal device is not available.\n");
        return;
    }

    // Create the MetalKit view.
    _pMtkView = MTK::View::alloc()->init(frame, _pDevice);

    if (_pMtkView == nullptr) {
        std::fprintf(stderr, "Failed to create MTKView.\n");
        return;
    }

    _pMtkView->setColorPixelFormat(
        MTL::PixelFormat::PixelFormatBGRA8Unorm_sRGB);

    _pMtkView->setClearColor(MTL::ClearColor::Make(1.0, 0.0, 0.0, 1.0));

    // Create the view delegate.
    _pViewDelegate = new MyMTKViewDelegate(_pDevice);

    _pMtkView->setDelegate(_pViewDelegate);

    // Attach the view to the window.
    _pWindow->setContentView(_pMtkView);

    _pWindow->setTitle(NS::String::string(
        "00 - Window", NS::StringEncoding::UTF8StringEncoding));

    _pWindow->makeKeyAndOrderFront(nullptr);

    // Activate the application.
    NS::Application *pApp =
        reinterpret_cast<NS::Application *>(pNotification->object());

    pApp->activateIgnoringOtherApps(true);
}

bool MyAppDelegate::applicationShouldTerminateAfterLastWindowClosed(
    NS::Application *) {
    return true;
}

#pragma endregion AppDelegate }

#pragma mark - ViewDelegate
#pragma region ViewDelegate {

MyMTKViewDelegate::MyMTKViewDelegate(MTL::Device *pDevice)
    : MTK::ViewDelegate(), _pRenderer(new Renderer(pDevice)) {}

MyMTKViewDelegate::~MyMTKViewDelegate() { delete _pRenderer; }

void MyMTKViewDelegate::drawInMTKView(MTK::View *pView) {
    _pRenderer->draw(pView);
}

#pragma endregion ViewDelegate }

#pragma mark - Renderer
#pragma region Renderer {

Renderer::Renderer(MTL::Device *pDevice)
    : _pDevice(pDevice ? pDevice->retain() : nullptr), _pCommandQueue(nullptr) {
    if (_pDevice != nullptr) {
        _pCommandQueue = _pDevice->newCommandQueue();
    }
}

Renderer::~Renderer() {
    if (_pCommandQueue != nullptr) {
        _pCommandQueue->release();
        _pCommandQueue = nullptr;
    }

    if (_pDevice != nullptr) {
        _pDevice->release();
        _pDevice = nullptr;
    }
}

void Renderer::draw(MTK::View *pView) {
    if (pView == nullptr || _pCommandQueue == nullptr) {
        return;
    }

    NS::AutoreleasePool *pPool = NS::AutoreleasePool::alloc()->init();

    // These can be null when the view is not ready to render.
    MTL::RenderPassDescriptor *pRpd = pView->currentRenderPassDescriptor();

    MTL::Drawable *pDrawable = pView->currentDrawable();

    if (pRpd == nullptr || pDrawable == nullptr) {
        pPool->release();
        return;
    }

    MTL::CommandBuffer *pCmd = _pCommandQueue->commandBuffer();

    if (pCmd == nullptr) {
        pPool->release();
        return;
    }

    MTL::RenderCommandEncoder *pEnc = pCmd->renderCommandEncoder(pRpd);

    if (pEnc == nullptr) {
        pPool->release();
        return;
    }

    pEnc->endEncoding();

    pCmd->presentDrawable(pDrawable);
    pCmd->commit();

    pPool->release();
}

#pragma endregion Renderer }
