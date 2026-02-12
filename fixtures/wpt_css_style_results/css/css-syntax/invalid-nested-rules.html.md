# css/css-syntax/invalid-nested-rules.html

```json
{
  "format_version": 3,
  "file": "css/css-syntax/invalid-nested-rules.html"
}
```

## style[0]

```css

  .a {
    .b <::::invalid::::> {}
    & .c {}
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
