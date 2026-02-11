# css/css-align/abspos/align-self-htb-ltr-vrl.html

```json
{
  "format_version": 3,
  "file": "css/css-align/abspos/align-self-htb-ltr-vrl.html"
}
```

## style[0]

```css

body {
  margin: 0;
}

.container {
  writing-mode: horizontal-tb;
  direction: ltr;
  display: inline-block;
  position: relative;
  margin: 20px;
  border: solid 4px;
  width: 40px;
  height: 40px;
}

.item {
  writing-mode: vertical-rl;
  direction: ltr;
  position: absolute;
  background: green;
  inset: 0;
}

.item::before {
  width: 20px;
  height: 20px;
  content: '';
  display: block;
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
