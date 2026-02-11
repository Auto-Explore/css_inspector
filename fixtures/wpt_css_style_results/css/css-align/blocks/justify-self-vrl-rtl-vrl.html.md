# css/css-align/blocks/justify-self-vrl-rtl-vrl.html

```json
{
  "format_version": 3,
  "file": "css/css-align/blocks/justify-self-vrl-rtl-vrl.html"
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
  width: 40px;
  height: 40px;
}

.item {
  writing-mode: vertical-rl;
  direction: ltr;
  background: green;
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
