# css/css-overflow/overflow-clip-margin-border-radius-002.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/overflow-clip-margin-border-radius-002.html"
}
```

## style[0]

```css

div {
  display: inline-block;
  font-size: 10px;
  width: 2em;
  height: 2em;
  padding: 2em;
  border: 2em solid transparent;
  margin: 1em;
  border-radius: 4em;
  overflow: clip;
  vertical-align: middle;
}
div::before {
  content: "";
  display: block;
  height: 18em;
  width: 18em;
  margin: -8em 0 0 -8em;
  background: black;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
