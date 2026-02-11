# css/css-images/gradient/gradient-analogous-missing-components-004.html

```json
{
  "format_version": 3,
  "file": "css/css-images/gradient/gradient-analogous-missing-components-004.html"
}
```

## style[0]

```css

        div {
            display: flex;
            align-items: center;
            width: 200px;
            height: 50px;
            background: red;
        }
        .test1 {
            background: linear-gradient(to right in oklab, rgb(none 255 0), rgb(255, 0, 0) );
        }
        .test2 {
            background: linear-gradient(to right in oklab, rgb(none 255 0), rgb(255 none 0) );
        }
        .test3 {
            background: linear-gradient(to right, rgb(none 255 0), rgb(255 none 0) );
        }
        .test4 {
            background: linear-gradient(to right, rgb(none 255 0), rgb(255, 0, 0) );
        }
        .test5 {
            background: linear-gradient(to right in oklab, rgb(0 255 0), rgb(255 0 0) );
        }
    
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
