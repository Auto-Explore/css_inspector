# css/css-images/gradient/gradient-longer-hue-lch-005-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-images/gradient/gradient-longer-hue-lch-005-ref.html"
}
```

## style[0]

```css

div {
  margin: 20px 0px 20px 50px;
  height: 40px;
  width: 100px;
  position: relative;
  background: linear-gradient(to right in lch,
                              red var(--from),
                              color-mix(in lch longer hue, red, blue),
                              blue var(--to));
}

.swatch {
  --from: -20px;
  --to: 30px;
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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
