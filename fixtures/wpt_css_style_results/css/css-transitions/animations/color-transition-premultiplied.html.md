# css/css-transitions/animations/color-transition-premultiplied.html

```json
{
  "format_version": 3,
  "file": "css/css-transitions/animations/color-transition-premultiplied.html"
}
```

## style[0]

```css

    .box {
      width: 100px;
      height: 100px;
      margin: 10px;
      border: 1px solid black;
      transition: background-color 1s linear;
    }

    #one {
      background-color: transparent;
    }

    #one.changed {
      background-color: green;
    }

    #two {
      background-color: rgba(0, 255, 0, 0);
    }

    #two.changed {
      background-color: rgba(0, 0, 255, 1);
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
