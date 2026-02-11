# css/css-masking/clip-path/clip-path-document-element-will-change.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/clip-path-document-element-will-change.html"
}
```

## style[0]

```css

html {
  background: red;
  /* an "L" shape */
  clip-path: polygon(50px 50px, 100px 50px, 100px 100px, 150px 100px, 150px 150px, 50px 150px);
  will-change: transform;
}
div {
  width: 500px;
  height: 500px;
  background: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
