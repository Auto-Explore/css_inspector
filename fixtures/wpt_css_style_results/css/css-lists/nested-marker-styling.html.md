# css/css-lists/nested-marker-styling.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/nested-marker-styling.html"
}
```

## style[0]

```css

* {
  font-family: Ahem;
}
div::before {
  content: "before";
  display: list-item;
  list-style-position: inside;
}
div::after {
  content: "after";
  display: list-item;
  list-style-position: inside;
}
div::before::marker {
  content: "before-marker";
}
div::after::marker {
  content: "after-marker";
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
