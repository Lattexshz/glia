#include "include/gl/gldk.h"
#include <GL/GL.h>
#include <stdio.h>

GLDKWindow *window;

void redraw_requested(WindowEvent event) {
    glClearColor(1.0,1.0,1.0,1.0);
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
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
    gldkSetRedrawRequestedCallback(redraw_requested);
    gldkRunWindow(window);
    return 0;
}

