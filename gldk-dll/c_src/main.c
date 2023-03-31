#include "../include/gl/gldk.h"
#include <GL/GL.h>
#include <stddef.h>

void callback(WindowEvent event) {
    glClearColor(1.0,1.0,1.0,1.0);
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
}

int main() {
    GLConfig config;
    config.version = V4_6;
    GLDKWindow *window = gldkCreateWindow(200,200,"GLDK from C!",config);
    gldkShowWindow(window,callback);
    return 0;
}

