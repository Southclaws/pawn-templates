#define RUN_TESTS

#include <a_samp>
#include <YSI\y_testing>

#include "templates.inc"

main() {
    //
}

enum {
    TYPE_STR = 1,
    TYPE_INT,
    TYPE_FLOAT
}

#define PAIR_STR(%0,%1) TYPE_STR, %0, %1
#define PAIR_INT(%0,%1) (TYPE_INT, %0, %1)
#define PAIR_FLOAT(%0,%1) (TYPE_FLOAT, %0, %1)
#define PAIR_REF(%0,%1) (TYPE_REF, %0, %1)


Test:Simple() {
    new Template:t = CreateTemplate("Hello, {{ name }}! Today is {{ date }}");
    new rendered[64];
    new ret = RenderTemplate(t, rendered, sizeof rendered,
        TYPE_STR, "name", "Southclaws",
        TYPE_STR, "date", "Monday"
    );

    printf("ret: %d", ret);
    printf("rendered: %s", rendered);

    ASSERT(ret == 0);
    // ASSERT(!strcmp(rendered, "Hello, Southclaws!"));
}
