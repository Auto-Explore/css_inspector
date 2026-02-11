# css/css-align/abspos/align-self-default-overflow-vrl-rtl-vrl.html

```json
{
  "format_version": 3,
  "file": "css/css-align/abspos/align-self-default-overflow-vrl-rtl-vrl.html"
}
```

## style[0]

```css

body {
  margin: 0;
}

.container {
  writing-mode: vertical-rl;
  direction: rtl;
  display: inline-block;
  position: relative;
  margin: 20px;
  border: solid 4px;
  width: 100px;
  height: 100px;
}

.item {
  writing-mode: vertical-rl;
  direction: ltr;
  position: absolute;
  background: green;
  align-self: center;
}

.inner {
  width: 20px;
  height: 20px;
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
