# css/css-position/sticky/position-sticky-overflow-clip-container.html

```json
{
  "format_version": 3,
  "file": "css/css-position/sticky/position-sticky-overflow-clip-container.html"
}
```

## style[0]

```css

body {
  margin: 0;
  overflow: hidden; /* hide scrollbars */
}

#container {
  height: 300px;
  overflow: auto;
}

#overflowClipContainer {
  overflow: clip;
  height: 600px;
}

#sticky {
  position: sticky;
  top: 0;
  height: 50px;
  background-color: yellow;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
