# css/css-grid/child-border-box-and-max-content-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/child-border-box-and-max-content-002.html"
}
```

## style[0]

```css

    .grid {
        display: grid;
        grid-template-columns: fit-content(30px) fit-content(80px);
        border: 1px solid red;
        width: max-content;
    }
    .item {
        max-width: max-content;
        box-sizing: border-box;

        padding: 10px 20px;
        background-color: blue;
    }
    .content {
        width: 50px;
        height: 50px;
        background-color: green;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
