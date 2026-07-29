/* gl-triangle — a spinning triangle through a merged GL + GLES2 gloam loader.
 *
 * Demonstrates the merged-loader production pattern: one generated loader
 * (and one context struct) serves both desktop OpenGL and OpenGL ES, and a
 * generated EGL loader drives ANGLE backend selection. Three context flavors
 * are supported:
 *
 *   OpenGL 3.3 core       ->  gloamLoadGL(...)
 *   OpenGL ES 3.0 native  ->  gloamLoadGLES2(...)   (WGL/GLX/EGL, OS-provided)
 *   OpenGL ES 3.0 ANGLE   ->  gloamLoadGLES2(...)   (libEGL + ANGLE platform
 *                                                     attributes via SDL's EGL
 *                                                     attribute callbacks and
 *                                                     gloamLoadEGL)
 *
 * With no mode flag the example tries them in that order and uses the first
 * that works. With an explicit mode flag there is no fallback — if that
 * context cannot be created, the example fails:
 *
 *   --gl                   force OpenGL 3.3 core
 *   --es                   force native OpenGL ES 3.0
 *   --use-angle <backend>  force OpenGL ES 3.0 through ANGLE's libEGL;
 *                          backend is one of d3d11 (Windows), metal (macOS),
 *                          vulkan, opengl, opengles. ANGLE's libEGL (and
 *                          libGLESv2) must be findable by the dynamic linker,
 *                          e.g. next to the executable.
 *
 * Either way the same dispatch macros (glCreateShader, glDrawArrays, ...)
 * work afterwards, and the gloam extension flags report what the driver
 * advertises (GL_KHR_debug is wired up to a debug callback when present).
 *
 * Run with --ci to render a single frame headlessly, verify a pixel, and
 * exit; this is how automated environments exercise the example.
 *
 * Exit codes: 0 = pass, 1 = failure, 77 = skipped (no usable GL driver).
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <SDL3/SDL.h>
#include <SDL3/SDL_main.h>

/* egl.h first: on Windows its eglplatform.h pulls in <windows.h>, whose
 * APIENTRY definition gl.h then reuses instead of clashing with it. */
#include <gloam/egl.h>
#include <gloam/gl.h>

#define EXIT_SKIP 77

/* Context flavors, in automatic-fallback order. */
typedef enum {
    TRY_GL,     /* desktop OpenGL 3.3 core profile */
    TRY_ES,     /* native OpenGL ES 3.0 (WGL/GLX/EGL, whatever the OS uses) */
    TRY_ANGLE   /* OpenGL ES 3.0 through libEGL, i.e. ANGLE where present */
} TryKind;

/* SDL_GL_GetProcAddress returns SDL_FunctionPointer; gloam wants a
 * GloamAPIProc-returning callback in the default calling convention.
 * A tiny proxy keeps both type systems honest. */
static GloamAPIProc load_proxy(const char *name)
{
    return (GloamAPIProc)SDL_GL_GetProcAddress(name);
}

static GloamAPIProc egl_load_proxy(const char *name)
{
    return (GloamAPIProc)SDL_EGL_GetProcAddress(name);
}

/* Requested ANGLE backend (EGL_PLATFORM_ANGLE_TYPE_*_ANGLE), or 0 to let
 * ANGLE pick its platform default. Read by platform_attrib_callback, which
 * SDL calls while creating the EGL display. */
static EGLAttrib angle_backend;

static EGLAttrib angle_backend_from_name(const char *name)
{
    if (strcmp(name, "d3d11") == 0)
        return EGL_PLATFORM_ANGLE_TYPE_D3D11_ANGLE;
    if (strcmp(name, "metal") == 0)
        return EGL_PLATFORM_ANGLE_TYPE_METAL_ANGLE;
    if (strcmp(name, "vulkan") == 0)
        return EGL_PLATFORM_ANGLE_TYPE_VULKAN_ANGLE;
    if (strcmp(name, "opengl") == 0)
        return EGL_PLATFORM_ANGLE_TYPE_OPENGL_ANGLE;
    if (strcmp(name, "opengles") == 0)
        return EGL_PLATFORM_ANGLE_TYPE_OPENGLES_ANGLE;
    return 0;
}

/* SDL calls this before eglGetPlatformDisplay; the returned SDL_malloc'd
 * list is appended to the display attributes (returning NULL aborts the
 * window creation). By this point SDL has loaded libEGL, so gloamLoadEGL
 * with EGL_NO_DISPLAY picks up the *client* extensions — enough to know
 * whether this libEGL is ANGLE and which backends it was built with. */
static SDL_EGLAttrib * SDLCALL platform_attrib_callback(void *userdata)
{
    SDL_EGLAttrib *attribs;
    int supported, n = 0;

    (void)userdata;

    if (!gloamLoadEGL(EGL_NO_DISPLAY, egl_load_proxy)) {
        fprintf(stderr, "gl-triangle: failed to load the EGL client API\n");
        return NULL;
    }

    switch (angle_backend) {
    case EGL_PLATFORM_ANGLE_TYPE_D3D11_ANGLE:
        supported = GLOAM_EGL_ANGLE_platform_angle_d3d;
        break;
    case EGL_PLATFORM_ANGLE_TYPE_METAL_ANGLE:
        supported = GLOAM_EGL_ANGLE_platform_angle_metal;
        break;
    case EGL_PLATFORM_ANGLE_TYPE_OPENGL_ANGLE:
    case EGL_PLATFORM_ANGLE_TYPE_OPENGLES_ANGLE:
        supported = GLOAM_EGL_ANGLE_platform_angle_opengl;
        break;
    case EGL_PLATFORM_ANGLE_TYPE_VULKAN_ANGLE:
        supported = GLOAM_EGL_ANGLE_platform_angle_vulkan;
        break;
    default:
        supported = 1; /* no specific backend requested */
        break;
    }
    if (!GLOAM_EGL_ANGLE_platform_angle || !supported) {
        if (angle_backend != 0) {
            fprintf(stderr, "gl-triangle: this libEGL does not support the "
                            "requested ANGLE backend\n");
            return NULL;
        }
        /* Not ANGLE (or no backend preference): pass no extra attributes. */
    }

    attribs = SDL_malloc(4 * sizeof(*attribs));
    if (!attribs)
        return NULL;
    if (GLOAM_EGL_ANGLE_platform_angle && angle_backend != 0) {
        attribs[n++] = EGL_PLATFORM_ANGLE_TYPE_ANGLE;
        attribs[n++] = angle_backend;
    }
    attribs[n++] = EGL_NONE;
    attribs[n] = EGL_NONE;
    return attribs;
}

/* GL_KHR_debug callback: on desktop the entry point is glDebugMessageCallback,
 * on ES it is glDebugMessageCallbackKHR; --alias resolves whichever one the
 * driver exports into both context slots. */
static void APIENTRY debug_callback(GLenum source, GLenum type, GLuint id,
                                    GLenum severity, GLsizei length,
                                    const GLchar *message, const void *user)
{
    (void)source; (void)type; (void)id; (void)length; (void)user;
    if (severity == GL_DEBUG_SEVERITY_HIGH || severity == GL_DEBUG_SEVERITY_MEDIUM)
        fprintf(stderr, "GL debug: %s\n", message);
}

typedef struct {
    SDL_Window *window;
    SDL_GLContext context;
    TryKind kind;
} GLSetup;

static int try_create(GLSetup *out, TryKind kind, EGLAttrib backend, int hidden)
{
    SDL_WindowFlags flags = SDL_WINDOW_OPENGL | SDL_WINDOW_RESIZABLE;
    if (hidden)
        flags |= SDL_WINDOW_HIDDEN;

    SDL_GL_ResetAttributes();
    SDL_GL_SetAttribute(SDL_GL_ACCELERATED_VISUAL, 1);

    if (kind == TRY_GL) {
        SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, SDL_GL_CONTEXT_PROFILE_CORE);
        SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 3);
        SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 3);
    } else {
        SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, SDL_GL_CONTEXT_PROFILE_ES);
        SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 3);
        SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 0);
    }

    if (kind == TRY_ANGLE) {
        /* Route SDL through libEGL instead of the OS-native WGL/GLX path,
         * and inject the ANGLE platform attributes at display creation. */
        angle_backend = backend;
        SDL_SetHint(SDL_HINT_OPENGL_ES_DRIVER, "1");
        SDL_SetHint(SDL_HINT_VIDEO_FORCE_EGL, "1");
        SDL_EGL_SetAttributeCallbacks(platform_attrib_callback, NULL, NULL, NULL);
        SDL_GL_SetAttribute(SDL_GL_EGL_PLATFORM, EGL_PLATFORM_ANGLE_ANGLE);
    } else {
        SDL_SetHint(SDL_HINT_OPENGL_ES_DRIVER, "0");
        SDL_SetHint(SDL_HINT_VIDEO_FORCE_EGL, "0");
        SDL_EGL_SetAttributeCallbacks(NULL, NULL, NULL, NULL);
    }

    out->window = SDL_CreateWindow("gloam gl-triangle", 800, 600, flags);
    if (!out->window)
        return 0;

    out->context = SDL_GL_CreateContext(out->window);
    if (!out->context) {
        SDL_DestroyWindow(out->window);
        out->window = NULL;
        return 0;
    }
    out->kind = kind;
    return 1;
}

static GLuint compile_shader(GLenum kind, const char *version_line, const char *body)
{
    const char *sources[2];
    GLuint shader = glCreateShader(kind);
    GLint ok = 0;

    sources[0] = version_line;
    sources[1] = body;
    glShaderSource(shader, 2, sources, NULL);
    glCompileShader(shader);
    glGetShaderiv(shader, GL_COMPILE_STATUS, &ok);
    if (!ok) {
        char log[1024];
        glGetShaderInfoLog(shader, sizeof(log), NULL, log);
        fprintf(stderr, "shader compile failed:\n%s\n", log);
        return 0;
    }
    return shader;
}

static const char *VS_BODY =
    "in vec2 a_pos;\n"
    "in vec3 a_color;\n"
    "uniform float u_angle;\n"
    "out vec3 v_color;\n"
    "void main() {\n"
    "    float c = cos(u_angle), s = sin(u_angle);\n"
    "    gl_Position = vec4(mat2(c, s, -s, c) * a_pos, 0.0, 1.0);\n"
    "    v_color = a_color;\n"
    "}\n";

static const char *FS_BODY =
    "in vec3 v_color;\n"
    "out vec4 o_frag;\n"
    "void main() { o_frag = vec4(v_color, 1.0); }\n";

/* x, y, r, g, b */
static const float VERTICES[] = {
     0.0f,  0.6f, 1.0f, 0.2f, 0.2f,
    -0.6f, -0.5f, 0.2f, 1.0f, 0.2f,
     0.6f, -0.5f, 0.2f, 0.2f, 1.0f,
};

static int usage(const char *argv0)
{
    fprintf(stderr,
            "usage: %s [--ci] [--gl | --es | --use-angle <backend>]\n"
            "  --ci                   render one frame headlessly, verify a pixel, exit\n"
            "  --gl                   force desktop OpenGL 3.3 core\n"
            "  --es                   force native OpenGL ES 3.0\n"
            "  --use-angle <backend>  force OpenGL ES 3.0 via ANGLE's libEGL;\n"
            "                         backend: d3d11, metal, vulkan, opengl, opengles\n",
            argv0);
    return 1;
}

int main(int argc, char **argv)
{
    int ci = 0, forced = 0, i;
    TryKind kind = TRY_GL;
    const char *backend_name = NULL;
    EGLAttrib backend = 0;
    const char *desc;
    GLSetup gl = { NULL, NULL, TRY_GL };
    const char *vs_version, *fs_version;
    GLuint vs, fs, program, vao, vbo;
    GLint angle_loc, ok = 0;
    int is_es, version;

    for (i = 1; i < argc; ++i) {
        if (strcmp(argv[i], "--ci") == 0) {
            ci = 1;
        } else if (strcmp(argv[i], "--gl") == 0) {
            forced = 1;
            kind = TRY_GL;
        } else if (strcmp(argv[i], "--es") == 0) {
            forced = 1;
            kind = TRY_ES;
        } else if (strcmp(argv[i], "--use-angle") == 0) {
            if (i + 1 >= argc)
                return usage(argv[0]);
            forced = 1;
            kind = TRY_ANGLE;
            backend_name = argv[++i];
        } else {
            return usage(argv[0]);
        }
    }

    if (backend_name) {
        backend = angle_backend_from_name(backend_name);
        if (!backend) {
            fprintf(stderr, "gl-triangle: unknown ANGLE backend \"%s\"\n", backend_name);
            return usage(argv[0]);
        }
    }

    if (!SDL_Init(SDL_INIT_VIDEO)) {
        fprintf(stderr, "gl-triangle: SDL video init failed (%s), skipping\n", SDL_GetError());
        return EXIT_SKIP;
    }

    if (forced) {
        /* An explicit mode was requested: no fallback, failure is failure. */
        desc = kind == TRY_GL ? "OpenGL 3.3 core"
             : kind == TRY_ES ? "native OpenGL ES 3.0"
                              : "OpenGL ES 3.0 via ANGLE";
        if (!try_create(&gl, kind, backend, ci)) {
            fprintf(stderr, "gl-triangle: could not create a %s context (%s)\n",
                    desc, SDL_GetError());
            SDL_Quit();
            return 1;
        }
    } else {
        /* Automatic fallback: desktop core, then native ES, then ANGLE with
         * its default backend — the merged loader handles all of them. */
        if (!try_create(&gl, TRY_GL, 0, ci) &&
            !try_create(&gl, TRY_ES, 0, ci) &&
            !try_create(&gl, TRY_ANGLE, 0, ci)) {
            fprintf(stderr, "gl-triangle: no GL 3.3 core or ES 3.0 context available, skipping\n");
            SDL_Quit();
            return EXIT_SKIP;
        }
    }

    is_es = gl.kind != TRY_GL;
    version = is_es ? gloamLoadGLES2(load_proxy) : gloamLoadGL(load_proxy);
    if (!version) {
        fprintf(stderr, "gl-triangle: gloam failed to load the %s API\n",
                is_es ? "GLES2" : "GL");
        return 1;
    }

    printf("Loaded %s %d.%d (%s)\n", is_es ? "OpenGL ES" : "OpenGL",
           version >> 8, version & 0xff,
           gl.kind == TRY_GL ? "desktop core" :
           gl.kind == TRY_ES ? "native" : "libEGL/ANGLE");
    printf("  GL_RENDERER: %s\n", (const char *)glGetString(GL_RENDERER));
    printf("  GL_VERSION:  %s\n", (const char *)glGetString(GL_VERSION));
    printf("  GL_KHR_debug: %s\n", GLOAM_GL_KHR_debug ? "detected" : "not present");
    printf("  GL_EXT_texture_filter_anisotropic: %s\n",
           GLOAM_GL_EXT_texture_filter_anisotropic ? "detected" : "not present");
    if (gl.kind == TRY_ANGLE)
        printf("  EGL_ANGLE_platform_angle: %s\n",
               GLOAM_EGL_ANGLE_platform_angle ? "detected" : "not present");

    if (GLOAM_GL_KHR_debug && gloam_gl_context.DebugMessageCallback) {
        glDebugMessageCallback(debug_callback, NULL);
        glEnable(is_es ? GL_DEBUG_OUTPUT_KHR : GL_DEBUG_OUTPUT);
    }

    if (is_es) {
        vs_version = "#version 300 es\n";
        fs_version = "#version 300 es\nprecision mediump float;\n";
    } else {
        vs_version = "#version 330 core\n";
        fs_version = "#version 330 core\n";
    }

    vs = compile_shader(GL_VERTEX_SHADER, vs_version, VS_BODY);
    fs = compile_shader(GL_FRAGMENT_SHADER, fs_version, FS_BODY);
    if (!vs || !fs)
        return 1;

    program = glCreateProgram();
    glAttachShader(program, vs);
    glAttachShader(program, fs);
    glBindAttribLocation(program, 0, "a_pos");
    glBindAttribLocation(program, 1, "a_color");
    glLinkProgram(program);
    glGetProgramiv(program, GL_LINK_STATUS, &ok);
    if (!ok) {
        char log[1024];
        glGetProgramInfoLog(program, sizeof(log), NULL, log);
        fprintf(stderr, "program link failed:\n%s\n", log);
        return 1;
    }
    glDeleteShader(vs);
    glDeleteShader(fs);
    angle_loc = glGetUniformLocation(program, "u_angle");

    glGenVertexArrays(1, &vao);
    glBindVertexArray(vao);
    glGenBuffers(1, &vbo);
    glBindBuffer(GL_ARRAY_BUFFER, vbo);
    glBufferData(GL_ARRAY_BUFFER, sizeof(VERTICES), VERTICES, GL_STATIC_DRAW);
    glEnableVertexAttribArray(0);
    glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE, 5 * sizeof(float), (void *)0);
    glEnableVertexAttribArray(1);
    glVertexAttribPointer(1, 3, GL_FLOAT, GL_FALSE, 5 * sizeof(float), (void *)(2 * sizeof(float)));

    SDL_GL_SetSwapInterval(1);

    for (;;) {
        SDL_Event ev;
        int w, h, quit = 0;
        float angle = ci ? 0.0f : (float)((double)SDL_GetTicks() * 0.001);

        while (SDL_PollEvent(&ev)) {
            if (ev.type == SDL_EVENT_QUIT ||
                (ev.type == SDL_EVENT_KEY_DOWN && ev.key.key == SDLK_ESCAPE))
                quit = 1;
        }
        if (quit)
            break;

        SDL_GetWindowSizeInPixels(gl.window, &w, &h);
        glViewport(0, 0, w, h);
        glClearColor(0.0f, 0.0f, 0.0f, 1.0f);
        glClear(GL_COLOR_BUFFER_BIT);

        glUseProgram(program);
        glUniform1f(angle_loc, angle);
        glDrawArrays(GL_TRIANGLES, 0, 3);

        if (ci) {
            /* Verify a pixel inside the triangle before the buffer swap. */
            unsigned char px[4] = { 0, 0, 0, 0 };
            glFinish();
            glReadPixels(w / 2, h / 2, 1, 1, GL_RGBA, GL_UNSIGNED_BYTE, px);
            printf("center pixel: %u %u %u\n", px[0], px[1], px[2]);
            if (px[0] + px[1] + px[2] < 60) {
                fprintf(stderr, "gl-triangle: FAIL — center pixel is background\n");
                return 1;
            }
            printf("gl-triangle: PASS\n");
            break;
        }

        SDL_GL_SwapWindow(gl.window);
    }

    SDL_GL_DestroyContext(gl.context);
    SDL_DestroyWindow(gl.window);
    SDL_Quit();
    return 0;
}
