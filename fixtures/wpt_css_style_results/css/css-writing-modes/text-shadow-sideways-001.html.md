# css/css-writing-modes/text-shadow-sideways-001.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/text-shadow-sideways-001.html"
}
```

## style[0]

```css

body > div {
  /* styling */
  border: solid gray;
  font: 30px/1 Ahem;
  color: silver;
  float: left;
  margin: 8px;

  /* test */
  text-decoration: underline;
  text-decoration-color: orange;
  text-decoration-thickness: 4px;
  text-shadow: 2px 4px aqua;
}
.lr {
  writing-mode: sideways-lr;
}
.rl {
  writing-mode: sideways-rl;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “text-shadow”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
