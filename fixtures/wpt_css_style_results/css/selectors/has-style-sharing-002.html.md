# css/selectors/has-style-sharing-002.html

```json
{
  "format_version": 3,
  "file": "css/selectors/has-style-sharing-002.html"
}
```

## style[0]

```css

div {
  background: blue;
  padding: 1em;
  margin: 1em;
}

:has(> span) {
  background: green;
}

span {
  display: inline-block;
  width: 1em;
  height: 1em;
  background-color: pink;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
