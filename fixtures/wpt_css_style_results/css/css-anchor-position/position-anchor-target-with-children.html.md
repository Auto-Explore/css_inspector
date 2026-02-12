# css/css-anchor-position/position-anchor-target-with-children.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-anchor-target-with-children.html"
}
```

## style[0]

```css

.anchor {
  width: 100px;
  height: 100px;
  background: orange;
}

.target {
  position: fixed;
  width: 100px;
  height: 100px;
  background: lime;
  top: anchor(bottom);
  left: anchor(left);
}

body {
  margin: 0;
  font: 20px/1 Ahem;
}

#anchor1 {
  anchor-name: --a1;
  margin-left: 100px;
}

#target1 {
  position-anchor: --a1;
}

#anchor2 {
  anchor-name: --a2;
  margin-left: 300px;
  margin-top: 100px;
}

#target2 {
  position-anchor: --a2;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
