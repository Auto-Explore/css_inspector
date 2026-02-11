# css/css-sizing/stretch/flex-line-002.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/stretch/flex-line-002.html"
}
```

## style[0]

```css

  .container {
    /* inline just so they don't overflow on each other */
    display: inline-flex;
    width: 100px;
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
    height: -moz-available;
    height: -webkit-fill-available;
    height: stretch;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
