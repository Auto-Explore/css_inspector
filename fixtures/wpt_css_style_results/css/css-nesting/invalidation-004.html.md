# css/css-nesting/invalidation-004.html

```json
{
  "format_version": 3,
  "file": "css/css-nesting/invalidation-004.html"
}
```

## style[0]

```css

  .b {
    color: red;
  }
  & {
    @media screen {
      &.b { color: green; }
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
