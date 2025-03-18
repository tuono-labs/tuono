# A Simple Routing Usage Example

This example covers how to use Tuono's route mechanism to access Chuck Norris jokes by category. Sample outputs are as follows.

Index view;

[Runtime 00](./norris_runtime_01.png)

Any category view

[Runtime 01](./norris_runtime_02.png)

## Important Points

When any category name is clicked on the index page, the category/[name] redirection works. For example, if the dev category is clicked, the [name].rs code is interpreted as dev.rs. Similarly, the typescript page is interpreted as dev.tsx.
