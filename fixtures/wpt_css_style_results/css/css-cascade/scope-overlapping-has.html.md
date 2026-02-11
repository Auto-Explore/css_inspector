# css/css-cascade/scope-overlapping-has.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/scope-overlapping-has.html"
}
```

## style[0]

```css

  :where(*) {
    background-color: green;
  }
  @scope (.a) to (.limit) {
    :scope:has(.b:not(:scope .a *)) .hello {
      background-color: red;
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
