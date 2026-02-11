# css/css-cascade/scope-inner-pseudo-scope-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/scope-inner-pseudo-scope-crash.html"
}
```

## style[0]

```css

@scope (.a) {
  @scope(:scope) {
    :scope {
      background: green;
    }
  }
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
