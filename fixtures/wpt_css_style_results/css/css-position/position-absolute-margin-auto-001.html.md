# css/css-position/position-absolute-margin-auto-001.html

```json
{
  "format_version": 3,
  "file": "css/css-position/position-absolute-margin-auto-001.html"
}
```

## style[0]

```css

.cb {
  position: relative;
  width: 400px;
  height: 400px;
  outline: 1px dashed black;
}

.target {
  writing-mode: vertical-lr;
  position: absolute;
  background: green;
  width: calc(100% - 100px);
  height: calc(100% - 100px);
  inset: 100px;
  margin: auto;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
