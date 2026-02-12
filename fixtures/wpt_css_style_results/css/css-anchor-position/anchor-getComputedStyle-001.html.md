# css/css-anchor-position/anchor-getComputedStyle-001.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-getComputedStyle-001.html"
}
```

## style[0]

```css

.vrl { writing-mode: vertical-rl; }
.htb { writing-mode: horizontal-tb; }
.ltr { direction: ltr; }
.rtl { direction: rtl; }

.cb {
  transform: scale(1);
  width: 200px;
  height: 150px;
  outline: 1px dashed black;
}

.padding-10 {
  box-sizing: border-box;
  padding: 10px;
}

.anchor {
  width: 40px;
  height: 30px;
  background: orange;
  anchor-name: --a;
  position: relative;
  top: 50px;
  left: 60px;
}

.anchored-topleft {
  position: absolute;
  width: anchor-size(--a width);
  height: anchor-size(--a height);
  bottom: anchor(--a top);
  right: anchor(--a left);
  background: lime;
}

.anchored-bottomright {
  position: absolute;
  width: anchor-size(--a width);
  height: anchor-size(--a height);
  top: anchor(--a bottom);
  left: anchor(--a right);
  background: lime;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
