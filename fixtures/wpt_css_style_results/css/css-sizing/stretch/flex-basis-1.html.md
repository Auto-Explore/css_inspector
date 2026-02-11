# css/css-sizing/stretch/flex-basis-1.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/stretch/flex-basis-1.html"
}
```

## style[0]

```css

  .flexbox {
    display: flex;
    width: 100px;
  }

  .item {
    flex: 0 1 stretch;
    background: lime;
  }

  .grandchild {
    width: 30px;
    height: 50px;
  }

  p {
    margin-bottom: 4px;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “flex”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
