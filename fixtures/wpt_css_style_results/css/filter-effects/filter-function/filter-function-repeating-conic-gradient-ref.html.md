# css/filter-effects/filter-function/filter-function-repeating-conic-gradient-ref.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/filter-function/filter-function-repeating-conic-gradient-ref.html"
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
        background-image: repeating-conic-gradient(red, red 5%, black 5%, black 10%);
        filter: invert(1);
    }
    #foreground {
        color: purple;
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
