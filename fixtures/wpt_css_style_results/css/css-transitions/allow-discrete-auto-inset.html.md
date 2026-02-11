# css/css-transitions/allow-discrete-auto-inset.html

```json
{
  "format_version": 3,
  "file": "css/css-transitions/allow-discrete-auto-inset.html"
}
```

## style[0]

```css

  #container {
    position: relative;
    width: 100px;
    height: 100px;
    background: green;
  }
  #abs {
    position: absolute;
    transition: top 1000s step-start allow-discrete;
    inset: 0px;
    background: red;
  }
  #abs.auto {
    top: auto;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
