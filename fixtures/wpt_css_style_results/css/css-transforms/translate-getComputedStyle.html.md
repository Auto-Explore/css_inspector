# css/css-transforms/translate-getComputedStyle.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/translate-getComputedStyle.html"
}
```

## style[0]

```css

    #container {
      transform-style: preserve-3d;;
    }
    #first {
      font-size: 10px;
      translate: 10px 2em;
    }
    #second {
      translate: 30% 40% 50px;
    }
    #third {
      font-size: 10px;
      width: 98px;
      height: 76px;
      translate: calc(7em + 80%) -9em;
    }
  
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “translate”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “translate”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “translate”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
