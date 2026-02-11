# css/css-align/abspos/justify-self-default-overflow-vrl-ltr-htb.html

```json
{
  "format_version": 3,
  "file": "css/css-align/abspos/justify-self-default-overflow-vrl-ltr-htb.html"
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
  width: 100px;
  height: 100px;
}

.item {
  writing-mode: horizontal-tb;
  direction: ltr;
  position: absolute;
  background: green;
  justify-self: center;
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
