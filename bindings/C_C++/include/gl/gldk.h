typedef enum glversion {
  V3_0,
  V3_1,
  V3_2,
  V3_3,

  V4_0,
  V4_1,
  V4_2,
  V4_3,
  V4_4,
  V4_5,
  V4_6,
} GLVersion;

typedef enum windowevent {
    Update
} WindowEvent;

typedef struct glconfig {
    GLVersion version;
} GLConfig;

typedef struct gldkwindow {

} GLDKWindow;

GLDKWindow *gldkCreateWindow(unsigned int width,unsigned int height,const char *title,const GLConfig config);

void gldkMakeCurrent(GLDKWindow *window);
void gldkSwapBuffers(GLDKWindow *window);

void gldkRunWindow(GLDKWindow *window,void (* callback)(WindowEvent));
void gldkShowWindow(GLDKWindow *window);
void gldkSetWindowTitle(GLDKWindow *window,const char *title);
void gldkSetWindowSize(GLDKWindow *window,unsigned int *width,unsigned int *height);
void gldkSetWindowPos(GLDKWindow *window,unsigned int *x,unsigned int *y);
void gldkSetUndecoratedWindow(GLDKWindow *window,unsigned char bool);
void gldkHideWindow(GLDKWindow *window);