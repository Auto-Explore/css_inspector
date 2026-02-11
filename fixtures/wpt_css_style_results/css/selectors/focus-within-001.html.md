# css/selectors/focus-within-001.html

```json
{
  "format_version": 3,
  "file": "css/selectors/focus-within-001.html"
}
```

## style[0]

```css

/* Suppress things that cannot be reproduced in the reference file */
:focus {
  outline: none;
}

/* Use blue to indicate that the user needs to pay attention.
   This element needs to be focused for the test to make sense. */
div {
border: solid 15px blue;
}
div:focus {
border-color: red;
}
div:focus-within {
border-color: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
