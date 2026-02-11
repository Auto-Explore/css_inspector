# css/css-conditional/container-queries/resize-while-content-visibility-hidden.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/resize-while-content-visibility-hidden.html"
}
```

## style[0]

```css

#container {
  container-name: container;
  container-type: size;
  width: 300px;
  height: 300px;
}

#child {
  width: 200px;
  height: 200px;
  background: red;
}

#container.wide { width: 500px; }
.locked { content-visibility: hidden; }

@container container (min-width: 400px) { #child { background: green; } }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
