# css/css-text/hyphens/hyphens-vs-float-clearance-001.html

```json
{
  "format_version": 3,
  "file": "css/css-text/hyphens/hyphens-vs-float-clearance-001.html"
}
```

## style[0]

```css

.outer {
  font: 16px/1.4 monospace;
  width: 6ch;
  border: 1px solid gray;
  float: left;
  margin: 10px;
  hyphens: auto;
}
.float {
  float: right;
  width: 2ch;
  background: lightblue;
}
.h1 {
  height: 1em;
}
.h2 {
  height: 2em;
}
.h3 {
  height: 3em;
}
.h4 {
  height: 4em;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
