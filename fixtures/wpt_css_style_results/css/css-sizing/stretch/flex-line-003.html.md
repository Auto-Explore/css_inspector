# css/css-sizing/stretch/flex-line-003.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/stretch/flex-line-003.html"
}
```

## style[0]

```css

  .container {
    display: flex;
    flex-direction: column;
    width: 100px;
    height: 100px;
    border: solid;
    flex-wrap: wrap;
    /* Prevent the lines from stretching. */
    align-content: start;
    font: 20px/1 Ahem;
  }

  .container>div {
    border: 3px solid;
  }

  .stretch {
    width: -moz-available;
    width: -webkit-fill-available;
    width: stretch;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
