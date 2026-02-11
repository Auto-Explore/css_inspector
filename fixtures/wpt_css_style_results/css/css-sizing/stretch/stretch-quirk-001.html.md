# css/css-sizing/stretch/stretch-quirk-001.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/stretch/stretch-quirk-001.html"
}
```

## style[0]

```css

  body {
    margin: 0;
  }
  body > * {
    box-sizing: border-box;
    height: -webkit-fill-available;
    height: stretch;

    /* The rest of these styles are just for cosmetics & consistency: */
    width: 40px;
    border: 5px solid blue;
    vertical-align: top;
    margin: 0 10px 0 0;
    background: cyan;
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
