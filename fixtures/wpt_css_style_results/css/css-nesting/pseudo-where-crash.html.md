# css/css-nesting/pseudo-where-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-nesting/pseudo-where-crash.html"
}
```

## style[0]

```css

  .foo {
    ::before:where(&) {
      color: red;
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
