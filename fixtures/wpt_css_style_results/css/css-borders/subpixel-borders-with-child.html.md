# css/css-borders/subpixel-borders-with-child.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/subpixel-borders-with-child.html"
}
```

## style[0]

```css

    .outer {
        border: solid .5px black;
        background: red;
        width: 100px;
        height: 100px;
        margin: 5px;
    }
    .inner {
        background: lightgreen
    }
    #inner1 {
        width: 100px;
        height: 100px;
    }
    #inner2 {
        width: 100%;
        height: 100%;
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
