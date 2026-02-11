# css/css-pseudo/marker-content-012.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-content-012.html"
}
```

## style[0]

```css

:root {
  --red-image: url('data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" width="32" height="16" style="background: red"></svg>');
  --green-image: url('data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" width="32" height="16" style="background: green"></svg>');
}
ul {
  float: left;
}
.inside {
  list-style-position: inside;
}
.text::marker {
  content: "text";
}
.image::marker {
  content: var(--green-image);
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
