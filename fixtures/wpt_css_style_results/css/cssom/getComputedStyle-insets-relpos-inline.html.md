# css/cssom/getComputedStyle-insets-relpos-inline.html

```json
{
  "format_version": 3,
  "file": "css/cssom/getComputedStyle-insets-relpos-inline.html"
}
```

## style[0]

```css

.ifc {
  position: relative;
  width: max-content;
  font: 20px/1 Ahem;
  margin-bottom: 2em;
}

.relpos {
  position: relative;
  background: yellow;
  color: yellow;
}

.target {
  position: absolute;
  background: green;
  width: 5em;
  height: 1em;
  top: 1em;
}

.fix-start {
  inset-inline-start: 0;
}

.fix-end {
  inset-inline-end: 0;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
