# css/css-cascade/scope-media.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/scope-media.html"
}
```

## style[0]

```css

  @scope (.a) {
    @media (width > 0px) {
      :scope {
        z-index: 1;
      }

      .b {
        background-color: green;
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
