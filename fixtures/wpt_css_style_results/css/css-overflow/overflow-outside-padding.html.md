# css/css-overflow/overflow-outside-padding.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/overflow-outside-padding.html"
}
```

## style[0]

```css

.container {
  position: relative;
  display: inline-block;
  border: 5px solid rgba(0,0,0,0.5);
  border-width: 0px 0px 50px 80px;
  overflow: auto;
  width: 200px;
  height: 200px;
  background: gray;
}
.target {
  position: absolute;
  width: 1000px;
  height: 1000px;
  background: red;
}
.htb {
  writing-mode: horizontal-tb;
}
.vrl {
  writing-mode: vertical-rl;
}
.vlr {
  writing-mode: vertical-lr;
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
