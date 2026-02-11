# css/css-nesting/has-nesting.html

```json
{
  "format_version": 3,
  "file": "css/css-nesting/has-nesting.html"
}
```

## style[0]

```css

ul { background: green }

li:has(strong) {
  display: none;

  :has(> &) {
    background: red;
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
