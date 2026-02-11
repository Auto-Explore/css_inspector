# css/css-cascade/scope-supports.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/scope-supports.html"
}
```

## style[0]

```css

  @scope (.a) {
    @supports (width:0px) {
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
