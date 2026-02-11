# css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-008.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-008.html"
}
```

## style[0]

```css

.border {
  border: 1px solid blue;
  width: max-content;
  margin: 5px;
}

.box {
  background: lightblue;
  contain-intrinsic-size: 55px 66px;
  contain: size;
  border-style: solid;
  border-color: black;
  border-width: 2px 3px 5px 7px;
  padding: 11px 13px 17px 19px;
}
.verticalrl {
  writing-mode: vertical-rl;
}
.verticallr {
  writing-mode: vertical-lr;
}
.horizontaltb {
  writing-mode: horizontal-tb;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
