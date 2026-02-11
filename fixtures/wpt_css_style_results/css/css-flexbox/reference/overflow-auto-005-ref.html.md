# css/css-flexbox/reference/overflow-auto-005-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/reference/overflow-auto-005-ref.html"
}
```

## style[0]

```css

.test-row {
    display: flex;
    margin-bottom: 5px;
}
.test-row > div {
    flex: none;
}

.container {
    margin-right: 5px;
    border: 5px solid lightgreen;
    width: 100px;
}

.flexbox {
    display: block;
    height: 100px;
    width: 100px;
    overflow: auto;
}

.flexbox > div {
    width: 200px;
    height: 200px;
    background: radial-gradient(at right 60%, red, yellow, green);
}

.vertical-rl {
  writing-mode: vertical-rl;
}

```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
