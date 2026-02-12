# css/selectors/invalidation/has-with-nesting-parent-containing-complex.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/has-with-nesting-parent-containing-complex.html"
}
```

## style[0]

```css

  .anchor { background-color: white; }

  .ancestor .descendant {
    .anchor:has(&) { background-color: blue; }
  }

  .ancestor .child {
    .anchor:has(> &) { background-color: lightblue; }
  }

  .ancestor_prev ~ div .descendant {
    .anchor:has(&) { background-color: yellow; }
  }

  .ancestor_prev ~ div.ancestor .descendant {
    .anchor:has(&) { background-color: yellowgreen; }
  }

  .prev ~ .indirect_next {
    .anchor:has(~ &) { background-color: green; }
  }

  .prev ~ .direct_next {
    .anchor:has(+ &) { background-color: lightgreen; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
