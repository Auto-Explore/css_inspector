# css/css-writing-modes/forms/range-input-vertical-rtl-painting.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/forms/range-input-vertical-rtl-painting.html"
}
```

## style[0]

```css

    #container {
        position: relative;
    }
    #cover {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background-color: red;
    }
    @supports (writing-mode: vertical-lr) and (direction: rtl) {
        #cover {
            background-color: Canvas;
        }
    }
    input {
        appearance: none;
        writing-mode: vertical-lr;
        direction: rtl;
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
