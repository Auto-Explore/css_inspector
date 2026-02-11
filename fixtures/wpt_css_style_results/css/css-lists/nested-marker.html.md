# css/css-lists/nested-marker.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/nested-marker.html"
}
```

## style[0]

```css

  ::marker {
    color: red;
  }
  li::marker {
    color: blue;
  }
  li::before, li::after {
    display: list-item;
    content: "Before";
  }
  li::after {
    content: "After";
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
