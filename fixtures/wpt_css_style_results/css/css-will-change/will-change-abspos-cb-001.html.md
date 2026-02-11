# css/css-will-change/will-change-abspos-cb-001.html

```json
{
  "format_version": 3,
  "file": "css/css-will-change/will-change-abspos-cb-001.html"
}
```

## style[0]

```css

  .container {
    border: 1px solid green;
    width: 100px;
    height: 100px;
    margin-top: 100px;
    display: flex;
    will-change: position;
  }
  .abspos {
    position: absolute;
    top: 0;
    left: 0;
    background: orange;
    height: 20px;
    width: 20px;
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
