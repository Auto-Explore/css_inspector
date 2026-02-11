# css/css-lists/nested-marker-dynamic.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/nested-marker-dynamic.html"
}
```

## style[0]

```css

  li, ::marker {
    color: red;
  }
  li::before, li::after {
    display: list-item;
    content: "Before";
  }
  li::after {
    content: "After";
  }
  .tweak::marker {
    color: blue;
  }
  .tweak, .tweak::before, .tweak::after {
    color: initial;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
