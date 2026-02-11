# css/css-backgrounds/reference/background-attachment-local-hidden-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/reference/background-attachment-local-hidden-ref.html"
}
```

## style[0]

```css

  .outer {
    position:relative;
    width: 120px;
    height: 120px;
    border-radius: 40px;
    background-color: rgba(255,0,0,0.5);
  }

  .inner {
    position:absolute;
    top:10px;
    left:10px;
    width: 100px;
    height: 100px;
    border-radius: 30px;
    background-color: lightblue;
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
