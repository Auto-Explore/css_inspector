# css/css-anchor-position/position-area-justify-self-safe-001.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-area-justify-self-safe-001.html"
}
```

## style[0]

```css

.container {
  width: 100px;
  height: 50px;
  border: 1px solid gray;
  margin: 10px;
  position: relative;
}

.anchor {
  width: 10px;
  height: 10px;
  top: 30px;
  position: absolute;
  background: blue;
  anchor-name: --a;
}

.anchored, .anchored2 {
  width: 50px;
  height: 20px;
  position: absolute;
  position-area: top;
  position-anchor: --a;
  background: green;
}

.anchored2 {
  margin: 0 10px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
