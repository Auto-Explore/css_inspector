# css/css-borders/subpixel-borders-with-child-border-box-sizing.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/subpixel-borders-with-child-border-box-sizing.html"
}
```

## style[0]

```css

    body {    }
    .outer {
        border: solid .5px black;
        background: red;
        width: 101px;
        height: 101px;
        margin: 5px;
        padding: 0px;
        box-sizing: border-box;
    }
    .inner {
        background: lightgreen;
        padding: 0px;
        box-sizing: border-box;
    }
    #inner1 {
        width: 99px;
        height: 99px;
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
