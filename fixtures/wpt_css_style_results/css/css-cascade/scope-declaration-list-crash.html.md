# css/css-cascade/scope-declaration-list-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/scope-declaration-list-crash.html"
}
```

## style[0]

```css

  @scope (div) {
    z-index: 1;
  }
  @scope (div) {
    .a;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
