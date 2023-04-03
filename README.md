# gldk



// Clear The Screen And The Depth Buffer
gl.glClear( GL2.GL_COLOR_BUFFER_BIT | GL2.GL_DEPTH_BUFFER_BIT );
gl.glLoadIdentity(); // Reset The View
gl.glTranslatef( -0.5f, 0.0f, -6.0f ); // Move the triangle
gl.glRotatef( rtri, 0.0f, 1.0f, 0.0f );
gl.glBegin( GL2.GL_TRIANGLES );

      //drawing triangle in all dimensions
      // Front
      gl.glColor3f( 1.0f, 0.0f, 0.0f ); // Red
      gl.glVertex3f( 1.0f, 2.0f, 0.0f ); // Top Of Triangle (Front)
		
      gl.glColor3f( 0.0f, 1.0f, 0.0f ); // Green
      gl.glVertex3f( -1.0f, -1.0f, 1.0f ); // Left Of Triangle (Front)
		
      gl.glColor3f( 0.0f, 0.0f, 1.0f ); // Blue
      gl.glVertex3f( 1.0f, -1.0f, 1.0f ); // Right Of Triangle (Front)
        
      // Right
      gl.glColor3f( 1.0f, 0.0f, 0.0f ); // Red
      gl.glVertex3f( 1.0f, 2.0f, 0.0f ); // Top Of Triangle (Right)
		
      gl.glColor3f( 0.0f, 0.0f, 1.0f ); // Blue
      gl.glVertex3f( 1.0f, -1.0f, 1.0f ); // Left Of Triangle (Right)
		
      gl.glColor3f( 0.0f, 1.0f, 0.0f ); // Green
      gl.glVertex3f( 1.0f, -1.0f, -1.0f ); // Right Of Triangle (Right)
        
      // Left
      gl.glColor3f( 1.0f, 0.0f, 0.0f ); // Red
      gl.glVertex3f( 1.0f, 2.0f, 0.0f ); // Top Of Triangle (Back)
		
      gl.glColor3f( 0.0f, 1.0f, 0.0f ); // Green
      gl.glVertex3f( 1.0f, -1.0f, -1.0f ); // Left Of Triangle (Back)
		
      gl.glColor3f( 0.0f, 0.0f, 1.0f ); // Blue
      gl.glVertex3f( -1.0f, -1.0f, -1.0f ); // Right Of Triangle (Back)
        
      //left
      gl.glColor3f( 0.0f, 1.0f, 0.0f ); // Red
      gl.glVertex3f( 1.0f, 2.0f, 0.0f ); // Top Of Triangle (Left)
		
      gl.glColor3f( 0.0f, 0.0f, 1.0f ); // Blue
      gl.glVertex3f( -1.0f, -1.0f, -1.0f ); // Left Of Triangle (Left)
		
      gl.glColor3f( 0.0f, 1.0f, 0.0f ); // Green
      gl.glVertex3f( -1.0f, -1.0f, 1.0f ); // Right Of Triangle (Left)