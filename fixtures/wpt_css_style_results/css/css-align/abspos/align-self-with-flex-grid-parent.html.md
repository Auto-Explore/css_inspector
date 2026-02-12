# css/css-align/abspos/align-self-with-flex-grid-parent.html

```json
{
  "format_version": 3,
  "file": "css/css-align/abspos/align-self-with-flex-grid-parent.html"
}
```

## style[0]

```css

.container {
  position: relative;
  width: 300px;
  height: 300px;
  background: purple;
}

.inner {
  width: 30px;
  height: 30px;
  margin-left: 270px;
}

.flex {
  display: flex;
}

.grid {
  display: grid;
}

.abs {
  position: absolute;
  width: 100px;
  height: 100px;
  inset: 0;
  /* Should align to the center of .container. */
  align-self: center;
  justify-self: center;
  background: pink;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
