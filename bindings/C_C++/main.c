#include "include/gl/gldk.h"
#include <GL/GL.h>
#include <stdio.h>

GLDKWindow *window;

void callback(WindowEvent event) {
    glClearColor(1.0,1.0,1.0,1.0);
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
    gldkSwapBuffers(window);
}

int main() {
    GLConfig config;
    config.version = V4_6;
    window = gldkCreateWindow(200,200,"GLDK from C!",config);
    gldkMakeCurrent(window);
    printf("%s",glGetString(GL_VERSION));
    gldkShowWindow(window);
    gldkSetUndecoratedWindow(window,8);
    gldkRunWindow(window,callback);
    return 0;
}

