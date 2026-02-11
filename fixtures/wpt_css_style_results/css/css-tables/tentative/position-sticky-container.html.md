# css/css-tables/tentative/position-sticky-container.html

```json
{
  "format_version": 3,
  "file": "css/css-tables/tentative/position-sticky-container.html"
}
```

## style[0]

```css

  body {
    margin: 0;
  }
  main * {
    box-sizing: border-box;
  }

  main .scroller {
    width: 350px;
    height: 302px;
    overflow-y: scroll;
    outline-offset: -1px;
    outline: gray solid 1px;
  }
  main .padblock {
    width: 300px;
    height: 400px;
    outline-offset: -2px;
    outline: black dotted 2px;
  }
  main table {
    border-spacing: 0;
  }

  main td {
    width: 100px;
    height: 25px;
  }
  .sticky {
    position:sticky;
    top: 0;
    background: rgba(0,255,0, 0.3);
  }

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
