/* gl-triangle — a spinning triangle through a merged GL + GLES2 gloam loader.
 *
 * Demonstrates the merged-loader production pattern: one generated loader
 * (and one context struct) serves both desktop OpenGL and OpenGL ES. The
 * example asks SDL for a desktop core-profile context first and falls back
 * to OpenGL ES (native ES or ANGLE, wherever the system provides one), then
 * loads whichever API it got:
 *
 *   desktop GL 3.3 core  ->  gloamLoadGL(...)
 *   OpenGL ES 3.0        ->  gloamLoadGLES2(...)
 *
 * Either way the same dispatch macros (glCreateShader, glDrawArrays, ...)
 * work afterwards, and the gloam extension flags report what the driver
 * advertises (GL_KHR_debug is wired up to a debug callback when present).
 *
 * Run with --ci to render a single frame headlessly, verify a pixel, and
 * exit; this is how automated environments exercise the example. Run with
 * --es to skip the desktop attempt and force the OpenGL ES path.
 *
 * Exit codes: 0 = pass, 1 = failure, 77 = skipped (no usable GL driver).
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <SDL3/SDL.h>
#include <SDL3/SDL_main.h>

#include <gloam/gl.h>

#define EXIT_SKIP 77

/* SDL_GL_GetProcAddress returns SDL_FunctionPointer; gloam wants a
 * GloamAPIProc-returning callback in the default calling convention.
 * A tiny proxy keeps both type systems honest. */
static GloamAPIProc load_proxy(const char *name)
{
    return (GloamAPIProc)SDL_GL_GetProcAddress(name);
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
    int is_es;
} GLSetup;

static int try_create(GLSetup *out, int es, int hidden)
{
    SDL_WindowFlags flags = SDL_WINDOW_OPENGL | SDL_WINDOW_RESIZABLE;
    if (hidden)
        flags |= SDL_WINDOW_HIDDEN;

    SDL_GL_ResetAttributes();
    SDL_GL_SetAttribute(SDL_GL_ACCELERATED_VISUAL, 1);
    if (es) {
        SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, SDL_GL_CONTEXT_PROFILE_ES);
        SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 3);
        SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 0);
    } else {
        SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, SDL_GL_CONTEXT_PROFILE_CORE);
        SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 3);
        SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 3);
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
    out->is_es = es;
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

int main(int argc, char **argv)
{
    int ci = 0, force_es = 0, i;
    GLSetup gl = { NULL, NULL, 0 };
    const char *vs_version, *fs_version;
    GLuint vs, fs, program, vao, vbo;
    GLint angle_loc, ok = 0;
    int version;

    for (i = 1; i < argc; ++i) {
        if (strcmp(argv[i], "--ci") == 0)
            ci = 1;
        else if (strcmp(argv[i], "--es") == 0)
            force_es = 1;
    }

    if (!SDL_Init(SDL_INIT_VIDEO)) {
        fprintf(stderr, "gl-triangle: SDL video init failed (%s), skipping\n", SDL_GetError());
        return EXIT_SKIP;
    }

    /* Desktop core profile first, ES second — the merged loader handles both. */
    if (!(!force_es && try_create(&gl, 0, ci)) && !try_create(&gl, 1, ci)) {
        fprintf(stderr, "gl-triangle: no GL 3.3 core or ES 3.0 context available, skipping\n");
        SDL_Quit();
        return EXIT_SKIP;
    }

    version = gl.is_es ? gloamLoadGLES2(load_proxy) : gloamLoadGL(load_proxy);
    if (!version) {
        fprintf(stderr, "gl-triangle: gloam failed to load the %s API\n",
                gl.is_es ? "GLES2" : "GL");
        return 1;
    }

    printf("Loaded %s %d.%d\n", gl.is_es ? "OpenGL ES" : "OpenGL",
           version >> 8, version & 0xff);
    printf("  GL_RENDERER: %s\n", (const char *)glGetString(GL_RENDERER));
    printf("  GL_VERSION:  %s\n", (const char *)glGetString(GL_VERSION));
    printf("  GL_KHR_debug: %s\n", GLOAM_GL_KHR_debug ? "detected" : "not present");
    printf("  GL_EXT_texture_filter_anisotropic: %s\n",
           GLOAM_GL_EXT_texture_filter_anisotropic ? "detected" : "not present");

    if (GLOAM_GL_KHR_debug && gloam_gl_context.DebugMessageCallback) {
        glDebugMessageCallback(debug_callback, NULL);
        glEnable(gl.is_es ? GL_DEBUG_OUTPUT_KHR : GL_DEBUG_OUTPUT);
    }

    if (gl.is_es) {
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
