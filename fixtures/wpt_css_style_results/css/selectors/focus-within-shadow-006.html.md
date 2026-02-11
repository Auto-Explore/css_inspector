# css/selectors/focus-within-shadow-006.html

```json
{
  "format_version": 3,
  "file": "css/selectors/focus-within-shadow-006.html"
}
```

## style[0]

```css

  /* Suppress things that cannot be reproduced in the reference file */
  :focus {
    outline: none;
  }
  :focus-within {
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

## style[1]

```css
" +
    "  #shadow-div:focus-within { border: solid 15px green; }" +
    "
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
