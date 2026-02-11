# css/css-flexbox/percentage-heights-004-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/percentage-heights-004-ref.html"
}
```

## style[0]

```css

#outer {
  height: 10em;
  display: inline-block;
  background: tan;
  vertical-align: top;
}

#middle {
  overflow-x: scroll;
  height: 100%;
}

#inner {
  width: 200px;
  height: 100%;
  background: green;
}

#outer2 {
  height: 10em;
  display: inline-block;
  background: tan;
}

#middle2 {
  overflow-x: scroll;
  height: 10em;
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
