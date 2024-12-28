/*
 * See Licensing and Copyright notice in naev.h
 */
#pragma once

/** @cond */
#include "SDL_endian.h"
#include "SDL_rwops.h"
#include <stdint.h>
/** @endcond */

#include "attributes.h"
#include "colour.h"

/* Recommended for compatibility and such */
#define RMASK SDL_SwapLE32( 0x000000ff ) /**< Red bit mask. */
#define GMASK SDL_SwapLE32( 0x0000ff00 ) /**< Green bit mask. */
#define BMASK SDL_SwapLE32( 0x00ff0000 ) /**< Blue bit mask. */
#define AMASK SDL_SwapLE32( 0xff000000 ) /**< Alpha bit mask. */
#define RGBAMASK RMASK, GMASK, BMASK, AMASK

/*
 * Texture flags.
 */
#define OPENGL_TEX_MAPTRANS ( 1 << 0 ) /**< Create a transparency map. */
#define OPENGL_TEX_MIPMAPS ( 1 << 1 )  /**< Creates mipmaps. */
#define OPENGL_TEX_VFLIP                                                       \
   ( 1 << 2 ) /**< Assume loaded from an image (where positive y means down).  \
               */
#define OPENGL_TEX_SKIPCACHE                                                   \
   ( 1 << 3 ) /**< Skip caching checks and create new texture. */
#define OPENGL_TEX_SDF                                                         \
   ( 1 << 4 ) /**< Convert to an SDF. Only the alpha channel gets used. */
#define OPENGL_TEX_CLAMP_ALPHA                                                 \
   ( 1 << 5 ) /**< Clamp image border to transparency. */
#define OPENGL_TEX_NOTSRGB ( 1 << 6 ) /**< Texture is not in SRGB format. */

struct glTexture;
typedef struct glTexture glTexture;

/*
 * Init/exit.
 */
int  gl_initTextures( void );
void gl_exitTextures( void );

/*
 * Creating.
 */
USE_RESULT glTexture *gl_texExistsOrCreate( const char  *path,
                                            unsigned int flags, int sx, int sy,
                                            int *created );
USE_RESULT glTexture *gl_loadImageData( float *data, int w, int h, int sx,
                                        int sy, const char *name );
USE_RESULT glTexture *gl_newImage( const char *path, const unsigned int flags );
USE_RESULT glTexture                       *
gl_newImageRWops( const char *path, SDL_RWops *rw,
                                        const unsigned int flags ); /* Does not close the RWops. */
USE_RESULT glTexture *gl_newSprite( const char *path, const int sx,
                                    const int sy, const unsigned int flags );
USE_RESULT glTexture *gl_newSpriteRWops( const char *path, SDL_RWops *rw,
                                         const int sx, const int sy,
                                         const unsigned int flags );
USE_RESULT glTexture *gl_dupTexture( const glTexture *texture );
USE_RESULT glTexture *gl_rawTexture( const char *name, GLuint tex, double w,
                                     double h );

/*
 * Clean up.
 */
void gl_freeTexture( glTexture *texture );

/*
 * FBO stuff.
 */
int gl_fboCreate( GLuint *fbo, GLuint *tex, GLsizei width, GLsizei height );
int gl_fboAddDepth( GLuint fbo, GLuint *tex, GLsizei width, GLsizei height );

/*
 * Misc.
 */
void        gl_contextSet( void );
void        gl_contextUnset( void );
int         gl_isTrans( const glTexture *t, const int x, const int y );
void        gl_getSpriteFromDir( int *x, int *y, int sx, int sy, double dir );
glTexture **gl_copyTexArray( glTexture **tex );
glTexture **gl_addTexArray( glTexture **tex, glTexture *t );

/* Transition getters. */
const char  *tex_name( const glTexture *tex );
double       tex_w( const glTexture *tex );
double       tex_h( const glTexture *tex );
double       tex_sw( const glTexture *tex );
double       tex_sh( const glTexture *tex );
double       tex_sx( const glTexture *tex );
double       tex_sy( const glTexture *tex );
double       tex_srw( const glTexture *tex );
double       tex_srh( const glTexture *tex );
int          tex_isSDF( const glTexture *tex );
int          tex_hasTrans( const glTexture *tex );
GLuint       tex_tex( const glTexture *tex );
unsigned int tex_flags( const glTexture *tex );
double       tex_vmax( const glTexture *tex );
void         tex_setTex( glTexture *tex, GLuint texture );
void         tex_setVFLIP( glTexture *tex, int flip );
