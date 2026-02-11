# css/css-grid/reference/grid-different-gutters-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/reference/grid-different-gutters-ref.html"
}
```

## style[0]

```css

    #grid {
        width:200px;
        height: 220px;
        background-color: green;
        position: relative;
    }

    #grid > div {
        background-color: silver;
        width: 90px;
        height: 90px;
        position: absolute;
    }

    #grid :nth-child(1) {
        top: 0;
        left: 0;
    }

    #grid :nth-child(2) {
        top: 0;
        left: 110px;
    }

    #grid :nth-child(3) {
        top: 130px;
        left: 0;
    }

    #grid :nth-child(4) {
        top: 130px;
        left: 110px;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
