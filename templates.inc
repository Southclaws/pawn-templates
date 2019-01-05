// built-in include guard removal
// just in case the user has a local dependency with the same file name
#if defined _inc_templates
	#undef _inc_templates
#endif
// custom include-guard to ensure we don't duplicate
#if defined _templates_included
	#endinput
#endif
#define _templates_included


native Template:CreateTemplate(const template[]);
native RenderTemplate(Template:id, dest[]);
