# css/css-backgrounds/background-clip/clip-border-area-on-body-not-propagated-to-root.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-clip/clip-border-area-on-body-not-propagated-to-root.html"
}
```

## style[0]

```css

html, body {
  box-sizing: border-box;
  height: 100%;
  margin: 0;
}
html {
  background-color: white;
}
body {
  border: 20px solid transparent;
  background-color: green;
  background-clip: border-area;
  padding: 10px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
