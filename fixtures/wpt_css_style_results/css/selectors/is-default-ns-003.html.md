# css/selectors/is-default-ns-003.html

```json
{
  "format_version": 3,
  "file": "css/selectors/is-default-ns-003.html"
}
```

## style[0]

```css

@namespace "http://www.w3.org/1999/xhtml";

*|*.a {
  display:none;
}

/* This should not apply, since the '.container' compound is affected by the
   default namespace (even though the '.a' compound is not). */
*|*:is(.container .a) {
  display:initial;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
