# css/css-flexbox/percentage-heights-004.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/percentage-heights-004.html"
}
```

## style[0]

```css

#outer {
  height: 10em;
  display: inline-flex;
  background: tan;
}

#middle {
  overflow-x: scroll;
}

#inner {
  width: 200px;
  height: 100%;
  background: green;
}

#outer2 {
  height: 10em;
  display: inline-flex;
  flex-direction: column;
  background: tan;
}

#middle2 {
  overflow-x: scroll;
  flex: 1.0;
}

#inner2 {
  width: 200px;
  height: 100%;
  background: green;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
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
