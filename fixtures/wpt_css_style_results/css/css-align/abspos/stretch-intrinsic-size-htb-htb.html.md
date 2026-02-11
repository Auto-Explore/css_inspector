# css/css-align/abspos/stretch-intrinsic-size-htb-htb.html

```json
{
  "format_version": 3,
  "file": "css/css-align/abspos/stretch-intrinsic-size-htb-htb.html"
}
```

## style[0]

```css

body {
  margin: 0;
}

.container {
  writing-mode: horizontal-tb;
  display: inline-block;
  position: relative;
  margin: 20px;
  border: solid 4px;
  width: 60px;
  height: 60px;
}

.item {
  writing-mode: horizontal-tb;
  position: absolute;
  background: green;
  inset: 5px 10px 5px 10px;
}

.child::before {
  aspect-ratio: 1/1;
  min-width: 20px;
  min-height: 20px;
  width: 100%;
  height: 100%;
  content: '';
  display: block;
}

.ar {
  aspect-ratio: 1/1;
  min-width: 20px;
  min-height: 20px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “inset”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
