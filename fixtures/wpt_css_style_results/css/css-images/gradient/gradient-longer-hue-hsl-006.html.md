# css/css-images/gradient/gradient-longer-hue-hsl-006.html

```json
{
  "format_version": 3,
  "file": "css/css-images/gradient/gradient-longer-hue-hsl-006.html"
}
```

## style[0]

```css

div {
  margin: 20px 0px 20px 50px;
  height: 40px;
  width: 100px;
  position: relative;
  background: linear-gradient(to right in hsl longer hue,
                              red var(--from),
                              blue var(--to));
}

.swatch {
  --from: 50px;
  --to: 100px;
}

.swatch::before {
  content: '\feff';
  position: absolute;
  bottom: 40px;
  height: 10px;
  left: calc(var(--from) - 0.5px);
  border: 0.5px solid red;
}

.swatch::after {
  content: '\feff';
  position: absolute;
  top: 40px;
  height: 10px;
  left: calc(var(--to) - 0.5px);
  border: 0.5px solid blue;
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
