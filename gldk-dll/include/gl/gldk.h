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
void gldkShowWindow(GLDKWindow *window,void (* callback)(WindowEvent));