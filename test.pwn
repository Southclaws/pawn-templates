#define RUN_TESTS

#include <a_samp>
#include <YSI\y_testing>

#include "templates.inc"

main() {
    //
}

Test:Simple() {
    new Template:t = CreateTemplate("Hello, {{ name }}!");
    new rendered[64];
    new ret = RenderTemplate(t, rendered);

    printf("ret: %d", ret);
    printf("rendered: %s", rendered);

    ASSERT(ret == 0);
    // ASSERT(!strcmp(rendered, "Hello, Southclaws!"));
}
