# css/css-backgrounds/background-size-019.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-size-019.html"
}
```

## style[0]

```css

        #ref-overlapped-red {
            background-color: red;
            height: 100px;
            width: 100px;
        }
        #test-overlapping-green {
            background-image: url(support/1x1-green.png);
            background-repeat: no-repeat;
            background-size: 50% 100px;
            bottom: 100px;
            height: 100px;
            position: relative;
            width: 200px;
        }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
