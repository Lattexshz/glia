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
    RedrawRequested,
    Keydown,
    Keyup,
    CloseRequested
} WindowEvent;

typedef struct glconfig {
    GLVersion version;
} GLConfig;

typedef struct gldkwindow {

} GLDKWindow;

GLDKWindow *gldkCreateWindow(unsigned int width,unsigned int height,const char *title,const GLConfig config);

void gldkMakeCurrent(GLDKWindow *window);
void gldkSwapBuffers(GLDKWindow *window);
void *gldkGetProcAddress(GLDKWindow *window,const char *s);

void gldkRunWindow(GLDKWindow *window);
void gldkShowWindow(GLDKWindow *window);
void gldkSetWindowTitle(GLDKWindow *window,const char *title);
void gldkGetWindowSize(GLDKWindow *window,unsigned int *width,unsigned int *height);
void gldkGetWindowPos(GLDKWindow *window,unsigned int *x,unsigned int *y);
void gldkSetUndecoratedWindow(GLDKWindow *window,unsigned char bool);
void gldkHideWindow(GLDKWindow *window);
void gldkSetWindowMinimized(GLDKWindow *window,unsigned char bool);
void gldkSetWindowMaximized(GLDKWindow *window,unsigned char bool);

void gldkSetRedrawRequestedCallback(void (* callback)());
void gldkSetCloseRequestedCallback(void (* callback)());
void gldkSetKeyDownedCallBack(void (* callback)(unsigned int));
void gldkSetKeyUppedCallBack(void (* callback)(unsigned int));