# css/css-contain/contain-layout-ink-overflow-020.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-layout-ink-overflow-020.html"
}
```

## style[0]

```css

  div
    {
      height: 100px;
      width: 100px;
    }

    /* this means that each and all 4 div's use the same definite height and width */

  div#parent-with-overflow-auto
    {
      overflow: auto;
    }

  div#contain
    {
      contain: layout;
    }

  div#pass
    {
      background-color: green;
    }

  div#fail
    {
      background-color: red;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
