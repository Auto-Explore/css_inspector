# css/css-flexbox/overflow-auto-003.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/overflow-auto-003.html"
}
```

## style[0]

```css

html, body {
  margin: 0;
  padding: 0;
}

.flexbox {
  display: flex;
  background: papayawhip;
  width: 800px;
}

.flexbox > div {
  width: 300px;
  height: 300px;
  margin: 0 auto;
  background: olive;
}

.flexbox > div > div {
  overflow-y: auto;
  height: 50px;
  visibility: hidden;
}

.elm {
  height: 10px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
