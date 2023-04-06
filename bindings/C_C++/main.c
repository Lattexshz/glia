#include "include/gl/gldk.h"
#include <GL/GL.h>
#include <stdio.h>

GLDKWindow *window;

void callback(WindowEvent event) {
    switch(event) {
        case RedrawRequested:
            glClearColor(1.0,1.0,1.0,1.0);
            glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

            break;
        case Keydown:
            char key = gldkGetLatestDownedKey();
                                    printf("%s",key);
        default:

    }
    gldkSwapBuffers(window);
}

int main() {
    GLConfig config;
    config.version = V4_6;
    window = gldkCreateWindow(200,200,"GLDK from C!",config);
    printf("Create window");
    gldkMakeCurrent(window);
    printf("%s",glGetString(GL_VERSION));
    gldkShowWindow(window);
    gldkRunWindow(window,callback);
    return 0;
}

