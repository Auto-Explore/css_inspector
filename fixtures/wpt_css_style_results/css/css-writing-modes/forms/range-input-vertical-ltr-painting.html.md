# css/css-writing-modes/forms/range-input-vertical-ltr-painting.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/forms/range-input-vertical-ltr-painting.html"
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
    @supports (writing-mode: vertical-lr) {
        #cover {
            background-color: Canvas;
        }
    }
    input {
        appearance: none;
        writing-mode: vertical-lr;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
