#define RUN_TESTS

#include <a_samp>
#include <YSI\y_testing>

#include "templates.inc"

main() {
    //
}

Test:Simple() {
    new Template:t = CreateTemplate("Hello, {{ name }}! Today is {{ date }}");
    new rendered[64];
    new ret = RenderTemplate(t, rendered, sizeof rendered,
        PAIR_STR("name", "Southclaws"),
        PAIR_STR("date", "Monday")
    );

    printf("ret: %d", ret);
    printf("rendered: %s", rendered);
    ASSERT(ret == 0);
    new bool:res = !strcmp(rendered, "Hello, Southclaws! Today is Monday");
    ASSERT(res);
}
