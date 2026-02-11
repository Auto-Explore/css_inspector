# css/css-borders/reference/subpixel-borders-with-child-border-box-sizing-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/reference/subpixel-borders-with-child-border-box-sizing-ref.html"
}
```

## style[0]

```css

    .outer {
        border: solid 1px black;
        background: red;
        width: 101px;
        height: 101px;
        margin: 5px;
        padding: 0px;
        box-sizing: border-box;
    }
    .inner {
        background: lightgreen;
        box-sizing: border-box;
        padding: 0px;
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
