# css/filter-effects/filter-function/filter-function-linear-gradient-ref.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/filter-function/filter-function-linear-gradient-ref.html"
}
```

## style[0]

```css

    #container {
        position: relative;
        width: 400px;
        height: 400px;
    }
    #container > div {
        position: absolute;
        width: 100%;
        height: 100%;
    }
    #background {
        background-image: linear-gradient(red, orange);
        filter: invert(1);
    }
    #foreground {
        color: purple;
    }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
