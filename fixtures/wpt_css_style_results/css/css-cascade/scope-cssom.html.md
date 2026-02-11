# css/css-cascade/scope-cssom.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/scope-cssom.html"
}
```

## style[0]

```css

  @scope {}
  @scope (.a) {}
  @scope (.a) to (.b) {
    div {
      display: block;
    }
  }
  @scope to (.b) {}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
