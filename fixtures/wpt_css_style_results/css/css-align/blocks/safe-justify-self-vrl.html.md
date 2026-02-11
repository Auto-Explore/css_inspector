# css/css-align/blocks/safe-justify-self-vrl.html

```json
{
  "format_version": 3,
  "file": "css/css-align/blocks/safe-justify-self-vrl.html"
}
```

## style[0]

```css

body {
  margin: 0;
}

.container {
  writing-mode: vertical-rl;
  direction: ltr;
  display: inline-block;
  position: relative;
  margin: 15px;
  outline: solid 4px;
  width: 40px;
  height: 40px;
}

.item {
  background: green;
  width: 50px;
  height: 50px;
}

.safe {
  justify-self: safe end;
}
.unsafe {
  justify-self: unsafe end;
}

.rtl {
  direction: rtl;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “justify-self”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “justify-self”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
