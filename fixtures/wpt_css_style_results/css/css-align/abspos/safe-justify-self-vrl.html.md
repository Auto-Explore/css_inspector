# css/css-align/abspos/safe-justify-self-vrl.html

```json
{
  "format_version": 3,
  "file": "css/css-align/abspos/safe-justify-self-vrl.html"
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
  margin: 20px;
  border: solid 4px;
  width: 40px;
  height: 40px;
}

.item {
  position: absolute;
  background: green;
  inset: 5px;
  margin: 10px;
  width: 30px;
  height: 30px;
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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
