# css/css-anchor-position/anchor-center-vrl-htb.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-center-vrl-htb.html"
}
```

## style[0]

```css

.container {
  writing-mode: vertical-rl;
  width: 100px;
  height: 100px;
  border: solid 3px;
  position: relative;
  margin: 50px;
}

.anchor {
  anchor-name: --anchor;
  position: relative;
  width: 50px;
  height: 50px;
  right: 10px;
  top: 40px;
  background: lime;
}

.target {
  writing-mode: horizontal-tb;
  position-anchor: --anchor;
  position: absolute;
  background: cyan;
  align-self: anchor-center;
  top: anchor(bottom);
  height: 20px;
}
/* used to test the available-size given to the element */
.target::after {
  color: transparent;
  content: 'a a a a a a a a a a a a a a a a a a';
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
