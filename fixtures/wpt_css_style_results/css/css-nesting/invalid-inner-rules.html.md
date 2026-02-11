# css/css-nesting/invalid-inner-rules.html

```json
{
  "format_version": 3,
  "file": "css/css-nesting/invalid-inner-rules.html"
}
```

## style[0]

```css

div {
  /* This is not a conditional rule, and thus cannot be in nesting context. */
  @font-face {
    &.a { font-size: 10px; }
  }

  @media screen {
    &.a { color: red; }

    /* Same. */
    @font-face {
      &.a { font-size: 10px; }
    }
  }
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
