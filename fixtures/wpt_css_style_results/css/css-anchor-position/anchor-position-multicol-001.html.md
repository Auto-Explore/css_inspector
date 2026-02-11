# css/css-anchor-position/anchor-position-multicol-001.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-position-multicol-001.html"
}
```

## style[0]

```css

#container {
  position: relative;
  font-family: Ahem;
  font-size: 10px;
  line-height: 1;
  width: 10em;
}
.columns {
  column-width: 100px;
  column-count: 3;
  column-gap: 10px;
  height: 100px;
}
#anchor1 {
  anchor-name: --a1;
  background: blue;
}
.target {
  position: absolute;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
