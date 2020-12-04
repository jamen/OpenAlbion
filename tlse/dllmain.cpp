#include "pch.h"

bool* GFullscreen = (bool*) 0x137544a;

void init(HINSTANCE hInstance) {
    *GFullscreen = false;

    IDirectInput8* pDirectInput = NULL;

    if (DirectInput8Create(hInstance, DIRECTINPUT_VERSION, IID_IDirectInput8, (LPVOID*)&pDirectInput, NULL) != DI_OK) {
        printf("Hello");
        return;
    }
}

BOOL APIENTRY DllMain(HINSTANCE hInstance, DWORD ul_reason_for_call, LPVOID lpReserved) {
    switch (ul_reason_for_call) {
        case DLL_PROCESS_ATTACH:
            AllocConsole();
            init(hInstance);
            break;
        case DLL_PROCESS_DETACH:
            FreeConsole();
            break;
        default:
            break;
    }

    return TRUE;
}