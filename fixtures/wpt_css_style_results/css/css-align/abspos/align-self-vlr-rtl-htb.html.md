# css/css-align/abspos/align-self-vlr-rtl-htb.html

```json
{
  "format_version": 3,
  "file": "css/css-align/abspos/align-self-vlr-rtl-htb.html"
}
```

## style[0]

```css

body {
  margin: 0;
}

.container {
  writing-mode: vertical-lr;
  direction: rtl;
  display: inline-block;
  position: relative;
  margin: 20px;
  border: solid 4px;
  width: 40px;
  height: 40px;
}

.item {
  writing-mode: horizontal-tb;
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
