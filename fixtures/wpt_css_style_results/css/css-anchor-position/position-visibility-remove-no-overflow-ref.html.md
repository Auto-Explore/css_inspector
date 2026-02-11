# css/css-anchor-position/position-visibility-remove-no-overflow-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-visibility-remove-no-overflow-ref.html"
}
```

## style[0]

```css

  #container {
    position: relative;
    width: 400px;
    height: 150px;
  }

  .anchor {
    width: 100px;
    height: 100px;
    background: orange;
    display: inline-block;
  }

  .target {
    position: absolute;
    width: 100px;
    height: 100px;
    background: green;
    top: 50px;
    left: 0;
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
