# css/css-position/position-absolute-fit-content-auto-margin.html

```json
{
  "format_version": 3,
  "file": "css/css-position/position-absolute-fit-content-auto-margin.html"
}
```

## style[0]

```css

.container {
  width: 40px;
  height: 40px;
  border: 1px solid;
  position: relative;
}

.abspos {
  inset: 0;
  margin: auto;
  background: orange;
  position: absolute;
}

:is(.vrl, .vlr) .abspos {
  writing-mode: horizontal-tb;
}

.vrl {
  writing-mode: vertical-rl;
}

.vlr {
  writing-mode: vertical-lr;
}

.content {
  width: 20px;
  height: 20px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
