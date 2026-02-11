# css/css-text/tab-size/tab-size-inheritance-001.html

```json
{
  "format_version": 3,
  "file": "css/css-text/tab-size/tab-size-inheritance-001.html"
}
```

## style[0]

```css

.test {
  font-family: monospace;
  font-size: 10px;
  tab-size: 5em;
}
.test div {
  white-space: pre;
  font-size: 20px
}
.test span {
  display: inline-block;
  width: 20px;
  height: 20px;
  background: green;
}
.ref {
  z-index: -1;
  margin-left: 50px;
  position: absolute;
  width: 20px;
  height: 20px;
  background: red;

  /* this is to avoid antialiasing effects at the edge */
  box-sizing: border-box;
  border: 2px solid white;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
