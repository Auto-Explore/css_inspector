# css/css-flexbox/flex-minimum-height-flex-items-010.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flex-minimum-height-flex-items-010.html"
}
```

## style[0]

```css

.container {
  height: 300px;
  outline: 2px solid black;
}

.inner
{
  width: 400px;
  flex: 1;
  background-color: green;
}
#container2 .flexbox > * { flex-basis: 0; }
#container2 .column > * { flex-basis: auto; }
.container .flexbox { min-height: min-content; }
.container > .flexbox { min-height: 0; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
