# css/selectors/focus-within-010.html

```json
{
  "format_version": 3,
  "file": "css/selectors/focus-within-010.html"
}
```

## style[0]

```css

/* Suppress things that cannot be reproduced in the reference file */
:focus {
  outline: none;
}

#target {
  display: none;
}

#wrapper:focus-within > #target {
  display: block;
}

#target:focus {
  border: solid 15px green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
