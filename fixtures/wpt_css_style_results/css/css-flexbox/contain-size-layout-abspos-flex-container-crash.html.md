# css/css-flexbox/contain-size-layout-abspos-flex-container-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/contain-size-layout-abspos-flex-container-crash.html"
}
```

## style[0]

```css

  html { columns: 0 }
  body {
    transform: rotate(14deg);
  }
  #flex-container {
    display: flex;
    contain: size layout;
    position: absolute;
    width: 100px;
    height: 100px;
  }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “contain”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
